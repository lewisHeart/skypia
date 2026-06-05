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

    rsx! {
        if is_group {
            {
                let g = group.unwrap();
                let gid_leave = contact_id.clone();
                let gid_add = contact_id.clone();
                rsx! {
                    div { class: "hidden sm:flex w-44 flex-col p-2.5 bg-white/15 border-l {theme.titlebar_border()} flex-shrink-0 text-xs {theme.titlebar_text()} space-y-3 justify-between h-full select-none",
                        
                        div { class: "flex flex-col space-y-2 flex-1 min-h-0",
                            div { class: "font-bold text-center border-b {theme.titlebar_border()}/40 pb-1.5 w-full flex items-center justify-center space-x-1 flex-shrink-0",
                                span { "👥" }
                                span { "Participantes ({g.members.len()})" }
                            }

                            div { class: "flex-1 overflow-y-auto pr-0.5 space-y-1.5 max-h-[280px]",
                                for member in g.members.clone() {
                                    {
                                        let member_id = member.id.clone();
                                        let member_name = member.nickname.clone().unwrap_or(member.display_name.clone());
                                        let gid = contact_id.clone();
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
                                        
                                        rsx! {
                                            div { class: "flex items-center justify-between p-1 hover:bg-white/35 rounded group transition-all",
                                                div { class: "flex items-center space-x-1.5 min-w-0 flex-1",
                                                    div { class: "relative flex-shrink-0",
                                                        div { class: "w-[18px] h-[18px] rounded overflow-hidden border border-slate-300 flex items-center justify-center bg-white shadow-sm",
                                                            {render_avatar(member.avatar_url.as_deref(), 18)}
                                                        }
                                                        div { class: "absolute -bottom-0.5 -right-0.5 w-2 h-2 rounded-full {member_status.color_class()} border border-white" }
                                                    }
                                                    span { class: "truncate font-medium text-[10.5px]", "{member_name}" }
                                                    if member.role.as_deref() == Some("admin") {
                                                        span { class: "text-[8px] bg-sky-650 text-white font-extrabold px-1 rounded ml-1 scale-90 flex-shrink-0", "Dono" }
                                                    }
                                                }
                                                if !is_self && is_local_user_admin {
                                                    button {
                                                        class: "w-4 h-4 rounded hover:bg-red-500 hover:text-white flex items-center justify-center text-[9px] font-bold border border-transparent cursor-pointer opacity-0 group-hover:opacity-100 transition-all focus:outline-none flex-shrink-0",
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

                        div { class: "flex flex-col space-y-1.5 border-t {theme.titlebar_border()}/40 pt-2 flex-shrink-0",
                            if is_local_user_admin {
                                if show_add_member() {
                                    div { class: "flex flex-col space-y-1 p-1 bg-white/20 rounded border border-white/20",
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
            div { class: "hidden sm:flex w-44 flex-col items-center p-3 bg-white/15 border-l {theme.titlebar_border()} flex-shrink-0 text-xs {theme.titlebar_text()} space-y-3 shadow-inner",
                div { class: "font-bold text-center border-b {theme.titlebar_border()}/40 pb-1 w-full flex items-center justify-center space-x-1",
                    span { "🎮" }
                    span { "Jogo da Velha" }
                }
                
                // 3x3 board grid
                div { class: "grid grid-cols-3 gap-1.5 w-full aspect-square bg-slate-900/10 p-1.5 rounded-lg border {theme.titlebar_border()}",
                    for (idx, cell) in game.board.iter().enumerate() {
                        {
                            let cell_text = match cell {
                                TicTacToeCell::Empty => "",
                                TicTacToeCell::X => "X",
                                TicTacToeCell::O => "O",
                            };
                            let cell_color = match cell {
                                TicTacToeCell::Empty => "bg-white/40 hover:bg-white/60".to_string(),
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
                
                // Game Status info
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
                
                // Controls
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
        } else {
            {
                let c = contact.as_ref().unwrap();
                rsx! {
                    div { class: "hidden sm:flex w-28 flex-col items-center justify-between p-3 bg-white/10 flex-shrink-0 border-l {theme.titlebar_border()}",
                        
                        // Contact's avatar frame with MSN status contour
                        div { class: "flex flex-col items-center space-y-1.5",
                            div { 
                                class: "relative p-[2.5px] rounded-[9px] border {c.status.avatar_frame_class()} bg-transparent shadow-[inset_0_0.5px_0_rgba(255,255,255,0.4)] flex items-center justify-center shadow-md flex-shrink-0",
                                div {
                                    class: "rounded-[6px] overflow-hidden border border-white/35 bg-white flex-shrink-0 flex items-center justify-center",
                                    {render_avatar(c.avatar_url.as_deref(), 64)}
                                }
                            }
                            span { class: "text-[10px] text-slate-500 font-bold max-w-[85px] truncate text-center", "{c.display_name}" }
                        }

                        // User's own avatar frame with MSN status contour
                        div { class: "flex flex-col items-center space-y-1.5",
                            div { 
                                class: "relative p-[2.5px] rounded-[9px] border {state.user_status().avatar_frame_class()} bg-transparent shadow-[inset_0_0.5px_0_rgba(255,255,255,0.4)] flex items-center justify-center shadow-md flex-shrink-0",
                                div {
                                    class: "rounded-[6px] overflow-hidden border border-white/35 bg-white flex-shrink-0 flex items-center justify-center",
                                    {render_avatar(state.user_avatar_url().as_deref(), 64)}
                                }
                            }
                            span { class: "text-[10px] text-slate-500 font-bold max-w-[85px] truncate text-center", "Você" }
                        }
                    }
                }
            }
        }
    }
}
