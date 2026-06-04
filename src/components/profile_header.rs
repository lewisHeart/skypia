use crate::models::{render_avatar, AppTheme, UserStatus};
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn ProfileHeader(mut state: AppState) -> Element {
    // Sinais locais de controle de edição e menus dropdown
    let mut is_editing_name = use_signal(|| false);
    let mut temp_name = use_signal(|| state.user_name());

    let mut is_editing_msg = use_signal(|| false);
    let mut temp_msg = use_signal(|| state.user_personal_message());

    let mut show_status_menu = use_signal(|| false);
    let mut show_theme_menu = use_signal(|| false);
    let mut music_search_query = use_signal(|| String::new());

    let mut save_personal_msg = move |_| {
        state.set_user_personal_message(temp_msg());
        is_editing_msg.set(false);
    };

    rsx! {
        // User Profile Section
        div { class: "px-4 py-3 flex items-center space-x-3 bg-white/20 border-b border-white/20 relative",

            // Top Right Tools inside Profile
            div { class: "absolute right-2 top-2 flex items-center space-x-1",
                button {
                    class: "w-5 h-5 flex items-center justify-center rounded hover:bg-white/40 border border-transparent hover:border-white/50 text-[#1e395b] cursor-pointer text-xs transition-colors",
                    title: "Configurações",
                    onclick: move |_| state.show_settings_modal.set(true),
                    "⚙️"
                }
                button {
                    class: "w-5 h-5 flex items-center justify-center rounded hover:bg-white/40 border border-transparent hover:border-white/50 text-[#1e395b] cursor-pointer text-xs transition-colors",
                    title: "Mudar cor da skin",
                    onclick: move |_| show_theme_menu.set(!show_theme_menu()),
                    "🎨"
                }
                button {
                    class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-100/40 border border-transparent hover:border-red-200/50 text-red-600 cursor-pointer text-xs transition-colors",
                    title: "Desconectar",
                    onclick: move |_| {
                        *state.logged_in.write() = false;
                    },
                    "🚪"
                }
            }

            // Theme selection dropdown menu
            if show_theme_menu() {
                div {
                    class: "absolute right-2 top-8 w-40 bg-white border border-slate-300 rounded shadow-lg z-50 p-1 flex flex-col text-xs text-slate-700",
                    button {
                        class: "px-2 py-1.5 text-left hover:bg-sky-100 rounded transition-colors flex items-center space-x-2 cursor-pointer",
                        onclick: move |_| {
                            state.set_settings(state.interface_scale(), state.use_custom_titlebar(), AppTheme::AeroBlue);
                            show_theme_menu.set(false);
                        },
                        div { class: "w-2.5 h-2.5 rounded bg-sky-400 border border-sky-500" }
                        span { "Azul Aero" }
                    }
                    button {
                        class: "px-2 py-1.5 text-left hover:bg-pink-100 rounded transition-colors flex items-center space-x-2 cursor-pointer",
                        onclick: move |_| {
                            state.set_settings(state.interface_scale(), state.use_custom_titlebar(), AppTheme::RubyPink);
                            show_theme_menu.set(false);
                        },
                        div { class: "w-2.5 h-2.5 rounded bg-pink-400 border border-pink-500" }
                        span { "Rosa Choque" }
                    }
                    button {
                        class: "px-2 py-1.5 text-left hover:bg-emerald-100 rounded transition-colors flex items-center space-x-2 cursor-pointer",
                        onclick: move |_| {
                            state.set_settings(state.interface_scale(), state.use_custom_titlebar(), AppTheme::ForestGreen);
                            show_theme_menu.set(false);
                        },
                        div { class: "w-2.5 h-2.5 rounded bg-emerald-400 border border-emerald-500" }
                        span { "Verde Natureza" }
                    }
                    button {
                        class: "px-2 py-1.5 text-left hover:bg-slate-100 rounded transition-colors flex items-center space-x-2 cursor-pointer",
                        onclick: move |_| {
                            state.set_settings(state.interface_scale(), state.use_custom_titlebar(), AppTheme::SilverMetallic);
                            show_theme_menu.set(false);
                        },
                        div { class: "w-2.5 h-2.5 rounded bg-slate-400 border border-slate-500" }
                        span { "Prata Metálico" }
                    }
                }
            }

            // Avatar Frame with Fixed MSN Frame and Status Badge Overlay
            div {
                class: "relative p-[3px] flex-shrink-0 cursor-pointer shadow rounded-[10px] border border-[#a1c6e7] bg-white transition-all",
                onclick: move |_| {
                    // Cycle avatar ID
                    let curr = state.user_avatar_id();
                    state.set_user_avatar((curr + 1) % 7);
                },
                {render_avatar(state.user_avatar_id(), 48)}

                // Status Badge overlay
                div {
                    class: "absolute -bottom-0.5 -right-0.5 w-[15px] h-[15px] rounded-full bg-white border border-[#a1c6e7] flex items-center justify-center cursor-pointer hover:scale-110 transition-transform z-10 shadow-sm",
                    onclick: move |e| {
                        e.stop_propagation();
                        show_status_menu.set(!show_status_menu());
                    },
                    div { class: "w-[9px] h-[9px] rounded-full {state.user_status().color_class()} border border-black/10" }
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
                                    state.set_user_name(temp_name());
                                    is_editing_name.set(false);
                                }
                            },
                            onblur: move |_| {
                                state.set_user_name(temp_name());
                                is_editing_name.set(false);
                            },
                            autofocus: true,
                        }
                    } else {
                        span {
                            class: "font-bold text-sm text-[#1b324d] truncate cursor-pointer hover:bg-white/40 hover:underline px-1 rounded transition-colors",
                            onclick: move |_| {
                                temp_name.set(state.user_name());
                                is_editing_name.set(true);
                            },
                            "{state.user_name()}"
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
                            *state.user_personal_message.write() = temp_msg();
                            is_editing_msg.set(false);
                        },
                        autofocus: true,
                    }
                } else {
                    p {
                        class: "text-xs text-[#3a5879]/85 italic truncate cursor-pointer hover:bg-white/40 hover:underline px-1 rounded transition-colors",
                        onclick: move |_| {
                            temp_msg.set(state.user_personal_message());
                            is_editing_msg.set(true);
                        },
                        "{state.user_personal_message()}"
                    }
                }

                // Music Display
                div {
                    class: "flex items-center space-x-1 text-[10px] text-[#0066cc] font-medium truncate cursor-pointer hover:underline",
                    onclick: move |_| state.show_music_player_modal.set(true),
                    span { "🎵" }
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
                    class: "px-2 py-1 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                    onclick: move |_| {
                        state.set_user_status(UserStatus::Online);
                        show_status_menu.set(false);
                    },
                    div { class: "w-2.5 h-2.5 rounded-full bg-[#3cd070] border border-[#2fa558]" }
                    span { "Disponível" }
                }
                button {
                    class: "px-2 py-1 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                    onclick: move |_| {
                        state.set_user_status(UserStatus::Ocupado);
                        show_status_menu.set(false);
                    },
                    div { class: "w-2.5 h-2.5 rounded-full bg-[#e81123] border border-[#b50a18]" }
                    span { "Ocupado" }
                }
                button {
                    class: "px-2 py-1 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                    onclick: move |_| {
                        state.set_user_status(UserStatus::Ausente);
                        show_status_menu.set(false);
                    },
                    div { class: "w-2.5 h-2.5 rounded-full bg-[#ffb900] border border-[#c99200]" }
                    span { "Ausente" }
                }
                button {
                    class: "px-2 py-1 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
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
                    class: "w-80 bg-gradient-to-b from-[#e6f1fc] to-[#c8def5] border border-[#7ba9d4] rounded-lg shadow-2xl p-4 flex flex-col space-y-4 text-xs text-[#1e395b] pointer-events-auto",
                    onclick: move |e| e.stop_propagation(),

                    div { class: "flex items-center justify-between border-b border-white/40 pb-2",
                        span { class: "font-bold text-sm", "🎵 O que estou ouvindo?" }
                        button {
                            class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-500 hover:text-white border border-transparent font-bold cursor-pointer transition-colors",
                            onclick: move |_| state.show_music_player_modal.set(false),
                            "✕"
                        }
                    }

                    div { class: "flex flex-col space-y-1.5",
                        label { class: "font-bold text-slate-700", "Digite uma música personalizada" }
                        input {
                            class: "px-2 py-1.5 msn-input rounded text-xs",
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
                                class: "px-2 py-1.5 text-left hover:bg-sky-200/50 rounded transition-colors text-[#0066cc] hover:underline cursor-pointer",
                                onclick: move |_| {
                                    state.set_user_music(Some(hit.to_string()));
                                    state.show_music_player_modal.set(false);
                                },
                                "{hit}"
                            }
                        }
                    }

                    div { class: "flex space-x-2 pt-2 border-t border-white/20",
                        button {
                            class: "flex-1 py-1.5 bg-red-100 hover:bg-red-200 text-red-700 border border-red-200 rounded font-bold cursor-pointer transition-colors text-center",
                            onclick: move |_| {
                                state.set_user_music(None);
                                state.show_music_player_modal.set(false);
                            },
                            "Desativar Música"
                        }
                        button {
                            class: "flex-1 py-1.5 bg-gradient-to-b from-sky-400 to-sky-500 hover:from-sky-500 hover:to-sky-600 text-white rounded font-bold shadow-md cursor-pointer transition-all text-center",
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
    }
}
