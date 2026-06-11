use std::process::Command;

pub async fn detect_current_song() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        // Tenta playerctl primeiro
        if let Ok(output) = Command::new("playerctl")
            .args(&["-p", "spotify", "metadata", "--format", "{{ artist }} - {{ title }}"])
            .output()
        {
            if output.status.success() {
                let song = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !song.is_empty() {
                    return Some(song);
                }
            }
        }
        
        // Fallback para dbus-send
        if let Ok(output) = Command::new("dbus-send")
            .args(&[
                "--print-reply",
                "--dest=org.mpris.MediaPlayer2.spotify",
                "/org/mpris/MediaPlayer2",
                "org.freedesktop.DBus.Properties.Get",
                "string:org.mpris.MediaPlayer2.Player",
                "string:Metadata"
            ])
            .output()
        {
            if output.status.success() {
                let reply = String::from_utf8_lossy(&output.stdout);
                let mut title = String::new();
                let mut artist = String::new();
                let lines: Vec<&str> = reply.lines().map(|l| l.trim()).collect();
                for i in 0..lines.len() {
                    if lines[i].contains("xesam:title") && i + 1 < lines.len() {
                        if let Some(val_line) = lines.get(i + 2) {
                            if let Some(start) = val_line.find('"') {
                                if let Some(end) = val_line[start + 1..].find('"') {
                                    title = val_line[start + 1..start + 1 + end].to_string();
                                }
                            }
                        }
                    }
                    if lines[i].contains("xesam:artist") && i + 1 < lines.len() {
                        if let Some(val_line) = lines.get(i + 3) {
                            if let Some(start) = val_line.find('"') {
                                if let Some(end) = val_line[start + 1..].find('"') {
                                    artist = val_line[start + 1..start + 1 + end].to_string();
                                }
                            }
                        }
                    }
                }
                if !title.is_empty() {
                    if !artist.is_empty() {
                        return Some(format!("{} - {}", artist, title));
                    } else {
                        return Some(title);
                    }
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Executa o PowerShell para buscar o título da janela principal do processo Spotify
        if let Ok(output) = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-Command",
                "Get-Process spotify -ErrorAction SilentlyContinue | Where-Object {$_.MainWindowTitle} | Select-Object -ExpandProperty MainWindowTitle -First 1"
            ])
            .output()
        {
            if output.status.success() {
                let window_title = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !window_title.is_empty() && window_title != "Spotify" && window_title != "Spotify Premium" && window_title != "Spotify Free" {
                    return Some(window_title);
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Executa AppleScript para ler metadados do player do Spotify local
        if let Ok(output) = Command::new("osascript")
            .args(&[
                "-e",
                "if application \"Spotify\" is running then tell application \"Spotify\" to get artist of current track & \" - \" & name of current track"
            ])
            .output()
        {
            if output.status.success() {
                let song = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !song.is_empty() {
                    return Some(song);
                }
            }
        }
    }

    #[cfg(target_os = "android")]
    {
        // O Android não permite monitoramento direto de processos de terceiros via terminal/IPC sem permissões especiais
        // Retornamos None para manter a consistência estrutural multiplataforma compilando sem erros
        let _ = ();
    }

    None
}
