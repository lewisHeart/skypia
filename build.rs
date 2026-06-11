use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn apply_corrections() {
    let source = Path::new("src/MainActivity.kt");
    if !source.exists() {
        return;
    }

    let target_dirs = vec![
        "target/dx/skypia/debug/android/app/app/",
        "target/dx/skypia/release/android/app/app/",
    ];

    for target_dir_str in target_dirs {
        let target_dir = Path::new(target_dir_str);
        if target_dir.exists() {
            // 1. Copia MainActivity.kt
            let target_main_activity = target_dir.join("src/main/kotlin/dev/dioxus/main/MainActivity.kt");
            if let Some(parent) = target_main_activity.parent() {
                let _ = fs::create_dir_all(parent);
            }
            
            // Verifica se precisa copiar (se nao existe ou se o conteudo esta diferente da nossa)
            let need_copy = match fs::read_to_string(&target_main_activity) {
                Ok(content) => !content.contains("spotifyReceiver"),
                Err(_) => true,
            };

            if need_copy {
                let _ = fs::copy(source, &target_main_activity);
            }

            // 1.5. Corrige AndroidManifest.xml para injetar android:usesCleartextTraffic="true"
            let target_manifest = target_dir.join("src/main/AndroidManifest.xml");
            if target_manifest.exists() {
                if let Ok(content) = fs::read_to_string(&target_manifest) {
                    if !content.contains("android:usesCleartextTraffic=\"true\"") {
                        let new_content = content.replace(
                            "<application ",
                            "<application android:usesCleartextTraffic=\"true\" "
                        );
                        let _ = fs::write(&target_manifest, new_content);
                    }
                }
            }

            // 2. Corrige Logger.kt adicionando o import do BuildConfig ausente
            let target_logger = target_dir.join("src/main/kotlin/dev/dioxus/main/Logger.kt");
            if target_logger.exists() {
                if let Ok(content) = fs::read_to_string(&target_logger) {
                    if !content.contains("import app.skypia.messenger.BuildConfig") {
                        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
                        if let Some(pos) = lines.iter().position(|l| l.trim().starts_with("package ")) {
                            lines.insert(pos + 1, "import app.skypia.messenger.BuildConfig".to_string());
                            let new_content = lines.join("\n");
                            let _ = fs::write(&target_logger, new_content);
                        }
                    }
                }
            }

            // 3. Corrige network_security_config.xml para permitir Cleartext Traffic globalmente
            let target_net_config = target_dir.join("src/main/res/xml/network_security_config.xml");
            let new_net_config = r#"<?xml version="1.0" encoding="utf-8"?>
<network-security-config>
    <base-config cleartextTrafficPermitted="true" />
</network-security-config>
"#;
            let need_write_net = match fs::read_to_string(&target_net_config) {
                Ok(content) => !content.contains("base-config"),
                Err(_) => true,
            };
            if need_write_net {
                if let Some(parent) = target_net_config.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let _ = fs::write(&target_net_config, new_net_config);
            }

            // 4. Copia as pastas de emojis locais para os assets do APK do Android
            let target_assets = target_dir.join("src/main/assets/");
            if target_assets.exists() {
                let _ = copy_dir_all("assets/emojis", target_assets.join("emojis"));
                let _ = copy_dir_all("assets/emojis_anim", target_assets.join("emojis_anim"));
            }
        }
    }
}

fn main() {
    // Registra dependencia para re-executar caso o MainActivity ou o .env mudem
    println!("cargo:rerun-if-changed=src/MainActivity.kt");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.env");

    // Se o .env mudou, força a recompilação de api.rs para atualizar o SERVER_BASE_URL no APK
    let api_rs = Path::new("src/services/api.rs");
    if api_rs.exists() {
        if let Ok(content) = fs::read_to_string(api_rs) {
            let _ = fs::write(api_rs, content);
        }
    }

    // Aplica imediatamente
    apply_corrections();

    // Spawna uma thread em background para continuar monitorando e aplicando as correções
    // enquanto o Dioxus CLI gera o esqueleto gradle e roda o Gradle Build (aumentado para 10 minutos)
    thread::spawn(move || {
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(600) {
            apply_corrections();
            thread::sleep(Duration::from_millis(200));
        }
    });
}
