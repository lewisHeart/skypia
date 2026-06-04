use dioxus::prelude::*;
use crate::models::FileTransferState;
use crate::state::AppState;
use std::collections::HashSet;

#[component]
pub fn ChatFeed(contact_id: usize, mut state: AppState) -> Element {
    let mut show_images = use_signal(|| HashSet::new());
    
    let messages = state.chat_messages();
    let chat_history = messages.get(&contact_id).cloned().unwrap_or_default();
    
    let contact = state.contacts().into_iter().find(|c| c.id == contact_id);
    if contact.is_none() {
        return rsx! {};
    }
    let contact = contact.unwrap();

    let format_message_text = |text: &str| -> Element {
        let mut parsed = text.to_string();
        parsed = parsed.replace("(H)", "😎");
        parsed = parsed.replace("(Y)", "👍");
        parsed = parsed.replace("(N)", "👎");
        parsed = parsed.replace("(K)", "💋");
        parsed = parsed.replace("(A)", "😇");
        parsed = parsed.replace("(L)", "❤️");
        parsed = parsed.replace("(O)", "⏰");
        parsed = parsed.replace(":-D", "😀");
        parsed = parsed.replace(":-)", "🙂");
        parsed = parsed.replace(";-)", "😉");
        parsed = parsed.replace(":-(", "😢");
        parsed = parsed.replace(":-@", "😡");
        
        rsx! { span { "{parsed}" } }
    };

    rsx! {
        div { 
            class: "flex-1 overflow-y-auto p-4 space-y-3 bg-white/40 min-h-0",
            
            if chat_history.is_empty() {
                div { class: "h-full flex items-center justify-center text-slate-400 text-xs italic",
                    "Inicie uma conversa nostálgica com {contact.display_name}!"
                }
            } else {
                for msg in chat_history {
                    {
                        let name_color = if msg.sender_id == 0 { "text-[#0066cc]" } else { "text-[#e6007e]" };
                        
                        rsx! {
                            div { class: "flex flex-col space-y-0.5 text-xs text-slate-800 select-text",
                                if msg.is_nudge {
                                    div { class: "py-1.5 px-3 bg-red-100/70 border border-red-200 rounded text-red-700 font-bold flex items-center space-x-2 my-1 animate-pulse shadow-sm",
                                        span { "🔔" }
                                        span { "{msg.text}" }
                                        span { class: "text-[9px] text-red-500 font-normal ml-auto", "{msg.timestamp}" }
                                    }
                                } else if let Some(ref _wink_type) = msg.is_wink {
                                    div { class: "py-1.5 px-3 bg-purple-100/70 border border-purple-200 rounded text-purple-700 font-bold flex items-center space-x-2 my-1 animate-pulse shadow-sm",
                                        span { "✨" }
                                        span { "{msg.text}" }
                                        span { class: "text-[9px] text-purple-500 font-normal ml-auto", "{msg.timestamp}" }
                                    }
                                } else if msg.is_game_invite {
                                    div { class: "py-2 px-3 bg-emerald-100/80 border border-emerald-200 rounded text-emerald-700 font-bold flex flex-col space-y-1 my-1 shadow-sm",
                                        div { class: "flex items-center space-x-2",
                                            span { "🎮" }
                                            span { "{msg.text}" }
                                            span { class: "text-[9px] text-emerald-600 font-normal ml-auto", "{msg.timestamp}" }
                                        }
                                    }
                                } else if let Some(ref transfer) = msg.file_transfer {
                                    div { class: "py-2 px-3 bg-slate-100/80 border border-slate-200 rounded text-slate-700 font-bold flex flex-col space-y-1.5 my-1 shadow-sm",
                                        div { class: "flex items-center space-x-2",
                                            span { "📂" }
                                            span { "{msg.sender_name} enviou um convite de arquivo." }
                                            span { class: "text-[9px] text-slate-500 font-normal ml-auto", "{msg.timestamp}" }
                                        }
                                        {
                                            match transfer {
                                                FileTransferState::Waiting => {
                                                    if msg.sender_id != 0 {
                                                        rsx! {
                                                            div { class: "flex items-center space-x-2 text-[11px] font-normal pt-1",
                                                                span { "Arquivo pendente: {msg.text}" }
                                                                button {
                                                                    class: "px-2 py-0.5 bg-gradient-to-b from-[#8fc1e9] to-[#4585c5] text-white rounded border border-[#4074a8] hover:from-[#9bd0fa] hover:to-[#579adf] font-bold cursor-pointer transition-colors",
                                                                    onclick: move |_| state.accept_file_transfer(contact_id, msg.id),
                                                                    "Aceitar"
                                                                }
                                                                button {
                                                                    class: "px-2 py-0.5 bg-white hover:bg-slate-100 border border-slate-350 rounded cursor-pointer transition-colors",
                                                                    onclick: move |_| state.reject_file_transfer(contact_id, msg.id),
                                                                    "Recusar"
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        rsx! { span { class: "text-[11px] font-normal text-slate-500", "Aguardando resposta do contato..." } }
                                                    }
                                                }
                                                FileTransferState::Downloading(prog) => {
                                                    rsx! {
                                                        div { class: "flex flex-col space-y-1 pt-1 font-normal text-[11px] w-full",
                                                            span { "Baixando: {prog}%" }
                                                            div { class: "w-full h-2 bg-white rounded-full overflow-hidden border border-slate-300",
                                                                div { class: "h-full bg-gradient-to-r from-sky-400 to-sky-600 transition-all duration-300", style: "width: {prog}%;" }
                                                            }
                                                        }
                                                    }
                                                }
                                                FileTransferState::Completed(filename) => {
                                                    let is_image_visible = show_images().contains(&msg.id);
                                                    rsx! {
                                                        div { class: "flex flex-col space-y-1 pt-1 font-normal text-[11px]",
                                                            div { class: "flex items-center space-x-2",
                                                                span { "✓ Transferência Concluída: {filename}" }
                                                                if filename.ends_with(".jpg") {
                                                                    button {
                                                                        class: "text-[#0066cc] hover:underline font-bold cursor-pointer transition-all",
                                                                        onclick: move |_| {
                                                                            if show_images().contains(&msg.id) {
                                                                                show_images.write().remove(&msg.id);
                                                                            } else {
                                                                                show_images.write().insert(msg.id);
                                                                            }
                                                                        },
                                                                        if is_image_visible { "Ocultar Foto" } else { "Visualizar Foto" }
                                                                    }
                                                                }
                                                            }
                                                            if is_image_visible {
                                                                    div { class: "w-32 h-32 border border-slate-300 rounded mt-1 bg-white p-1 shadow flex items-center justify-center overflow-hidden",
                                                                        img {
                                                                            src: "https://picsum.photos/150/150?random={msg.id}",
                                                                            class: "w-full h-full object-cover rounded-sm"
                                                                        }
                                                                    }
                                                            }
                                                        }
                                                    }
                                                }
                                                FileTransferState::Rejected => {
                                                    rsx! { span { class: "text-[11px] font-normal text-red-500/80 italic", "Transferência cancelada ou rejeitada." } }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    div { class: "flex items-baseline space-x-1.5",
                                        span { class: "font-bold {name_color}", "{msg.sender_name}" }
                                        span { class: "text-[9px] text-slate-400 font-normal", "[{msg.timestamp}] diz:" }
                                    }
                                    p {
                                        class: "pl-2 select-text",
                                        style: "font-family: {msg.font_family}; color: {msg.font_color}; font-size: 13px;",
                                        {format_message_text(&msg.text)}
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
