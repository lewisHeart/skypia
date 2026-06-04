use crate::components::contact_list::ContactList;
use crate::components::profile_header::ProfileHeader;
use crate::models::{AppTheme, UserStatus};
use crate::sound::play_sound;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn MainWindow(mut state: AppState) -> Element {
    // Sinais para os modais locais
    let mut add_contact_email = use_signal(|| String::new());
    let mut add_contact_name = use_signal(|| String::new());
    let mut add_contact_pm = use_signal(|| String::new());
    let mut add_contact_status = use_signal(|| UserStatus::Online);



    rsx! {
        div {
            class: "w-full h-full flex flex-col select-none bg-bubbles bg-gradient-to-b from-[#e6f1fc]/90 to-[#c8def5]/80 overflow-hidden",

            // Header do Perfil do usuário
            ProfileHeader { state }

            // Barra de Inbox e Skypia Hoje (Portal de novidades e e-mails)
            div {
                class: "h-6 bg-white/20 border-b border-[#a6b9cd]/25 px-4 flex items-center justify-between text-[10px] text-[#2f4b6c]/90 flex-shrink-0 select-none",

                // Caixa de Entrada (Email)
                button {
                    class: "hover:text-[#0066cc] font-medium flex items-center space-x-1 cursor-pointer transition-colors focus:outline-none",
                    onclick: move |_| {
                        state.add_toast("Hotmail".to_string(), "Abrindo sua caixa de entrada...".to_string(), 0);
                        let _ = document::eval("window.open('https://outlook.live.com', '_blank')");
                    },
                    span { class: "text-xs", "✉" }
                    span { "testando" }
                }

                // Portal Hoje
                button {
                    class: "hover:text-[#0066cc] font-medium flex items-center space-x-1 cursor-pointer transition-colors focus:outline-none",
                    onclick: move |_| {
                        state.add_toast("Skypia Hoje".to_string(), "Abrindo portal de novidades...".to_string(), 0);
                        let _ = document::eval("window.open('https://msn.com', '_blank')");
                    },
                    span { "Skypia Hoje" }
                }
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
                        label { class: "font-semibold text-slate-700", "Endereço de email:" }
                        input {
                            r#type: "email",
                            class: "w-full p-1.5 border border-[#a6b9cd] rounded msn-input",
                            placeholder: "exemplo@skypia.io",
                            value: "{add_contact_email}",
                            oninput: move |e| add_contact_email.set(e.value()),
                        }
                    }

                    div { class: "flex flex-col space-y-1",
                        label { class: "font-semibold text-slate-700", "Nome de Exibição (Apelido):" }
                        input {
                            class: "w-full p-1.5 border border-[#a6b9cd] rounded msn-input",
                            placeholder: "Apelido do contato",
                            value: "{add_contact_name}",
                            oninput: move |e| add_contact_name.set(e.value()),
                        }
                    }

                    div { class: "flex flex-col space-y-1",
                        label { class: "font-semibold text-slate-700", "Frase Pessoal Inicial:" }
                        input {
                            class: "w-full p-1.5 border border-[#a6b9cd] rounded msn-input",
                            placeholder: "Olá, sou novo no Skypia!",
                            value: "{add_contact_pm}",
                            oninput: move |e| add_contact_pm.set(e.value()),
                        }
                    }

                    div { class: "flex flex-col space-y-1",
                        label { class: "font-semibold text-slate-700", "Status Inicial:" }
                        select {
                            class: "w-full p-1.5 border border-[#a6b9cd] rounded bg-white text-slate-700 font-medium",
                            onchange: move |e| {
                                match e.value().as_str() {
                                    "online" => add_contact_status.set(UserStatus::Online),
                                    "busy" => add_contact_status.set(UserStatus::Ocupado),
                                    "away" => add_contact_status.set(UserStatus::Ausente),
                                    "offline" => add_contact_status.set(UserStatus::Offline),
                                    _ => {}
                                }
                            },
                            option { value: "online", "Disponível" }
                            option { value: "busy", "Ocupado" }
                            option { value: "away", "Ausente" }
                            option { value: "offline", "Offline" }
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
                            onclick: move |_| {
                                if !add_contact_email().is_empty() && !add_contact_name().is_empty() {
                                    state.add_contact_dynamic(
                                        add_contact_email(),
                                        add_contact_name(),
                                        add_contact_status(),
                                        add_contact_pm()
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
    }
}
