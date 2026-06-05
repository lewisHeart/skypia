use crate::services::api;
use crate::state::AppState;
use dioxus::prelude::*;

// Avatares predefinidos embutidos em tempo de compilação (funciona em qualquer plataforma)
static PRESET_AVATARS: &[(&str, &str, &[u8])] = &[
    (
        "Margarida",
        "image/png",
        include_bytes!("../../../assets/usertiles/daisy.png"),
    ),
    (
        "Cachorrinho",
        "image/png",
        include_bytes!("../../../assets/usertiles/dog.png"),
    ),
    (
        "Gatinho",
        "image/png",
        include_bytes!("../../../assets/usertiles/kitten.png"),
    ),
    (
        "Robozinho",
        "image/png",
        include_bytes!("../../../assets/usertiles/robot.png"),
    ),
    (
        "Futebol",
        "image/gif",
        include_bytes!("../../../assets/usertiles/soccer.gif"),
    ),
    (
        "Sol",
        "image/gif",
        include_bytes!("../../../assets/usertiles/summer.gif"),
    ),
    (
        "Flores",
        "image/gif",
        include_bytes!("../../../assets/usertiles/spring.gif"),
    ),
    (
        "Outono",
        "image/gif",
        include_bytes!("../../../assets/usertiles/fall.gif"),
    ),
];

// IDs únicos para os inputs de arquivo HTML (mobile)
static PRESET_ASSET_PATHS: &[&str] = &[
    "/assets/usertiles/daisy.png",
    "/assets/usertiles/dog.png",
    "/assets/usertiles/kitten.png",
    "/assets/usertiles/robot.png",
    "/assets/usertiles/soccer.gif",
    "/assets/usertiles/summer.gif",
    "/assets/usertiles/spring.gif",
    "/assets/usertiles/fall.gif",
];

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

                div { class: "p-4 flex flex-col space-y-4",

                    // Erro de upload
                    if let Some(err) = upload_error() {
                        div { class: "px-3 py-2 bg-red-50 border border-red-200 rounded-lg text-xs text-red-700 flex items-center space-x-2",
                            span { "⚠️" }
                            span { "{err}" }
                        }
                    }

                    // Seção 1: Avatares Predefinidos (via include_bytes! — funciona em todas as plataformas)
                    div { class: "flex flex-col space-y-2 border-b border-white/20 pb-4",
                        p { class: "text-xs font-bold {theme.titlebar_text()}", "Escolha uma imagem predefinida" }
                        div { class: "grid grid-cols-4 gap-2",
                            for (idx, &(name, mime, _bytes)) in PRESET_AVATARS.iter().enumerate() {
                                button {
                                    class: "relative aspect-square rounded-lg border {theme.titlebar_border()} bg-white/60 p-1 hover:border-[#5c98d6] hover:bg-white transition-all cursor-pointer flex flex-col items-center justify-center group overflow-hidden shadow-sm",
                                    disabled: is_uploading(),
                                    onclick: move |_| {
                                        let mut state = state;
                                        let mut uploading = is_uploading;
                                        let mut err_sig = upload_error;

                                        // Atualização otimista local com a URL do asset (funciona em desktop/web)
                                        let asset_path = PRESET_ASSET_PATHS[idx];
                                        *state.user_avatar_url.write() = Some(asset_path.to_string());
                                        state.show_avatar_picker.set(false);

                                        // Upload dos bytes embutidos para o servidor em background
                                        if let Some(token) = state.auth_token() {
                                            let bytes_vec = PRESET_AVATARS[idx].2.to_vec();
                                            let mime_str = mime.to_string();
                                            spawn(async move {
                                                uploading.set(true);
                                                err_sig.set(None);
                                                match api::upload_avatar(&token, bytes_vec, &mime_str).await {
                                                    Ok(uploaded_url) => {
                                                        *state.user_avatar_url.write() = Some(uploaded_url);
                                                        uploading.set(false);
                                                    }
                                                    Err(e) => {
                                                        uploading.set(false);
                                                        err_sig.set(Some(format!("Falha no upload: {}", e)));
                                                    }
                                                }
                                            });
                                        }
                                    },
                                    img {
                                        src: PRESET_ASSET_PATHS[idx],
                                        class: "w-full h-full object-cover rounded-md group-hover:scale-105 transition-transform",
                                        alt: name
                                    }
                                }
                            }
                        }
                    }

                    // Seção 2: Upload de foto própria
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
                            // Preview do avatar atual
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

                            // Área de upload — usa a estratégia certa para cada plataforma
                            div { class: "relative flex flex-col items-center p-4 border-2 border-dashed {theme.modal_border()}/50 rounded-xl bg-white/20 hover:bg-white/40 transition-all cursor-pointer group",

                                // ── INPUT HTML NATIVO (mobile e web) ──────────────────────────────
                                // No Android abre a câmera/galeria; no desktop serve como fallback
                                input {
                                    r#type: "file",
                                    id: "avatar-file-input",
                                    accept: "image/*",
                                    // No mobile capture="user" abre câmera frontal, mas vamos deixar
                                    // sem para que o usuário escolha câmera OU galeria
                                    class: "absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10",
                                    onchange: move |e| {
                                        let mut state = state;
                                        let mut uploading = is_uploading;
                                        let mut err_sig = upload_error;

                                        // Dioxus 0.7: e.files() retorna Vec<FileData> diretamente
                                        let files = e.files();
                                        if let Some(file) = files.into_iter().next() {
                                            let token_opt = state.auth_token();
                                            let file_name = file.name();
                                            spawn(async move {
                                                if let Some(token) = token_opt {
                                                    uploading.set(true);
                                                    err_sig.set(None);

                                                    match file.read_bytes().await {
                                                        Ok(bytes) => {
                                                            let mime = detect_mime_from_name(&file_name);
                                                            match api::upload_avatar(&token, bytes.to_vec(), &mime).await {
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
                                                        Err(e) => {
                                                            uploading.set(false);
                                                            err_sig.set(Some(format!("Não foi possível ler o arquivo: {}", e)));
                                                        }
                                                    }
                                                }
                                            });
                                        }
                                    }
                                }

                                // Conteúdo visual por baixo do input transparente
                                span { class: "text-2xl mb-1 group-hover:scale-110 transition-transform pointer-events-none", "📷" }
                                p { class: "text-xs font-semibold {theme.titlebar_text()} pointer-events-none", "Toque para selecionar ou tirar foto" }
                                p { class: "text-[10px] text-slate-500 pointer-events-none", "JPG, PNG, GIF ou WebP • máx 5MB" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Detecta o MIME type pelo nome do arquivo (funciona em qualquer plataforma)
fn detect_mime_from_name(name: &str) -> String {
    let lower = name.to_lowercase();
    if lower.ends_with(".png") {
        "image/png".to_string()
    } else if lower.ends_with(".gif") {
        "image/gif".to_string()
    } else if lower.ends_with(".webp") {
        "image/webp".to_string()
    } else {
        "image/jpeg".to_string()
    }
}
