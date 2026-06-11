use std::process::Command;

pub async fn detect_current_song() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        // Verifica se está tocando
        let is_playing = if let Ok(status_out) = Command::new("playerctl")
            .args(&["-p", "spotify", "status"])
            .output()
        {
            String::from_utf8_lossy(&status_out.stdout).trim() == "Playing"
        } else {
            false
        };

        if is_playing {
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
                "if application \"Spotify\" is running then tell application \"Spotify\" to if player state is playing then get artist of current track & \" - \" & name of current track"
            ])
            .output()
        {
            if output.status.success() {
                let song = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !song.is_empty() && song != "missing value" {
                    return Some(song);
                }
            }
        }
    }

    #[cfg(target_os = "android")]
    {
        use jni::objects::{JObject, JValue};
        use jni::JavaVM;
        
        if let Ok(song_opt) = (|| -> Result<Option<String>, String> {
            let ctx = ndk_context::android_context();
            let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }
                .map_err(|e| format!("Failed to get JavaVM: {:?}", e))?;
            
            let mut env = vm.attach_current_thread()
                .map_err(|e| format!("Failed to attach thread: {:?}", e))?;
            
            env.with_local_frame(16, |local_env| {
                let context_obj = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };
                
                let pref_name = local_env.new_string("spotify_pref")?;
                
                let shared_pref = local_env.call_method(
                    &context_obj,
                    "getSharedPreferences",
                    "(Ljava/lang/String;I)Landroid/content/SharedPreferences;",
                    &[JValue::Object(&pref_name), JValue::Int(0)]
                )?.l()?;
                
                let is_playing_key = local_env.new_string("is_playing")?;
                let is_playing = local_env.call_method(
                    &shared_pref,
                    "getBoolean",
                    "(Ljava/lang/String;Z)Z",
                    &[JValue::Object(&is_playing_key), JValue::Bool(0)]
                )?.z()?;
                
                if !is_playing {
                    return Ok(None);
                }
                
                let song_key = local_env.new_string("current_song")?;
                let current_song = local_env.call_method(
                    &shared_pref,
                    "getString",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    &[
                        JValue::Object(&song_key),
                        JValue::Object(&JObject::null())
                    ]
                )?.l()?;
                
                if current_song.is_null() {
                    return Ok(None);
                }
                
                let song_jstr: jni::objects::JString = current_song.into();
                let song_str: String = local_env.get_string(&song_jstr)?.into();
                
                if song_str.trim().is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(song_str))
                }
            }).map_err(|e: jni::errors::Error| format!("JNI local frame error: {:?}", e))
        })() {
            if let Some(song) = song_opt {
                return Some(song);
            }
        }
    }

    None
}
