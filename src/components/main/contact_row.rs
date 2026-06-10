use dioxus::prelude::*;
use crate::state::AppState;
use crate::models::{Contact, render_avatar, UserStatus};

#[component]
pub fn ContactRow(contact: Contact, mut state: AppState, density: String) -> Element {
    let theme = state.theme();
    let unread_count = state.unread_count_for(&contact.id);
    let name_font_weight = if unread_count > 0 { "font-bold" } else { "font-normal" };
    let is_typing = state.typing_contacts().get(&contact.id).map(|ids| !ids.is_empty()).unwrap_or(false);
    let show_msg_or_typing = is_typing || !contact.personal_message.trim().is_empty();

    let mut show_tooltip = use_signal(|| false);
    let mut tooltip_x = use_signal(|| 0i32);
    let mut tooltip_y = use_signal(|| 0i32);

    let mut show_context_menu = use_signal(|| false);
    let mut menu_x = use_signal(|| 0i32);
    let mut menu_y = use_signal(|| 0i32);

    let mut show_rename_modal = use_signal(|| false);
    let mut new_nickname = use_signal(|| contact.nickname.clone().unwrap_or_default());

    let contact_id = contact.id.clone();
    let cid_double = contact_id.clone();
    let cid_context_open = contact_id.clone();
    let cid_context_block = contact_id.clone();
    let cid_rename_enter = contact_id.clone();
    let cid_rename_click = contact_id.clone();

    let handle_double_click = move |_| {
        state.open_chat(cid_double.clone());
    };

    let name_to_show = if let Some(ref nick) = contact.nickname {
        format!("{} ({})", nick, contact.display_name)
    } else {
        contact.display_name.clone()
    };

    let is_blocked = contact.relation_status == "Bloqueado";
    let container_padding = match density.as_str() {
        "large" => "py-2 px-1.5",
        "small" => "py-0.5 px-1.5",
        _ => "p-1",
    };

    let dragged_cid = (state.dragged_contact_id)();
    let is_currently_dragged = dragged_cid.as_ref() == Some(&contact.id);
    let opacity_class = if is_currently_dragged { "opacity-40" } else { "" };

    rsx! {
        div {
            class: "flex items-center space-x-2.5 {container_padding} rounded hover:bg-white/45 cursor-pointer relative group transition-colors {opacity_class}",
            onmousedown: {
                let cid = contact.id.clone();
                move |e| {
                    e.stop_propagation();
                    *state.dragged_contact_id.write() = Some(cid.clone());
                }
            },
            onmouseup: {
                move |e| {
                    e.stop_propagation();
                    *state.dragged_contact_id.write() = None;
                }
            },
            ondoubleclick: handle_double_click,
            oncontextmenu: move |e| {
                e.prevent_default();
                let scale = state.interface_scale();
                menu_x.set((e.client_coordinates().x as f64 / scale) as i32);
                menu_y.set((e.client_coordinates().y as f64 / scale) as i32);
                show_context_menu.set(true);
                show_tooltip.set(false);
            },
            onmouseenter: move |e| {
                if !show_context_menu() {
                    let scale = state.interface_scale();
                    tooltip_x.set((e.client_coordinates().x as f64 / scale) as i32);
                    tooltip_y.set((e.client_coordinates().y as f64 / scale) as i32);
                    show_tooltip.set(true);
                }
            },
            onmousemove: move |e| {
                if !show_context_menu() {
                    let scale = state.interface_scale();
                    tooltip_x.set((e.client_coordinates().x as f64 / scale) as i32);
                    tooltip_y.set((e.client_coordinates().y as f64 / scale) as i32);
                }
            },
            onmouseleave: move |_| show_tooltip.set(false),
            
            // Renderização condicional por densidade (Grande com Avatar vs Compacta com Ícone de Status)
            if density == "large" {
                {
                    let frame_src = match contact.status {
                        UserStatus::Online => asset!("/assets/status/Disponível Perfil.svg"),
                        UserStatus::Ocupado => asset!("/assets/status/Ocupado Perfil.svg"),
                        UserStatus::Ausente => asset!("/assets/status/Ausente Perfil.svg"),
                        _ => asset!("/assets/status/Offline Perfil.svg"),
                    };
                    rsx! {
                        div { class: "msn-avatar-container w-[44px] h-[44px] flex-shrink-0",
                            img {
                                src: frame_src,
                                class: "msn-avatar-frame-img"
                            }
                            div {
                                class: "msn-avatar-content w-[36px] h-[36px] rounded-[3px] bg-transparent flex items-center justify-center",
                                {render_avatar(contact.avatar_url.as_deref(), 36)}
                            }
                        }
                        div { class: "flex-1 min-w-0 flex flex-col space-y-0.25",
                            div { class: "flex items-center space-x-1",
                                span { class: "{name_font_weight} text-xs {theme.titlebar_text()} truncate hover:underline", "{name_to_show}" }
                                if is_blocked {
                                    span { class: "text-[9px] opacity-75", "🚫" }
                                }
                                if unread_count > 0 {
                                    span { 
                                        class: "bg-red-500 text-white text-[9px] font-extrabold px-1.5 py-0.25 rounded-full min-w-[15px] h-3.5 flex items-center justify-center animate-pulse border border-white/80 shadow-sm flex-shrink-0",
                                        "{unread_count}"
                                    }
                                }
                            }
                            if is_typing {
                                span { class: "text-[10px] text-emerald-600 font-semibold animate-pulse truncate", "✍️ digitando..." }
                            } else {
                                span { class: "text-[10px] text-[#a5a5a5] truncate font-normal", "{contact.personal_message}" }
                            }
                        }
                    }
                }
            } else {
                {
                    let icon_src = match contact.status {
                        UserStatus::Online => asset!("/assets/status/Disponível Icone.svg"),
                        UserStatus::Ocupado => asset!("/assets/status/Ocupado Icone.svg"),
                        UserStatus::Ausente => asset!("/assets/status/Ausente Icone.svg"),
                        _ => asset!("/assets/status/Offline Icone.svg"),
                    };
                    rsx! {
                        img {
                            src: icon_src,
                            class: "w-3.5 h-3.5 object-contain flex-shrink-0 select-none mr-1"
                        }
                        div { class: "flex-1 min-w-0 flex items-center space-x-1.5 text-[10px]",
                            span { class: "{name_font_weight} {theme.titlebar_text()} truncate hover:underline flex-shrink-0", "{name_to_show}" }
                            if is_blocked {
                                span { class: "text-[9px] opacity-75 flex-shrink-0", "🚫" }
                            }
                            if unread_count > 0 {
                                span { 
                                    class: "bg-red-500 text-white text-[9px] font-extrabold px-1.5 py-0.25 rounded-full min-w-[15px] h-3.5 flex items-center justify-center animate-pulse border border-white/80 shadow-sm flex-shrink-0",
                                    "{unread_count}"
                                }
                            }
                            if show_msg_or_typing {
                                if is_typing {
                                    span { class: "text-[10px] text-emerald-600 font-semibold animate-pulse truncate flex-1", "✍️ digitando..." }
                                } else {
                                    span { class: "text-[10px] text-[#a5a5a5] truncate font-normal flex-1", "{contact.personal_message}" }
                                }
                            }
                        }
                    }
                }
            }
            
            // Listening music indicator icon
            if contact.music_listening.is_some() {
                span { class: "text-[10px] pr-1 opacity-70", "🎵" }
            }

            // Nostalgic hover card tooltip
            if show_tooltip() {
                div { 
                    class: "fixed w-64 bg-gradient-to-b {theme.tooltip_bg()} border rounded-lg shadow-xl z-50 p-3 flex flex-col space-y-2 text-xs text-slate-700 pointer-events-none",
                    style: "left: {tooltip_x + 15}px; top: {tooltip_y + 15}px;",
                    div { class: "flex items-start space-x-3",
                        // Tooltip Avatar with fixed border
                        div { 
                            class: "flex-shrink-0 shadow rounded-[6px] border border-slate-300/70 overflow-hidden bg-transparent",
                            {render_avatar(contact.avatar_url.as_deref(), 44)}
                        }
                        div { class: "flex-1 min-w-0 flex flex-col space-y-1",
                            span { class: "font-bold text-sm {theme.titlebar_text()} truncate", "{name_to_show}" }
                            span { class: "text-[10px] text-slate-400 select-all font-semibold", "{contact.email}" }
                            span { class: "font-semibold text-[10px] text-slate-500", "Status: {contact.status.as_str()}" }
                        }
                    }
                    div { class: "border-t border-slate-200/80 pt-1.5 flex flex-col space-y-1",
                        p { class: "text-[10px] text-slate-600 italic select-text", "“{contact.personal_message}”" }
                        if let Some(ref song) = contact.music_listening {
                            div { 
                                class: "flex items-center space-x-1 text-[9px] {theme.titlebar_text()} font-medium",
                                style: "opacity: 0.90;",
                                span { "🎵" }
                                span { "{song}" }
                            }
                        }
                    }
                }
            }

            // Menu de Contexto MSN Style com Overlay para fechar ao clicar fora
            if show_context_menu() {
                div {
                    class: "fixed inset-0 z-[9998] bg-transparent cursor-default",
                    onclick: move |e| {
                        e.stop_propagation();
                        show_context_menu.set(false);
                    }
                }
                div { 
                    class: "fixed w-44 bg-white/95 border border-slate-300 rounded-lg shadow-2xl backdrop-blur-md z-[9999] p-1 flex flex-col text-[11px] text-slate-700 transition-all",
                    style: "left: {menu_x}px; top: {menu_y}px;",
                    onclick: move |e| e.stop_propagation(),
                    onmouseleave: move |_| show_context_menu.set(false),
                    
                    button { 
                        class: "px-2.5 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-medium transition-colors",
                        onclick: {
                            let cid = cid_context_open.clone();
                            move |_| {
                                show_context_menu.set(false);
                                state.open_chat(cid.clone());
                            }
                        },
                        span { "💬" }
                        span { "Enviar mensagem" }
                    }
                    button { 
                        class: "px-2.5 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-medium transition-colors",
                        onclick: move |_| {
                            show_context_menu.set(false);
                            show_rename_modal.set(true);
                        },
                        span { "✏️" }
                        span { "Renomear (Apelido)" }
                    }
                    button { 
                        class: "px-2.5 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-medium transition-colors",
                        onclick: {
                            let cid = contact_id.clone();
                            move |_| {
                                show_context_menu.set(false);
                                state.open_contact_profile(cid.clone());
                            }
                        },
                        span { "👤" }
                        span { "Ver perfil" }
                    }

                    // Divisor
                    div { class: "h-[1px] bg-slate-200/60 my-0.5" }

                    // Botão Favorito
                    {
                        let cid = contact_id.clone();
                        let is_fav = contact.is_favorite;
                        rsx! {
                            button { 
                                class: "px-2.5 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-medium transition-colors",
                                onclick: move |_| {
                                    show_context_menu.set(false);
                                    state.toggle_favorite(cid.clone());
                                },
                                span { if is_fav { "⭐" } else { "☆" } }
                                span { if is_fav { "Remover dos favoritos" } else { "Adicionar aos favoritos" } }
                            }
                        }
                    }

                    // Divisor
                    div { class: "h-[1px] bg-slate-200/60 my-0.5" }
                    
                    button { 
                        class: "px-2.5 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-medium transition-colors",
                        onclick: {
                            let cid = cid_context_block.clone();
                            move |_| {
                                show_context_menu.set(false);
                                state.block_contact(cid.clone(), !is_blocked);
                            }
                        },
                        span { if is_blocked { "🟢" } else { "🚫" } }
                        span { if is_blocked { "Desbloquear contato" } else { "Bloquear contato" } }
                    }
                }
            }

            // Modal de Renomeação (MSN Style)
            if show_rename_modal() {
                div { 
                    class: "fixed inset-0 bg-black/45 backdrop-blur-sm z-[99999] flex items-center justify-center p-4 cursor-default",
                    onclick: move |_| show_rename_modal.set(false),
                    div { 
                        class: "w-80 bg-gradient-to-b {theme.modal_gradient()} border-2 {theme.modal_border()} rounded shadow-2xl p-4 flex flex-col space-y-4 text-xs {theme.titlebar_text()}",
                        onclick: move |e| e.stop_propagation(),
                        
                        div { class: "flex items-center justify-between border-b {theme.titlebar_border()} pb-2",
                            span { class: "font-bold text-sm {theme.titlebar_text()}", "Renomear Contato" }
                            button { 
                                class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-500 hover:text-white border border-transparent font-bold cursor-pointer transition-colors focus:outline-none",
                                onclick: move |_| show_rename_modal.set(false),
                                "✕"
                            }
                        }
                        
                        div { class: "flex flex-col space-y-1.5",
                            label { class: "font-semibold text-slate-700", "Digite o apelido para {contact.display_name}:" }
                            input { 
                                class: "w-full px-2.5 py-1.5 border {theme.titlebar_border()} rounded bg-white focus:outline-none focus:border-slate-400 text-xs text-slate-800",
                                placeholder: "Apelido personalizado...",
                                value: "{new_nickname}",
                                oninput: move |e| new_nickname.set(e.value()),
                                onkeydown: {
                                    let cid = cid_rename_enter.clone();
                                    move |e| {
                                        if e.key() == Key::Enter {
                                            let nick = new_nickname();
                                            let final_nick = if nick.trim().is_empty() { None } else { Some(nick) };
                                            state.rename_contact(cid.clone(), final_nick);
                                            show_rename_modal.set(false);
                                        }
                                    }
                                }
                            }
                        }
                        
                        div { class: "flex items-center justify-end space-x-2 pt-2 border-t {theme.titlebar_border()}/50",
                            button { 
                                class: "px-4 py-1.5 {theme.btn_primary()} rounded font-bold shadow-md cursor-pointer transition-all focus:outline-none",
                                onclick: {
                                    let cid = cid_rename_click.clone();
                                    move |_| {
                                        let nick = new_nickname();
                                        let final_nick = if nick.trim().is_empty() { None } else { Some(nick) };
                                        state.rename_contact(cid.clone(), final_nick);
                                        show_rename_modal.set(false);
                                    }
                                },
                                "Salvar"
                            }
                            button { 
                                class: "px-4 py-1.5 bg-gradient-to-b from-slate-200 to-slate-300 hover:from-slate-300 hover:to-slate-400 text-slate-700 rounded font-bold shadow border border-slate-400/40 cursor-pointer transition-all focus:outline-none",
                                onclick: move |_| show_rename_modal.set(false),
                                "Cancelar"
                            }
                        }
                    }
                }
            }
        }
    }
}
