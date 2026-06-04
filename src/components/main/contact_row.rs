use dioxus::prelude::*;
use crate::state::AppState;
use crate::models::{Contact, render_avatar};

#[component]
pub fn ContactRow(contact: Contact, mut state: AppState) -> Element {
    let mut show_tooltip = use_signal(|| false);
    let mut tooltip_y = use_signal(|| 0i32);

    let handle_double_click = move |_| {
        state.open_chat(contact.id);
    };

    rsx! {
        div {
            class: "flex items-center space-x-2.5 p-1 rounded hover:bg-white/45 cursor-pointer relative group transition-colors",
            ondoubleclick: handle_double_click,
            onmouseenter: move |e| {
                tooltip_y.set(e.client_coordinates().y as i32);
                show_tooltip.set(true);
            },
            onmousemove: move |e| {
                tooltip_y.set(e.client_coordinates().y as i32);
            },
            onmouseleave: move |_| show_tooltip.set(false),
            
            // Status Icon Buddy Dot
            div { class: "relative flex-shrink-0",
                div { class: "w-3 h-3 rounded-full {contact.status.color_class()} border border-black/10 shadow-sm" }
            }
            
            // Small Avatar with fixed border
            div { 
                class: "flex-shrink-0 shadow-sm rounded-[4px] border border-slate-300/60 overflow-hidden bg-transparent",
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
                    class: "fixed left-[360px] w-64 bg-gradient-to-b from-sky-50 to-sky-100/95 border border-[#a6b9cd] rounded-lg shadow-xl z-50 p-3 flex flex-col space-y-2 text-xs text-slate-700 pointer-events-none",
                    style: "top: {tooltip_y - 45}px;",
                    div { class: "flex items-start space-x-3",
                        // Tooltip Avatar with fixed border
                        div { 
                            class: "flex-shrink-0 shadow rounded-[6px] border border-slate-300/70 overflow-hidden bg-transparent",
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
