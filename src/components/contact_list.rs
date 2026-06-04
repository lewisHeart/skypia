use dioxus::prelude::*;
use crate::state::AppState;
use crate::models::{Contact, UserStatus};
use crate::components::contact_row::ContactRow;

#[component]
pub fn ContactList(mut state: AppState) -> Element {
    let mut search_query = use_signal(|| String::new());
    let mut fav_collapsed = use_signal(|| false);
    let mut online_collapsed = use_signal(|| false);
    let mut offline_collapsed = use_signal(|| false);

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

    rsx! {
        div { class: "flex flex-col flex-1 min-h-0",
            // Search Input Container
            div { class: "px-3 py-1.5 flex items-center relative",
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

            // Scroll Area
            div { class: "flex-1 overflow-y-auto px-1 py-2 space-y-3 bg-white/35",
                // Group: Favorites
                div { class: "space-y-1",
                    div { 
                        class: "flex items-center space-x-1 px-2 py-0.5 hover:bg-white/30 rounded cursor-pointer transition-colors text-xs font-bold text-[#1b324d]/85",
                        onclick: move |_| fav_collapsed.set(!fav_collapsed()),
                        span { class: "w-3 text-center text-[10px] text-slate-500", if fav_collapsed() { "▶" } else { "▼" } }
                        span { "Favoritos ({favorites().len()})" }
                    }
                    
                    if !fav_collapsed() {
                        div { class: "pl-2 space-y-0.5",
                            if favorites().is_empty() {
                                div { class: "text-[10px] text-slate-500/80 italic pl-5 py-1", "Nenhum contato favorito" }
                            } else {
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
        }
    }
}
