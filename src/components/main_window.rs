use dioxus::prelude::*;
use crate::models::{Contact, UserStatus, AppTheme, render_avatar};
use crate::state::AppState;
use crate::sound::play_sound;

#[component]
pub fn MainWindow(mut state: AppState) -> Element {
    let mut search_query = use_signal(|| String::new());
    let mut is_editing_msg = use_signal(|| false);
    let mut temp_msg = use_signal(|| state.user_personal_message());
    
    // Collapsible group states
    let mut fav_collapsed = use_signal(|| false);
    let mut online_collapsed = use_signal(|| false);
    let mut offline_collapsed = use_signal(|| false);
    
    // Theme selector menu visibility
    let mut show_theme_menu = use_signal(|| false);

    // Active status menu visibility
    let mut show_status_menu = use_signal(|| false);

    // Simulate contact activity in background
    use_effect(move || {
        let mut app_state = state;
        spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(18)).await;
                
                // Only run simulation if logged in
                if !app_state.logged_in() {
                    continue;
                }

                // Random action selection (0 = status change, 1 = message notification, 2 = sign in)
                let now_ms = chrono::Utc::now().timestamp_millis();
                let action = (now_ms % 3) as usize;
                
                match action {
                    0 => {
                        // Toggle a contact's status
                        let mut contacts = app_state.contacts.write();
                        if let Some(c) = contacts.iter_mut().find(|c| c.id == 3) {
                            if c.status == UserStatus::Ocupado {
                                c.status = UserStatus::Online;
                                c.personal_message = "Voltei! CS 1.6 fechado (Y)".to_string();
                            } else {
                                c.status = UserStatus::Ocupado;
                                c.personal_message = "Dando HS no de_dust2!".to_string();
                            }
                        }
                    }
                    1 => {
                        // Simulates a message from Lucas if not already in chat, or adds to chat
                        if !app_state.active_chats().contains(&1) {
                            app_state.add_toast(
                                "Lucas [Emo Core]".to_string(),
                                "diz: eae cara! viu a nova música do green day? (H)".to_string(),
                                1,
                            );
                            play_sound("message");
                        }
                    }
                    2 => {
                        // Mariana signs in
                        let mut was_offline = false;
                        {
                            let mut contacts = app_state.contacts.write();
                            if let Some(c) = contacts.iter_mut().find(|c| c.id == 4) {
                                if c.status == UserStatus::Offline {
                                    c.status = UserStatus::Online;
                                    c.personal_message = "Mari na área! ✨".to_string();
                                    was_offline = true;
                                }
                            }
                        }
                        if was_offline {
                            app_state.add_toast(
                                "Mariana ✨".to_string(),
                                "acaba de entrar!".to_string(),
                                4,
                            );
                            play_sound("online");
                        }
                    }
                    _ => {}
                }
            }
        });
    });

    let _current_theme = state.theme();

    // Filter contacts based on query
    let filtered_contacts = use_memo(move || {
        let query = search_query().to_lowercase();
        let list = state.contacts();
        if query.is_empty() {
            list
        } else {
            list.into_iter()
                .filter(|c| c.display_name.to_lowercase().contains(&query) || c.email.to_lowercase().contains(&query))
                .collect()
        }
    });

    // Partition contacts
    let favorites = use_memo(move || {
        filtered_contacts()
            .into_iter()
            .filter(|c| c.is_favorite)
            .collect::<Vec<Contact>>()
    });

    let online_contacts = use_memo(move || {
        filtered_contacts()
            .into_iter()
            .filter(|c| !c.is_favorite && c.status != UserStatus::Offline && c.status != UserStatus::Invisivel)
            .collect::<Vec<Contact>>()
    });

    let offline_contacts = use_memo(move || {
        filtered_contacts()
            .into_iter()
            .filter(|c| !c.is_favorite && (c.status == UserStatus::Offline || c.status == UserStatus::Invisivel))
            .collect::<Vec<Contact>>()
    });

    let mut save_personal_msg = move |_| {
        state.set_user_personal_message(temp_msg());
        is_editing_msg.set(false);
    };

    rsx! {
        div {
            class: "w-full h-full flex flex-col select-none relative bg-bubbles flex-shrink-0 bg-gradient-to-b from-[#e6f1fc]/90 to-[#c8def5]/80",
            
            // User Profile Section
            div { class: "px-4 py-3 flex items-center space-x-3 bg-white/20 border-b border-white/20 relative",
                
                // Top Right Tools inside Profile
                div { class: "absolute right-2 top-2 flex items-center space-x-1",
                    button { 
                        class: "w-5 h-5 flex items-center justify-center rounded hover:bg-white/40 border border-transparent hover:border-white/50 text-[#1e395b] cursor-pointer text-xs",
                        title: "Mudar cor da skin",
                        onclick: move |_| show_theme_menu.set(!show_theme_menu()),
                        "🎨"
                    }
                    button { 
                        class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-100/40 border border-transparent hover:border-red-200/50 text-red-600 cursor-pointer text-xs",
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
                        class: "absolute right-2 top-8 w-40 bg-white/95 border border-slate-300 rounded shadow-lg z-50 p-1 flex flex-col text-xs text-slate-700",
                        button { 
                            class: "px-2 py-1.5 text-left hover:bg-sky-100 rounded transition-colors flex items-center space-x-2",
                            onclick: move |_| {
                                *state.theme.write() = AppTheme::AeroBlue;
                                show_theme_menu.set(false);
                            },
                            div { class: "w-2.5 h-2.5 rounded bg-sky-400 border border-sky-500" }
                            span { "Azul Aero" }
                        }
                        button { 
                            class: "px-2 py-1.5 text-left hover:bg-pink-100 rounded transition-colors flex items-center space-x-2",
                            onclick: move |_| {
                                *state.theme.write() = AppTheme::RubyPink;
                                show_theme_menu.set(false);
                            },
                            div { class: "w-2.5 h-2.5 rounded bg-pink-400 border border-pink-500" }
                            span { "Rosa Choque" }
                        }
                        button { 
                            class: "px-2 py-1.5 text-left hover:bg-emerald-100 rounded transition-colors flex items-center space-x-2",
                            onclick: move |_| {
                                *state.theme.write() = AppTheme::ForestGreen;
                                show_theme_menu.set(false);
                            },
                            div { class: "w-2.5 h-2.5 rounded bg-emerald-400 border border-emerald-500" }
                            span { "Verde Natureza" }
                        }
                        button { 
                            class: "px-2 py-1.5 text-left hover:bg-slate-100 rounded transition-colors flex items-center space-x-2",
                            onclick: move |_| {
                                *state.theme.write() = AppTheme::SilverMetallic;
                                show_theme_menu.set(false);
                            },
                            div { class: "w-2.5 h-2.5 rounded bg-slate-400 border border-slate-500" }
                            span { "Prata Metálico" }
                        }
                    }
                }

                // Avatar Frame
                div { 
                    class: "relative avatar-frame bg-white flex-shrink-0 cursor-pointer shadow-md",
                    onclick: move |_| {
                        // Cycle avatar ID
                        let curr = state.user_avatar_id();
                        state.set_user_avatar((curr + 1) % 7);
                    },
                    {render_avatar(state.user_avatar_id(), 48)}
                    
                    // Status Badge overlay
                    div { 
                        class: "absolute -bottom-1 -right-1 w-4 h-4 rounded-full bg-white border border-[#a6b9cd] flex items-center justify-center cursor-pointer hover:scale-110 transition-transform",
                        onclick: move |e| {
                            e.stop_propagation();
                            show_status_menu.set(!show_status_menu());
                        },
                        div { class: "w-2 h-2 rounded-full {state.user_status().color_class()} border border-black/10" }
                    }
                }

                // Profile Info
                div { class: "flex-1 min-w-0 flex flex-col space-y-0.5",
                    div { class: "flex items-center justify-between",
                        span { class: "font-bold text-sm text-[#1b324d] truncate", "{state.user_name()}" }
                        span { class: "text-[10px] text-slate-500 font-semibold px-1 py-0.2 bg-white/50 border border-white/60 rounded", "2010" }
                    }
                    
                    // Sub-status (Editable)
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
                    if let Some(music) = state.user_music() {
                        div { class: "flex items-center space-x-1 text-[10px] text-[#0066cc] font-medium truncate",
                            span { "🎵" }
                            span { class: "hover:underline cursor-pointer", "{music}" }
                        }
                    }
                }
            }

            // User status selection dropdown popup
            if show_status_menu() {
                div { 
                    class: "absolute left-4 top-20 w-36 bg-white/95 border border-slate-300 rounded shadow-lg z-50 p-1 flex flex-col text-xs text-slate-700",
                    button { 
                        class: "px-2 py-1 hover:bg-sky-100 rounded text-left flex items-center space-x-2",
                        onclick: move |_| {
                            state.set_user_status(UserStatus::Online);
                            show_status_menu.set(false);
                        },
                        div { class: "w-2.5 h-2.5 rounded-full bg-[#3cd070] border border-[#2fa558]" }
                        span { "Disponível" }
                    }
                    button { 
                        class: "px-2 py-1 hover:bg-sky-100 rounded text-left flex items-center space-x-2",
                        onclick: move |_| {
                            state.set_user_status(UserStatus::Ocupado);
                            show_status_menu.set(false);
                        },
                        div { class: "w-2.5 h-2.5 rounded-full bg-[#e81123] border border-[#b50a18]" }
                        span { "Ocupado" }
                    }
                    button { 
                        class: "px-2 py-1 hover:bg-sky-100 rounded text-left flex items-center space-x-2",
                        onclick: move |_| {
                            state.set_user_status(UserStatus::Ausente);
                            show_status_menu.set(false);
                        },
                        div { class: "w-2.5 h-2.5 rounded-full bg-[#ffb900] border border-[#c99200]" }
                        span { "Ausente" }
                    }
                    button { 
                        class: "px-2 py-1 hover:bg-sky-100 rounded text-left flex items-center space-x-2",
                        onclick: move |_| {
                            state.set_user_status(UserStatus::Offline);
                            show_status_menu.set(false);
                        },
                        div { class: "w-2.5 h-2.5 rounded-full bg-gray-400 border border-gray-500" }
                        span { "Offline" }
                    }
                }
            }

            // Quick Actions / Mail bar
            div { class: "h-7 px-3 bg-white/40 border-b border-white/20 flex items-center justify-between text-[11px] text-[#2f4b6c]/90",
                div { class: "flex items-center space-x-3",
                    button { class: "hover:text-sky-600 transition-colors flex items-center space-x-0.5",
                        span { "✉" }
                        span { "Caixa de Entrada (0)" }
                    }
                }
                div { class: "flex items-center space-x-2",
                    span { class: "hover:underline cursor-pointer", "MSN Hoje" }
                }
            }

            // Search Bar
            div { class: "p-2 bg-white/10 border-b border-white/10",
                div { class: "relative w-full flex items-center",
                    input {
                        class: "w-full pl-7 pr-2.5 py-1 text-xs rounded border border-[#a6b9cd] msn-input placeholder-slate-400",
                        placeholder: "Procurar um contato...",
                        value: "{search_query}",
                        oninput: move |e| search_query.set(e.value()),
                    }
                    span { class: "absolute left-2.5 text-xs text-slate-400 pointer-events-none", "🔍" }
                }
            }

            // Contacts List Scroll Area
            div { class: "flex-1 overflow-y-auto px-1 py-2 space-y-3 bg-white/35",
                
                // Group: Favorites
                if !favorites().is_empty() {
                    div { class: "space-y-1",
                        div { 
                            class: "flex items-center space-x-1 px-2 py-0.5 hover:bg-white/30 rounded cursor-pointer transition-colors text-xs font-bold text-[#1b324d]/85",
                            onclick: move |_| fav_collapsed.set(!fav_collapsed()),
                            span { class: "w-3 text-center text-[10px] text-slate-500", if fav_collapsed() { "▶" } else { "▼" } }
                            span { "Favoritos ({favorites().len()})" }
                        }
                        
                        if !fav_collapsed() {
                            div { class: "pl-2 space-y-0.5",
                                for contact in favorites() {
                                    ContactRow { contact, state }
                                }
                            }
                        }
                    }
                }

                // Group: Online
                div { class: "space-y-1",
                    div { 
                        class: "flex items-center space-x-1 px-2 py-0.5 hover:bg-white/30 rounded cursor-pointer transition-colors text-xs font-bold text-[#1b324d]/85",
                        onclick: move |_| online_collapsed.set(!online_collapsed()),
                        span { class: "w-3 text-center text-[10px] text-slate-500", if online_collapsed() { "▶" } else { "▼" } }
                        span { "Disponíveis ({online_contacts().len()})" }
                    }
                    
                    if !online_collapsed() {
                        div { class: "pl-2 space-y-0.5",
                            if online_contacts().is_empty() {
                                div { class: "text-[10px] text-slate-500/80 italic pl-5 py-1", "Nenhum contato online" }
                            } else {
                                for contact in online_contacts() {
                                    ContactRow { contact, state }
                                }
                            }
                        }
                    }
                }

                // Group: Offline
                div { class: "space-y-1",
                    div { 
                        class: "flex items-center space-x-1 px-2 py-0.5 hover:bg-white/30 rounded cursor-pointer transition-colors text-xs font-bold text-[#1b324d]/85",
                        onclick: move |_| offline_collapsed.set(!offline_collapsed()),
                        span { class: "w-3 text-center text-[10px] text-slate-500", if offline_collapsed() { "▶" } else { "▼" } }
                        span { "Offlines ({offline_contacts().len()})" }
                    }
                    
                    if !offline_collapsed() {
                        div { class: "pl-2 space-y-0.5",
                            for contact in offline_contacts() {
                                ContactRow { contact, state }
                            }
                        }
                    }
                }
            }

            // MSN Advertisement Banner at the bottom (Nostalgic 2010 style!)
            div { 
                class: "h-[50px] w-full bg-gradient-to-r from-sky-100 to-sky-200 border-t border-sky-300 px-3 flex items-center justify-between text-[10px] shadow-inner flex-shrink-0 cursor-pointer overflow-hidden",
                onclick: move |_| {
                    play_sound("message");
                },
                div { class: "flex flex-col flex-1 text-[#2f4b6c]",
                    span { class: "font-bold text-[#0066cc]", "Internet Explorer 8" }
                    span { class: "text-slate-500", "Navegue com muito mais velocidade e segurança." }
                }
                div { class: "px-2 py-1 bg-gradient-to-b from-sky-400 to-sky-600 border border-sky-600 rounded text-white font-bold", "Baixar" }
            }

            // Bottom Add Contact Toolbar
            div { class: "h-9 bg-white/45 border-t border-white/20 px-3 flex items-center justify-between text-xs text-[#2f4b6c]/90 flex-shrink-0",
                button { 
                    class: "hover:text-[#0066cc] font-semibold flex items-center space-x-1 transition-colors",
                    onclick: move |_| {
                        // Quick add contact simulation
                        let new_id;
                        {
                            let mut contacts = state.contacts.write();
                            new_id = contacts.len() + 1;
                            contacts.push(Contact {
                                id: new_id,
                                email: format!("contato{}@msn.com", new_id),
                                display_name: format!("Novo Contato {}", new_id),
                                status: UserStatus::Online,
                                personal_message: "Acabei de ser adicionado!".to_string(),
                                music_listening: None,
                                avatar_id: 0,
                                is_favorite: false,
                            });
                        }
                        play_sound("online");
                        state.add_toast("Contato adicionado".to_string(), format!("Novo Contato {} entrou na sua lista.", new_id), 0);
                    },
                    span { "➕" }
                    span { "Adicionar contato" }
                }
                span { class: "text-slate-400 text-[10px]", "v14.0.8117" }
            }
        }
    }
}

#[component]
fn ContactRow(contact: Contact, mut state: AppState) -> Element {
    let mut show_tooltip = use_signal(|| false);

    let handle_double_click = move |_| {
        state.open_chat(contact.id);
    };

    rsx! {
        div {
            class: "flex items-center space-x-2.5 p-1 rounded hover:bg-white/45 cursor-pointer relative group transition-colors",
            ondoubleclick: handle_double_click,
            onmouseenter: move |_| show_tooltip.set(true),
            onmouseleave: move |_| show_tooltip.set(false),
            
            // Status Icon Buddy Dot
            div { class: "relative flex-shrink-0",
                div { class: "w-3 h-3 rounded-full {contact.status.color_class()} border border-black/10 shadow-sm" }
            }
            
            // Small Avatar
            div { class: "w-6 h-6 border border-slate-300/80 rounded overflow-hidden flex-shrink-0 bg-white shadow-sm",
                {render_avatar(contact.avatar_id, 24)}
            }
            
            // Name and Sub-status
            div { class: "flex-1 min-w-0 flex flex-col space-y-0.25",
                span { class: "font-semibold text-xs text-[#1e395b] truncate group-hover:text-sky-700", "{contact.display_name}" }
                span { class: "text-[10px] text-slate-500 truncate italic font-normal", "{contact.personal_message}" }
            }
            
            // Listening music indicator icon
            if contact.music_listening.is_some() {
                span { class: "text-[10px] pr-1 opacity-70", "🎵" }
            }

            // Nostalgic hover card tooltip
            if show_tooltip() {
                div { 
                    class: "absolute left-32 top-[-10px] w-64 bg-gradient-to-b from-sky-50 to-sky-100/95 border border-[#a6b9cd] rounded-lg shadow-xl z-50 p-3 flex flex-col space-y-2 text-xs text-slate-700",
                    div { class: "flex items-start space-x-3",
                        div { class: "avatar-frame bg-white flex-shrink-0 shadow",
                            {render_avatar(contact.avatar_id, 44)}
                        }
                        div { class: "flex-1 min-w-0 flex flex-col space-y-1",
                            span { class: "font-bold text-sm text-[#1b324d] truncate", "{contact.display_name}" }
                            span { class: "text-[10px] text-slate-400 select-all font-semibold", "{contact.email}" }
                            span { class: "font-semibold text-[10px] text-slate-500", "Status: {contact.status.as_str()}" }
                        }
                    }
                    div { class: "border-t border-slate-200/80 pt-1.5 flex flex-col space-y-1",
                        p { class: "text-[10px] text-slate-600 italic select-text", "“{contact.personal_message}”" }
                        if let Some(ref song) = contact.music_listening {
                            div { class: "flex items-center space-x-1 text-[9px] text-[#0066cc] font-medium",
                                span { "🎵" }
                                span { "{song}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
