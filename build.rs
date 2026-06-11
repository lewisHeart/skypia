use std::fs;
use std::path::{Path, PathBuf};

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

fn main() {
    // Registra dependencia para re-executar caso o MainActivity mude
    println!("cargo:rerun-if-changed=src/MainActivity.kt");

    let source = Path::new("src/MainActivity.kt");
    if source.exists() {
        // Tenta copiar para os diretorios de build do Dioxus CLI
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
                if let Err(e) = fs::copy(source, &target_main_activity) {
                    println!("cargo:warning=Falha ao copiar MainActivity.kt para {:?}: {:?}", target_main_activity, e);
                } else {
                    println!("cargo:warning=MainActivity.kt copiado com sucesso para {:?}", target_main_activity);
                }

                // 2. Corrige Logger.kt adicionando o import do BuildConfig ausente
                let target_logger = target_dir.join("src/main/kotlin/dev/dioxus/main/Logger.kt");
                if target_logger.exists() {
                    if let Ok(content) = fs::read_to_string(&target_logger) {
                        if !content.contains("import app.skypia.messenger.BuildConfig") {
                            let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
                            // Acha a linha do pacote
                            if let Some(pos) = lines.iter().position(|l| l.trim().starts_with("package ")) {
                                lines.insert(pos + 1, "import app.skypia.messenger.BuildConfig".to_string());
                                let new_content = lines.join("\n");
                                if let Err(e) = fs::write(&target_logger, new_content) {
                                    println!("cargo:warning=Falha ao gravar correcao em Logger.kt: {:?}", e);
                                } else {
                                    println!("cargo:warning=Logger.kt corrigido com sucesso com import BuildConfig!");
                                }
                            }
                        }
                    }
                }

                // 3. Corrige network_security_config.xml para permitir Cleartext Traffic globalmente
                let target_net_config = target_dir.join("src/main/res/xml/network_security_config.xml");
                if target_net_config.exists() {
                    let new_net_config = r#"<?xml version="1.0" encoding="utf-8"?>
<network-security-config>
    <base-config cleartextTrafficPermitted="true" />
</network-security-config>
"#;
                    if let Err(e) = fs::write(&target_net_config, new_net_config) {
                        println!("cargo:warning=Falha ao gravar network_security_config.xml: {:?}", e);
                    } else {
                        println!("cargo:warning=network_security_config.xml corrigido para cleartext global!");
                    }
                }

                // 4. Copia as pastas de emojis locais para os assets do APK do Android
                let target_assets = target_dir.join("src/main/assets/");
                if target_assets.exists() {
                    let _ = copy_dir_all("assets/emojis", target_assets.join("emojis"));
                    let _ = copy_dir_all("assets/emojis_anim", target_assets.join("emojis_anim"));
                    println!("cargo:warning=Emojis copiados com sucesso para os assets nativos do Android!");
                }
            }
        }
    }
}
