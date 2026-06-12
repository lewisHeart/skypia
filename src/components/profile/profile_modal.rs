use crate::components::render_avatar;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn ProfileModal(mut state: AppState) -> Element {
    let theme = state.theme();
    let contact_opt = state.profile_modal_contact_id().and_then(|id| {
        state.contacts().iter().find(|c| c.id == id).cloned()
    });

    let is_own_profile = contact_opt.is_none();

    // Sinais locais para o formulário de edição própria
    let mut temp_name = use_signal(|| state.user_name());
    let mut temp_msg = use_signal(|| state.user_personal_message());
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut is_saving = use_signal(|| false);

    let save_profile = move |_| {
        if temp_name().trim().is_empty() {
            error_msg.set(Some("O nome de exibição não pode ser vazio.".to_string()));
            return;
        }

        is_saving.set(true);
        error_msg.set(None);

        let name = temp_name().trim().to_string();
        let msg = temp_msg().trim().to_string();
        let mut state_clone = state;
        
        spawn(async move {
            state_clone.set_user_name(name.clone());
            state_clone.set_user_personal_message(msg.clone());
            is_saving.set(false);
            state_clone.show_profile_modal.set(false);
        });
    };
    let contacts_count = state.contacts().len();

    rsx! {
        div {
            class: "fixed inset-0 bg-black/15 backdrop-blur-[1px] z-[200] flex items-center justify-center p-4 pointer-events-auto",
            onclick: move |_| state.show_profile_modal.set(false),

            div {
                class: "w-[380px] border rounded-lg shadow-2xl flex flex-col overflow-hidden pointer-events-auto",
                style: "background: {theme.bg_chat()}; border: 1.5px solid rgba(255, 255, 255, 0.45); box-shadow: 0 10px 30px rgba(0, 0, 0, 0.25), inset 0 1px 0 rgba(255, 255, 255, 0.6);",
                onclick: move |e| e.stop_propagation(),

                // Barra de Título Aero Clássica
                div { class: "h-9 bg-gradient-to-r {theme.titlebar_gradient()} border-b {theme.titlebar_border()} flex items-center justify-between px-3 flex-shrink-0 select-none",
                    div { class: "flex items-center space-x-1.5 font-bold text-[11px] {theme.titlebar_text()}",
                        img {
                            src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Person/SVG/ic_fluent_person_24_color.svg",
                            class: "w-5 h-5 object-contain pointer-events-none"
                        }
                        span {
                            if is_own_profile {
                                "Meu Perfil Pessoal"
                            } else if let Some(ref contact) = contact_opt {
                                "Perfil de {contact.display_name}"
                            } else {
                                "Perfil"
                            }
                        }
                    }
                    button {
                        class: "w-[28px] h-[18px] bg-white border border-[#d1d1d1] rounded-[3px] shadow-sm flex items-center justify-center cursor-pointer transition-all hover:bg-[#e81123] hover:border-[#e81123] hover:text-white text-[#6f6f6f] focus:outline-none text-[8px] font-bold",
                        title: "Fechar",
                        onclick: move |_| state.show_profile_modal.set(false),
                        "✕"
                    }
                }

                // Conteúdo
                div { class: "p-4 flex flex-col space-y-4 text-xs {theme.titlebar_text()}",
                    
                    if is_own_profile && error_msg().is_some() {
                        div { class: "px-3 py-2 bg-red-50 border border-red-200 rounded-lg text-[11px] text-red-700 flex items-center space-x-2 animate-pulse",
                            span { "⚠️" }
                            span { "{error_msg().unwrap()}" }
                        }
                    }

                    // Seção Superior: Avatar e Status
                    div { class: "flex items-center space-x-4 bg-white/40 p-3 rounded-xl border border-white/50 shadow-sm",
                        if is_own_profile {
                            div { 
                                class: "relative p-[2px] rounded-[8px] border {state.user_status().avatar_frame_class()} bg-transparent shadow-[inset_0_0.5px_0_rgba(255,255,255,0.4)] flex items-center justify-center cursor-pointer hover:brightness-105 transition-all",
                                title: "Alterar avatar",
                                onclick: move |_| {
                                    state.show_profile_modal.set(false);
                                    state.show_avatar_picker.set(true);
                                },
                                div {
                                    class: "rounded-[5px] overflow-hidden border border-white/35 bg-white flex-shrink-0 flex items-center justify-center",
                                    {render_avatar(state.user_avatar_url().as_deref(), 54)}
                                }
                                div {
                                    class: "absolute inset-[2px] rounded-[5px] bg-black/0 hover:bg-black/25 flex items-center justify-center opacity-0 hover:opacity-100 transition-opacity z-20",
                                    span { class: "text-white text-xs drop-shadow", "Mudar" }
                                }
                            }
                        } else if let Some(ref contact) = contact_opt {
                            div { 
                                class: "relative p-[2px] rounded-[8px] border {contact.status.avatar_frame_class()} bg-transparent shadow-[inset_0_0.5px_0_rgba(255,255,255,0.4)] flex items-center justify-center select-none",
                                div {
                                    class: "rounded-[5px] overflow-hidden border border-white/35 bg-white flex-shrink-0 flex items-center justify-center",
                                    {render_avatar(contact.avatar_url.as_deref(), 54)}
                                }
                            }
                        }
                        
                        div { class: "flex-1 min-w-0 flex flex-col space-y-1.5",
                            if is_own_profile {
                                span { class: "font-bold text-sm {theme.titlebar_text()} truncate", "{state.user_name()}" }
                                span { class: "text-[10px] text-slate-500 truncate select-all", "{state.user_email()}" }
                                div { class: "flex items-center space-x-1.5",
                                    div { class: "w-2.5 h-2.5 rounded-full {state.user_status().color_class()} border border-black/10 shadow-sm" }
                                    span { class: "font-semibold text-[10px] text-slate-600", "{state.user_status().as_str()}" }
                                }
                            } else if let Some(ref contact) = contact_opt {
                                span { class: "font-bold text-sm {theme.titlebar_text()} truncate", "{contact.display_name}" }
                                span { class: "text-[10px] text-slate-500 truncate select-all", "{contact.email}" }
                                div { class: "flex items-center space-x-1.5",
                                    div { class: "w-2.5 h-2.5 rounded-full {contact.status.color_class()} border border-black/10 shadow-sm" }
                                    span { class: "font-semibold text-[10px] text-slate-600", "{contact.status.as_str()}" }
                                }
                            }
                        }
                    }

                    // Form / Detalhes
                    div { class: "space-y-3",
                        if is_own_profile {
                            // Nome de Exibição
                            div { class: "flex flex-col space-y-1",
                                label { class: "font-bold {theme.titlebar_text()}", "Nome de Exibição (Apelido):" }
                                input {
                                    class: "w-full h-[27px] px-2.5 text-xs text-slate-800 bg-white border border-[#d1d1d1] rounded-[4px] msn-input placeholder-[#a5a5a5] placeholder:text-[10px] focus:outline-none focus:border-slate-400",
                                    placeholder: "Digite seu nome...",
                                    value: "{temp_name}",
                                    oninput: move |e| temp_name.set(e.value()),
                                }
                            }

                            // Frase Pessoal
                            div { class: "flex flex-col space-y-1",
                                label { class: "font-bold {theme.titlebar_text()}", "Frase Pessoal:" }
                                input {
                                    class: "w-full h-[27px] px-2.5 text-xs text-slate-800 bg-white border border-[#d1d1d1] rounded-[4px] msn-input placeholder-[#a5a5a5] placeholder:text-[10px] focus:outline-none focus:border-slate-400",
                                    placeholder: "O que você está pensando?",
                                    value: "{temp_msg}",
                                    oninput: move |e| temp_msg.set(e.value()),
                                }
                            }

                            // Música
                            div { class: "flex flex-col space-y-1",
                                label { class: "font-bold {theme.titlebar_text()}", "Música Ouvindo:" }
                                div { class: "flex items-center justify-between p-2 bg-white/50 border {theme.titlebar_border()} rounded text-[11px] {theme.titlebar_text()} font-medium",
                                    span { "🎵 {state.user_music().unwrap_or_else(|| \"Nenhuma música definida\".to_string())}" }
                                    button {
                                        class: "text-[10px] {theme.titlebar_text()} hover:underline cursor-pointer font-bold focus:outline-none",
                                        onclick: move |_| {
                                            state.show_profile_modal.set(false);
                                            state.show_music_player_modal.set(true);
                                        },
                                        "Mudar"
                                    }
                                }
                            }
                        } else if let Some(ref contact) = contact_opt {
                            // Detalhes do contato estáticos
                            div { class: "flex flex-col space-y-1",
                                label { class: "font-bold {theme.titlebar_text()}", "Nome de Exibição / Apelido:" }
                                div { class: "p-2 bg-white/50 border border-white/70 rounded text-xs text-slate-700",
                                    if let Some(ref nick) = contact.nickname {
                                        "{nick} ({contact.display_name})"
                                    } else {
                                        "{contact.display_name}"
                                    }
                                }
                            }

                            div { class: "flex flex-col space-y-1",
                                label { class: "font-bold {theme.titlebar_text()}", "Frase Pessoal:" }
                                div { class: "p-2 bg-white/50 border border-white/70 rounded text-xs text-slate-700 italic",
                                    if contact.personal_message.is_empty() {
                                        "Nenhuma frase pessoal definida."
                                    } else {
                                        "“{contact.personal_message}”"
                                    }
                                }
                            }

                            div { class: "flex flex-col space-y-1",
                                label { class: "font-bold {theme.titlebar_text()}", "Música Ouvindo:" }
                                div { class: "p-2 bg-white/50 border border-white/70 rounded text-xs text-[#0066cc] font-medium flex items-center space-x-1.5",
                                    span { "🎵" }
                                    span { "{contact.music_listening.as_deref().unwrap_or(\"Nenhuma música sendo ouvida no momento\")}" }
                                }
                            }

                            div { class: "flex flex-col space-y-1",
                                label { class: "font-bold {theme.titlebar_text()}", "Relação no MSN:" }
                                div { class: "p-2 bg-white/50 border border-white/70 rounded text-xs text-slate-700 flex items-center space-x-1.5",
                                    span { if contact.is_favorite { "⭐ Favorito" } else { "👥 Contato Comum" } }
                                    span { "•" }
                                    span { "{contact.relation_status}" }
                                }
                            }
                        }
                    }

                    // Estatísticas de Uso / Info Geral
                    div { class: "bg-white/40 border border-white/50 rounded-xl p-3 flex flex-col space-y-1.5 shadow-sm text-[10.5px] leading-relaxed text-slate-600",
                        span { class: "font-bold {theme.titlebar_text()} mb-0.5 block", "Estatísticas da Conta" }
                        if is_own_profile {
                            div { class: "flex justify-between",
                                span { "Total de contatos na lista:" }
                                span { class: "font-bold text-slate-800", "{contacts_count} contatos" }
                            }
                            div { class: "flex justify-between",
                                span { "ID de Usuário Skypia:" }
                                span { class: "font-mono font-medium text-[9px] text-slate-500 select-all truncate max-w-[150px]", "{state.server_user_id().unwrap_or_default()}" }
                            }
                        } else if let Some(ref contact) = contact_opt {
                            div { class: "flex justify-between",
                                span { "ID de Contato:" }
                                span { class: "font-mono font-medium text-[9px] text-slate-500 select-all truncate max-w-[170px]", "{contact.id}" }
                            }
                            div { class: "flex justify-between",
                                span { "Status de Amizade:" }
                                span { class: "font-bold text-slate-850", "Conectado" }
                            }
                        }
                    }
                }

                // Botões de Ação
                div { class: "px-4 py-3 bg-white/10 border-t {theme.titlebar_border()}/30 flex items-center justify-end space-x-2",
                    if is_own_profile {
                        button {
                            class: "px-4 py-1.5 bg-white hover:bg-slate-100 border border-slate-350 text-slate-700 rounded-[4px] font-bold cursor-pointer transition-colors focus:outline-none text-[10px]",
                            onclick: move |_| state.show_profile_modal.set(false),
                            "Cancelar"
                        }
                        button {
                            class: "px-5 py-1.5 {theme.btn_primary()} rounded-[4px] font-bold shadow-md cursor-pointer transition-all disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none text-[10px]",
                            disabled: is_saving(),
                            onclick: save_profile,
                            if is_saving() { "Salvando..." } else { "Salvar" }
                        }
                    } else {
                        button {
                            class: "px-6 py-1.5 {theme.btn_primary()} rounded-[4px] font-bold shadow-md cursor-pointer transition-all focus:outline-none text-[10px]",
                            onclick: move |_| state.show_profile_modal.set(false),
                            "Fechar"
                        }
                    }
                }
            }
        }
    }
}

