use dioxus::prelude::*;
use crate::state::AppState;
use crate::models::{Contact, UserStatus};
use crate::components::main::contact_row::ContactRow;

#[component]
pub fn ContactList(mut state: AppState) -> Element {
    let theme = state.theme();
    let mut search_query = use_signal(|| String::new());
    let mut fav_collapsed = use_signal(|| false);
    let mut online_collapsed = use_signal(|| false);
    let mut offline_collapsed = use_signal(|| false);
    let mut groups_collapsed = use_signal(|| false);
    let mut show_create_group_modal = use_signal(|| false);
    let mut group_name = use_signal(|| String::new());
    let mut group_desc = use_signal(|| String::new());
    let mut group_avatar = use_signal(|| String::new());
    let mut selected_emails = use_signal(|| Vec::<String>::new());
    let group_chats = state.group_chats;
    let mut active_category_menu = use_signal(|| Option::<(String, i32, i32)>::None);
    let fav_density = state.fav_density();
    let online_density = state.online_density();
    let offline_density = state.offline_density();

    let filtered_contacts = use_memo(move || {
        let query = search_query().to_lowercase();
        let list = state.contacts();
        if query.is_empty() {
            list
        } else {
            list.into_iter()
                .filter(|c| {
                    c.display_name.to_lowercase().contains(&query)
                    || c.email.to_lowercase().contains(&query)
                    || c.nickname.as_ref().map(|n| n.to_lowercase().contains(&query)).unwrap_or(false)
                })
                .collect()
        }
    });

    let favorites = use_memo(move || {
        filtered_contacts()
            .into_iter()
            .filter(|c| c.is_favorite)
            .collect::<Vec<Contact>>()
    });

    let favorites_online_count = use_memo(move || {
        favorites()
            .iter()
            .filter(|c| c.status != UserStatus::Offline && c.status != UserStatus::Invisivel)
            .count()
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

    let favorites_unread_count = use_memo(move || {
        favorites()
            .iter()
            .map(|c| state.unread_count_for(&c.id))
            .sum::<usize>()
    });

    let favorites_unread_text = use_memo(move || {
        let unread = favorites_unread_count();
        if unread == 0 {
            "".to_string()
        } else if unread == 1 {
            " • 1 nova".to_string()
        } else {
            format!(" • {} novas", unread)
        }
    });

    let online_unread_count = use_memo(move || {
        online_contacts()
            .iter()
            .map(|c| state.unread_count_for(&c.id))
            .sum::<usize>()
    });

    let online_unread_text = use_memo(move || {
        let unread = online_unread_count();
        if unread == 0 {
            "".to_string()
        } else if unread == 1 {
            " • 1 nova".to_string()
        } else {
            format!(" • {} novas", unread)
        }
    });

    let offline_unread_count = use_memo(move || {
        offline_contacts()
            .iter()
            .map(|c| state.unread_count_for(&c.id))
            .sum::<usize>()
    });

    let offline_unread_text = use_memo(move || {
        let unread = offline_unread_count();
        if unread == 0 {
            "".to_string()
        } else if unread == 1 {
            " • 1 nova".to_string()
        } else {
            format!(" • {} novas", unread)
        }
    });

    let self_id = state.server_user_id();
    let online_groups_count = use_memo(move || {
        let groups = group_chats();
        let contacts = state.contacts();
        let s_id = self_id.clone();
        
        groups.iter().filter(|g| {
            g.members.iter().any(|m| {
                if Some(m.id.clone()) == s_id {
                    true
                } else if let Some(c) = contacts.iter().find(|c| c.id == m.id) {
                    c.status != UserStatus::Offline && c.status != UserStatus::Invisivel
                } else {
                    false
                }
            })
        }).count()
    });

    rsx! {
        div { class: "flex flex-col flex-1 min-h-0",
            // Search Input Container com o botão Adicionar Contato do design do usuário
            div { class: "px-3.5 py-2.5 flex items-center flex-shrink-0 relative",
                div { class: "relative flex-1 flex items-center",
                    input { 
                        class: "w-full pl-2.5 pr-7 h-[27px] text-[11px] rounded-[4px] border border-[#d1d1d1] focus:border-slate-400 msn-input placeholder-[#a5a5a5]",
                        placeholder: "Procure um contato...",
                        value: "{search_query}",
                        oninput: move |e| search_query.set(e.value()),
                    }
                    span { class: "absolute right-2.5 text-xs text-slate-400 pointer-events-none", "🔍" }
                }
                button {
                    class: "w-[30px] h-[20px] bg-transparent hover:bg-black/5 flex items-center justify-center rounded cursor-pointer transition-colors focus:outline-none flex-shrink-0 ml-2",
                    title: "Adicionar contato",
                    onclick: move |_| {
                        state.show_add_contact_modal.set(true);
                    },
                    svg {
                        view_box: "0 0 24 24",
                        class: "w-4.5 h-4.5 select-none pointer-events-none stroke-current text-[#4f5d73]/90 fill-none",
                        path { d: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2", stroke_width: "1.8", stroke_linecap: "round", stroke_linejoin: "round" }
                        circle { cx: "9", cy: "7", r: "4", stroke_width: "1.8", stroke_linecap: "round", stroke_linejoin: "round" }
                        line { x1: "19", y1: "8", x2: "19", y2: "14", stroke: "#4aa333", stroke_width: "2.5", stroke_linecap: "round" }
                        line { x1: "16", y1: "11", x2: "22", y2: "11", stroke: "#4aa333", stroke_width: "2.5", stroke_linecap: "round" }
                    }
                }
            }

            // Scroll Area
            div { 
                class: "flex-1 overflow-y-auto px-1 py-2 space-y-3 bg-transparent",
                onmouseup: move |_| {
                    if (state.dragged_contact_id)().is_some() {
                        *state.dragged_contact_id.write() = None;
                    }
                },
                // Seção de Solicitações Pendentes (Discreta e Inline na Lista de Contatos!)
                if !state.pending_requests().is_empty() {
                    div { class: "space-y-1 bg-amber-50/25 border border-amber-200/40 rounded p-1.5 my-1 mx-1.5 shadow-sm",
                        div { class: "flex items-center space-x-1.5 px-2 py-0.5 text-xs font-bold text-amber-800",
                            span { "👥" }
                            span { "Solicitações Pendentes ({state.pending_requests().len()})" }
                        }
                        div { class: "pl-2 space-y-1.5 pt-1",
                            for request in state.pending_requests() {
                                {
                                    let req_id_accept = request.id.clone();
                                    let req_id_reject = request.id.clone();
                                    let name = request.nickname.clone().unwrap_or(request.display_name.clone());
                                    rsx! {
                                        div { class: "flex items-center justify-between p-1.5 bg-white/60 rounded border border-slate-200 shadow-sm text-[11px]",
                                            div { class: "flex flex-col min-w-0 mr-2",
                                                span { class: "font-semibold text-slate-800 truncate", "{name}" }
                                                span { class: "text-[9px] text-slate-500 truncate", "{request.email}" }
                                            }
                                            div { class: "flex space-x-1 flex-shrink-0",
                                                button {
                                                    class: "px-2 py-0.5 bg-green-600 hover:bg-green-700 text-white rounded text-[9px] font-bold cursor-pointer transition-colors shadow-sm focus:outline-none",
                                                    onclick: move |_| {
                                                        state.accept_friend_request(req_id_accept.clone());
                                                    },
                                                    "Aceitar"
                                                }
                                                button {
                                                    class: "px-2 py-0.5 bg-red-600 hover:bg-red-700 text-white rounded text-[9px] font-bold cursor-pointer transition-colors shadow-sm focus:outline-none",
                                                    onclick: move |_| {
                                                        state.reject_friend_request(req_id_reject.clone());
                                                    },
                                                    "Recusar"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // Group: Favorites
                div { 
                    class: "space-y-1",
                    onmouseup: move |e| {
                        e.stop_propagation();
                        if let Some(cid) = (state.dragged_contact_id)() {
                            if let Some(c) = state.contacts().iter().find(|c| c.id == cid) {
                                if !c.is_favorite {
                                    state.toggle_favorite(cid);
                                }
                            }
                            *state.dragged_contact_id.write() = None;
                        }
                    },
                    div { 
                        class: "flex items-center space-x-1 px-2 py-0.5 hover:bg-white/30 rounded cursor-pointer transition-colors text-xs font-bold {theme.titlebar_text()}",
                        style: "opacity: 0.85;",
                        onclick: move |_| fav_collapsed.set(!fav_collapsed()),
                        oncontextmenu: move |e| {
                            e.prevent_default();
                            let scale = state.interface_scale();
                            let x = (e.client_coordinates().x as f64 / scale) as i32;
                            let y = (e.client_coordinates().y as f64 / scale) as i32;
                            active_category_menu.set(Some(("fav".to_string(), x, y)));
                        },
                        span { class: "w-3 text-center text-[10px] text-slate-500", if fav_collapsed() { "▶" } else { "▼" } }
                        span { class: "font-bold text-[11px] text-[#2d517a] mr-1", "Favoritos" }
                        span { class: "font-normal text-[10px] text-[#a5a5a5]", "({favorites_online_count()}/{favorites().len()}){favorites_unread_text}" }
                    }
                    
                    if !fav_collapsed() {
                        div { class: "pl-2 space-y-0.5",
                            if favorites().is_empty() {
                                div { class: "text-[10px] text-slate-500/80 italic pl-5 py-1", "Nenhum contato favorito" }
                            } else {
                                for contact in favorites() {
                                    ContactRow { contact, state, density: fav_density.clone() }
                                }
                            }
                        }
                    }
                }

                // Group: Grupos
                div { class: "space-y-1",
                    div { 
                        class: "flex items-center space-x-1 px-2 py-0.5 hover:bg-white/30 rounded cursor-pointer transition-colors text-xs font-bold {theme.titlebar_text()} justify-between",
                        style: "opacity: 0.85;",
                        onclick: move |_| groups_collapsed.set(!groups_collapsed()),
                        oncontextmenu: move |e| {
                            e.prevent_default();
                            let scale = state.interface_scale();
                            let x = (e.client_coordinates().x as f64 / scale) as i32;
                            let y = (e.client_coordinates().y as f64 / scale) as i32;
                            active_category_menu.set(Some(("groups".to_string(), x, y)));
                        },
                        div { class: "flex items-center space-x-1.5",
                            span { class: "w-3 text-center text-[10px] text-slate-500", if groups_collapsed() { "▶" } else { "▼" } }
                            span { class: "font-bold text-[11px] text-[#2d517a] mr-1", "Grupos" }
                            span { class: "font-normal text-[10px] text-[#a5a5a5]", "({online_groups_count()}/{group_chats().len()})" }
                        }
                        button {
                            class: "text-[10px] text-[#1f92d6] hover:text-[#1871a6] font-bold px-1.5 py-0.5 rounded hover:bg-white/50 cursor-pointer focus:outline-none transition-colors",
                            onclick: move |e| {
                                e.stop_propagation();
                                show_create_group_modal.set(true);
                            },
                            "Novo Grupo"
                        }
                    }
                    
                    if !groups_collapsed() {
                        div { class: "pl-2 space-y-0.5",
                            if group_chats().is_empty() {
                                div { class: "text-[10px] text-slate-500/80 italic pl-5 py-1", "Nenhum grupo de chat" }
                            } else {
                                for group in group_chats() {
                                    GroupRow { group, state, density: state.groups_density() }
                                }
                            }
                        }
                    }
                }

                // Group: Online
                div { 
                    class: "space-y-1",
                    onmouseup: move |e| {
                        e.stop_propagation();
                        if let Some(cid) = (state.dragged_contact_id)() {
                            if let Some(c) = state.contacts().iter().find(|c| c.id == cid) {
                                if c.is_favorite {
                                    state.toggle_favorite(cid);
                                }
                            }
                            *state.dragged_contact_id.write() = None;
                        }
                    },
                    div { 
                        class: "flex items-center space-x-1 px-2 py-0.5 hover:bg-white/30 rounded cursor-pointer transition-colors text-xs font-bold {theme.titlebar_text()}",
                        style: "opacity: 0.85;",
                        onclick: move |_| online_collapsed.set(!online_collapsed()),
                        oncontextmenu: move |e| {
                            e.prevent_default();
                            let scale = state.interface_scale();
                            let x = (e.client_coordinates().x as f64 / scale) as i32;
                            let y = (e.client_coordinates().y as f64 / scale) as i32;
                            active_category_menu.set(Some(("online".to_string(), x, y)));
                        },
                        span { class: "w-3 text-center text-[10px] text-slate-500", if online_collapsed() { "▶" } else { "▼" } }
                        span { class: "font-bold text-[11px] text-[#2d517a] mr-1", "Disponíveis" }
                        span { class: "font-normal text-[10px] text-[#a5a5a5]", "({online_contacts().len()}/{online_contacts().len()}){online_unread_text}" }
                    }
                    
                    if !online_collapsed() {
                        div { class: "pl-2 space-y-0.5",
                            if online_contacts().is_empty() {
                                div { class: "text-[10px] text-slate-500/80 italic pl-5 py-1", "Nenhum contato online" }
                            } else {
                                for contact in online_contacts() {
                                    ContactRow { contact, state, density: online_density.clone() }
                                }
                            }
                        }
                    }
                }

                // Group: Offline
                div { 
                    class: "space-y-1",
                    onmouseup: move |e| {
                        e.stop_propagation();
                        if let Some(cid) = (state.dragged_contact_id)() {
                            if let Some(c) = state.contacts().iter().find(|c| c.id == cid) {
                                if c.is_favorite {
                                    state.toggle_favorite(cid);
                                }
                            }
                            *state.dragged_contact_id.write() = None;
                        }
                    },
                    div { 
                        class: "flex items-center space-x-1 px-2 py-0.5 hover:bg-white/30 rounded cursor-pointer transition-colors text-xs font-bold {theme.titlebar_text()}",
                        style: "opacity: 0.85;",
                        onclick: move |_| offline_collapsed.set(!offline_collapsed()),
                        oncontextmenu: move |e| {
                            e.prevent_default();
                            let scale = state.interface_scale();
                            let x = (e.client_coordinates().x as f64 / scale) as i32;
                            let y = (e.client_coordinates().y as f64 / scale) as i32;
                            active_category_menu.set(Some(("offline".to_string(), x, y)));
                        },
                        span { class: "w-3 text-center text-[10px] text-slate-500", if offline_collapsed() { "▶" } else { "▼" } }
                        span { class: "font-bold text-[11px] text-[#2d517a] mr-1", "Offline" }
                        span { class: "font-normal text-[10px] text-[#a5a5a5]", "(0/{offline_contacts().len()}){offline_unread_text}" }
                    }
                    
                    if !offline_collapsed() {
                        div { class: "pl-2 space-y-0.5",
                            for contact in offline_contacts() {
                                ContactRow { contact, state, density: offline_density.clone() }
                            }
                        }
                    }
                }
            }



            // Modal de Criação de Grupo
            if show_create_group_modal() {
                div {
                    class: "fixed inset-0 bg-black/45 backdrop-blur-sm z-[200] flex items-center justify-center p-4 select-none cursor-default",
                    onclick: move |_| show_create_group_modal.set(false),
                    div {
                        class: "w-80 bg-gradient-to-b {theme.modal_gradient()} border {theme.modal_border()} rounded-lg shadow-2xl p-4 flex flex-col space-y-3.5 text-xs {theme.titlebar_text()} pointer-events-auto",
                        onclick: move |e| e.stop_propagation(),

                        div { class: "flex items-center justify-between border-b {theme.titlebar_border()} pb-2",
                            div { class: "flex items-center space-x-1.5 font-bold text-sm {theme.titlebar_text()}",
                                span { "👥" }
                                span { "Criar Novo Grupo" }
                            }
                            button {
                                class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-500 hover:text-white border border-transparent font-bold cursor-pointer transition-colors focus:outline-none",
                                onclick: move |_| {
                                    show_create_group_modal.set(false);
                                    group_name.set(String::new());
                                    group_desc.set(String::new());
                                    group_avatar.set(String::new());
                                    selected_emails.write().clear();
                                },
                                "✕"
                            }
                        }

                        div { class: "flex flex-col space-y-1.5",
                            label { class: "font-semibold text-slate-700", "Nome do Grupo:" }
                            input {
                                class: "w-full px-2.5 py-1.5 border {theme.titlebar_border()} rounded bg-white focus:outline-none focus:border-slate-450 text-xs text-slate-800",
                                placeholder: "Digite o nome do grupo...",
                                value: "{group_name}",
                                oninput: move |e| group_name.set(e.value()),
                            }
                        }

                        div { class: "flex flex-col space-y-1.5",
                            label { class: "font-semibold text-slate-700", "Descrição do Grupo (opcional):" }
                            input {
                                class: "w-full px-2.5 py-1.5 border {theme.titlebar_border()} rounded bg-white focus:outline-none focus:border-slate-450 text-xs text-slate-800",
                                placeholder: "Uma breve descrição do grupo...",
                                value: "{group_desc}",
                                oninput: move |e| group_desc.set(e.value()),
                            }
                        }

                        div { class: "flex flex-col space-y-1.5",
                            label { class: "font-semibold text-slate-700", "URL da Foto do Grupo (opcional):" }
                            input {
                                class: "w-full px-2.5 py-1.5 border {theme.titlebar_border()} rounded bg-white focus:outline-none focus:border-slate-450 text-xs text-slate-800",
                                placeholder: "https://exemplo.com/foto.jpg",
                                value: "{group_avatar}",
                                oninput: move |e| group_avatar.set(e.value()),
                            }
                        }

                        div { class: "flex flex-col space-y-1.5 flex-1 min-h-0",
                            label { class: "font-semibold text-slate-700", "Selecione os contatos para adicionar:" }
                            div { class: "border rounded bg-white/50 p-2 overflow-y-auto max-h-40 flex flex-col space-y-1.5",
                                if state.contacts().is_empty() {
                                    div { class: "text-center text-slate-500 py-4 italic", "Nenhum contato disponível." }
                                } else {
                                    for contact in state.contacts() {
                                        {
                                            let email = contact.email.clone();
                                            let name = contact.nickname.clone().unwrap_or(contact.display_name.clone());
                                            let is_checked = selected_emails().contains(&email);
                                            rsx! {
                                                label { class: "flex items-center space-x-2 cursor-pointer hover:bg-white/45 p-1 rounded",
                                                    input {
                                                        type: "checkbox",
                                                        checked: is_checked,
                                                        class: "cursor-pointer",
                                                        onchange: move |_| {
                                                            let mut list = selected_emails.write();
                                                            if list.contains(&email) {
                                                                list.retain(|e| e != &email);
                                                            } else {
                                                                list.push(email.clone());
                                                            }
                                                        }
                                                    }
                                                    div { class: "flex flex-col min-w-0",
                                                        span { class: "font-medium text-slate-800 truncate", "{name}" }
                                                        span { class: "text-[9px] text-slate-500 truncate", "{contact.email}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "flex justify-end space-x-2 pt-2 border-t {theme.titlebar_border()}/40",
                            button {
                                class: "px-4 py-1.5 {theme.btn_primary()} rounded font-bold shadow hover:brightness-105 cursor-pointer transition-colors focus:outline-none",
                                disabled: group_name().trim().is_empty() || selected_emails().is_empty(),
                                onclick: move |_| {
                                    state.create_group_chat(
                                        group_name().trim().to_string(),
                                        group_desc().trim().to_string(),
                                        group_avatar().trim().to_string(),
                                        selected_emails().clone()
                                    );
                                    show_create_group_modal.set(false);
                                    group_name.set(String::new());
                                    group_desc.set(String::new());
                                    group_avatar.set(String::new());
                                    selected_emails.write().clear();
                                },
                                "Criar"
                            }
                            button {
                                class: "px-4 py-1.5 bg-gradient-to-b from-slate-200 to-slate-300 hover:from-slate-300 hover:to-slate-400 text-slate-700 rounded font-bold shadow border border-slate-400/40 cursor-pointer transition-all focus:outline-none",
                                onclick: move |_| {
                                    show_create_group_modal.set(false);
                                    group_name.set(String::new());
                                    group_desc.set(String::new());
                                    group_avatar.set(String::new());
                                    selected_emails.write().clear();
                                },
                                "Cancelar"
                            }
                        }
                    }
                }
            }

            // Menu de Contexto para Categorias (Densidade)
            if let Some((cat, x, y)) = active_category_menu() {
                div {
                    class: "fixed inset-0 z-[9998] bg-transparent cursor-default",
                    onclick: move |e| {
                        e.stop_propagation();
                        active_category_menu.set(None);
                    }
                }
                div { 
                    class: "fixed w-44 bg-white/95 border border-slate-300 rounded-lg shadow-2xl backdrop-blur-md z-[9999] p-1 flex flex-col text-[11px] text-slate-700 transition-all select-none cursor-default",
                    style: "left: {x}px; top: {y}px;",
                    onclick: move |e| e.stop_propagation(),
                    onmouseleave: move |_| active_category_menu.set(None),
                    
                    div { class: "px-2 py-1 text-[9px] font-bold text-slate-400 border-b border-slate-100", "Mudar Densidade" }

                    if cat == "fav" {
                        button { 
                            class: "px-2.5 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-medium transition-colors",
                            onclick: move |_| {
                                active_category_menu.set(None);
                                state.set_category_density("fav", "medium".to_string());
                            },
                            span { if fav_density == "medium" { "🟢" } else { "⚪" } }
                            span { "Média (Padrão)" }
                        }
                        button { 
                            class: "px-2.5 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-medium transition-colors",
                            onclick: move |_| {
                                active_category_menu.set(None);
                                state.set_category_density("fav", "large".to_string());
                            },
                            span { if fav_density == "large" { "🟢" } else { "⚪" } }
                            span { "Grande (Espaçado)" }
                        }
                    } else {
                        {
                            let current_density = match cat.as_str() {
                                "online" => online_density.clone(),
                                "offline" => offline_density.clone(),
                                _ => state.groups_density(),
                            };
                            let cat_clone1 = cat.clone();
                            let cat_clone2 = cat.clone();
                            rsx! {
                                button { 
                                    class: "px-2.5 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-medium transition-colors",
                                    onclick: move |_| {
                                        active_category_menu.set(None);
                                        state.set_category_density(&cat_clone1, "small".to_string());
                                    },
                                    span { if current_density == "small" { "🟢" } else { "⚪" } }
                                    span { "Pequena (Compacto)" }
                                }
                                button { 
                                    class: "px-2.5 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-medium transition-colors",
                                    onclick: move |_| {
                                        active_category_menu.set(None);
                                        state.set_category_density(&cat_clone2, "medium".to_string());
                                    },
                                    span { if current_density == "medium" { "🟢" } else { "⚪" } }
                                    span { "Média (Padrão)" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn GroupRow(group: crate::models::Conversation, mut state: AppState, density: String) -> Element {
    let theme = state.theme();
    let group_id = group.id.clone();
    let group_id_double = group_id.clone();
    
    let handle_double_click = move |_| {
        state.open_chat(group_id_double.clone());
    };

    let online_members = group.members.iter().filter(|m| {
        if Some(m.id.clone()) == state.server_user_id() {
            state.user_status() != UserStatus::Offline
        } else if let Some(c) = state.contacts().iter().find(|c| c.id == m.id) {
            c.status != UserStatus::Offline
        } else {
            m.status != "Offline" && m.status != "Invisivel"
        }
    }).count();

    let group_svg = if online_members > 0 {
        asset!("/assets/status/Disponível Grupo.svg")
    } else {
        asset!("/assets/status/Offline Grupo.svg")
    };

    let container_padding = if density == "small" { "py-0.5 px-1.5" } else { "py-1 px-1.5" };

    rsx! {
        div {
            class: "flex items-center space-x-1.5 {container_padding} rounded hover:bg-white/45 cursor-pointer relative group transition-colors",
            ondoubleclick: handle_double_click,
            onmouseup: {
                let gid = group_id.clone();
                let members_list = group.members.clone();
                move |e| {
                    e.stop_propagation();
                    if let Some(cid) = (state.dragged_contact_id)() {
                        if let Some(c) = state.contacts().iter().find(|c| c.id == cid) {
                            if !members_list.iter().any(|m| m.id == cid) {
                                state.add_group_member(gid.clone(), c.email.clone());
                            }
                        }
                        *state.dragged_contact_id.write() = None;
                    }
                }
            },
            
            img {
                src: group_svg,
                class: "w-[14px] h-[12px] object-contain flex-shrink-0 select-none mr-1.5"
            }

            div { class: "flex-1 min-w-0 flex items-center space-x-1 text-xs",
                span { class: "font-semibold text-xs {theme.titlebar_text()} truncate hover:underline flex-shrink-0", 
                    "{group.name.as_deref().unwrap_or(\"Grupo sem nome\")}" 
                }
                span { class: "text-[10px] text-slate-400 font-normal flex-shrink-0", 
                    "({online_members}/{group.members.len()} Disponível)" 
                }
            }
        }
    }
}
