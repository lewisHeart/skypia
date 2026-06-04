use crate::models::render_avatar;
use crate::services::api;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn AvatarPicker(mut state: AppState) -> Element {
    let is_uploading = use_signal(|| false);
    let upload_error = use_signal(|| Option::<String>::None);

    rsx! {
        div {
            class: "fixed inset-0 bg-black/55 backdrop-blur-sm z-[9999] flex items-center justify-center p-4 pointer-events-auto",
            onclick: move |_| state.show_avatar_picker.set(false),

            div {
                class: "w-[400px] bg-gradient-to-b from-[#e6f1fc] to-[#c8def5] border border-[#7ba9d4] rounded-2xl shadow-2xl flex flex-col overflow-hidden pointer-events-auto",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "px-4 py-3 flex items-center justify-between border-b border-white/40 bg-white/20",
                    div { class: "flex items-center space-x-2",
                        span { class: "text-lg", "🖼️" }
                        div {
                            h2 { class: "font-bold text-sm text-[#1b324d]", "Escolher avatar" }
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

                    // Seção 1: Avatares SVG embutidos
                    div { class: "flex flex-col space-y-2",
                        p { class: "text-xs font-bold text-[#1b324d]", "Avatares do Skypia" }
                        div { class: "grid grid-cols-4 gap-2",
                            for avatar_id in 0usize..=6 {
                                {
                                    let is_selected = state.user_avatar_id() == avatar_id;
                                    rsx! {
                                        button {
                                            class: if is_selected {
                                                "relative p-1 rounded-xl border-2 border-sky-400 bg-sky-100/50 cursor-pointer transition-all shadow-md hover:scale-105"
                                            } else {
                                                "relative p-1 rounded-xl border-2 border-transparent hover:border-sky-300/60 bg-white/30 cursor-pointer transition-all hover:scale-105"
                                            },
                                            onclick: move |_| {
                                                state.set_user_avatar(avatar_id);
                                                // Limpa avatar URL do servidor ao escolher SVG built-in
                                                *state.user_avatar_url.write() = None;
                                                state.show_avatar_picker.set(false);

                                                // Sincroniza com servidor se autenticado
                                                if let Some(token) = state.auth_token() {
                                                    spawn(async move {
                                                        let _ = api::update_profile(&token, api::UpdateProfileRequest {
                                                            display_name: None,
                                                            personal_message: None,
                                                            status: None,
                                                            music: None,
                                                        }).await;
                                                    });
                                                }
                                            },
                                            {render_avatar(avatar_id, 56)}
                                            if is_selected {
                                                div { class: "absolute -top-1 -right-1 w-4 h-4 bg-sky-500 rounded-full flex items-center justify-center shadow",
                                                    span { class: "text-white text-[8px] font-bold", "✓" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Divisor
                    div { class: "flex items-center space-x-2",
                        div { class: "flex-1 h-px bg-[#7ba9d4]/30" }
                        span { class: "text-[10px] text-slate-500", "ou" }
                        div { class: "flex-1 h-px bg-[#7ba9d4]/30" }
                    }

                    // Seção 2: Upload de foto real
                    div { class: "flex flex-col space-y-2",
                        p { class: "text-xs font-bold text-[#1b324d]", "Enviar foto própria" }

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
                            if let Some(url) = state.user_avatar_url() {
                                div { class: "flex items-center space-x-3 p-2 bg-white/40 rounded-xl border border-white/50",
                                    img {
                                        src: "{url}",
                                        class: "w-12 h-12 rounded-lg object-cover border border-white/60 shadow",
                                        alt: "Avatar atual"
                                    }
                                    div {
                                        p { class: "text-xs font-semibold text-[#1b324d]", "Foto atual" }
                                        p { class: "text-[10px] text-slate-500", "Envie uma nova para substituir" }
                                    }
                                }
                            }

                            // Botão de upload via sistema de arquivos nativo
                            div { class: "flex flex-col items-center p-4 border-2 border-dashed border-[#7ba9d4]/50 rounded-xl bg-white/20 hover:bg-white/40 transition-all cursor-pointer group",
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
                                p { class: "text-xs font-semibold text-[#1b324d]", "Clique para selecionar uma foto" }
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
