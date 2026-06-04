use crate::models::UserStatus;
use crate::services::api;
use crate::sound::play_sound;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn Login(mut state: AppState) -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut selected_status = use_signal(|| UserStatus::Online);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut remember_me = use_signal(|| true);

    // Tenta auto-login ao montar o componente
    use_effect(move || {
        let mut state = state;
        spawn(async move {
            if let Ok(Some((token, _user_id))) =
                crate::services::db::DatabaseService::load_auth_token().await
            {
                match api::get_profile(&token).await {
                    Ok(profile) => {
                        state.apply_server_profile(profile, token).await;
                        state.set_user_status(UserStatus::Online);
                        *state.logged_in.write() = true;
                        play_sound("online");
                        state.add_toast(
                            "Bem-vindo de volta!".to_string(),
                            "Sessão restaurada com sucesso.".to_string(),
                            None,
                        );
                    }
                    Err(_) => {
                        // Token expirado — limpa
                        let _ = crate::services::db::DatabaseService::clear_auth_token().await;
                    }
                }
            }
        });
    });

    // Extrai a lógica em uma fn separada para poder usar em onkeydown e onclick sem mover
    let mut do_login = move || {
        if state.signing_in() {
            return;
        }
        if email().trim().is_empty() || password().is_empty() {
            error_msg.set(Some("Preencha email e senha.".to_string()));
            return;
        }

        error_msg.set(None);
        *state.signing_in.write() = true;

        let email_val = email().trim().to_string();
        let password_val = password();
        let status = selected_status();
        let mut state = state;

        spawn(async move {
            match api::login(email_val, password_val).await {
                Ok(auth) => {
                    state.apply_server_profile(auth.user, auth.token).await;
                    state.set_user_status(status);
                    *state.signing_in.write() = false;
                    *state.logged_in.write() = true;
                    play_sound("online");
                    state.add_toast(
                        "Bem-vindo de volta!".to_string(),
                        "Você entrou no Skypia Messenger.".to_string(),
                        None,
                    );
                }
                Err(e) => {
                    *state.signing_in.write() = false;
                    *error_msg.write() = Some(e);
                }
            }
        });
    };

    rsx! {
        div {
            class: "w-full h-full flex flex-col items-center justify-center select-none relative p-4",
            style: "background: linear-gradient(180deg, rgba(230, 241, 252, 0.95) 0%, rgba(200, 222, 245, 0.9) 100%);",

            div {
                class: "w-full max-w-[340px] flex flex-col items-center p-6 rounded-xl aero-glass bg-white/30 border border-white/50 shadow-2xl backdrop-blur-md",

                // Logo animado
                div { class: "h-24 flex items-center justify-center relative mb-4",
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
                        div { class: "avatar-frame border-2 border-white/70 bg-gradient-to-b from-sky-100 to-sky-200 shadow-md",
                            svg { view_box: "0 0 100 100", class: "w-16 h-16",
                                g { fill: "#cbdde8",
                                    circle { cx: "50", cy: "38", r: "18" }
                                    path { d: "M20 82 C20 58, 80 58, 80 82 Z" }
                                }
                            }
                            div { class: "absolute bottom-1 right-1 w-4 h-4 rounded-full bg-white border border-[#a6b9cd] flex items-center justify-center shadow",
                                div { class: "w-2 h-2 rounded-full {selected_status().color_class()} border border-black/10" }
                            }
                        }
                    }
                }

                // Título
                div { class: "mb-4 text-center",
                    h1 { class: "text-sm font-bold text-[#1b324d]", "Skypia Messenger" }
                    p { class: "text-[10px] text-slate-500 mt-0.5", "Entre na sua conta" }
                }

                if state.signing_in() {
                    div { class: "w-full text-center space-y-3 mt-2",
                        p { class: "text-xs text-[#1e395b] font-semibold animate-pulse", "Entrando..." }
                        div { class: "w-40 h-2 bg-white/80 border border-[#a6b9cd] rounded-full mx-auto overflow-hidden shadow-inner",
                            div {
                                class: "h-full bg-gradient-to-r from-sky-400 via-blue-500 to-sky-400 rounded-full animate-pulse",
                                style: "width: 75%;"
                            }
                        }
                        span { class: "text-[10px] text-slate-500 block hover:underline cursor-pointer",
                            onclick: move |_| { *state.signing_in.write() = false; },
                            "Cancelar"
                        }
                    }
                } else {
                    div { class: "w-full space-y-3",

                        // Mensagem de erro
                        if let Some(err) = error_msg() {
                            div { class: "w-full px-3 py-2 bg-red-50 border border-red-200 rounded-lg text-xs text-red-700 flex items-center space-x-2",
                                span { "⚠️" }
                                span { "{err}" }
                            }
                        }

                        // Email
                        div { class: "space-y-1",
                            label { class: "block text-xs font-semibold text-[#2f4b6c]/90", "Email:" }
                            input {
                                r#type: "email",
                                class: "w-full px-2.5 py-1.5 text-xs msn-input rounded-lg",
                                placeholder: "exemplo@hotmail.com",
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter { do_login(); }
                                }
                            }
                        }

                        // Senha
                        div { class: "space-y-1",
                            label { class: "block text-xs font-semibold text-[#2f4b6c]/90", "Senha:" }
                            input {
                                r#type: "password",
                                class: "w-full px-2.5 py-1.5 text-xs msn-input rounded-lg",
                                placeholder: "••••••••",
                                value: "{password}",
                                oninput: move |e| password.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter { do_login(); }
                                }
                            }
                        }

                        // Status
                        div { class: "space-y-1",
                            label { class: "block text-xs font-semibold text-[#2f4b6c]/90", "Entrar como:" }
                            select {
                                class: "w-full px-2 py-1.5 text-xs msn-input rounded-lg bg-white font-medium text-slate-700",
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
                                option { value: "invisible", "Invisível" }
                            }
                        }

                        // Lembrar
                        label { class: "flex items-center space-x-2 cursor-pointer text-xs text-[#2f4b6c]/90",
                            input {
                                r#type: "checkbox",
                                class: "rounded border-slate-300 text-sky-600",
                                checked: remember_me(),
                                onchange: move |e| remember_me.set(e.value() == "true"),
                            }
                            span { "Lembrar-me nesta máquina" }
                        }

                        // Botão entrar
                        button {
                            class: "w-full py-2 bg-gradient-to-b from-[#8fc1e9] via-[#5c98d6] to-[#4585c5] hover:from-[#9bd0fa] hover:via-[#70abeb] hover:to-[#579adf] text-white border border-[#4074a8] rounded-lg font-bold text-xs shadow-md cursor-pointer active:scale-[0.98] transition-all",
                            onclick: move |_| do_login(),
                            "Entrar"
                        }
                    }
                }

                // Footer
                div { class: "w-full flex items-center justify-between mt-4 pt-3 border-t border-white/40 text-[10px] text-slate-500/80",
                    button {
                        class: "hover:underline text-[#245284] font-semibold cursor-pointer",
                        onclick: move |_| state.show_register_modal.set(true),
                        "Criar conta"
                    }
                    span { "Skypia © 2026" }
                }
            }

            // Modal de cadastro
            if state.show_register_modal() {
                crate::components::auth::register::Register { state }
            }
        }
    }
}
