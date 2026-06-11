use crate::services::api;
use crate::state::AppState;
use dioxus::prelude::*;

static PRESET_AVATARS: &[(&str, &str, &[u8])] = &[
    ("Margarida", "image/png", include_bytes!("../../../assets/usertiles/daisy.png")),
    ("Cachorrinho", "image/png", include_bytes!("../../../assets/usertiles/dog.png")),
    ("Gatinho", "image/png", include_bytes!("../../../assets/usertiles/kitten.png")),
    ("Robozinho", "image/png", include_bytes!("../../../assets/usertiles/robot.png")),
    ("Futebol", "image/gif", include_bytes!("../../../assets/usertiles/soccer.gif")),
    ("Sol", "image/gif", include_bytes!("../../../assets/usertiles/summer.gif")),
    ("Flores", "image/gif", include_bytes!("../../../assets/usertiles/spring.gif")),
    ("Outono", "image/gif", include_bytes!("../../../assets/usertiles/fall.gif")),
];

async fn save_avatar_local_file(bytes: &[u8]) -> Option<String> {
    let data_dir = crate::services::db::get_app_data_dir();
    let _ = std::fs::create_dir_all(&data_dir);
    let file_path = data_dir.join("user_avatar.png");
    if std::fs::write(&file_path, bytes).is_ok() {
        let abs = std::fs::canonicalize(&file_path).unwrap_or(file_path);
        return Some(format!("dioxus-asset://{}", abs.to_string_lossy()));
    }
    None
}

#[component]
pub fn AvatarPicker(mut state: AppState) -> Element {
    let is_uploading = use_signal(|| false);
    let upload_error = use_signal(|| Option::<String>::None);

    let preset_paths = [
        asset!("/assets/usertiles/daisy.png").to_string(),
        asset!("/assets/usertiles/dog.png").to_string(),
        asset!("/assets/usertiles/kitten.png").to_string(),
        asset!("/assets/usertiles/robot.png").to_string(),
        asset!("/assets/usertiles/soccer.gif").to_string(),
        asset!("/assets/usertiles/summer.gif").to_string(),
        asset!("/assets/usertiles/spring.gif").to_string(),
        asset!("/assets/usertiles/fall.gif").to_string(),
    ];

    rsx! {
        // Overlay com o mesmo fundo do login
        div {
            class: "fixed inset-0 bg-black/40 z-[9999] flex items-center justify-center pointer-events-auto",
            onclick: move |_| state.show_avatar_picker.set(false),

            // Painel — mesmo visual da janela de login: fundo branco/gradiente azul suave
            div {
                class: "w-[309px] flex flex-col select-none pointer-events-auto",
                style: "background: linear-gradient(180deg, #c2ddf4 0%, #ffffff 10%, #ffffff 90%, #eff8fa 100%); border: 1px solid #a6b9cd; border-radius: 8px; box-shadow: 0 8px 32px rgba(0,0,0,0.22);",
                onclick: move |e| e.stop_propagation(),

                // Topo: título
                div {
                    class: "flex items-center justify-between px-4 pt-4 pb-2",
                    span { class: "text-xs font-bold text-[#1e395b]", "Escolher imagem de exibição" }
                    button {
                        class: "w-5 h-5 flex items-center justify-center text-slate-400 hover:text-red-500 font-bold cursor-pointer transition-colors focus:outline-none text-sm leading-none",
                        onclick: move |_| state.show_avatar_picker.set(false),
                        "×"
                    }
                }

                // Linha divisória
                div { class: "h-px bg-[#a6b9cd]/50 mx-3" }

                // Corpo
                div { class: "px-4 py-3 flex flex-col space-y-3.5",

                    // Erro
                    if let Some(err) = upload_error() {
                        div { class: "w-full px-3 py-2 bg-red-50 border border-red-200 rounded text-[11px] text-red-700 flex items-center space-x-2 shadow-sm",
                            span { "⚠️" }
                            span { "{err}" }
                        }
                    }

                    // Seção 1: Predefinidos
                    div { class: "flex flex-col space-y-2",
                        span { class: "text-[10px] font-semibold text-[#1e395b]", "Selecione uma imagem predefinida:" }
                        div { class: "grid grid-cols-4 gap-[6px]",
                            for (idx, &(name, mime, _bytes)) in PRESET_AVATARS.iter().enumerate() {
                                {
                                    let asset_path = preset_paths[idx].clone();
                                    let asset_path_src = asset_path.clone();
                                    rsx! {
                                        button {
                                            class: "aspect-square rounded-[4px] border border-[#d1d1d1] hover:border-[#5c98d6] overflow-hidden cursor-pointer transition-all disabled:opacity-40 disabled:cursor-not-allowed bg-white p-0 focus:outline-none",
                                            disabled: is_uploading(),
                                            title: name,
                                            onclick: move |_| {
                                                let mut state = state;
                                                let mut uploading = is_uploading;
                                                let mut err_sig = upload_error;

                                                *state.user_avatar_url.write() = Some(asset_path.clone());

                                                if let Some(token) = state.auth_token() {
                                                    let bytes_vec = PRESET_AVATARS[idx].2.to_vec();
                                                    let mime_str = mime.to_string();
                                                    let preview_url = asset_path.clone();
                                                    spawn(async move {
                                                        uploading.set(true);
                                                        err_sig.set(None);
                                                        match api::upload_avatar(&token, bytes_vec, &mime_str).await {
                                                            Ok(server_url) => {
                                                                *state.user_avatar_url.write() = Some(server_url.clone());
                                                                let _ = crate::services::db::DatabaseService::save_user_avatar_url(Some(server_url)).await;
                                                                uploading.set(false);
                                                                state.show_avatar_picker.set(false);
                                                            }
                                                            Err(e) => {
                                                                *state.user_avatar_url.write() = None;
                                                                let _ = crate::services::db::DatabaseService::save_user_avatar_url(Some(preview_url)).await;
                                                                uploading.set(false);
                                                                err_sig.set(Some(format!("Falha ao enviar: {}", e)));
                                                            }
                                                        }
                                                    });
                                                } else {
                                                    let local = asset_path.clone();
                                                    spawn(async move {
                                                        let _ = crate::services::db::DatabaseService::save_user_avatar_url(Some(local)).await;
                                                    });
                                                    state.show_avatar_picker.set(false);
                                                }
                                            },
                                            img {
                                                src: "{asset_path_src}",
                                                class: "w-full h-full object-cover",
                                                alt: name
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Linha divisória interna
                    div { class: "h-px bg-[#d1d1d1]/60" }

                    // Seção 2: Foto própria
                    div { class: "flex flex-col space-y-2",
                        span { class: "text-[10px] font-semibold text-[#1e395b]", "Ou envie uma foto do computador:" }

                        if state.auth_token().is_none() {
                            div { class: "w-full px-3 py-2 bg-amber-50 border border-amber-200 rounded text-[11px] text-amber-700 shadow-sm",
                                "⚠️ Faça login para enviar uma foto."
                            }
                        } else if is_uploading() {
                            div { class: "flex items-center justify-center py-4 space-x-2",
                                div { class: "w-4 h-4 border-2 border-sky-500 border-t-transparent rounded-full animate-spin" }
                                span { class: "text-[10px] text-slate-500 animate-pulse", "Enviando foto..." }
                            }
                        } else {
                            // Preview atual (se tiver)
                            if state.user_avatar_url().is_some() {
                                div { class: "flex items-center space-x-2 px-2 py-1.5 bg-white/60 border border-[#d1d1d1] rounded-[4px]",
                                    div { class: "w-9 h-9 rounded-[3px] overflow-hidden border border-[#d1d1d1] flex-shrink-0",
                                        {crate::models::render_avatar(state.user_avatar_url().as_deref(), 36)}
                                    }
                                    div {
                                        p { class: "text-[10px] font-semibold text-[#1e395b]", "Foto atual" }
                                        p { class: "text-[10px] text-slate-400", "Envie uma nova para substituir" }
                                    }
                                }
                            }

                            // Botão upload estilo input do login
                            div { class: "relative",
                                input {
                                    r#type: "file",
                                    id: "avatar-file-input",
                                    accept: "image/*",
                                    class: "absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10",
                                    onchange: move |e| {
                                        let mut state = state;
                                        let mut uploading = is_uploading;
                                        let mut err_sig = upload_error;
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
                                                            if let Some(local) = save_avatar_local_file(&bytes).await {
                                                                *state.user_avatar_url.write() = Some(local.clone());
                                                                let _ = crate::services::db::DatabaseService::save_user_avatar_url(Some(local)).await;
                                                            }
                                                            let mime = detect_mime_from_name(&file_name);
                                                            match api::upload_avatar(&token, bytes.to_vec(), &mime).await {
                                                                Ok(url) => {
                                                                    *state.user_avatar_url.write() = Some(url.clone());
                                                                    let _ = crate::services::db::DatabaseService::save_user_avatar_url(Some(url)).await;
                                                                    uploading.set(false);
                                                                    state.show_avatar_picker.set(false);
                                                                }
                                                                Err(e) => {
                                                                    err_sig.set(Some(format!("Não foi possível enviar ao servidor: {}", e)));
                                                                    uploading.set(false);
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
                                // Visual — mesmo estilo dos inputs do login
                                div {
                                    class: "w-full h-[27px] px-2.5 text-xs text-slate-800 bg-white border border-[#d1d1d1] rounded-[4px] hover:border-slate-400 transition-colors flex items-center space-x-2 cursor-pointer pointer-events-none",
                                    span { class: "text-slate-400 text-[11px]", "📁" }
                                    span { class: "text-slate-500 text-[10px]", "Procurar foto..." }
                                }
                            }
                        }
                    }

                    // Botão Cancelar — mesmo estilo de botão do login
                    div { class: "pt-1 flex justify-end",
                        button {
                            class: "h-[27px] px-4 bg-[#cde3f6] hover:bg-[#b8d6f0] text-[#012d93] border border-transparent rounded-[4px] font-bold text-[10px] shadow-sm cursor-pointer transition-colors flex items-center justify-center focus:outline-none",
                            onclick: move |_| state.show_avatar_picker.set(false),
                            "Cancelar"
                        }
                    }
                }
            }
        }
    }
}

fn detect_mime_from_name(name: &str) -> String {
    let lower = name.to_lowercase();
    if lower.ends_with(".png") { "image/png".to_string() }
    else if lower.ends_with(".gif") { "image/gif".to_string() }
    else if lower.ends_with(".webp") { "image/webp".to_string() }
    else { "image/jpeg".to_string() }
}
