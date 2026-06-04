use dioxus::prelude::*;
use crate::models::UserStatus;
use crate::state::AppState;
use crate::sound::play_sound;

#[component]
pub fn Login(mut state: AppState) -> Element {
    let mut email = use_signal(|| "lewis_vintage@hotmail.com".to_string());
    let mut password = use_signal(|| "••••••••".to_string());
    let mut remember_me = use_signal(|| true);
    let mut auto_sign_in = use_signal(|| false);
    let mut selected_status = use_signal(|| UserStatus::Online);

    let handle_login = move |_| {
        if state.signing_in() {
            return;
        }
        
        *state.signing_in.write() = true;
        
        // Simulate network login delay (nostalgic spinning animation)
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(2200)).await;
            
            state.set_user_status(selected_status());
            *state.user_email.write() = email();
            // Extrapolate name from email
            if let Some(name_part) = email().split('@').next() {
                let formatted = name_part
                    .replace('.', " ")
                    .replace('_', " ");
                let capitalized = formatted
                    .split_whitespace()
                    .map(|w| format!("{}{}", &w[..1].to_uppercase(), &w[1..]))
                    .collect::<Vec<String>>()
                    .join(" ");
                *state.user_name.write() = capitalized;
            }
            
            *state.signing_in.write() = false;
            *state.logged_in.write() = true;
            
            // Play MSN sign-in sound
            play_sound("online");
            
            // Add initial greeting toast
            state.add_toast(
                "Bem-vindo de volta!".to_string(),
                "Você acabou de entrar no Skypia Messenger.".to_string(),
                0,
            );
        });
    };

    rsx! {
        div {
            class: "w-full h-full flex flex-col items-center justify-center select-none bg-bubbles relative p-4",
            style: "background: linear-gradient(180deg, rgba(230, 241, 252, 0.95) 0%, rgba(200, 222, 245, 0.9) 100%);",
            
            div {
                class: "w-full max-w-[340px] flex flex-col items-center p-6 rounded-lg aero-glass bg-white/20 border border-white/40 shadow-lg",
                
                // Rotating Avatar or Loading Animation
                div { class: "h-28 flex items-center justify-center relative mb-4",
                    if state.signing_in() {
                        div { class: "flex flex-col items-center space-y-2 animate-msn-spin",
                            svg { view_box: "0 0 100 100", class: "w-20 h-20 filter drop-shadow-md",
                                g { fill: "#00a1df",
                                    circle { cx: "38", cy: "35", r: "16" }
                                    path { d: "M18 75 C18 53, 58 53, 58 75 Z" }
                                }
                                g { fill: "#6bb566",
                                    circle { cx: "62", cy: "45", r: "14" }
                                    path { d: "M45 80 C45 60, 79 60, 79 80 Z" }
                                }
                            }
                        }
                    } else {
                        // Standard avatar frame
                        div { class: "avatar-frame border-2 border-white bg-gradient-to-b from-sky-100 to-sky-200",
                            svg { view_box: "0 0 100 100", class: "w-16 h-16",
                                g { fill: "#cbdde8",
                                    circle { cx: "50", cy: "38", r: "18" }
                                    path { d: "M20 82 C20 58, 80 58, 80 82 Z" }
                                }
                            }
                            // Small overlay buddy
                            div { class: "absolute bottom-1 right-1 w-4 h-4 rounded-full bg-white border border-[#a6b9cd] flex items-center justify-center shadow",
                                div { class: "w-2 h-2 rounded-full {selected_status().color_class()} border border-black/10" }
                            }
                        }
                    }
                }

                // Sign In Form
                if state.signing_in() {
                    div { class: "w-full text-center space-y-3 mt-2",
                        p { class: "text-xs text-[#1e395b] font-semibold animate-pulse", "Entrando..." }
                        div { class: "w-40 h-2 bg-white/80 border border-[#a6b9cd] rounded-full mx-auto overflow-hidden shadow-inner",
                            div { 
                                class: "h-full bg-gradient-to-r from-sky-400 via-blue-500 to-sky-400 rounded-full animate-pulse",
                                style: "width: 75%; transition: width 1s ease;"
                            }
                        }
                        span { class: "text-[10px] text-slate-500 block hover:underline cursor-pointer",
                            onclick: move |_| {
                                *state.signing_in.write() = false;
                            },
                            "Cancelar"
                        }
                    }
                } else {
                    div { class: "w-full space-y-3.5",
                        // Email field
                        div { class: "space-y-1",
                            label { class: "block text-xs font-semibold text-[#2f4b6c]/90", "Endereço de email:" }
                            input {
                                r#type: "email",
                                class: "w-full px-2.5 py-1.5 text-xs msn-input rounded",
                                placeholder: "exemplo@hotmail.com",
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                            }
                        }

                        // Password field
                        div { class: "space-y-1",
                            label { class: "block text-xs font-semibold text-[#2f4b6c]/90", "Senha:" }
                            input {
                                r#type: "password",
                                class: "w-full px-2.5 py-1.5 text-xs msn-input rounded",
                                value: "{password}",
                                oninput: move |e| password.set(e.value()),
                            }
                        }

                        // Status selection
                        div { class: "space-y-1",
                            label { class: "block text-xs font-semibold text-[#2f4b6c]/90", "Entrar como:" }
                            select {
                                class: "w-full px-2 py-1.5 text-xs msn-input rounded bg-white font-medium text-slate-700",
                                onchange: move |e| {
                                    match e.value().as_str() {
                                        "online" => selected_status.set(UserStatus::Online),
                                        "busy" => selected_status.set(UserStatus::Ocupado),
                                        "away" => selected_status.set(UserStatus::Ausente),
                                        "invisible" => selected_status.set(UserStatus::Invisivel),
                                        _ => {}
                                    }
                                },
                                option { value: "online", selected: true, "Disponível" }
                                option { value: "busy", "Ocupado" }
                                option { value: "away", "Ausente" }
                                option { value: "invisible", "Invisível (Offline)" }
                            }
                        }

                        // Checkboxes
                        div { class: "space-y-1.5 pt-0.5 text-xs text-[#2f4b6c]/95",
                            label { class: "flex items-center space-x-2 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded border-slate-300 text-sky-600 focus:ring-sky-500",
                                    checked: remember_me(),
                                    onchange: move |e| remember_me.set(e.value() == "true"),
                                }
                                span { "Lembrar-me nesta máquina" }
                            }
                            label { class: "flex items-center space-x-2 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded border-slate-300 text-sky-600 focus:ring-sky-500",
                                    checked: auto_sign_in(),
                                    onchange: move |e| auto_sign_in.set(e.value() == "true"),
                                }
                                span { "Entrar automaticamente" }
                            }
                        }

                        // Sign In button
                        button {
                            class: "w-full py-2 bg-gradient-to-b from-[#8fc1e9] via-[#5c98d6] to-[#4585c5] hover:from-[#9bd0fa] hover:via-[#70abeb] hover:to-[#579adf] text-white border border-[#4074a8] rounded font-bold text-xs shadow-md shadow-sky-900/10 cursor-pointer active:scale-[0.98] transition-transform",
                            onclick: handle_login,
                            "Entrar"
                        }
                    }
                }
                
                // Footer details
                div { 
                    class: "w-full flex items-center justify-between mt-5 pt-3 border-t border-slate-350 text-[9px] text-slate-500/80",
                    span { "Skypia © 2010" }
                    a { 
                        href: "#", 
                        class: "hover:underline text-[#245284] font-semibold",
                        "Recuperar senha"
                    }
                }
            }
        }
    }
}
