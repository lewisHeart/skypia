use crate::models::{render_avatar, AppTheme, UserStatus};
use crate::services::api;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn ProfileHeader(mut state: AppState) -> Element {
    let theme = state.theme();
    // Sinais locais de controle de edição e menus dropdown
    let mut is_editing_name = use_signal(|| false);
    let mut temp_name = use_signal(|| state.user_name());

    let mut is_editing_msg = use_signal(|| false);
    let mut temp_msg = use_signal(|| state.user_personal_message());

    let mut show_status_menu_name = use_signal(|| false);
    let mut show_status_menu_avatar = use_signal(|| false);
    let mut show_actions_menu = use_signal(|| false);
    let mut music_search_query = use_signal(|| String::new());

    let mut save_personal_msg = move |_| {
        state.set_user_personal_message(temp_msg());
        is_editing_msg.set(false);
    };

    rsx! {
        // User Profile Section
        div { class: "px-4 py-3 flex items-center space-x-3 bg-transparent relative",

            // Top Right Tools inside Profile
            div { class: "absolute right-2 top-2 flex items-center z-50",
                button {
                    class: "w-6 h-6 flex items-center justify-center rounded hover:bg-white/40 border border-transparent hover:border-white/50 cursor-pointer transition-colors focus:outline-none",
                    title: "Opções",
                    onclick: move |_| show_actions_menu.set(!show_actions_menu()),
                    img {
                        src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Settings/SVG/ic_fluent_settings_24_color.svg",
                        class: "w-4 h-4 pointer-events-none"
                    }
                }
            }

            // Click outside overlay to close actions menu
            if show_actions_menu() {
                div {
                    class: "fixed inset-0 z-40 bg-transparent cursor-default",
                    onclick: move |_| show_actions_menu.set(false),
                }
            }

            // Click outside overlay to close status menus
            if show_status_menu_name() || show_status_menu_avatar() {
                div {
                    class: "fixed inset-0 z-40 bg-transparent cursor-default",
                    onclick: move |_| {
                        show_status_menu_name.set(false);
                        show_status_menu_avatar.set(false);
                    },
                }
            }

            // Actions Dropdown Menu
            if show_actions_menu() {
                div {
                    class: "absolute right-2 top-8 w-48 bg-white border border-slate-300 rounded-lg shadow-xl z-50 p-1.5 flex flex-col text-xs text-slate-700 font-normal",

                    button {
                        class: "px-2 py-1.5 text-left hover:bg-slate-100 rounded transition-colors flex items-center space-x-2 cursor-pointer",
                        onclick: move |_| {
                            show_actions_menu.set(false);
                            state.open_my_profile();
                        },
                        img {
                            src: "/assets/emojis/person.svg",
                            class: "w-3.5 h-3.5 object-contain mr-1.5"
                        }
                        span { "Meu Perfil..." }
                    }
                    button {
                        class: "px-2 py-1.5 text-left hover:bg-slate-100 rounded transition-colors flex items-center space-x-2 cursor-pointer",
                        onclick: move |_| {
                            show_actions_menu.set(false);
                            state.show_settings_modal.set(true);
                        },
                        img {
                            src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Settings/SVG/ic_fluent_settings_24_color.svg",
                            class: "w-3.5 h-3.5 pointer-events-none mr-1.5"
                        }
                        span { "Configurações..." }
                    }
                    button {
                        class: "px-2 py-1.5 text-left hover:bg-slate-100 rounded transition-colors flex items-center space-x-2 cursor-pointer",
                        onclick: move |_| {
                            show_actions_menu.set(false);
                            state.show_about.set(true);
                        },
                        img {
                            src: "/assets/emojis/information.svg",
                            class: "w-3.5 h-3.5 object-contain mr-1.5"
                        }
                        span { "Sobre o Skypia..." }
                    }

                    div { class: "h-[1px] bg-slate-200 my-1" }

                    div { class: "px-2 py-0.5 text-slate-400 font-bold text-[9px]", "Cor da skin" }
                    for (theme_opt, label, color_class) in &[
                        (AppTheme::AeroBlue, "Azul Aero", "bg-sky-400 border-sky-500"),
                        (AppTheme::RubyPink, "Rosa Choque", "bg-pink-400 border-pink-500"),
                        (AppTheme::ForestGreen, "Verde Natureza", "bg-emerald-400 border-emerald-500"),
                        (AppTheme::SilverMetallic, "Prata Metálico", "bg-slate-400 border-slate-500")
                    ] {
                        button {
                            class: "px-2 py-1 text-left hover:bg-slate-100 rounded transition-colors flex items-center space-x-2 cursor-pointer text-[11px]",
                            onclick: move |_| {
                                state.set_settings(state.interface_scale(), state.use_custom_titlebar(), *theme_opt);
                                show_actions_menu.set(false);
                            },
                            div { class: "w-2.5 h-2.5 rounded {color_class} border flex-shrink-0" }
                            span { "{label}" }
                        }
                    }

                    div { class: "h-[1px] bg-slate-200 my-1" }

                    button {
                        class: "px-2 py-1.5 text-left hover:bg-red-50 text-red-600 rounded transition-colors flex items-center space-x-2 cursor-pointer font-semibold",
                        onclick: move |_| {
                            show_actions_menu.set(false);
                            state.logout();
                        },
                        img {
                            src: "/assets/emojis/door.svg",
                            class: "w-3.5 h-3.5 object-contain mr-1.5"
                        }
                        span { "Desconectar" }
                    }
                }
            }

            // Avatar do cabeçalho com moldura de status clássica do MSN do design do usuário
            div {
                class: "msn-avatar-container w-[56px] h-[55px] cursor-pointer hover:brightness-105 transition-all flex-shrink-0 relative",
                onclick: move |_| {
                    state.show_avatar_picker.set(true);
                },
                img {
                    src: match state.user_status() {
                        UserStatus::Online => asset!("/assets/status/disponivel_perfil.svg"),
                        UserStatus::Ocupado => asset!("/assets/status/ocupado_perfil.svg"),
                        UserStatus::Ausente => asset!("/assets/status/ausente_perfil.svg"),
                        _ => asset!("/assets/status/offline_perfil.svg"),
                    },
                    class: "msn-avatar-frame-img"
                }
                div {
                    class: "msn-avatar-content w-[48px] h-[47px] rounded-[4px] bg-transparent flex items-center justify-center",
                    {render_avatar(state.user_avatar_url().as_deref(), 48)}
                }

                // Ícone de edição sobre o avatar
                div {
                    class: "absolute inset-0 rounded-[4px] bg-black/0 hover:bg-black/25 transition-all flex items-center justify-center opacity-0 hover:opacity-100 z-20",
                    span { class: "text-white text-xs drop-shadow", "✏️" }
                }
            }

            // Profile Info
            div { class: "flex-1 min-w-0 flex flex-col space-y-0.5",
                div { class: "flex items-center justify-between",
                    if is_editing_name() {
                        input {
                            class: "px-1.5 py-0.5 text-xs msn-input rounded w-full font-bold",
                            value: "{temp_name}",
                            oninput: move |e| temp_name.set(e.value()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter {
                                    let name = temp_name();
                                    state.set_user_name(name.clone());
                                    is_editing_name.set(false);
                                    if let Some(token) = state.auth_token() {
                                        spawn(async move {
                                            let _ = api::update_profile(&token, api::UpdateProfileRequest {
                                                display_name: Some(name),
                                                personal_message: None,
                                                status: None,
                                                music: None,
                                            }).await;
                                        });
                                    }
                                }
                            },
                            onblur: move |_| {
                                let name = temp_name();
                                state.set_user_name(name.clone());
                                is_editing_name.set(false);
                                if let Some(token) = state.auth_token() {
                                    spawn(async move {
                                        let _ = api::update_profile(&token, api::UpdateProfileRequest {
                                            display_name: Some(name),
                                            personal_message: None,
                                            status: None,
                                            music: None,
                                        }).await;
                                    });
                                }
                            },
                            autofocus: true,
                        }
                    } else {
                        div { class: "flex items-center space-x-1.5 min-w-0 max-w-full",
                            span {
                                class: "font-black text-sm text-black truncate cursor-pointer hover:bg-white/40 hover:underline px-1 rounded transition-colors",
                                onclick: move |_| {
                                    temp_name.set(state.user_name());
                                    is_editing_name.set(true);
                                },
                                {crate::models::parse_emoticons_inline(&state.user_name(), "w-4 h-4")}
                            }
                            div { class: "relative flex items-center flex-shrink-0",
                                button {
                                    class: "text-[10px] font-normal px-1 rounded text-[#a5a5a5] hover:bg-white/40 cursor-pointer flex items-center space-x-0.5 transition-colors focus:outline-none flex-shrink-0",
                                    onclick: move |_| {
                                        show_status_menu_name.set(!show_status_menu_name());
                                        show_status_menu_avatar.set(false);
                                    },
                                    span { "({state.user_status().as_str()})" }
                                    span { "▼" }
                                }
                                if show_status_menu_name() {
                                    StatusDropdown {
                                        state,
                                        show_menu: show_status_menu_name,
                                        class: "absolute left-0 top-full mt-1 z-50".to_string()
                                    }
                                }
                            }
                        }
                    }
                }

                // Sub-status (Editable Phrase)
                if is_editing_msg() {
                    input {
                        class: "px-1.5 py-0.5 text-xs msn-input rounded w-full",
                        value: "{temp_msg}",
                        oninput: move |e| temp_msg.set(e.value()),
                        onkeydown: move |e| {
                            if e.key() == Key::Enter {
                                save_personal_msg(e);
                            }
                        },
                        onblur: move |_| {
                            let msg = temp_msg();
                            *state.user_personal_message.write() = msg.clone();
                            is_editing_msg.set(false);
                            // Sincroniza com servidor
                            if let Some(token) = state.auth_token() {
                                spawn(async move {
                                    let _ = api::update_profile(&token, api::UpdateProfileRequest {
                                        display_name: None,
                                        personal_message: Some(msg),
                                        status: None,
                                        music: None,
                                    }).await;
                                });
                            }
                        },
                        autofocus: true,
                    }
                } else {
                    {
                        let display_msg = if state.user_personal_message().trim().is_empty() {
                            "<Insira uma mensagem pessoal>".to_string()
                        } else {
                            state.user_personal_message()
                        };
                        rsx! {
                            p {
                                class: "text-xs text-[#8a8a8a] truncate cursor-pointer hover:bg-white/40 hover:underline px-1 rounded transition-colors",
                                onclick: move |_| {
                                    temp_msg.set(state.user_personal_message());
                                    is_editing_msg.set(true);
                                },
                                {crate::models::parse_emoticons_inline(&display_msg, "w-3.5 h-3.5")}
                            }
                        }
                    }
                }

                // Music Display
                div {
                    class: "flex items-center space-x-1 text-[10px] text-[#a5a5a5] font-normal truncate cursor-pointer hover:underline",
                    onclick: move |_| state.show_music_player_modal.set(true),
                    img {
                        src: "/assets/emojis/musical-note.svg",
                        class: "w-3 h-3 object-contain pointer-events-none mr-0.5 inline-block align-middle"
                    }
                    if let Some(music) = state.user_music() {
                        span { "{music}" }
                    } else {
                        span { "Silêncio (Adicionar música)" }
                    }
                }
            }
        }
        // ==========================================
        // MUSIC PLAYER MODAL (Orkut/MSN Style)
        // ==========================================
        if state.show_music_player_modal() {
            div {
                class: "fixed inset-0 bg-black/45 backdrop-blur-sm z-[9999] flex items-center justify-center p-4 pointer-events-auto",
                onclick: move |_| state.show_music_player_modal.set(false),
                div {
                    class: "w-80 bg-gradient-to-b {theme.modal_gradient()} border {theme.modal_border()} rounded-lg shadow-2xl p-4 flex flex-col space-y-4 text-xs {theme.titlebar_text()} pointer-events-auto backdrop-blur-md",
                    onclick: move |e| e.stop_propagation(),

                    div { class: "flex items-center justify-between border-b {theme.titlebar_border()} pb-2",
                        span { class: "font-bold text-sm {theme.titlebar_text()}", "🎵 O que estou ouvindo?" }
                        button {
                            class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-500 hover:text-white border border-transparent font-bold cursor-pointer transition-colors",
                            onclick: move |_| state.show_music_player_modal.set(false),
                            "✕"
                        }
                    }

                    div { class: "flex flex-col space-y-1.5",
                        label { class: "font-bold text-slate-700", "Digite uma música personalizada" }
                        input {
                            class: "px-2 py-1.5 border {theme.titlebar_border()} msn-input rounded text-xs bg-white/80 focus:bg-white focus:outline-none transition-colors",
                            placeholder: "Ex: Projeto Sola - Entre Nós",
                            value: "{music_search_query}",
                            oninput: move |e| music_search_query.set(e.value()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter && !music_search_query().is_empty() {
                                    state.set_spotify_rpc_enabled(false);
                                    state.set_user_music(Some(music_search_query().clone()));
                                    state.show_music_player_modal.set(false);
                                }
                            }
                        }
                    }

                    // Seção de Integração com o Spotify RPC
                    div { class: "flex flex-col space-y-2 p-3 bg-white/40 border {theme.titlebar_border()} rounded-lg backdrop-blur-md",
                        div { class: "flex items-center justify-between",
                            span { class: "font-bold text-slate-700", "Spotify (Autodetectar)" }
                            button {
                                class: if state.spotify_rpc_enabled() {
                                    "px-2.5 py-1 text-[10px] rounded border transition-all cursor-pointer font-bold select-none bg-emerald-500 hover:bg-emerald-600 text-white border-emerald-600 shadow-sm"
                                } else {
                                    "px-2.5 py-1 text-[10px] rounded border transition-all cursor-pointer font-bold select-none bg-slate-200 hover:bg-slate-300 text-slate-700 border-slate-300 shadow-sm"
                                },
                                onclick: move |_| {
                                    let next_state = !state.spotify_rpc_enabled();
                                    state.set_spotify_rpc_enabled(next_state);
                                    if next_state {
                                        state.set_user_music(None);
                                    }
                                },
                                if state.spotify_rpc_enabled() { "Ativado" } else { "Desativado" }
                            }
                        }
                        div { class: "flex items-center space-x-2 text-[11px]",
                            div {
                                class: if state.spotify_rpc_enabled() {
                                    "w-2.5 h-2.5 rounded-full bg-emerald-500 shadow-[0_0_5px_rgba(16,185,129,0.6)] animate-pulse"
                                } else {
                                    "w-2.5 h-2.5 rounded-full bg-slate-400"
                                }
                            }
                            span {
                                class: "text-slate-700 font-semibold",
                                if state.spotify_rpc_enabled() {
                                    "Status: Detectando do Spotify"
                                } else {
                                    "Status: Inativo/Desativado"
                                }
                            }
                        }
                        p { class: "text-[10px] text-slate-500 leading-relaxed font-normal",
                            "O Skypia detecta automaticamente o que você está ouvindo no aplicativo Spotify."
                        }
                    }

                    div { class: "flex space-x-2 pt-2 border-t {theme.titlebar_border()}/30",
                        button {
                            class: "flex-1 py-1.5 bg-red-100 hover:bg-red-200 text-red-700 border border-red-200 rounded font-bold cursor-pointer transition-colors text-center shadow-sm",
                            onclick: move |_| {
                                state.set_spotify_rpc_enabled(false);
                                state.set_user_music(None);
                                state.show_music_player_modal.set(false);
                            },
                            "Desativar Música"
                        }
                        button {
                            class: "flex-1 py-1.5 {theme.btn_primary()} rounded font-bold shadow-md cursor-pointer transition-all text-center",
                            onclick: move |_| {
                                if !music_search_query().is_empty() {
                                    state.set_spotify_rpc_enabled(false);
                                    state.set_user_music(Some(music_search_query().clone()));
                                }
                                state.show_music_player_modal.set(false);
                            },
                            "Confirmar"
                        }
                    }
                }
            }
        }
        // Modal do AvatarPicker
        if state.show_avatar_picker() {
            crate::components::profile::avatar_picker::AvatarPicker { state }
        }
    }
}

#[component]
fn StatusDropdown(mut state: AppState, mut show_menu: Signal<bool>, class: String) -> Element {
    rsx! {
        div {
            class: "{class} w-36 bg-white border border-slate-300 rounded shadow-lg z-50 p-1 flex flex-col text-xs text-slate-700 font-normal",
            onclick: move |e| e.stop_propagation(),
            button {
                class: "px-2 py-1 hover:bg-slate-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                onclick: move |_| {
                    state.set_user_status(UserStatus::Online);
                    show_menu.set(false);
                },
                div { class: "w-2.5 h-2.5 rounded-full bg-[#3cd070] border border-[#2fa558]" }
                span { "Disponível" }
            }
            button {
                class: "px-2 py-1 hover:bg-slate-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                onclick: move |_| {
                    state.set_user_status(UserStatus::Ocupado);
                    show_menu.set(false);
                },
                div { class: "w-2.5 h-2.5 rounded-full bg-[#e81123] border border-[#b50a18]" }
                span { "Ocupado" }
            }
            button {
                class: "px-2 py-1 hover:bg-slate-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                onclick: move |_| {
                    state.set_user_status(UserStatus::Ausente);
                    show_menu.set(false);
                },
                div { class: "w-2.5 h-2.5 rounded-full bg-[#ffb900] border border-[#c99200]" }
                span { "Ausente" }
            }
            button {
                class: "px-2 py-1 hover:bg-slate-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                onclick: move |_| {
                    state.set_user_status(UserStatus::Offline);
                    show_menu.set(false);
                },
                div { class: "w-2.5 h-2.5 rounded-full bg-gray-400 border border-gray-500" }
                span { "Offline" }
            }
        }
    }
}
