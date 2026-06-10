use crate::components::main::contact_list::ContactList;
use crate::components::profile::profile_header::ProfileHeader;
use crate::models::{AppTheme, UserStatus};
use crate::sound::play_sound;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn MainWindow(mut state: AppState) -> Element {
    let theme = state.theme();
    // Sinais para os modais locais
    let mut add_contact_email = use_signal(|| String::new());

    let mut search_result = use_signal(|| None::<crate::models::UserProfile>);
    let mut search_error = use_signal(|| None::<String>);
    let mut is_searching = use_signal(|| false);

    // Reseta o estado de busca quando o modal é fechado
    use_effect(move || {
        if !state.show_add_contact_modal() {
            search_result.set(None);
            search_error.set(None);
            is_searching.set(false);
            add_contact_email.set(String::new());
        }
    });

    let mut handle_search = move || {
        let query = add_contact_email().trim().to_string();
        if query.is_empty() {
            return;
        }

        let token_opt = state.auth_token();
        is_searching.set(true);
        search_error.set(None);
        search_result.set(None);

        spawn(async move {
            if let Some(token) = token_opt {
                match crate::services::api::search_user(&token, &query).await {
                    Ok(user) => {
                        search_result.set(Some(user));
                    }
                    Err(e) => {
                        search_error.set(Some(e));
                    }
                }
            } else {
                search_error.set(Some("Você precisa estar conectado na rede.".to_string()));
            }
            is_searching.set(false);
        });
    };

    rsx! {
        div {
            class: "w-full h-full flex flex-col select-none bg-transparent overflow-hidden",

            // Header do Perfil do usuário
            ProfileHeader { state }

            // Lista de Contatos com pesquisa integrada
            ContactList { state }

            // Banner dinâmico de anúncios do banco de dados
            if let Some(banner) = state.banner_info() {
                div {
                    class: "h-[50px] w-full bg-gradient-to-r {theme.titlebar_gradient()} border-t {theme.titlebar_border()} px-3 flex items-center justify-between text-[11px] shadow-inner flex-shrink-0 cursor-pointer overflow-hidden transition-all hover:brightness-105",
                    onclick: move |_| {
                        let _ = document::eval(&format!("window.open('{}', '_blank')", banner.link));
                    },
                    div { class: "flex items-center space-x-2 flex-1 {theme.titlebar_text()} min-w-0",
                        style: "opacity: 0.90;",
                        span { class: "text-base flex-shrink-0", "{banner.icon}" }
                        div { class: "flex flex-col min-w-0 flex-1",
                            span { class: "font-bold {theme.titlebar_text()} truncate", "{banner.text}" }
                            span { class: "text-[10px] text-slate-500 truncate hover:underline", "{banner.action_label}" }
                        }
                    }
                }
            }

            // Rodapé
            div { class: "h-6 bg-white/10 border-t border-[#d1d1d1]/30 px-3 flex items-center justify-center text-[10px] text-slate-400 flex-shrink-0",
                span { "Skypia Messenger v0.0.1" }
            }
        }

        // ==========================================
        // MODAL DE CONFIGURAÇÕES
        // ==========================================
        if state.show_settings_modal() {
            div {
                class: "fixed inset-0 bg-black/45 backdrop-blur-sm z-[200] flex items-center justify-center p-4",
                onclick: move |_| state.show_settings_modal.set(false),
                div {
                    class: "w-80 bg-gradient-to-b {theme.modal_gradient()} border {theme.modal_border()} rounded-lg shadow-2xl p-4 flex flex-col space-y-4 text-xs {theme.titlebar_text()} pointer-events-auto",
                    onclick: move |e| e.stop_propagation(),

                    div { class: "flex items-center justify-between border-b {theme.titlebar_border()} pb-2",
                        div { class: "flex items-center space-x-1.5 font-bold text-sm {theme.titlebar_text()}",
                            img {
                                src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/gear.webp",
                                class: "w-4.5 h-4.5 object-contain pointer-events-none"
                            }
                            span { "Configurações do Skypia" }
                        }
                        button {
                            class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-500 hover:text-white border border-transparent font-bold cursor-pointer transition-colors",
                            onclick: move |_| state.show_settings_modal.set(false),
                            "✕"
                        }
                    }

                    div { class: "flex flex-col space-y-1.5",
                        label { class: "font-bold text-slate-700", "Estilo de Decorações da Janela" }
                        label { class: "flex items-center space-x-2 cursor-pointer",
                            input {
                                r#type: "checkbox",
                                checked: state.use_custom_titlebar(),
                                onchange: move |e| {
                                    let val = e.value() == "true";
                                    state.set_settings(state.interface_scale(), val, state.theme());
                                    #[cfg(feature = "desktop")]
                                    dioxus::desktop::use_window().set_decorations(!val);
                                }
                            }
                            span { "Usar barra de título Aero do app" }
                        }
                    }

                    div { class: "flex flex-col space-y-1.5",
                        label { class: "font-bold text-slate-700", "Escala da Interface" }
                        select {
                            class: "w-full p-1.5 border {theme.titlebar_border()} rounded bg-white text-slate-700 font-medium",
                            onchange: move |e| {
                                let scale = e.value().parse::<f64>().unwrap_or(1.0);
                                state.set_settings(scale, state.use_custom_titlebar(), state.theme());
                            },
                            option { value: "0.8", selected: state.interface_scale() == 0.8, "80% (Pequeno)" }
                            option { value: "0.9", selected: state.interface_scale() == 0.9, "90%" }
                            option { value: "1.0", selected: state.interface_scale() == 1.0, "100% (Padrão)" }
                            option { value: "1.1", selected: state.interface_scale() == 1.1, "110%" }
                            option { value: "1.2", selected: state.interface_scale() == 1.2, "120%" }
                            option { value: "1.3", selected: state.interface_scale() == 1.3, "130%" }
                            option { value: "1.4", selected: state.interface_scale() == 1.4, "140%" }
                            option { value: "1.5", selected: state.interface_scale() == 1.5, "150% (Grande)" }
                        }
                    }

                    div { class: "flex flex-col space-y-1.5",
                        label { class: "font-bold text-slate-700", "Aparência (Skins)" }
                        select {
                            class: "w-full p-1.5 border {theme.titlebar_border()} rounded bg-white text-slate-700 font-medium",
                            onchange: move |e| {
                                let new_theme = match e.value().as_str() {
                                    "blue" => AppTheme::AeroBlue,
                                    "pink" => AppTheme::RubyPink,
                                    "green" => AppTheme::ForestGreen,
                                    "silver" => AppTheme::SilverMetallic,
                                    _ => AppTheme::AeroBlue,
                                };
                                state.set_settings(state.interface_scale(), state.use_custom_titlebar(), new_theme);
                            },
                            option { value: "blue", selected: state.theme() == AppTheme::AeroBlue, "Azul Aero" }
                            option { value: "pink", selected: state.theme() == AppTheme::RubyPink, "Rosa Choque" }
                            option { value: "green", selected: state.theme() == AppTheme::ForestGreen, "Verde Natureza" }
                            option { value: "silver", selected: state.theme() == AppTheme::SilverMetallic, "Prata Metálico" }
                        }
                    }

                    div { class: "flex flex-col space-y-1.5",
                        label { class: "font-bold text-slate-700", "Modo de Chat" }
                        select {
                            class: "w-full p-1.5 border {theme.titlebar_border()} rounded bg-white text-slate-700 font-medium",
                            onchange: move |e| {
                                state.set_chat_mode(e.value());
                            },
                            option { value: "integrated", selected: state.chat_mode() == "integrated", "Chat Conectado" }
                            option { value: "detached", selected: state.chat_mode() == "detached", "Janela Separada" }
                        }
                    }


                    div { class: "flex flex-col space-y-2 pt-2 border-t {theme.titlebar_border()}/30",
                        button {
                            class: "w-full py-1.5 bg-white/60 hover:bg-white border border-slate-300 hover:border-slate-400 text-slate-700 rounded font-semibold transition-all flex items-center justify-center space-x-1.5 cursor-pointer",
                            onclick: move |_| {
                                state.show_settings_modal.set(false);
                                state.open_my_profile();
                            },
                            img {
                                src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/person.webp",
                                class: "w-3.5 h-3.5 object-contain pointer-events-none mr-1"
                            }
                            span { "Ver perfil completo" }
                        }
                        button {
                            class: "w-full py-1.5 bg-white/60 hover:bg-white border border-slate-300 hover:border-slate-400 text-slate-700 rounded font-semibold transition-all flex items-center justify-center space-x-1.5 cursor-pointer",
                            onclick: move |_| {
                                state.show_settings_modal.set(false);
                                state.show_about.set(true);
                            },
                            img {
                                src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/information.webp",
                                class: "w-3.5 h-3.5 object-contain pointer-events-none mr-1"
                            }
                            span { "Sobre o Skypia..." }
                        }
                    }

                    div { class: "flex justify-end pt-2 border-t {theme.titlebar_border()}",
                        button {
                            class: "px-4 py-1.5 {theme.btn_primary()} rounded font-bold shadow cursor-pointer transition-colors",
                            onclick: move |_| state.show_settings_modal.set(false),
                            "Ok"
                        }
                    }
                }
            }
        }

        // ==========================================
        // MODAL DE ADICIONAR CONTATO
        // ==========================================
        if state.show_add_contact_modal() {
            div {
                class: "fixed inset-0 bg-black/45 backdrop-blur-sm z-[200] flex items-center justify-center p-4",
                onclick: move |_| state.show_add_contact_modal.set(false),
                div {
                    class: "w-[340px] bg-gradient-to-b {theme.modal_gradient()} border {theme.modal_border()} rounded-lg shadow-2xl p-4 flex flex-col space-y-3.5 text-xs {theme.titlebar_text()} pointer-events-auto",
                    onclick: move |e| e.stop_propagation(),

                    div { class: "flex items-center justify-between border-b {theme.titlebar_border()} pb-2",
                        div { class: "flex items-center space-x-1.5 font-bold text-sm {theme.titlebar_text()}",
                            img {
                                src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/plus.webp",
                                class: "w-4.5 h-4.5 object-contain pointer-events-none"
                            }
                            span { "Adicionar Novo Contato" }
                        }
                        button {
                            class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-500 hover:text-white border border-transparent font-bold cursor-pointer transition-colors focus:outline-none",
                            onclick: move |_| state.show_add_contact_modal.set(false),
                            "✕"
                        }
                    }

                    // Campo de entrada e botão de busca
                    div { class: "flex flex-col space-y-1.5",
                        label { class: "font-semibold text-slate-700", "Email ou Nome de usuário:" }
                        div { class: "flex space-x-1.5",
                            input {
                                class: "flex-1 p-1.5 border {theme.titlebar_border()} rounded msn-input text-xs focus:outline-none bg-white",
                                placeholder: "Joao ou joao@mail.com",
                                value: "{add_contact_email}",
                                oninput: move |e| add_contact_email.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter && !add_contact_email().trim().is_empty() && !is_searching() {
                                        handle_search();
                                    }
                                }
                            }
                            button {
                                class: "px-3 py-1.5 {theme.btn_primary()} rounded font-bold shadow transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center focus:outline-none",
                                disabled: add_contact_email().trim().is_empty() || is_searching(),
                                onclick: move |_| handle_search(),
                                if is_searching() { "Buscando..." } else { "Buscar" }
                            }
                        }
                    }

                    // Painel de Resultados de busca
                    div { class: "min-h-[90px] border {theme.titlebar_border()}/40 bg-white/40 rounded p-2.5 flex flex-col justify-center items-center relative overflow-hidden",
                        if is_searching() {
                            div { class: "flex flex-col items-center space-y-2 text-slate-500 py-4",
                                div { class: "w-5 h-5 border-2 border-sky-600 border-t-transparent rounded-full animate-spin" }
                                span { "Buscando usuário no servidor..." }
                            }
                        } else if let Some(ref err) = search_error() {
                            div { class: "flex flex-col items-center space-y-1 text-center py-2 text-[#b50a18]",
                                span { class: "text-lg", "⚠️" }
                                span { class: "font-semibold", "{err}" }
                            }
                        } else if let Some(ref user) = search_result() {
                            {
                                let user_for_add = user.clone();
                                let status_enum = match user_for_add.status.as_str() {
                                    "Online" => crate::models::UserStatus::Online,
                                    "Ocupado" => crate::models::UserStatus::Ocupado,
                                    "Ausente" => crate::models::UserStatus::Ausente,
                                    "Invisivel" => crate::models::UserStatus::Invisivel,
                                    _ => crate::models::UserStatus::Offline,
                                };
                                rsx! {
                                    div { class: "w-full flex items-center space-x-3.5",
                                        // Avatar com moldura de status do MSN
                                        div {
                                            class: "flex-shrink-0 p-[2px] rounded-[7px] border {status_enum.avatar_frame_class()} bg-transparent shadow-[inset_0_0.5px_0_rgba(255,255,255,0.4)] flex items-center justify-center shadow-md",
                                            div {
                                                class: "rounded-[4px] overflow-hidden border border-white/30 bg-white flex-shrink-0 flex items-center justify-center",
                                                {crate::models::render_avatar(user_for_add.avatar_url.as_deref(), 48)}
                                            }
                                        }
                                        // Detalhes
                                        div { class: "flex-1 min-w-0 flex flex-col space-y-0.5",
                                            span { class: "font-bold text-sm {theme.titlebar_text()} truncate", "{user_for_add.display_name}" }
                                            span { class: "text-[10px] text-slate-500 font-semibold truncate", "{user_for_add.email}" }
                                            span { class: "text-[10px] text-slate-400 truncate italic", "“{user_for_add.personal_message}”" }
                                        }
                                    }
                                }
                            }
                        } else {
                            // Estado inicial/vazio
                            div { class: "text-center text-slate-400 py-4 font-normal",
                                "Digite as informações e clique em Buscar para encontrar um amigo."
                            }
                        }
                    }

                    // Botões de controle no rodapé
                    div { class: "flex justify-end space-x-2 pt-2 border-t {theme.titlebar_border()}/40",
                        button {
                            class: "px-4 py-1.5 bg-white hover:bg-slate-100 border border-slate-350 rounded font-bold cursor-pointer transition-colors focus:outline-none",
                            onclick: move |_| state.show_add_contact_modal.set(false),
                            "Cancelar"
                        }
                        if let Some(ref user) = search_result() {
                            {
                                let user_clone = user.clone();
                                rsx! {
                                    button {
                                        class: "px-4 py-1.5 bg-gradient-to-b from-[#22c55e] to-[#15803d] hover:from-[#4ade80] hover:to-[#166534] text-white border border-[#166534] rounded font-bold shadow transition-colors cursor-pointer focus:outline-none",
                                        onclick: move |_| {
                                            state.add_contact_dynamic(
                                                user_clone.email.clone(),
                                                user_clone.display_name.clone(),
                                                UserStatus::Offline,
                                                user_clone.personal_message.clone()
                                            );
                                            play_sound("online");
                                            state.show_add_contact_modal.set(false);
                                        },
                                        "Adicionar Contato"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
