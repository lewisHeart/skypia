use dioxus::prelude::*;
use crate::state::AppState;
use crate::models::{Contact, render_avatar};

#[component]
pub fn ContactRow(contact: Contact, mut state: AppState) -> Element {
    let mut show_tooltip = use_signal(|| false);
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

    rsx! {
        div {
            class: "flex items-center space-x-2.5 p-1 rounded hover:bg-white/45 cursor-pointer relative group transition-colors",
            ondoubleclick: handle_double_click,
            oncontextmenu: move |e| {
                e.prevent_default();
                menu_x.set(e.client_coordinates().x as i32);
                menu_y.set(e.client_coordinates().y as i32);
                show_context_menu.set(true);
                show_tooltip.set(false);
            },
            onmouseenter: move |e| {
                if !show_context_menu() {
                    tooltip_y.set(e.client_coordinates().y as i32);
                    show_tooltip.set(true);
                }
            },
            onmousemove: move |e| {
                if !show_context_menu() {
                    tooltip_y.set(e.client_coordinates().y as i32);
                }
            },
            onmouseleave: move |_| show_tooltip.set(false),
            
            // Small Avatar with MSN 3D border frame representing status
            div { 
                class: "flex-shrink-0 p-[1.5px] rounded-[5px] border {contact.status.avatar_frame_class()} overflow-hidden bg-transparent shadow-[inset_0_0.5px_0_rgba(255,255,255,0.4)] flex items-center justify-center",
                div {
                    class: "rounded-[3px] overflow-hidden border border-white/30 bg-white flex-shrink-0 flex items-center justify-center",
                    {render_avatar(contact.avatar_url.as_deref(), 24)}
                }
            }
            
            // Name and Sub-status
            div { class: "flex-1 min-w-0 flex flex-col space-y-0.25",
                div { class: "flex items-center space-x-1",
                    span { class: "font-semibold text-xs text-[#1e395b] truncate group-hover:text-sky-700", "{name_to_show}" }
                    if is_blocked {
                        span { class: "text-[9px] opacity-75", "🚫" }
                    }
                }
                span { class: "text-[10px] text-slate-500 truncate italic font-normal", "{contact.personal_message}" }
            }
            
            // Listening music indicator icon
            if contact.music_listening.is_some() {
                span { class: "text-[10px] pr-1 opacity-70", "🎵" }
            }

            // Nostalgic hover card tooltip
            if show_tooltip() {
                div { 
                    class: "fixed left-[360px] w-64 bg-gradient-to-b from-sky-50 to-sky-100/95 border border-[#a6b9cd] rounded-lg shadow-xl z-50 p-3 flex flex-col space-y-2 text-xs text-slate-700 pointer-events-none",
                    style: "top: {tooltip_y - 45}px;",
                    div { class: "flex items-start space-x-3",
                        // Tooltip Avatar with fixed border
                        div { 
                            class: "flex-shrink-0 shadow rounded-[6px] border border-slate-300/70 overflow-hidden bg-transparent",
                            {render_avatar(contact.avatar_url.as_deref(), 44)}
                        }
                        div { class: "flex-1 min-w-0 flex flex-col space-y-1",
                            span { class: "font-bold text-sm text-[#1b324d] truncate", "{name_to_show}" }
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

            // Menu de Contexto MSN Style
            if show_context_menu() {
                div { 
                    class: "fixed w-44 bg-white border border-slate-300 rounded shadow-2xl z-[9999] p-1 flex flex-col text-xs text-slate-700",
                    style: "left: {menu_x}px; top: {menu_y}px;",
                    onmouseleave: move |_| show_context_menu.set(false),
                    
                    button { 
                        class: "px-3 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full",
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
                        class: "px-3 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full",
                        onclick: move |_| {
                            show_context_menu.set(false);
                            show_rename_modal.set(true);
                        },
                        span { "✏️" }
                        span { "Renomear (Apelido)" }
                    }
                    
                    button { 
                        class: "px-3 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full",
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
                        class: "w-80 bg-gradient-to-b from-[#f2f7fc] to-[#d8e8f7] border-2 border-[#5c98d6] rounded shadow-2xl p-4 flex flex-col space-y-4 text-xs text-[#1e395b]",
                        onclick: move |e| e.stop_propagation(),
                        
                        div { class: "flex items-center justify-between border-b border-[#a8c9eb] pb-2",
                            span { class: "font-bold text-sm", "Renomear Contato" }
                            button { 
                                class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-500 hover:text-white border border-transparent font-bold cursor-pointer transition-colors focus:outline-none",
                                onclick: move |_| show_rename_modal.set(false),
                                "✕"
                            }
                        }
                        
                        div { class: "flex flex-col space-y-1.5",
                            label { class: "font-semibold text-slate-700", "Digite o apelido para {contact.display_name}:" }
                            input { 
                                class: "w-full px-2.5 py-1.5 border border-[#a8c9eb] rounded bg-white focus:outline-none focus:border-[#5c98d6] text-xs text-slate-800",
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
                        
                        div { class: "flex items-center justify-end space-x-2 pt-2 border-t border-[#a8c9eb]/50",
                            button { 
                                class: "px-4 py-1.5 bg-gradient-to-b from-sky-400 to-sky-500 hover:from-sky-500 hover:to-sky-600 text-white rounded font-bold shadow-md cursor-pointer transition-all focus:outline-none",
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
