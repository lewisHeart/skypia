use crate::models::{render_avatar, UserStatus, AppTheme};
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

    let mut show_status_menu = use_signal(|| false);
    let mut show_actions_menu = use_signal(|| false);
    let mut music_search_query = use_signal(|| String::new());

    let mut save_personal_msg = move |_| {
        state.set_user_personal_message(temp_msg());
        is_editing_msg.set(false);
    };

    rsx! {
        // User Profile Section
        div { class: "px-4 py-3 flex items-center space-x-3 bg-white/20 border-b border-white/20 relative",

            // Top Right Tools inside Profile
            div { class: "absolute right-2 top-2 flex items-center z-50",
                button {
                    class: "w-6 h-6 flex items-center justify-center rounded hover:bg-white/40 border border-transparent hover:border-white/50 cursor-pointer transition-colors focus:outline-none",
                    title: "Opções",
                    onclick: move |_| show_actions_menu.set(!show_actions_menu()),
                    img {
                        src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/gear.webp",
                        class: "w-4 h-4 object-contain pointer-events-none"
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
                            src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/person.webp",
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
                            src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/gear.webp",
                            class: "w-3.5 h-3.5 object-contain mr-1.5"
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
                            src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/information.webp",
                            class: "w-3.5 h-3.5 object-contain mr-1.5"
                        }
                        span { "Sobre o Skypia..." }
                    }
                    
                    div { class: "h-[1px] bg-slate-200 my-1" }
                    
                    div { class: "px-2 py-0.5 text-slate-400 font-bold text-[9px] uppercase tracking-wider", "Cor da skin" }
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
                            src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/door.webp",
                            class: "w-3.5 h-3.5 object-contain mr-1.5"
                        }
                        span { "Desconectar" }
                    }
                }
            }

            // Avatar Frame — MSN Style color status border
            div {
                class: "relative p-[2px] flex-shrink-0 cursor-pointer shadow rounded-[8px] border {state.user_status().avatar_frame_class()} bg-transparent shadow-[inset_0_0.5px_0_rgba(255,255,255,0.4)] flex items-center justify-center transition-all hover:brightness-105",
                onclick: move |_| {
                    state.show_avatar_picker.set(true);
                },
                div {
                    class: "rounded-[5px] overflow-hidden border border-white/35 bg-white flex-shrink-0 flex items-center justify-center",
                    {render_avatar(state.user_avatar_url().as_deref(), 48)}
                }

                // Ícone de edição sobre o avatar
                div {
                    class: "absolute inset-[2px] rounded-[5px] bg-black/0 hover:bg-black/25 transition-all flex items-center justify-center opacity-0 hover:opacity-100 z-20",
                    span { class: "text-white text-sm drop-shadow", "✏️" }
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
                                    // Sincroniza com servidor
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
                                class: "font-bold text-sm {theme.titlebar_text()} truncate cursor-pointer hover:bg-white/40 hover:underline px-1 rounded transition-colors",
                                onclick: move |_| {
                                    temp_name.set(state.user_name());
                                    is_editing_name.set(true);
                                },
                                "{state.user_name()}"
                            }
                            button {
                                class: "text-[10px] font-semibold px-1 rounded {theme.titlebar_text()} hover:bg-white/40 cursor-pointer flex items-center space-x-0.5 transition-colors focus:outline-none flex-shrink-0",
                                style: "opacity: 0.8;",
                                onclick: move |_| show_status_menu.set(!show_status_menu()),
                                span { "({state.user_status().as_str()})" }
                                span { "▼" }
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
                    p {
                        class: "text-xs {theme.titlebar_text()} italic truncate cursor-pointer hover:bg-white/40 hover:underline px-1 rounded transition-colors",
                        style: "opacity: 0.75;",
                        onclick: move |_| {
                            temp_msg.set(state.user_personal_message());
                            is_editing_msg.set(true);
                        },
                        "{state.user_personal_message()}"
                    }
                }

                // Music Display
                div {
                    class: "flex items-center space-x-1 text-[10px] {theme.titlebar_text()} font-medium truncate cursor-pointer hover:underline",
                    style: "opacity: 0.90;",
                    onclick: move |_| state.show_music_player_modal.set(true),
                    img {
                        src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/musical-note.webp",
                        class: "w-3 h-3 object-contain pointer-events-none mr-0.5 inline-block align-middle"
                    }
                    if let Some(music) = state.user_music() {
                        span { "{music}" }
                    } else {
                        span { class: "text-slate-400/80 italic", "Silêncio (Adicionar música)" }
                    }
                }
            }
        }

        // User status selection dropdown popup
        if show_status_menu() {
            div {
                class: "absolute left-4 top-20 w-36 bg-white/95 border border-slate-300 rounded shadow-lg z-50 p-1 flex flex-col text-xs text-slate-700",
                button {
                    class: "px-2 py-1 hover:bg-slate-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                    onclick: move |_| {
                        state.set_user_status(UserStatus::Online);
                        show_status_menu.set(false);
                    },
                    div { class: "w-2.5 h-2.5 rounded-full bg-[#3cd070] border border-[#2fa558]" }
                    span { "Disponível" }
                }
                button {
                    class: "px-2 py-1 hover:bg-slate-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                    onclick: move |_| {
                        state.set_user_status(UserStatus::Ocupado);
                        show_status_menu.set(false);
                    },
                    div { class: "w-2.5 h-2.5 rounded-full bg-[#e81123] border border-[#b50a18]" }
                    span { "Ocupado" }
                }
                button {
                    class: "px-2 py-1 hover:bg-slate-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                    onclick: move |_| {
                        state.set_user_status(UserStatus::Ausente);
                        show_status_menu.set(false);
                    },
                    div { class: "w-2.5 h-2.5 rounded-full bg-[#ffb900] border border-[#c99200]" }
                    span { "Ausente" }
                }
                button {
                    class: "px-2 py-1 hover:bg-slate-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                    onclick: move |_| {
                        state.set_user_status(UserStatus::Offline);
                        show_status_menu.set(false);
                    },
                    div { class: "w-2.5 h-2.5 rounded-full bg-gray-400 border border-gray-500" }
                    span { "Offline" }
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
                    class: "w-80 bg-gradient-to-b {theme.modal_gradient()} border {theme.modal_border()} rounded-lg shadow-2xl p-4 flex flex-col space-y-4 text-xs {theme.titlebar_text()} pointer-events-auto",
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
                            class: "px-2 py-1.5 border {theme.titlebar_border()} msn-input rounded text-xs",
                            placeholder: "Ex: Paramore - Decode",
                            value: "{music_search_query}",
                            oninput: move |e| music_search_query.set(e.value()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter && !music_search_query().is_empty() {
                                    state.set_user_music(Some(music_search_query().clone()));
                                    state.show_music_player_modal.set(false);
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col space-y-1",
                        span { class: "font-bold text-slate-500 mb-1", "Hits Nostálgicos de 2010" }
                        for hit in [
                            "Coldplay - Viva La Vida",
                            "Green Day - 21 Guns",
                            "Paramore - Decode",
                            "Nx Zero - Cedo Ou Tarde",
                            "Fresno - Desde Quando Você Se Foi",
                            "Linkin Park - In The End",
                            "Lady Gaga - Bad Romance",
                            "Justin Bieber - Baby"
                        ] {
                            button {
                                class: "px-2 py-1.5 text-left hover:bg-black/5 rounded transition-colors {theme.titlebar_text()} hover:underline cursor-pointer",
                                onclick: move |_| {
                                    state.set_user_music(Some(hit.to_string()));
                                    state.show_music_player_modal.set(false);
                                },
                                "{hit}"
                            }
                        }
                    }

                    div { class: "flex space-x-2 pt-2 border-t {theme.titlebar_border()}/30",
                        button {
                            class: "flex-1 py-1.5 bg-red-100 hover:bg-red-200 text-red-700 border border-red-200 rounded font-bold cursor-pointer transition-colors text-center",
                            onclick: move |_| {
                                state.set_user_music(None);
                                state.show_music_player_modal.set(false);
                            },
                            "Desativar Música"
                        }
                        button {
                            class: "flex-1 py-1.5 {theme.btn_primary()} rounded font-bold shadow-md cursor-pointer transition-all text-center",
                            onclick: move |_| {
                                if !music_search_query().is_empty() {
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
