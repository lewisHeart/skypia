use dioxus::prelude::*;
use crate::state::AppState;
use crate::models::{render_avatar, TicTacToeCell};

#[component]
pub fn ChatSidebar(contact_id: String, mut state: AppState) -> Element {
    let contact = state.contacts().into_iter().find(|c| c.id == contact_id);
    if contact.is_none() {
        return rsx! {};
    }
    let contact = contact.unwrap();

    let game_states = state.game_states();
    let active_game = game_states.get(&contact_id);

    rsx! {
        if let Some(game) = active_game {
            div { class: "hidden sm:flex w-44 flex-col items-center p-3 bg-white/15 border-l border-white/25 flex-shrink-0 text-xs text-[#1e395b] space-y-3 shadow-inner",
                div { class: "font-bold text-center border-b border-white/40 pb-1 w-full flex items-center justify-center space-x-1",
                    span { "🎮" }
                    span { "Jogo da Velha" }
                }
                
                // 3x3 board grid
                div { class: "grid grid-cols-3 gap-1.5 w-full aspect-square bg-slate-900/10 p-1.5 rounded-lg border border-[#a6b9cd]",
                    for (idx, cell) in game.board.iter().enumerate() {
                        {
                            let cell_text = match cell {
                                TicTacToeCell::Empty => "",
                                TicTacToeCell::X => "X",
                                TicTacToeCell::O => "O",
                            };
                            let cell_color = match cell {
                                TicTacToeCell::Empty => "bg-white/40 hover:bg-white/60",
                                TicTacToeCell::X => "bg-sky-500/80 text-white font-black text-sm cursor-default",
                                TicTacToeCell::O => "bg-rose-500/80 text-white font-black text-sm cursor-default",
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
            div { class: "hidden sm:flex w-28 flex-col items-center justify-between p-3 bg-white/10 flex-shrink-0 border-l border-white/20",
                
                // Contact's avatar frame with MSN status contour
                div { class: "flex flex-col items-center space-y-1.5",
                    div { 
                        class: "relative p-[2.5px] rounded-[9px] border {contact.status.avatar_frame_class()} bg-transparent shadow-[inset_0_0.5px_0_rgba(255,255,255,0.4)] flex items-center justify-center shadow-md flex-shrink-0",
                        div {
                            class: "rounded-[6px] overflow-hidden border border-white/35 bg-white flex-shrink-0 flex items-center justify-center",
                            {render_avatar(contact.avatar_url.as_deref(), 64)}
                        }
                    }
                    span { class: "text-[10px] text-slate-500 font-bold max-w-[85px] truncate text-center", "{contact.display_name}" }
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
