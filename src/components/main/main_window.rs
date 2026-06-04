use crate::components::main::contact_list::ContactList;
use crate::components::profile::profile_header::ProfileHeader;
use crate::models::{AppTheme, UserStatus};
use crate::sound::play_sound;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn MainWindow(mut state: AppState) -> Element {
    // Sinais para os modais locais
    let mut add_contact_email = use_signal(|| String::new());
    let mut show_pending_modal = use_signal(|| false);

    rsx! {
        div {
            class: "w-full h-full flex flex-col select-none bg-bubbles bg-gradient-to-b from-[#e6f1fc]/90 to-[#c8def5]/80 overflow-hidden",

            // Header do Perfil do usuário
            ProfileHeader { state }

            // Barra de Inbox e Solicitações de Amizade recebidas
            div {
                class: "h-6 bg-white/20 border-b border-[#a6b9cd]/25 px-4 flex items-center justify-between text-[10px] text-[#2f4b6c]/90 flex-shrink-0 select-none",

                // Botão de Solicitações Pendentes
                button {
                    class: "hover:text-[#0066cc] font-medium flex items-center space-x-1 cursor-pointer transition-colors focus:outline-none",
                    onclick: move |_| {
                        show_pending_modal.set(true);
                    },
                    span { class: "text-xs mr-0.5", "👥" }
                    span { "Solicitações de Amizade" }
                    if !state.pending_requests().is_empty() {
                        span { class: "ml-1.5 px-1.5 py-0.5 bg-red-500 text-white rounded-full text-[9px] font-bold shadow-sm animate-pulse", "{state.pending_requests().len()}" }
                    }
                }

                // Status de sincronização
                span { class: "text-slate-400 text-[9px]", "Rede Skypia Conectada" }
            }

            // Lista de Contatos com pesquisa integrada
            ContactList { state }

            // Banner dinâmico de anúncios do banco de dados
            if let Some(banner) = state.banner_info() {
                div {
                    class: "h-[50px] w-full bg-gradient-to-r from-sky-100 to-sky-200 border-t border-sky-300 px-3 flex items-center justify-between text-[11px] shadow-inner flex-shrink-0 cursor-pointer overflow-hidden transition-all hover:brightness-105",
                    onclick: move |_| {
                        let _ = document::eval(&format!("window.open('{}', '_blank')", banner.link));
                    },
                    div { class: "flex items-center space-x-2 flex-1 text-[#2f4b6c] min-w-0",
                        span { class: "text-base flex-shrink-0", "{banner.icon}" }
                        div { class: "flex flex-col min-w-0 flex-1",
                            span { class: "font-bold text-[#0066cc] truncate", "{banner.text}" }
                            span { class: "text-[10px] text-slate-500 truncate hover:underline", "{banner.action_label}" }
                        }
                    }
                }
            }

            // Rodapé com o botão Adicionar contato fixo
            div { class: "h-9 bg-white/45 border-t border-white/20 px-3 flex items-center justify-between text-xs text-[#2f4b6c]/90 flex-shrink-0",
                button {
                    class: "hover:text-[#0066cc] font-semibold flex items-center space-x-1 transition-colors cursor-pointer",
                    onclick: move |_| {
                        state.show_add_contact_modal.set(true);
                    },
                    span { "➕" }
                    span { "Adicionar contato" }
                }
                span { class: "text-slate-400 text-[10px]", "v0.0.1" }
            }
        }

        // ==========================================
        // MODAL DE CONFIGURAÇÕES
        // ==========================================
        if state.show_settings_modal() {
            div {
                class: "fixed inset-0 bg-black/45 backdrop-blur-sm z-[200] flex items-center justify-center p-4",
                onclick: move |_| state.show_settings_modal.set(false),
                div {
                    class: "w-80 bg-gradient-to-b from-[#e6f1fc] to-[#c8def5] border border-[#7ba9d4] rounded-lg shadow-2xl p-4 flex flex-col space-y-4 text-xs text-[#1e395b] pointer-events-auto",
                    onclick: move |e| e.stop_propagation(),

                    div { class: "flex items-center justify-between border-b border-white/40 pb-2",
                        span { class: "font-bold text-sm", "⚙️ Configurações do Skypia" }
                        button {
                            class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-500 hover:text-white border border-transparent font-bold cursor-pointer transition-colors",
                            onclick: move |_| state.show_settings_modal.set(false),
                            "✕"
                        }
                    }

                    div { class: "flex flex-col space-y-1.5",
                        label { class: "font-bold text-slate-700", "Estilo de Decorações da Janela" }
                        label { class: "flex items-center space-x-2 cursor-pointer",
                            input {
                                r#type: "checkbox",
                                checked: state.use_custom_titlebar(),
                                onchange: move |e| {
                                    let val = e.value() == "true";
                                    state.set_settings(state.interface_scale(), val, state.theme());
                                    #[cfg(feature = "desktop")]
                                    dioxus::desktop::use_window().set_decorations(!val);
                                }
                            }
                            span { "Usar barra de título Aero do app" }
                        }
                    }

                    div { class: "flex flex-col space-y-1.5",
                        label { class: "font-bold text-slate-700", "Escala da Interface" }
                        select {
                            class: "w-full p-1.5 border border-[#a6b9cd] rounded bg-white text-slate-700 font-medium",
                            onchange: move |e| {
                                let scale = e.value().parse::<f64>().unwrap_or(1.0);
                                state.set_settings(scale, state.use_custom_titlebar(), state.theme());
                            },
                            option { value: "0.8", selected: state.interface_scale() == 0.8, "80% (Pequeno)" }
                            option { value: "0.9", selected: state.interface_scale() == 0.9, "90%" }
                            option { value: "1.0", selected: state.interface_scale() == 1.0, "100% (Padrão)" }
                            option { value: "1.1", selected: state.interface_scale() == 1.1, "110%" }
                            option { value: "1.2", selected: state.interface_scale() == 1.2, "120%" }
                            option { value: "1.3", selected: state.interface_scale() == 1.3, "130%" }
                            option { value: "1.4", selected: state.interface_scale() == 1.4, "140%" }
                            option { value: "1.5", selected: state.interface_scale() == 1.5, "150% (Grande)" }
                        }
                    }

                    div { class: "flex flex-col space-y-1.5",
                        label { class: "font-bold text-slate-700", "Aparência (Skins)" }
                        select {
                            class: "w-full p-1.5 border border-[#a6b9cd] rounded bg-white text-slate-700 font-medium",
                            onchange: move |e| {
                                let new_theme = match e.value().as_str() {
                                    "blue" => AppTheme::AeroBlue,
                                    "pink" => AppTheme::RubyPink,
                                    "green" => AppTheme::ForestGreen,
                                    "silver" => AppTheme::SilverMetallic,
                                    _ => AppTheme::AeroBlue,
                                };
                                state.set_settings(state.interface_scale(), state.use_custom_titlebar(), new_theme);
                            },
                            option { value: "blue", selected: state.theme() == AppTheme::AeroBlue, "Azul Aero" }
                            option { value: "pink", selected: state.theme() == AppTheme::RubyPink, "Rosa Choque" }
                            option { value: "green", selected: state.theme() == AppTheme::ForestGreen, "Verde Natureza" }
                            option { value: "silver", selected: state.theme() == AppTheme::SilverMetallic, "Prata Metálico" }
                        }
                    }

                    div { class: "flex justify-end pt-2 border-t border-white/40",
                        button {
                            class: "px-4 py-1.5 bg-gradient-to-b from-[#8fc1e9] to-[#4585c5] text-white border border-[#4074a8] rounded font-bold shadow hover:from-[#9bd0fa] hover:to-[#579adf] cursor-pointer transition-colors",
                            onclick: move |_| state.show_settings_modal.set(false),
                            "Ok"
                        }
                    }
                }
            }
        }

        // ==========================================
        // MODAL DE ADICIONAR CONTATO
        // ==========================================
        if state.show_add_contact_modal() {
            div {
                class: "fixed inset-0 bg-black/45 backdrop-blur-sm z-[200] flex items-center justify-center p-4",
                onclick: move |_| state.show_add_contact_modal.set(false),
                div {
                    class: "w-80 bg-gradient-to-b from-[#e6f1fc] to-[#c8def5] border border-[#7ba9d4] rounded-lg shadow-2xl p-4 flex flex-col space-y-3.5 text-xs text-[#1e395b] pointer-events-auto",
                    onclick: move |e| e.stop_propagation(),

                    div { class: "flex items-center justify-between border-b border-white/40 pb-2",
                        span { class: "font-bold text-sm", "➕ Adicionar Novo Contato" }
                        button {
                            class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-500 hover:text-white border border-transparent font-bold cursor-pointer transition-colors",
                            onclick: move |_| state.show_add_contact_modal.set(false),
                            "✕"
                        }
                    }

                    div { class: "flex flex-col space-y-1",
                        label { class: "font-semibold text-slate-700", "Email ou Nome de usuário:" }
                        input {
                            class: "w-full p-1.5 border border-[#a6b9cd] rounded msn-input",
                            placeholder: "Ex: wellington ou wk.scbd@skypia.io",
                            value: "{add_contact_email}",
                            oninput: move |e| add_contact_email.set(e.value()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter && !add_contact_email().trim().is_empty() {
                                    state.add_contact_dynamic(
                                        add_contact_email().trim().to_string(),
                                        "".to_string(),
                                        UserStatus::Offline,
                                        "".to_string()
                                    );
                                    play_sound("online");
                                    state.show_add_contact_modal.set(false);
                                }
                            }
                        }
                    }

                    div { class: "flex justify-end space-x-2 pt-2 border-t border-white/40",
                        button {
                            class: "px-3 py-1 bg-white hover:bg-slate-100 border border-slate-350 rounded font-bold cursor-pointer transition-colors",
                            onclick: move |_| state.show_add_contact_modal.set(false),
                            "Cancelar"
                        }
                        button {
                            class: "px-4 py-1 bg-gradient-to-b from-[#8fc1e9] to-[#4585c5] text-white border border-[#4074a8] rounded font-bold shadow hover:from-[#9bd0fa] hover:to-[#579adf] cursor-pointer transition-colors",
                            disabled: add_contact_email().trim().is_empty(),
                            onclick: move |_| {
                                if !add_contact_email().trim().is_empty() {
                                    state.add_contact_dynamic(
                                        add_contact_email().trim().to_string(),
                                        "".to_string(),
                                        UserStatus::Offline,
                                        "".to_string()
                                    );
                                    play_sound("online");
                                    state.show_add_contact_modal.set(false);
                                }
                            },
                            "Adicionar"
                        }
                    }
                }
            }
        }

        // ==========================================
        // MODAL DE SOLICITAÇÕES PENDENTES
        // ==========================================
        if show_pending_modal() {
            div {
                class: "fixed inset-0 bg-black/45 backdrop-blur-sm z-[200] flex items-center justify-center p-4",
                onclick: move |_| show_pending_modal.set(false),
                div {
                    class: "w-80 bg-gradient-to-b from-[#e6f1fc] to-[#c8def5] border border-[#7ba9d4] rounded-lg shadow-2xl p-4 flex flex-col space-y-3.5 text-xs text-[#1e395b] pointer-events-auto",
                    onclick: move |e| e.stop_propagation(),

                    div { class: "flex items-center justify-between border-b border-white/40 pb-2",
                        span { class: "font-bold text-sm", "👥 Solicitações de Amizade" }
                        button {
                            class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-500 hover:text-white border border-transparent font-bold cursor-pointer transition-colors",
                            onclick: move |_| show_pending_modal.set(false),
                            "✕"
                        }
                    }

                    div { class: "flex flex-col space-y-2 max-h-48 overflow-y-auto pr-1",
                        if state.pending_requests().is_empty() {
                            div { class: "text-center text-slate-500 py-4", "Nenhuma solicitação pendente." }
                        } else {
                            for request in state.pending_requests() {
                                {
                                    let req_id_accept = request.id.clone();
                                    let req_id_reject = request.id.clone();
                                    rsx! {
                                        div { class: "flex items-center justify-between p-2 bg-white/40 rounded border border-white/20",
                                            div { class: "flex flex-col min-w-0 mr-2",
                                                span { class: "font-semibold truncate", "{request.display_name}" }
                                                span { class: "text-[10px] text-slate-500 truncate", "{request.email}" }
                                            }
                                            div { class: "flex space-x-1.5 flex-shrink-0",
                                                button {
                                                    class: "px-2 py-1 bg-green-600 hover:bg-green-700 text-white rounded text-[10px] font-bold cursor-pointer transition-colors shadow-sm",
                                                    onclick: move |_| {
                                                        state.accept_friend_request(req_id_accept.clone());
                                                    },
                                                    "Aceitar"
                                                }
                                                button {
                                                    class: "px-2 py-1 bg-red-600 hover:bg-red-700 text-white rounded text-[10px] font-bold cursor-pointer transition-colors shadow-sm",
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

                    div { class: "flex justify-end pt-2 border-t border-white/40",
                        button {
                            class: "px-4 py-1.5 bg-gradient-to-b from-[#8fc1e9] to-[#4585c5] text-white border border-[#4074a8] rounded font-bold shadow hover:from-[#9bd0fa] hover:to-[#579adf] cursor-pointer transition-colors",
                            onclick: move |_| show_pending_modal.set(false),
                            "Fechar"
                        }
                    }
                }
            }
        }
    }
}
