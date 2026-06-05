use crate::services::api;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn AvatarPicker(mut state: AppState) -> Element {
    let theme = state.theme();
    let is_uploading = use_signal(|| false);
    let upload_error = use_signal(|| Option::<String>::None);

    rsx! {
        div {
            class: "fixed inset-0 bg-black/55 backdrop-blur-sm z-[9999] flex items-center justify-center p-4 pointer-events-auto",
            onclick: move |_| state.show_avatar_picker.set(false),

            div {
                class: "w-[400px] bg-gradient-to-b {theme.modal_gradient()} border {theme.modal_border()} rounded-2xl shadow-2xl flex flex-col overflow-hidden pointer-events-auto",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "px-4 py-3 flex items-center justify-between border-b {theme.titlebar_border()} bg-white/20",
                    div { class: "flex items-center space-x-2",
                        span { class: "text-lg", "🖼️" }
                        div {
                            h2 { class: "font-bold text-sm {theme.titlebar_text()}", "Escolher avatar" }
                            p { class: "text-[10px] text-slate-500", "Selecione um avatar ou envie uma foto" }
                        }
                    }
                    button {
                        class: "w-6 h-6 flex items-center justify-center rounded-lg hover:bg-red-500 hover:text-white text-slate-500 font-bold cursor-pointer transition-all text-sm",
                        onclick: move |_| state.show_avatar_picker.set(false),
                        "✕"
                    }
                }

                // Abas (apenas visual, sem lógica de tab aqui pois é simples)
                div { class: "p-4 flex flex-col space-y-4",

                    // Erro de upload
                    if let Some(err) = upload_error() {
                        div { class: "px-3 py-2 bg-red-50 border border-red-200 rounded-lg text-xs text-red-700 flex items-center space-x-2",
                            span { "⚠️" }
                            span { "{err}" }
                        }
                    }

                    // Seção 1: GIFs Clássicos do MSN
                    div { class: "flex flex-col space-y-2 border-b border-white/20 pb-4",
                        p { class: "text-xs font-bold {theme.titlebar_text()}", "Escolha uma imagem predefinida" }
                        div { class: "grid grid-cols-4 gap-2",
                            for &(name, rel_path, mime) in &[
                                ("Margarida", "daisy.png", "image/png"),
                                ("Cachorrinho", "dog.png", "image/png"),
                                ("Gatinho", "kitten.png", "image/png"),
                                ("Robozinho", "robot.png", "image/png"),
                                ("Futebol", "soccer.gif", "image/gif"),
                                ("Sol", "summer.gif", "image/gif"),
                                ("Flores", "spring.gif", "image/gif"),
                                ("Outono", "fall.gif", "image/gif"),
                            ] {
                                button {
                                    class: "relative aspect-square rounded-lg border {theme.titlebar_border()} bg-white/60 p-1 hover:border-[#5c98d6] hover:bg-white transition-all cursor-pointer flex flex-col items-center justify-center group overflow-hidden shadow-sm",
                                    disabled: is_uploading(),
                                    onclick: move |_| {
                                        let mut state = state;
                                        let mut uploading = is_uploading;
                                        let mut err_sig = upload_error;

                                        // 1. Atualização otimista local instantânea na tela usando a URL do asset
                                        let local_src = match rel_path {
                                            "daisy.png" => asset!("/assets/usertiles/daisy.png").to_string(),
                                            "dog.png" => asset!("/assets/usertiles/dog.png").to_string(),
                                            "kitten.png" => asset!("/assets/usertiles/kitten.png").to_string(),
                                            "robot.png" => asset!("/assets/usertiles/robot.png").to_string(),
                                            "soccer.gif" => asset!("/assets/usertiles/soccer.gif").to_string(),
                                            "summer.gif" => asset!("/assets/usertiles/summer.gif").to_string(),
                                            "spring.gif" => asset!("/assets/usertiles/spring.gif").to_string(),
                                            "fall.gif" => asset!("/assets/usertiles/fall.gif").to_string(),
                                            _ => "".to_string(),
                                        };
                                        *state.user_avatar_url.write() = Some(local_src);

                                        // 2. Se estiver autenticado, faz o upload dos bytes locais em background
                                        if let Some(token) = state.auth_token() {
                                            spawn(async move {
                                                uploading.set(true);
                                                err_sig.set(None);

                                                let filepath = format!("assets/usertiles/{}", rel_path);
                                                match std::fs::read(&filepath) {
                                                    Ok(bytes_vec) => {
                                                        match api::upload_avatar(&token, bytes_vec, mime).await {
                                                            Ok(uploaded_url) => {
                                                                *state.user_avatar_url.write() = Some(uploaded_url);
                                                                uploading.set(false);
                                                                state.show_avatar_picker.set(false);
                                                            }
                                                            Err(e) => {
                                                                uploading.set(false);
                                                                err_sig.set(Some(format!("Falha no upload: {}", e)));
                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        uploading.set(false);
                                                        err_sig.set(Some(format!("Erro ao ler arquivo: {}", e)));
                                                    }
                                                }
                                            });
                                        } else {
                                            state.show_avatar_picker.set(false);
                                        }
                                    },
                                    img {
                                        src: match rel_path {
                                            "daisy.png" => asset!("/assets/usertiles/daisy.png").to_string(),
                                            "dog.png" => asset!("/assets/usertiles/dog.png").to_string(),
                                            "kitten.png" => asset!("/assets/usertiles/kitten.png").to_string(),
                                            "robot.png" => asset!("/assets/usertiles/robot.png").to_string(),
                                            "soccer.gif" => asset!("/assets/usertiles/soccer.gif").to_string(),
                                            "summer.gif" => asset!("/assets/usertiles/summer.gif").to_string(),
                                            "spring.gif" => asset!("/assets/usertiles/spring.gif").to_string(),
                                            "fall.gif" => asset!("/assets/usertiles/fall.gif").to_string(),
                                            _ => "".to_string(),
                                        },
                                        class: "w-full h-full object-cover rounded-md group-hover:scale-105 transition-transform",
                                        alt: name
                                    }
                                }
                            }
                        }
                    }

                    // Seção 2: Upload de foto real
                    div { class: "flex flex-col space-y-2",
                        p { class: "text-xs font-bold {theme.titlebar_text()}", "Enviar foto própria" }

                        if state.auth_token().is_none() {
                            div { class: "px-3 py-2 bg-amber-50 border border-amber-200 rounded-lg text-xs text-amber-700",
                                "⚠️ Faça login para enviar uma foto personalizada."
                            }
                        } else if is_uploading() {
                            div { class: "flex items-center justify-center py-4 space-x-2",
                                div { class: "w-4 h-4 border-2 border-sky-500 border-t-transparent rounded-full animate-spin" }
                                span { class: "text-xs text-slate-500", "Enviando foto..." }
                            }
                        } else {
                            // Preview do avatar atual (se for URL)
                            if state.user_avatar_url().is_some() {
                                div { class: "flex items-center space-x-3 p-2 bg-white/40 rounded-xl border border-white/50",
                                    div {
                                        class: "w-12 h-12 rounded-lg overflow-hidden border border-white/60 shadow flex-shrink-0 flex items-center justify-center bg-white",
                                        {crate::models::render_avatar(state.user_avatar_url().as_deref(), 48)}
                                    }
                                    div {
                                        p { class: "text-xs font-semibold {theme.titlebar_text()}", "Foto atual" }
                                        p { class: "text-[10px] text-slate-500", "Envie uma nova para substituir" }
                                    }
                                }
                            }

                            // Botão de upload via sistema de arquivos nativo
                            div { class: "flex flex-col items-center p-4 border-2 border-dashed {theme.modal_border()}/50 rounded-xl bg-white/20 hover:bg-white/40 transition-all cursor-pointer group",
                                onclick: move |_| {
                                    let mut state = state;
                                    let mut uploading = is_uploading;
                                    let mut err_sig = upload_error;

                                    spawn(async move {
                                        #[cfg(feature = "desktop")]
                                        {
                                            // Abre o diálogo de arquivo nativo
                                            let file_dialog = rfd::FileDialog::new()
                                                .add_filter("Imagens", &["jpg", "jpeg", "png", "gif", "webp"])
                                                .set_title("Escolher foto de perfil");

                                            if let Some(path) = file_dialog.pick_file() {
                                                uploading.set(true);
                                                err_sig.set(None);

                                                match std::fs::read(&path) {
                                                    Ok(bytes) => {
                                                        let mime = detect_mime(&path);
                                                        if let Some(token) = state.auth_token() {
                                                            match api::upload_avatar(&token, bytes, &mime).await {
                                                                Ok(url) => {
                                                                    *state.user_avatar_url.write() = Some(url);
                                                                    uploading.set(false);
                                                                    state.show_avatar_picker.set(false);
                                                                }
                                                                Err(e) => {
                                                                    uploading.set(false);
                                                                    err_sig.set(Some(e));
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        uploading.set(false);
                                                        err_sig.set(Some(format!("Erro ao ler arquivo: {}", e)));
                                                    }
                                                }
                                            }
                                        }
                                        #[cfg(not(feature = "desktop"))]
                                        {
                                            err_sig.set(Some("Upload disponível apenas na versão desktop.".to_string()));
                                        }
                                    });
                                },
                                span { class: "text-2xl mb-1 group-hover:scale-110 transition-transform", "📷" }
                                p { class: "text-xs font-semibold {theme.titlebar_text()}", "Clique para selecionar uma foto" }
                                p { class: "text-[10px] text-slate-500", "JPG, PNG, GIF ou WebP • máx 5MB" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "desktop")]
fn detect_mime(path: &std::path::Path) -> String {
    match path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png".to_string(),
        Some("gif") => "image/gif".to_string(),
        Some("webp") => "image/webp".to_string(),
        _ => "image/jpeg".to_string(),
    }
}
