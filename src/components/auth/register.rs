use crate::models::UserStatus;
use crate::services::api;
use crate::sound::play_sound;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn Register(mut state: AppState) -> Element {
    let mut display_name = use_signal(|| String::new());
    let mut username = use_signal(|| String::new());
    let mut full_name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut confirm_password = use_signal(|| String::new());
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut is_loading = use_signal(|| false);

    // Closure de registro — pode ser clonada pois signals são Copy
    // Usamos uma macro para não duplicar lógica
    let mut do_register = move || {
        if is_loading() {
            return;
        }

        let name = display_name().trim().to_string();
        let user_val = username().trim().to_lowercase();
        let full_val = full_name().trim().to_string();
        let email_val = email().trim().to_string();
        let pass = password();
        let confirm = confirm_password();

        if name.is_empty() {
            error_msg.set(Some("Nome de exibição é obrigatório.".to_string()));
            return;
        }
        if user_val.is_empty() {
            error_msg.set(Some("Nome de usuário é obrigatório.".to_string()));
            return;
        }
        if user_val.contains('@') {
            error_msg.set(Some("Nome de usuário não pode conter '@'.".to_string()));
            return;
        }
        if full_val.is_empty() {
            error_msg.set(Some("Nome completo é obrigatório.".to_string()));
            return;
        }
        if email_val.is_empty() || !email_val.contains('@') {
            error_msg.set(Some("Email inválido.".to_string()));
            return;
        }
        if pass.len() < 6 {
            error_msg.set(Some("A senha deve ter no mínimo 6 caracteres.".to_string()));
            return;
        }
        if pass != confirm {
            error_msg.set(Some("As senhas não coincidem.".to_string()));
            return;
        }

        error_msg.set(None);
        is_loading.set(true);

        let mut state = state;
        spawn(async move {
            match api::register(email_val, user_val, full_val, pass, name).await {
                Ok(auth) => {
                    state.apply_server_profile(auth.user, auth.token).await;
                    state.set_user_status(UserStatus::Online);
                    is_loading.set(false);
                    state.show_register_modal.set(false);
                    *state.logged_in.write() = true;
                    play_sound("online");
                    state.add_toast(
                        "Conta criada!".to_string(),
                        "Bem-vindo ao Skypia Messenger! 🦋".to_string(),
                        None,
                    );
                }
                Err(e) => {
                    is_loading.set(false);
                    error_msg.set(Some(e));
                }
            }
        });
    };

    rsx! {
        div {
            class: "fixed inset-0 bg-black/50 backdrop-blur-sm z-[9999] flex items-center justify-center p-4 pointer-events-auto",
            onclick: move |_| state.show_register_modal.set(false),

            div {
                class: "w-[360px] bg-gradient-to-b from-[#e6f1fc] to-[#c8def5] border border-[#7ba9d4] rounded-2xl shadow-2xl p-6 flex flex-col space-y-4 pointer-events-auto",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "flex items-center justify-between pb-2 border-b border-white/40",
                    div { class: "flex items-center space-x-2",
                        span { class: "text-xl", "🦋" }
                        div {
                            h2 { class: "font-bold text-sm text-[#1b324d]", "Criar nova conta" }
                            p { class: "text-[10px] text-slate-500", "Junte-se ao Skypia Messenger" }
                        }
                    }
                    button {
                        class: "w-6 h-6 flex items-center justify-center rounded-lg hover:bg-red-500 hover:text-white text-slate-500 border border-transparent font-bold cursor-pointer transition-all text-sm",
                        onclick: move |_| state.show_register_modal.set(false),
                        "✕"
                    }
                }

                // Formulário
                div { class: "flex flex-col space-y-3 max-h-[380px] overflow-y-auto pr-1",

                    // Erro
                    if let Some(err) = error_msg() {
                        div { class: "px-3 py-2 bg-red-50 border border-red-200 rounded-lg text-xs text-red-700 flex items-center space-x-2",
                            span { "⚠️" }
                            span { "{err}" }
                        }
                    }

                    // Nome de exibição
                    div { class: "space-y-1",
                        label { class: "block text-xs font-semibold text-[#2f4b6c]", "Nome de exibição" }
                        input {
                            r#type: "text",
                            class: "w-full px-2.5 py-1.5 text-xs msn-input rounded-lg",
                            placeholder: "Seu nome no chat",
                            value: "{display_name}",
                            maxlength: 40,
                            oninput: move |e| display_name.set(e.value()),
                        }
                    }

                    // Nome de usuário (username)
                    div { class: "space-y-1",
                        label { class: "block text-xs font-semibold text-[#2f4b6c]", "Nome de usuário" }
                        input {
                            r#type: "text",
                            class: "w-full px-2.5 py-1.5 text-xs msn-input rounded-lg",
                            placeholder: "Ex: wellington",
                            value: "{username}",
                            maxlength: 20,
                            oninput: move |e| username.set(e.value()),
                        }
                    }

                    // Nome completo
                    div { class: "space-y-1",
                        label { class: "block text-xs font-semibold text-[#2f4b6c]", "Nome completo" }
                        input {
                            r#type: "text",
                            class: "w-full px-2.5 py-1.5 text-xs msn-input rounded-lg",
                            placeholder: "Ex: João Silva",
                            value: "{full_name}",
                            maxlength: 80,
                            oninput: move |e| full_name.set(e.value()),
                        }
                    }

                    // Email
                    div { class: "space-y-1",
                        label { class: "block text-xs font-semibold text-[#2f4b6c]", "Email" }
                        input {
                            r#type: "email",
                            class: "w-full px-2.5 py-1.5 text-xs msn-input rounded-lg",
                            placeholder: "exemplo@hotmail.com",
                            value: "{email}",
                            oninput: move |e| email.set(e.value()),
                        }
                    }

                    // Senha
                    div { class: "space-y-1",
                        label { class: "block text-xs font-semibold text-[#2f4b6c]", "Senha" }
                        input {
                            r#type: "password",
                            class: "w-full px-2.5 py-1.5 text-xs msn-input rounded-lg",
                            placeholder: "Mínimo 6 caracteres",
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                        }
                    }

                    // Confirmar senha
                    div { class: "space-y-1",
                        label { class: "block text-xs font-semibold text-[#2f4b6c]", "Confirmar senha" }
                        input {
                            r#type: "password",
                            class: "w-full px-2.5 py-1.5 text-xs msn-input rounded-lg",
                            placeholder: "••••••••",
                            value: "{confirm_password}",
                            oninput: move |e| confirm_password.set(e.value()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter {
                                    do_register();
                                }
                            }
                        }
                    }
                }

                // Botões
                div { class: "flex space-x-2 pt-1",
                    button {
                        class: "flex-1 py-2 bg-white/60 hover:bg-white/80 border border-slate-300 text-slate-600 rounded-lg text-xs font-semibold cursor-pointer transition-all",
                        onclick: move |_| state.show_register_modal.set(false),
                        "Cancelar"
                    }
                    button {
                        class: "flex-1 py-2 bg-gradient-to-b from-[#8fc1e9] via-[#5c98d6] to-[#4585c5] hover:from-[#9bd0fa] hover:via-[#70abeb] hover:to-[#579adf] text-white border border-[#4074a8] rounded-lg font-bold text-xs shadow cursor-pointer transition-all disabled:opacity-50",
                        disabled: is_loading(),
                        onclick: move |_| do_register(),
                        if is_loading() { "Criando..." } else { "Criar conta" }
                    }
                }
            }
        }
    }
}
