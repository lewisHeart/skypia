use dioxus::prelude::*;
use crate::state::AppState;
use crate::models::{render_avatar, TicTacToeCell, UserStatus};

#[component]
pub fn ChatSidebar(contact_id: usize, mut state: AppState) -> Element {
    let contact = state.contacts().into_iter().find(|c| c.id == contact_id);
    if contact.is_none() {
        return rsx! {};
    }
    let contact = contact.unwrap();

    let game_states = state.game_states();
    let active_game = game_states.get(&contact_id);

    let contact_status_color = match contact.status {
        UserStatus::Online => "border-[#3cd070]",
        UserStatus::Ocupado => "border-[#e81123]",
        UserStatus::Ausente => "border-[#ffb900]",
        UserStatus::Offline | UserStatus::Invisivel => "border-slate-400",
    };

    let user_status_color = match state.user_status() {
        UserStatus::Online => "border-[#3cd070]",
        UserStatus::Ocupado => "border-[#e81123]",
        UserStatus::Ausente => "border-[#ffb900]",
        UserStatus::Offline | UserStatus::Invisivel => "border-slate-400",
    };

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
                            rsx! {
                                button {
                                    class: "w-full aspect-square rounded flex items-center justify-center border border-white/20 transition-all cursor-pointer {cell_color}",
                                    disabled: !game.active || *cell != TicTacToeCell::Empty || game.turn != TicTacToeCell::X,
                                    onclick: move |_| state.make_game_move(contact_id, idx),
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
                    onclick: move |_| state.start_game(contact_id),
                    if game.active { "Reiniciar" } else { "Jogar de Novo" }
                }
                button {
                    class: "w-full py-1 bg-red-100 hover:bg-red-200 border border-red-300 rounded font-bold transition-all text-[10px] text-red-700 cursor-pointer text-center",
                    onclick: move |_| {
                        state.game_states.write().remove(&contact_id);
                    },
                    "Sair do Jogo"
                }
            }
        } else {
            div { class: "hidden sm:flex w-28 flex-col items-center justify-between p-3 bg-white/10 flex-shrink-0 border-l border-white/20",
                
                // Contact's avatar frame with status contour in Tailwind CSS
                div { class: "flex flex-col items-center space-y-1.5",
                    div { 
                        class: "shadow-md relative rounded-[8px] border-[3.5px] {contact_status_color} overflow-hidden bg-transparent",
                        {render_avatar(contact.avatar_id, 64)}
                        div { 
                            class: "absolute -bottom-1 -right-1 w-4 h-4 rounded-full bg-white border border-[#a6b9cd] flex items-center justify-center shadow-sm",
                            div { class: "w-2.5 h-2.5 rounded-full {contact.status.color_class()} border border-black/10" }
                        }
                    }
                    span { class: "text-[10px] text-slate-500 font-bold max-w-[85px] truncate text-center", "{contact.display_name}" }
                }

                // User's own avatar frame with status contour in Tailwind CSS
                div { class: "flex flex-col items-center space-y-1.5",
                    div { 
                        class: "shadow-md relative rounded-[8px] border-[3.5px] {user_status_color} overflow-hidden bg-transparent",
                        {render_avatar(state.user_avatar_id(), 64)}
                        div { 
                            class: "absolute -bottom-1 -right-1 w-4 h-4 rounded-full bg-white border border-[#a6b9cd] flex items-center justify-center shadow-sm",
                            div { class: "w-2.5 h-2.5 rounded-full {state.user_status().color_class()} border border-black/10" }
                        }
                    }
                    span { class: "text-[10px] text-slate-500 font-bold max-w-[85px] truncate text-center", "Você" }
                }
            }
        }
    }
}
