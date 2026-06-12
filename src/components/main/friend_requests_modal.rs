use dioxus::prelude::*;
use crate::state::AppState;

#[component]
pub fn FriendRequestsModal(mut state: AppState) -> Element {
    let theme = state.theme();
    let pending_list = state.pending_requests();

    rsx! {
        div {
            class: "fixed inset-0 bg-black/15 backdrop-blur-[1px] z-[9999] flex items-center justify-center p-4 select-none cursor-default",
            onclick: move |_| state.show_friend_requests_modal.set(false),
            div {
                class: "w-[380px] border rounded-lg shadow-2xl flex flex-col overflow-hidden pointer-events-auto",
                style: "background: {theme.bg_chat()}; border: 1.5px solid rgba(255, 255, 255, 0.45); box-shadow: 0 10px 30px rgba(0, 0, 0, 0.25), inset 0 1px 0 rgba(255, 255, 255, 0.6);",
                onclick: move |e| e.stop_propagation(),

                div { class: "h-9 bg-gradient-to-r {theme.titlebar_gradient()} border-b {theme.titlebar_border()} flex items-center justify-between px-3 flex-shrink-0 select-none",
                    div { class: "flex items-center space-x-1.5 font-bold text-[11px] {theme.titlebar_text()}",
                        img {
                            src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/People/SVG/ic_fluent_people_24_color.svg",
                            class: "w-5 h-5 object-contain pointer-events-none"
                        }
                        span { "Solicitações de Amizade" }
                    }
                    button {
                        class: "w-[28px] h-[18px] bg-white border border-[#d1d1d1] rounded-[3px] shadow-sm flex items-center justify-center cursor-pointer transition-all hover:bg-[#e81123] hover:border-[#e81123] hover:text-white text-[#6f6f6f] hover:text-white focus:outline-none text-[8px] font-bold",
                        title: "Fechar",
                        onclick: move |_| state.show_friend_requests_modal.set(false),
                        "✕"
                    }
                }

                // Conteúdo
                div { class: "p-4 flex flex-col space-y-3 text-xs {theme.titlebar_text()} max-h-[300px] overflow-y-auto bg-white/20",
                    if pending_list.is_empty() {
                        div { class: "text-center text-slate-500 py-6 italic", "Nenhuma solicitação pendente." }
                    } else {
                        for request in pending_list {
                            {
                                let req_id_accept = request.id.clone();
                                let req_id_reject = request.id.clone();
                                let name = request.nickname.clone().unwrap_or(request.display_name.clone());
                                rsx! {
                                    div { class: "flex items-center justify-between p-2.5 bg-white/60 rounded border border-slate-200 shadow-sm text-[11px]",
                                        div { class: "flex items-center space-x-3.5 min-w-0 mr-2",
                                            // Avatar
                                            div {
                                                class: "flex-shrink-0 p-[1px] rounded-[5px] border border-slate-300 bg-white flex items-center justify-center shadow-sm",
                                                div {
                                                    class: "rounded-[3px] overflow-hidden flex items-center justify-center w-8 h-8",
                                                    {crate::components::render_avatar(request.avatar_url.as_deref(), 32)}
                                                }
                                            }
                                            div { class: "flex flex-col min-w-0",
                                                span { class: "font-semibold text-slate-800 truncate", "{name}" }
                                                span { class: "text-[9px] text-slate-500 truncate", "{request.email}" }
                                            }
                                        }
                                        div { class: "flex space-x-1.5 flex-shrink-0",
                                            button {
                                                class: "px-3 py-1 bg-green-600 hover:bg-green-700 text-white rounded text-[10px] font-bold cursor-pointer transition-colors shadow-sm focus:outline-none",
                                                onclick: move |_| {
                                                    state.accept_friend_request(req_id_accept.clone());
                                                },
                                                "Aceitar"
                                            }
                                            button {
                                                class: "px-3 py-1 bg-red-600 hover:bg-red-700 text-white rounded text-[10px] font-bold cursor-pointer transition-colors shadow-sm focus:outline-none",
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

                // Rodapé
                div { class: "h-[50px] bg-slate-50 border-t border-slate-200 px-4 flex items-center justify-end flex-shrink-0",
                    button {
                        class: "px-5 py-1.5 {theme.btn_primary()} rounded font-bold transition-all text-[11px] cursor-pointer shadow focus:outline-none",
                        onclick: move |_| state.show_friend_requests_modal.set(false),
                        "Ok"
                    }
                }
            }
        }
    }
}
