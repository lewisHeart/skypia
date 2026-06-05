use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn GamesModal(mut state: AppState) -> Element {
    let theme = state.theme();
    let contact_id_opt = state.selected_chat_id();
    if contact_id_opt.is_none() {
        return rsx! {};
    }
    let contact_id = contact_id_opt.unwrap();
    let contact = state.contacts().into_iter().find(|c| c.id == contact_id);
    let contact_name = contact
        .map(|c| c.nickname.unwrap_or(c.display_name))
        .unwrap_or_else(|| "Contato".to_string());

    rsx! {
        div {
            class: "fixed inset-0 bg-black/45 backdrop-blur-sm z-[9999] flex items-center justify-center p-4 pointer-events-auto",
            onclick: move |_| state.show_games_modal.set(false),
            div {
                class: "w-80 bg-gradient-to-b {theme.modal_gradient()} border {theme.modal_border()} rounded-lg shadow-2xl p-4 flex flex-col space-y-4 text-xs {theme.titlebar_text()} pointer-events-auto select-none",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "flex items-center justify-between border-b {theme.titlebar_border()} pb-2",
                    div { class: "flex items-center space-x-1.5 font-bold text-sm {theme.titlebar_text()}",
                        span { "🎮" }
                        span { "Skypia Jogos" }
                    }
                    button {
                        class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-500 hover:text-white border border-transparent font-bold cursor-pointer transition-colors focus:outline-none",
                        onclick: move |_| state.show_games_modal.set(false),
                        "✕"
                    }
                }

                p { class: "text-slate-600 leading-normal",
                    "Escolha um jogo para desafiar {contact_name}:"
                }

                // Grid/List of Games
                div { class: "flex flex-col space-y-2.5 max-h-64 overflow-y-auto pr-0.5 scrollbar-thin scrollbar-thumb-slate-300",
                    // Jogo da Velha (Ativo)
                    div { class: "flex items-center justify-between p-2.5 bg-white/60 border {theme.titlebar_border()}/40 rounded hover:bg-white/80 transition-colors shadow-sm",
                        div { class: "flex items-center space-x-3 min-w-0 flex-1",
                            span { class: "text-2xl flex-shrink-0", "❌" }
                            div { class: "flex flex-col min-w-0 flex-1",
                                span { class: "font-bold text-slate-800 text-[11px]", "Jogo da Velha" }
                                span { class: "text-[10px] text-slate-500 truncate", "Clássico duelo de X e O contra o oponente." }
                            }
                        }
                        button {
                            class: "px-3 py-1 bg-gradient-to-b from-emerald-400 to-emerald-500 hover:from-emerald-500 hover:to-emerald-600 text-white rounded font-bold cursor-pointer transition-colors focus:outline-none flex-shrink-0 text-[10px]",
                            onclick: move |_| {
                                state.start_game(contact_id.clone());
                                state.show_games_modal.set(false);
                            },
                            "Desafiar"
                        }
                    }

                    // Campo Minado (Em breve)
                    div { class: "flex items-center justify-between p-2.5 bg-slate-100/40 border border-slate-200 rounded opacity-65 shadow-inner",
                        div { class: "flex items-center space-x-3 min-w-0 flex-1",
                            span { class: "text-2xl flex-shrink-0 grayscale", "💣" }
                            div { class: "flex flex-col min-w-0 flex-1",
                                span { class: "font-bold text-slate-700 text-[11px]", "Campo Minado" }
                                span { class: "text-[10px] text-slate-400 truncate", "Evite as minas e libere o campo." }
                            }
                        }
                        span { class: "text-[9px] bg-slate-300 text-slate-600 font-bold px-1.5 py-0.5 rounded flex-shrink-0", "Em breve" }
                    }

                    // Paciência (Em breve)
                    div { class: "flex items-center justify-between p-2.5 bg-slate-100/40 border border-slate-200 rounded opacity-65 shadow-inner",
                        div { class: "flex items-center space-x-3 min-w-0 flex-1",
                            span { class: "text-2xl flex-shrink-0 grayscale", "🃏" }
                            div { class: "flex flex-col min-w-0 flex-1",
                                span { class: "font-bold text-slate-700 text-[11px]", "Paciência" }
                                span { class: "text-[10px] text-slate-400 truncate", "Organize as cartas em ordem." }
                            }
                        }
                        span { class: "text-[9px] bg-slate-300 text-slate-600 font-bold px-1.5 py-0.5 rounded flex-shrink-0", "Em breve" }
                    }

                    // Copas Online (Em breve)
                    div { class: "flex items-center justify-between p-2.5 bg-slate-100/40 border border-slate-200 rounded opacity-65 shadow-inner",
                        div { class: "flex items-center space-x-3 min-w-0 flex-1",
                            span { class: "text-2xl flex-shrink-0 grayscale", "🃏" }
                            div { class: "flex flex-col min-w-0 flex-1",
                                span { class: "font-bold text-slate-700 text-[11px]", "Copas" }
                                span { class: "text-[10px] text-slate-400 truncate", "Jogue copas online com amigos." }
                            }
                        }
                        span { class: "text-[9px] bg-slate-300 text-slate-600 font-bold px-1.5 py-0.5 rounded flex-shrink-0", "Em breve" }
                    }
                }

                // Footer
                div { class: "flex justify-end pt-2 border-t {theme.titlebar_border()}/30",
                    button {
                        class: "px-4 py-1.5 bg-white hover:bg-slate-100 border border-slate-350 rounded font-bold cursor-pointer transition-colors focus:outline-none",
                        onclick: move |_| state.show_games_modal.set(false),
                        "Fechar"
                    }
                }
            }
        }
    }
}
