use dioxus::prelude::*;
use crate::state::AppState;
use crate::models::{render_avatar, TicTacToeCell};

#[component]
pub fn ChatSidebar(contact_id: String, mut state: AppState) -> Element {
    let theme = state.theme();
    let contact = state.contacts().into_iter().find(|c| c.id == contact_id);
    let group = (state.group_chats)().into_iter().find(|g| g.id == contact_id);

    if contact.is_none() && group.is_none() {
        return rsx! {};
    }

    let is_group = group.is_some();
    let mut show_add_member = use_signal(|| false);
    let mut new_member_email = use_signal(|| String::new());

    let game_states = state.game_states();
    let active_game = game_states.get(&contact_id);

    let self_id = state.server_user_id();
    let is_local_user_admin = group.as_ref().map(|g| {
        g.members.iter().any(|m| Some(m.id.clone()) == self_id && m.role.as_deref() == Some("admin"))
    }).unwrap_or(false);

    // Variáveis auxiliares para o Jogo da Velha (fora da macro rsx!)
    let contact_name = contact.as_ref().map(|c| c.nickname.clone().unwrap_or(c.display_name.clone())).unwrap_or_else(|| "Contato".to_string());
    let messages = state.chat_messages();
    let chat_history = messages.get(&contact_id);
    let last_invite_msg = chat_history.and_then(|msgs| {
        msgs.iter().rfind(|m| m.is_game_invite)
    });
    let is_my_invite = last_invite_msg.map(|m| m.sender_id == "0").unwrap_or(true);

    rsx! {
        if is_group {
            {
                let g = group.unwrap();
                let gid_leave = contact_id.clone();
                let gid_add = contact_id.clone();

                // Separar os participantes em Online e Offline para melhor visualização (tarefa da lista)
                let mut online_members = Vec::new();
                let mut offline_members = Vec::new();

                for member in g.members.clone() {
                    let member_id = member.id.clone();
                    let is_self = Some(member_id.clone()) == self_id;
                    let member_status = if is_self {
                        state.user_status()
                    } else if let Some(c) = state.contacts().iter().find(|c| c.id == member_id) {
                        c.status
                    } else {
                        match member.status.as_str() {
                            "Online" => crate::models::UserStatus::Online,
                            "Ocupado" => crate::models::UserStatus::Ocupado,
                            "Ausente" => crate::models::UserStatus::Ausente,
                            "Invisivel" => crate::models::UserStatus::Invisivel,
                            _ => crate::models::UserStatus::Offline,
                        }
                    };
                    if member_status == crate::models::UserStatus::Offline || member_status == crate::models::UserStatus::Invisivel {
                        offline_members.push((member, member_status));
                    } else {
                        online_members.push((member, member_status));
                    }
                }

                rsx! {
                    div { class: "hidden sm:flex w-[220px] flex-col p-2.5 bg-transparent flex-shrink-0 text-xs {theme.titlebar_text()} space-y-3 justify-between h-full select-none",
                        
                        div { class: "flex flex-col space-y-2 flex-1 min-h-0",
                            div { class: "font-bold text-center border-b {theme.titlebar_border()}/40 pb-1.5 w-full flex items-center justify-center space-x-1 flex-shrink-0",
                                span { "👥" }
                                span { "Participantes ({g.members.len()})" }
                            }

                            div { class: "flex-1 overflow-y-auto pr-0.5 space-y-2 max-h-[220px]",
                                // Seção Online
                                if !online_members.is_empty() {
                                    div { class: "flex flex-col space-y-1",
                                        div { class: "text-[9px] uppercase tracking-wider text-slate-500 font-bold px-1 py-0.5", "Online ({online_members.len()})" }
                                        for (member, member_status) in online_members {
                                            {
                                                let member_id = member.id.clone();
                                                let member_name = member.nickname.clone().unwrap_or(member.display_name.clone());
                                                let gid = contact_id.clone();
                                                let is_self = Some(member_id.clone()) == self_id;
                                                rsx! {
                                                    div { class: "flex items-center justify-between p-1 hover:bg-white/70 rounded group transition-all",
                                                        div { class: "flex items-center space-x-1.5 min-w-0 flex-1",
                                                            div { class: "relative flex-shrink-0",
                                                                div { class: "w-[18px] h-[18px] rounded overflow-hidden border border-slate-300 flex items-center justify-center bg-white shadow-sm",
                                                                    {render_avatar(member.avatar_url.as_deref(), 18)}
                                                                }
                                                                div { class: "absolute -bottom-0.5 -right-0.5 w-2 h-2 rounded-full {member_status.color_class()} border border-white" }
                                                            }
                                                            span { class: "truncate font-medium text-[10.5px]", "{member_name}" }
                                                            if member.role.as_deref() == Some("admin") {
                                                                span { class: "text-[8px] bg-sky-600 text-white font-extrabold px-1 rounded ml-1 scale-90 flex-shrink-0", "Dono" }
                                                            }
                                                        }
                                                        div { class: "flex items-center space-x-1 flex-shrink-0 opacity-0 group-hover:opacity-100 transition-all",
                                                            if !is_self && is_local_user_admin {
                                                                {
                                                                    let is_admin = member.role.as_deref() == Some("admin");
                                                                    let next_role = if is_admin { "member".to_string() } else { "admin".to_string() };
                                                                    let label = if is_admin { "Membro" } else { "Dono" };
                                                                    let gid_role = gid.clone();
                                                                    let mid_role = member_id.clone();
                                                                    rsx! {
                                                                        button {
                                                                            class: "px-1 py-0.5 bg-white border border-slate-350 text-slate-700 hover:bg-slate-100 rounded font-bold text-[8px] cursor-pointer focus:outline-none",
                                                                            title: "Alternar papel do participante",
                                                                            onclick: move |e| {
                                                                                e.stop_propagation();
                                                                                state.update_group_member_role(gid_role.clone(), mid_role.clone(), next_role.clone());
                                                                            },
                                                                            "{label}"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            if !is_self && is_local_user_admin {
                                                                button {
                                                                    class: "w-4 h-4 rounded hover:bg-red-500 hover:text-white flex items-center justify-center text-[9px] font-bold border border-transparent cursor-pointer transition-all focus:outline-none",
                                                                    title: "Remover do grupo",
                                                                    onclick: move |e| {
                                                                        e.stop_propagation();
                                                                        state.remove_group_member(gid.clone(), member_id.clone());
                                                                    },
                                                                    "✕"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Seção Offline
                                if !offline_members.is_empty() {
                                    div { class: "flex flex-col space-y-1 pt-1",
                                        div { class: "text-[9px] uppercase tracking-wider text-slate-400 font-bold px-1 py-0.5", "Offline ({offline_members.len()})" }
                                        for (member, member_status) in offline_members {
                                            {
                                                let member_id = member.id.clone();
                                                let member_name = member.nickname.clone().unwrap_or(member.display_name.clone());
                                                let gid = contact_id.clone();
                                                let is_self = Some(member_id.clone()) == self_id;
                                                rsx! {
                                                    div { class: "flex items-center justify-between p-1 hover:bg-white/70 rounded group transition-all opacity-70 hover:opacity-100",
                                                        div { class: "flex items-center space-x-1.5 min-w-0 flex-1",
                                                            div { class: "relative flex-shrink-0",
                                                                div { class: "w-[18px] h-[18px] rounded overflow-hidden border border-slate-300 flex items-center justify-center bg-white shadow-sm",
                                                                    {render_avatar(member.avatar_url.as_deref(), 18)}
                                                                }
                                                                div { class: "absolute -bottom-0.5 -right-0.5 w-2 h-2 rounded-full {member_status.color_class()} border border-white" }
                                                            }
                                                            span { class: "truncate font-medium text-[10.5px]", "{member_name}" }
                                                            if member.role.as_deref() == Some("admin") {
                                                                span { class: "text-[8px] bg-sky-600 text-white font-extrabold px-1 rounded ml-1 scale-90 flex-shrink-0", "Dono" }
                                                            }
                                                        }
                                                        div { class: "flex items-center space-x-1 flex-shrink-0 opacity-0 group-hover:opacity-100 transition-all",
                                                            if !is_self && is_local_user_admin {
                                                                {
                                                                    let is_admin = member.role.as_deref() == Some("admin");
                                                                    let next_role = if is_admin { "member".to_string() } else { "admin".to_string() };
                                                                    let label = if is_admin { "Membro" } else { "Dono" };
                                                                    let gid_role = gid.clone();
                                                                    let mid_role = member_id.clone();
                                                                    rsx! {
                                                                        button {
                                                                            class: "px-1 py-0.5 bg-white border border-slate-350 text-slate-700 hover:bg-slate-100 rounded font-bold text-[8px] cursor-pointer focus:outline-none",
                                                                            title: "Alternar papel do participante",
                                                                            onclick: move |e| {
                                                                                e.stop_propagation();
                                                                                state.update_group_member_role(gid_role.clone(), mid_role.clone(), next_role.clone());
                                                                            },
                                                                            "{label}"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            if !is_self && is_local_user_admin {
                                                                button {
                                                                    class: "w-4 h-4 rounded hover:bg-red-500 hover:text-white flex items-center justify-center text-[9px] font-bold border border-transparent cursor-pointer transition-all focus:outline-none",
                                                                    title: "Remover do grupo",
                                                                    onclick: move |e| {
                                                                        e.stop_propagation();
                                                                        state.remove_group_member(gid.clone(), member_id.clone());
                                                                    },
                                                                    "✕"
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

                            // Painel de Configurações / Permissões do Grupo (Apenas para Admins)
                            if is_local_user_admin {
                                {
                                    let allow_send = g.allow_member_send.unwrap_or(true);
                                    let allow_invite = g.allow_member_invite.unwrap_or(true);
                                    rsx! {
                                        div { class: "flex flex-col space-y-1 p-1.5 bg-slate-50/80 border border-slate-200 rounded mt-1.5 flex-shrink-0",
                                            span { class: "font-bold text-[8.5px] uppercase tracking-wide text-slate-500", "Permissões do Grupo" }
                                            label { class: "flex items-center space-x-1.5 cursor-pointer text-[9.5px] text-slate-650",
                                                input {
                                                    r#type: "checkbox",
                                                    class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-[#1d528f] focus:ring-0 focus:outline-none w-3 h-3",
                                                    checked: allow_send,
                                                    onchange: {
                                                        let gid = contact_id.clone();
                                                        move |_| {
                                                            state.update_group_permissions(gid.clone(), !allow_send, allow_invite);
                                                        }
                                                    }
                                                }
                                                span { "Membros podem enviar msg" }
                                            }
                                            label { class: "flex items-center space-x-1.5 cursor-pointer text-[9.5px] text-slate-650",
                                                input {
                                                    r#type: "checkbox",
                                                    class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-[#1d528f] focus:ring-0 focus:outline-none w-3 h-3",
                                                    checked: allow_invite,
                                                    onchange: {
                                                        let gid = contact_id.clone();
                                                        move |_| {
                                                            state.update_group_permissions(gid.clone(), allow_send, !allow_invite);
                                                        }
                                                    }
                                                }
                                                span { "Membros podem convidar" }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "flex flex-col space-y-1.5 border-t {theme.titlebar_border()}/40 pt-2 flex-shrink-0",
                            // Controla o botão de Adicionar Membro baseado na permissão de convite do grupo
                            {
                                let allow_invite = g.allow_member_invite.unwrap_or(true) || is_local_user_admin;
                                if allow_invite {
                                    rsx! {
                                        if show_add_member() {
                                            div { class: "flex flex-col space-y-1 p-1 bg-[#d8e8f6] rounded border border-[#96badb]",
                                                input {
                                                    class: "w-full px-1.5 py-0.75 text-[10px] rounded border {theme.titlebar_border()} bg-white focus:outline-none focus:border-slate-400 text-slate-800",
                                                    placeholder: "Email do contato...",
                                                    value: "{new_member_email}",
                                                    oninput: move |e| new_member_email.set(e.value()),
                                                    onkeydown: {
                                                        let gid = gid_add.clone();
                                                        move |e| {
                                                            if e.key() == Key::Enter {
                                                                let email = new_member_email();
                                                                if !email.trim().is_empty() {
                                                                    state.add_group_member(gid.clone(), email.trim().to_string());
                                                                    new_member_email.set(String::new());
                                                                    show_add_member.set(false);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                div { class: "flex items-center space-x-1 justify-end",
                                                    button {
                                                        class: "px-1.5 py-0.5 bg-green-600 text-white rounded text-[9px] font-bold cursor-pointer hover:bg-green-700",
                                                        onclick: {
                                                            let gid = gid_add.clone();
                                                            move |_| {
                                                                let email = new_member_email();
                                                                if !email.trim().is_empty() {
                                                                    state.add_group_member(gid.clone(), email.trim().to_string());
                                                                    new_member_email.set(String::new());
                                                                    show_add_member.set(false);
                                                                }
                                                            }
                                                        },
                                                        "Adicionar"
                                                    }
                                                    button {
                                                        class: "px-1.5 py-0.5 bg-slate-350 text-slate-700 rounded text-[9px] font-bold cursor-pointer hover:bg-slate-400",
                                                        onclick: move |_| show_add_member.set(false),
                                                        "Cancelar"
                                                    }
                                                }
                                            }
                                        } else {
                                            button {
                                                class: "w-full py-1 bg-white hover:bg-slate-100 border border-slate-350 rounded font-bold transition-all text-[10px] cursor-pointer text-center mb-1",
                                                onclick: move |_| show_add_member.set(true),
                                                "+ Adicionar Membro"
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        div { class: "text-center text-[10px] text-slate-400 italic mb-1", "Adição de membros desativada" }
                                    }
                                }
                            }
                            
                            button {
                                class: "w-full py-1 bg-red-100 hover:bg-red-200 border border-red-300 rounded font-bold transition-all text-[10px] text-red-700 cursor-pointer text-center",
                                onclick: move |_| {
                                    state.leave_group_chat(gid_leave.clone());
                                },
                                "Sair do Grupo"
                            }
                        }
                    }
                }
            }
        } else if let Some(game) = active_game {
            div { class: "hidden sm:flex w-[220px] flex-col items-center p-3 bg-transparent flex-shrink-0 text-xs {theme.titlebar_text()} space-y-3 shadow-inner",
                div { class: "font-bold text-center border-b {theme.titlebar_border()}/40 pb-1 w-full flex items-center justify-center space-x-1",
                    span { "🎮" }
                    span { "Jogo da Velha" }
                }
                
                if !game.accepted {
                    div { class: "flex-1 flex flex-col items-center justify-center text-center space-y-4 py-6 w-full",
                        if is_my_invite {
                            div { class: "w-8 h-8 border-2 border-emerald-600 border-t-transparent rounded-full animate-spin" }
                            span { class: "font-semibold text-[10px] text-slate-600 px-1", "Aguardando aceitação de {contact_name}..." }
                            button {
                                class: "w-full py-1 bg-white hover:bg-slate-100 border border-slate-350 rounded font-bold transition-all text-[10px] text-slate-700 cursor-pointer text-center",
                                onclick: {
                                    let cid = contact_id.clone();
                                    move |_| {
                                        state.game_states.write().remove(&cid);
                                    }
                                },
                                "Cancelar"
                            }
                        } else {
                            span { class: "text-lg", "📨" }
                            span { class: "font-semibold text-[10px] text-slate-600 px-1", "{contact_name} convidou você para jogar!" }
                            div { class: "flex flex-col space-y-1.5 w-full pt-2",
                                button {
                                    class: "w-full py-1 {theme.btn_primary()} rounded font-bold transition-all text-[10px] cursor-pointer text-center",
                                    onclick: {
                                        let cid = contact_id.clone();
                                        move |_| state.accept_game_invite(cid.clone())
                                    },
                                    "Aceitar"
                                }
                                button {
                                    class: "w-full py-1 bg-white hover:bg-slate-100 border border-slate-350 rounded font-bold transition-all text-[10px] text-slate-700 cursor-pointer text-center",
                                    onclick: {
                                        let cid = contact_id.clone();
                                        move |_| state.reject_game_invite(cid.clone())
                                    },
                                    "Recusar"
                                }
                            }
                        }
                    }
                } else {
                    div { class: "grid grid-cols-3 gap-1.5 w-full aspect-square bg-[#d8e8f6]/50 p-1.5 rounded-lg border {theme.titlebar_border()}",
                        for (idx, cell) in game.board.iter().enumerate() {
                            {
                                let cell_text = match cell {
                                    TicTacToeCell::Empty => "",
                                    TicTacToeCell::X => "X",
                                    TicTacToeCell::O => "O",
                                };
                                let cell_color = match cell {
                                    TicTacToeCell::Empty => "bg-white hover:bg-[#eff5fb]".to_string(),
                                    TicTacToeCell::X => {
                                        let bg_class = theme.accent_color().split_whitespace().nth(1).unwrap_or("bg-[#3b82f6]");
                                        format!("{} text-white font-black text-sm cursor-default", bg_class)
                                    }
                                    TicTacToeCell::O => "bg-rose-500/80 text-white font-black text-sm cursor-default".to_string(),
                                };
                                let cid = contact_id.clone();
                                rsx! {
                                    button {
                                        class: "w-full aspect-square rounded flex items-center justify-center border border-white/20 transition-all cursor-pointer {cell_color}",
                                        disabled: !game.active || *cell != TicTacToeCell::Empty || game.turn != TicTacToeCell::X,
                                        onclick: move |_| state.make_game_move(cid.clone(), idx),
                                        "{cell_text}"
                                    }
                                }
                            }
                        }
                    }
                    
                    div { class: "text-[10px] text-center font-semibold pt-1 min-h-8 flex items-center justify-center w-full",
                        if !game.active {
                            if let Some(winner) = game.winner {
                                if winner == TicTacToeCell::X {
                                    span { "Você Venceu! 🎉" }
                                } else {
                                    span { "Você Perdeu! 😢" }
                                }
                            } else if game.is_draw {
                                span { "Deu Velha! 🤝" }
                            }
                        } else if game.turn == TicTacToeCell::X {
                            span { "Sua vez (X)" }
                        } else {
                            span { class: "animate-pulse", "Pensando (O)..." }
                        }
                    }
                    
                    button {
                        class: "w-full py-1 bg-white hover:bg-slate-100 border border-slate-350 rounded font-bold transition-all text-[10px] cursor-pointer text-center",
                        onclick: {
                            let cid = contact_id.clone();
                            move |_| state.start_game(cid.clone())
                        },
                        if game.active { "Reiniciar" } else { "Jogar de Novo" }
                    }
                    button {
                        class: "w-full py-1 bg-red-100 hover:bg-red-200 border border-red-300 rounded font-bold transition-all text-[10px] text-red-700 cursor-pointer text-center",
                        onclick: {
                            let cid = contact_id.clone();
                            move |_| {
                                state.game_states.write().remove(&cid);
                            }
                        },
                        "Sair do Jogo"
                    }
                }
            }
        } else {
            {
                let c = contact.as_ref().unwrap();
                rsx! {
                    div { class: "hidden sm:flex w-[220px] flex-col items-center justify-between py-6 px-4 bg-transparent flex-shrink-0 select-none h-full",
                        
                        // Contact's avatar
                        div { class: "flex flex-col items-center space-y-2.5",
                            {
                                let frame_src = match c.status {
                                    crate::models::UserStatus::Online => asset!("/assets/status/disponivel_conversa.svg"),
                                    crate::models::UserStatus::Ocupado => asset!("/assets/status/ocupado_conversa.svg"),
                                    crate::models::UserStatus::Ausente => asset!("/assets/status/ausente_conversa.svg"),
                                    _ => asset!("/assets/status/offline_conversa.svg"),
                                };
                                rsx! {
                                    div { class: "msn-avatar-container w-[120px] h-[120px] flex-shrink-0 relative",
                                        img {
                                            src: frame_src,
                                            class: "msn-avatar-frame-img"
                                        }
                                        div {
                                            class: "msn-avatar-content w-[100px] h-[100px] rounded-[10px] bg-transparent flex items-center justify-center",
                                            {render_avatar(c.avatar_url.as_deref(), 100)}
                                        }
                                    }
                                }
                            }
                        }

                        // User's own avatar
                        div { class: "flex flex-col items-center space-y-2.5",
                            {
                                let frame_src = match state.user_status() {
                                    crate::models::UserStatus::Online => asset!("/assets/status/disponivel_conversa.svg"),
                                    crate::models::UserStatus::Ocupado => asset!("/assets/status/ocupado_conversa.svg"),
                                    crate::models::UserStatus::Ausente => asset!("/assets/status/ausente_conversa.svg"),
                                    _ => asset!("/assets/status/offline_conversa.svg"),
                                };
                                rsx! {
                                    div { class: "msn-avatar-container w-[120px] h-[120px] flex-shrink-0 relative",
                                        img {
                                            src: frame_src,
                                            class: "msn-avatar-frame-img"
                                        }
                                        div {
                                            class: "msn-avatar-content w-[100px] h-[100px] rounded-[10px] bg-transparent flex items-center justify-center",
                                            {render_avatar(state.user_avatar_url().as_deref(), 100)}
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
}
