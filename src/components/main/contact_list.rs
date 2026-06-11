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

    let mut collapsed_categories = use_signal(|| std::collections::HashSet::<String>::new());
    
    let is_collapsed = move |cat: &str| collapsed_categories.read().contains(cat);
    let mut toggle_collapsed = move |cat: &str| {
        let mut set = collapsed_categories.write();
        if set.contains(cat) {
            set.remove(cat);
        } else {
            set.insert(cat.to_string());
        }
    };

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

    let contacts_by_category = move |cat: &str| {
        filtered_contacts()
            .into_iter()
            .filter(|c| c.category_name.as_deref() == Some(cat))
            .collect::<Vec<Contact>>()
    };

    let category_online_count = move |cat: &str| {
        filtered_contacts()
            .iter()
            .filter(|c| c.category_name.as_deref() == Some(cat) && c.status != UserStatus::Offline && c.status != UserStatus::Invisivel)
            .count()
    };

    let category_unread_count = move |cat: &str| {
        filtered_contacts()
            .iter()
            .filter(|c| c.category_name.as_deref() == Some(cat))
            .map(|c| state.unread_count_for(&c.id))
            .sum::<usize>()
    };

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
            .filter(|c| !c.is_favorite && c.category_name.is_none() && c.status != UserStatus::Offline && c.status != UserStatus::Invisivel)
            .collect::<Vec<Contact>>()
    });

    let offline_contacts = use_memo(move || {
        filtered_contacts()
            .into_iter()
            .filter(|c| !c.is_favorite && c.category_name.is_none() && (c.status == UserStatus::Offline || c.status == UserStatus::Invisivel))
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
                    img {
                        src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Person%20Add/SVG/ic_fluent_person_add_24_color.svg",
                        class: "w-5 h-5 select-none pointer-events-none"
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
                // Seção de Solicitações Pendentes (Aviso discreto no topo!)
                if !state.pending_requests().is_empty() {
                    div { 
                        class: "mx-1.5 my-1 px-3 py-2 bg-[#f4fafe] hover:bg-sky-100/60 border border-sky-200/50 rounded flex items-center justify-between text-[11px] text-[#2d517a] font-semibold cursor-pointer shadow-sm transition-all animate-pulse",
                        onclick: move |_| {
                            state.show_friend_requests_modal.set(true);
                        },
                        div { class: "flex items-center space-x-2",
                            span { "✉" }
                            span { "Você tem {state.pending_requests().len()} solicitação(ões) de amizade pendente(s)." }
                        }
                        span { class: "text-[9px] text-sky-600 hover:underline font-bold", "Visualizar" }
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
                            let offset_y = if state.use_custom_titlebar() { 40.0 } else { 0.0 };
                            let x = (e.client_coordinates().x as f64 / scale) as i32;
                            let y = ((e.client_coordinates().y as f64 - offset_y) / scale) as i32;
                            active_category_menu.set(Some(("fav".to_string(), x, y)));
                        },
                        span { class: "w-3 text-center text-[10px] text-slate-500", if fav_collapsed() { "▶" } else { "▼" } }
                        span { class: "font-bold text-[11px] text-[#2d517a] mr-1 flex items-center space-x-0.5",
                            img {
                                src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Star/SVG/ic_fluent_star_16_color.svg",
                                class: "w-3.5 h-3.5 object-contain inline-block mr-1 pointer-events-none"
                            }
                            span { "Favoritos" }
                        }
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
                            let offset_y = if state.use_custom_titlebar() { 40.0 } else { 0.0 };
                            let x = (e.client_coordinates().x as f64 / scale) as i32;
                            let y = ((e.client_coordinates().y as f64 - offset_y) / scale) as i32;
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

                // Categorias Dinâmicas Customizadas
                for cat in state.categories() {
                    {
                        let cat_name = cat.clone();
                        let cat_name_toggle = cat.clone();
                        let cat_name_menu = cat.clone();
                        let cat_contacts = contacts_by_category(&cat_name);
                        let online_count = category_online_count(&cat_name);
                        let unread_count = category_unread_count(&cat_name);
                        let unread_text = if unread_count == 0 {
                            "".to_string()
                        } else if unread_count == 1 {
                            " • 1 nova".to_string()
                        } else {
                            format!(" • {} novas", unread_count)
                        };
                        let collapsed = is_collapsed(&cat_name);
                        
                        rsx! {
                            div { 
                                class: "space-y-1",
                                onmouseup: {
                                    let cat_name_drop = cat_name.clone();
                                    move |e| {
                                        e.stop_propagation();
                                        if let Some(cid) = (state.dragged_contact_id)() {
                                            let mut s = state;
                                            s.update_contact_category(cid.clone(), Some(cat_name_drop.clone()));
                                            if let Some(c) = s.contacts().iter().find(|c| c.id == cid) {
                                                if c.is_favorite {
                                                    s.toggle_favorite(cid.clone());
                                                }
                                            }
                                            *state.dragged_contact_id.write() = None;
                                        }
                                    }
                                },
                                div { 
                                    class: "flex items-center space-x-1 px-2 py-0.5 hover:bg-white/30 rounded cursor-pointer transition-colors text-xs font-bold {theme.titlebar_text()} justify-between",
                                    style: "opacity: 0.85;",
                                    onclick: move |_| toggle_collapsed(&cat_name_toggle),
                                    
                                    div { class: "flex items-center space-x-1.5",
                                        span { class: "w-3 text-center text-[10px] text-slate-500", if collapsed { "▶" } else { "▼" } }
                                        span { class: "font-bold text-[11px] text-[#2d517a] mr-1", "📂 {cat_name}" }
                                        span { class: "font-normal text-[10px] text-[#a5a5a5]", "({online_count}/{cat_contacts.len()}){unread_text}" }
                                    }
                                    button {
                                        class: "text-[9px] text-red-500 hover:text-red-700 font-bold px-1.5 py-0.5 rounded hover:bg-white/50 cursor-pointer focus:outline-none transition-colors",
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            state.delete_category(cat_name_menu.clone());
                                        },
                                        "Excluir"
                                    }
                                }
                                
                                if !collapsed {
                                    div { class: "pl-2 space-y-0.5",
                                        if cat_contacts.is_empty() {
                                            div { class: "text-[10px] text-slate-500/80 italic pl-5 py-1", "Nenhum contato nesta categoria" }
                                        } else {
                                            for contact in cat_contacts {
                                                ContactRow { contact, state, density: online_density.clone() }
                                            }
                                        }
                                    }
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
                            let mut s = state;
                            if let Some(c) = s.contacts().iter().find(|c| c.id == cid) {
                                if c.is_favorite {
                                    s.toggle_favorite(cid.clone());
                                }
                                if c.category_name.is_some() {
                                    s.update_contact_category(cid.clone(), None);
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
                            let offset_y = if state.use_custom_titlebar() { 40.0 } else { 0.0 };
                            let x = (e.client_coordinates().x as f64 / scale) as i32;
                            let y = ((e.client_coordinates().y as f64 - offset_y) / scale) as i32;
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
                            let mut s = state;
                            if let Some(c) = s.contacts().iter().find(|c| c.id == cid) {
                                if c.is_favorite {
                                    s.toggle_favorite(cid.clone());
                                }
                                if c.category_name.is_some() {
                                    s.update_contact_category(cid.clone(), None);
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
                            let offset_y = if state.use_custom_titlebar() { 40.0 } else { 0.0 };
                            let x = (e.client_coordinates().x as f64 / scale) as i32;
                            let y = ((e.client_coordinates().y as f64 - offset_y) / scale) as i32;
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
                    class: "fixed inset-0 bg-black/15 backdrop-blur-[1px] z-[200] flex items-center justify-center p-4 select-none cursor-default",
                    onclick: move |_| show_create_group_modal.set(false),
                    div {
                        class: "w-[360px] border rounded-lg shadow-2xl flex flex-col overflow-hidden pointer-events-auto",
                        style: "background: {theme.bg_chat()}; border: 1.5px solid rgba(255, 255, 255, 0.45); box-shadow: 0 10px 30px rgba(0, 0, 0, 0.25), inset 0 1px 0 rgba(255, 255, 255, 0.6);",
                        onclick: move |e| e.stop_propagation(),

                        div { class: "h-9 bg-gradient-to-r {theme.titlebar_gradient()} border-b {theme.titlebar_border()} flex items-center justify-between px-3 flex-shrink-0 select-none",
                            div { class: "flex items-center space-x-1.5 font-bold text-[11px] {theme.titlebar_text()}",
                                img {
                                    src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/People/SVG/ic_fluent_people_24_color.svg",
                                    class: "w-5 h-5 object-contain pointer-events-none"
                                }
                                span { "Criar Novo Grupo" }
                            }
                            button {
                                class: "w-[28px] h-[18px] bg-white border border-[#d1d1d1] rounded-[3px] shadow-sm flex items-center justify-center cursor-pointer transition-all hover:bg-[#e81123] hover:border-[#e81123] hover:text-white text-[#6f6f6f] hover:text-white focus:outline-none text-[8px] font-bold",
                                title: "Fechar",
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

                        // Conteúdo do Modal com padding Aero
                        div { class: "p-4 flex flex-col space-y-4 text-xs {theme.titlebar_text()}",

                            div { class: "flex flex-col space-y-1.5",
                                label { class: "font-semibold text-slate-700", "Nome do Grupo:" }
                                input {
                                    class: "w-full h-[27px] px-2.5 text-xs text-slate-800 bg-white border border-[#d1d1d1] rounded-[4px] msn-input placeholder-[#a5a5a5] placeholder:text-[10px] focus:outline-none focus:border-slate-400",
                                    placeholder: "Digite o nome do grupo...",
                                    value: "{group_name}",
                                    oninput: move |e| group_name.set(e.value()),
                                }
                            }

                            div { class: "flex flex-col space-y-1.5",
                                label { class: "font-semibold text-slate-700", "Descrição do Grupo (opcional):" }
                                input {
                                    class: "w-full h-[27px] px-2.5 text-xs text-slate-800 bg-white border border-[#d1d1d1] rounded-[4px] msn-input placeholder-[#a5a5a5] placeholder:text-[10px] focus:outline-none focus:border-slate-400",
                                    placeholder: "Uma breve descrição do grupo...",
                                    value: "{group_desc}",
                                    oninput: move |e| group_desc.set(e.value()),
                                }
                            }

                            div { class: "flex flex-col space-y-1.5",
                                label { class: "font-semibold text-slate-700", "URL da Foto do Grupo (opcional):" }
                                input {
                                    class: "w-full h-[27px] px-2.5 text-xs text-slate-800 bg-white border border-[#d1d1d1] rounded-[4px] msn-input placeholder-[#a5a5a5] placeholder:text-[10px] focus:outline-none focus:border-slate-400",
                                    placeholder: "https://exemplo.com/foto.jpg",
                                    value: "{group_avatar}",
                                    oninput: move |e| group_avatar.set(e.value()),
                                }
                            }

                            div { class: "flex flex-col space-y-1.5 flex-1 min-h-0",
                                label { class: "font-semibold text-slate-700", "Selecione os contatos para adicionar:" }
                                div { class: "border border-[#d1d1d1] rounded bg-white/40 p-2 overflow-y-auto max-h-[120px] flex flex-col space-y-1.5 shadow-inner",
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
                                                            class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5 cursor-pointer",
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
                                                            span { class: "font-medium text-slate-800 truncate text-[11px]", {crate::models::parse_emoticons_inline(&name, "w-3 h-3")} }
                                                            span { class: "text-[9px] text-slate-500 truncate", "{contact.email}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "flex justify-end space-x-2 pt-2 border-t border-slate-200/50",
                                button {
                                    class: "px-4 py-1.5 bg-white hover:bg-slate-100 border border-slate-350 text-slate-700 rounded-[4px] font-bold shadow cursor-pointer transition-colors focus:outline-none text-[10px]",
                                    onclick: move |_| {
                                        show_create_group_modal.set(false);
                                        group_name.set(String::new());
                                        group_desc.set(String::new());
                                        group_avatar.set(String::new());
                                        selected_emails.write().clear();
                                    },
                                    "Cancelar"
                                }
                                button {
                                    class: "px-4 py-1.5 {theme.btn_primary()} rounded-[4px] font-bold shadow cursor-pointer transition-colors focus:outline-none text-[10px] disabled:opacity-50 disabled:cursor-not-allowed",
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
    
    let mut show_context_menu = use_signal(|| false);
    let mut menu_x = use_signal(|| 0i32);
    let mut menu_y = use_signal(|| 0i32);
    
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
        asset!("/assets/status/disponivel_grupo.svg")
    } else {
        asset!("/assets/status/offline_grupo.svg")
    };

    let container_padding = if density == "small" { "py-0.5 px-1.5" } else { "py-1 px-1.5" };

    rsx! {
        div {
            class: "flex items-center space-x-1.5 {container_padding} rounded hover:bg-white/45 cursor-pointer relative group transition-colors",
            ondoubleclick: handle_double_click,
            oncontextmenu: move |e| {
                e.prevent_default();
                let scale = state.interface_scale();
                let offset_y = if state.use_custom_titlebar() { 40.0 } else { 0.0 };
                menu_x.set((e.client_coordinates().x as f64 / scale) as i32);
                menu_y.set(((e.client_coordinates().y as f64 - offset_y) / scale) as i32);
                show_context_menu.set(true);
            },
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
            
            if density == "medium" {
                {
                    let frame_src = if online_members > 0 {
                        asset!("/assets/status/disponivel_perfil.svg")
                    } else {
                        asset!("/assets/status/offline_perfil.svg")
                    };
                    rsx! {
                        div { class: "msn-avatar-container w-[36px] h-[36px] flex-shrink-0 mr-1",
                            img {
                                src: frame_src,
                                class: "msn-avatar-frame-img"
                            }
                            div {
                                class: "msn-avatar-content w-[28px] h-[28px] rounded-[3px] bg-transparent flex items-center justify-center",
                                {crate::models::render_avatar(group.avatar_url.as_deref(), 28)}
                            }
                        }
                        div { class: "flex-1 min-w-0 flex flex-col space-y-0.25",
                            span { class: "font-semibold text-xs {theme.titlebar_text()} truncate hover:underline", 
                                "{group.name.as_deref().unwrap_or(\"Grupo sem nome\")}" 
                            }
                            span { class: "text-[10px] text-slate-400 font-normal", 
                                "({online_members}/{group.members.len()} Disponível)" 
                            }
                        }
                    }
                }
            } else {
                {
                    rsx! {
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
            
            // Menu de Contexto MSN Style para Grupo
            if show_context_menu() {
                {
                    let self_id = state.server_user_id();
                    let is_local_user_admin = group.members.iter().any(|m| Some(m.id.clone()) == self_id && m.role.as_deref() == Some("admin"));
                    let gid = group_id.clone();
                    rsx! {
                        div {
                            class: "fixed inset-0 z-[9998] bg-transparent cursor-default",
                            onclick: move |e| {
                                e.stop_propagation();
                                show_context_menu.set(false);
                            }
                        }
                        div { 
                            class: "fixed w-44 bg-white/95 border border-slate-300 rounded-lg shadow-2xl backdrop-blur-md z-[9999] p-1 flex flex-col text-[11px] text-slate-700 transition-all select-none cursor-default",
                            style: "left: {menu_x}px; top: {menu_y}px;",
                            onclick: move |e| e.stop_propagation(),
                            onmouseleave: move |_| show_context_menu.set(false),
                            
                            button { 
                                class: "px-2.5 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-medium transition-colors",
                                onclick: {
                                    let gid = gid.clone();
                                    move |_| {
                                        show_context_menu.set(false);
                                        state.open_chat(gid.clone());
                                    }
                                },
                                span { "💬" }
                                span { "Enviar mensagem" }
                            }

                            button { 
                                class: "px-2.5 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-medium transition-colors",
                                onclick: {
                                    let gid = gid.clone();
                                    move |_| {
                                        show_context_menu.set(false);
                                        state.open_chat(gid.clone());
                                    }
                                },
                                span { "ℹ️" }
                                span { "Ver Perfil do Grupo" }
                            }
                            
                            // Divisor
                            div { class: "h-[1px] bg-slate-200/60 my-0.5" }

                            button { 
                                class: "px-2.5 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-medium transition-colors text-red-600",
                                onclick: {
                                    let gid = gid.clone();
                                    move |_| {
                                        show_context_menu.set(false);
                                        state.leave_group_chat(gid.clone());
                                    }
                                },
                                span { "🚪" }
                                span { "Sair do grupo" }
                            }

                            if is_local_user_admin {
                                button { 
                                    class: "px-2.5 py-1.5 hover:bg-rose-100 hover:text-rose-700 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none w-full font-bold transition-colors text-rose-600",
                                    onclick: {
                                        let gid = gid.clone();
                                        move |_| {
                                            show_context_menu.set(false);
                                            state.delete_group_chat(gid.clone());
                                        }
                                    },
                                    span { "🗑️" }
                                    span { "Excluir Grupo" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
