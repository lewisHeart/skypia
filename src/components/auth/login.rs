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
    let mut auto_login = use_signal(|| false);

    let border_color = match selected_status() {
        UserStatus::Online => "#4aa333",
        UserStatus::Ocupado => "#b50a18",
        UserStatus::Ausente => "#c99200",
        _ => "#555555"
    };

    let (bg_color, border_color_status) = match selected_status() {
        UserStatus::Online => ("#7df25f", "#4aa333"),
        UserStatus::Ocupado => ("#e81123", "#b50a18"),
        UserStatus::Ausente => ("#ffb900", "#c99200"),
        _ => ("#7a7a7a", "#555555"),
    };

    // Inicialização consolidada (Token -> Configurações locais -> Auto-login por credenciais)
    use_effect(move || {
        let mut state = state;
        spawn(async move {
            let mut token_authed = false;
            
            // 1. Tenta auto-login com o token existente
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
                        token_authed = true;
                    }
                    Err(_) => {
                        // Token expirado — limpa
                        let _ = crate::services::db::DatabaseService::clear_auth_token().await;
                    }
                }
            }

            // 2. Se não foi autenticado por token, carrega as credenciais salvas
            if !token_authed {
                if let Ok(settings) = crate::services::db::DatabaseService::load_settings().await {
                    remember_me.set(settings.remember_password);
                    auto_login.set(settings.auto_login);
                    
                    let mut loaded_email = String::new();
                    let mut loaded_password = String::new();
                    
                    if settings.remember_password {
                        loaded_email = settings.saved_email.clone();
                        email.set(settings.saved_email);
                        
                        if !settings.saved_password.is_empty() {
                            use base64::Engine;
                            if let Ok(decoded_bytes) = base64::engine::general_purpose::STANDARD.decode(&settings.saved_password) {
                                if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
                                    loaded_password = decoded_str.clone();
                                    password.set(decoded_str);
                                }
                            }
                        }
                    }

                    // 3. Se auto-login por credenciais estiver ativo e tivermos dados
                    if settings.auto_login && !loaded_email.is_empty() && !loaded_password.is_empty() {
                        if state.signing_in() {
                            return;
                        }
                        
                        *state.signing_in.write() = true;
                        let email_val = loaded_email.trim().to_string();
                        let password_val = loaded_password;
                        let status = selected_status();
                        let mut state = state;

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
                                error_msg.set(Some(e));
                            }
                        }
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
        let rem = remember_me();
        let aut = auto_login();
        let mut state = state;

        spawn(async move {
            match api::login(email_val.clone(), password_val.clone()).await {
                Ok(auth) => {
                    state.apply_server_profile(auth.user, auth.token).await;
                    state.set_user_status(status);
                    
                    // Persiste as credenciais se o usuário desejar
                    let email_to_save = if rem { email_val } else { String::new() };
                    let pass_to_save = if rem {
                        use base64::Engine;
                        base64::engine::general_purpose::STANDARD.encode(password_val.as_bytes())
                    } else {
                        String::new()
                    };
                    
                    if let Ok(mut settings) = crate::services::db::DatabaseService::load_settings().await {
                        settings.remember_password = rem;
                        settings.auto_login = aut;
                        settings.saved_email = email_to_save;
                        settings.saved_password = pass_to_save;
                        let _ = crate::services::db::DatabaseService::save_settings(&settings).await;
                    }

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

    let mut show_status_dropdown = use_signal(|| false);

    rsx! {
        div {
            class: "w-full h-full flex flex-col items-center select-none relative p-4",
            style: "background: linear-gradient(180deg, #c2ddf4 0%, #ffffff 15%, #ffffff 89%, #eff8fa 100%);",

            // Clique fora para fechar o dropdown de status
            if show_status_dropdown() {
                div {
                    class: "fixed inset-0 z-40 bg-transparent cursor-default",
                    onclick: move |_| show_status_dropdown.set(false),
                }
            }

            // Centralizador dos elementos de Login
            div {
                class: "w-full max-w-[309px] flex flex-col items-center mt-6 md:mt-12 px-4 md:px-0 flex-1 justify-center",
 
                // Logo/Avatar com a Moldura de Status SVG dinâmica
                div { class: "h-[132px] w-[132px] flex items-center justify-center relative mb-8 flex-shrink-0",
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
                        div {
                            class: "msn-avatar-container w-[132px] h-[132px] relative flex items-center justify-center",
                            img {
                                src: match selected_status() {
                                    UserStatus::Online => asset!("/assets/status/disponivel_login.svg"),
                                    UserStatus::Ocupado => asset!("/assets/status/ocupado_login.svg"),
                                    UserStatus::Ausente => asset!("/assets/status/ausente_login.svg"),
                                    _ => asset!("/assets/status/offline_login.svg"),
                                },
                                class: "msn-avatar-frame-img"
                            }
                            div {
                                class: "msn-avatar-content w-[112px] h-[112px] rounded-[10px] bg-white flex items-center justify-center",
                                style: "border: 2px solid {border_color}",
                                svg { view_box: "0 0 100 100", class: "w-20 h-20",
                                    g { fill: "#cbdde8",
                                        circle { cx: "50", cy: "38", r: "18" }
                                        path { d: "M20 82 C20 58, 80 58, 80 82 Z" }
                                    }
                                }
                            }
                        }
                    }
                }
 
                if state.signing_in() {
                    div { class: "w-full text-center space-y-4 mt-6",
                        p { class: "text-xs text-[#1e395b] font-semibold animate-pulse", "Entrando no Skypia..." }
                        div { class: "w-48 h-2 bg-white/80 border border-[#a6b9cd] rounded-full mx-auto overflow-hidden shadow-inner",
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
                    div { class: "w-full flex flex-col space-y-3.5",
 
                        // Mensagem de erro
                        if let Some(err) = error_msg() {
                            div { class: "w-full px-3 py-2 bg-red-50 border border-red-200 rounded text-[11px] text-red-700 flex items-center space-x-2 shadow-sm",
                                span { "⚠️" }
                                span { "{err}" }
                            }
                        }
 
                        // Email
                        div { class: "w-full relative",
                            input {
                                r#type: "email",
                                class: "w-full h-[27px] px-2.5 text-xs text-slate-800 bg-white border border-[#d1d1d1] rounded-[4px] focus:border-slate-400 msn-input placeholder-[#a5a5a5] placeholder:text-[10px]",
                                placeholder: "exemplo@mail.com",
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter { do_login(); }
                                }
                            }
                        }
 
                        // Senha e Esqueci a senha
                        div { class: "w-full flex flex-col space-y-1.5",
                            input {
                                r#type: "password",
                                class: "w-full h-[27px] px-2.5 text-xs text-slate-800 bg-white border border-[#d1d1d1] rounded-[4px] focus:border-slate-400 msn-input placeholder-[#a5a5a5] placeholder:text-[10px]",
                                placeholder: "Senha",
                                value: "{password}",
                                oninput: move |e| password.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter { do_login(); }
                                }
                            }
                            span {
                                class: "text-[10px] text-[#2e83ed] hover:underline cursor-pointer self-start",
                                onclick: move |_| {
                                    state.add_toast(
                                        "Esqueci a senha".to_string(),
                                        "Entre em contato com o administrador do servidor Skypia para recuperar sua senha.".to_string(),
                                        None
                                    );
                                },
                                "Esqueci a senha?"
                            }
                        }
 
                        // Status Selection (Logar como:)
                        div { class: "w-full flex items-center space-x-2 relative text-xs",
                            span { class: "text-[#0d1825] text-[10px] font-normal", "Logar como: " }
                            button {
                                class: "flex items-center space-x-1.5 px-1.5 py-0.5 hover:bg-black/5 rounded cursor-pointer transition-colors focus:outline-none",
                                onclick: move |_| show_status_dropdown.set(!show_status_dropdown()),
                                div {
                                    class: "w-2 h-2 rounded-[2px] border flex-shrink-0",
                                    style: "background-color: {bg_color}; border-color: {border_color_status};",
                                }
                                span { class: "text-[#a5a5a5] text-[10px] font-normal",
                                    match selected_status() {
                                        UserStatus::Online => "(Online)",
                                        UserStatus::Ocupado => "(Ocupado)",
                                        UserStatus::Ausente => "(Ausente)",
                                        _ => "(Invisível)"
                                    }
                                }
                                span { class: "text-[#a5a5a5] text-[8px] ml-0.5", "▼" }
                            }
 
                            // Menu Dropdown de Status do Login
                            if show_status_dropdown() {
                                div {
                                    class: "absolute left-[70px] top-full mt-1 w-32 bg-white border border-[#d1d1d1] rounded shadow-lg z-50 p-1 flex flex-col text-[10px] text-slate-700 font-normal",
                                    button {
                                        class: "px-2 py-1 hover:bg-slate-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none",
                                        onclick: move |_| {
                                            selected_status.set(UserStatus::Online);
                                            show_status_dropdown.set(false);
                                        },
                                        div { class: "w-2 h-2 rounded-[2px] bg-[#7df25f] border border-[#4aa333]" }
                                        span { "Online" }
                                    }
                                    button {
                                        class: "px-2 py-1 hover:bg-slate-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none",
                                        onclick: move |_| {
                                            selected_status.set(UserStatus::Ocupado);
                                            show_status_dropdown.set(false);
                                        },
                                        div { class: "w-2 h-2 rounded-[2px] bg-[#e81123] border border-[#b50a18]" }
                                        span { "Ocupado" }
                                    }
                                    button {
                                        class: "px-2 py-1 hover:bg-slate-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none",
                                        onclick: move |_| {
                                            selected_status.set(UserStatus::Ausente);
                                            show_status_dropdown.set(false);
                                        },
                                        div { class: "w-2 h-2 rounded-[2px] bg-[#ffb900] border border-[#c99200]" }
                                        span { "Ausente" }
                                    }
                                    button {
                                        class: "px-2 py-1 hover:bg-slate-100 rounded text-left flex items-center space-x-2 cursor-pointer focus:outline-none",
                                        onclick: move |_| {
                                            selected_status.set(UserStatus::Invisivel);
                                            show_status_dropdown.set(false);
                                        },
                                        div { class: "w-2 h-2 rounded-[2px] bg-gray-400 border border-gray-500" }
                                        span { "Invisível" }
                                    }
                                }
                            }
                        }
 
                        // Lembrar e Entrar Automaticamente
                        div { class: "w-full flex flex-col space-y-2 text-[10px] text-[#0d1825] font-normal pt-1.5",
                            label { class: "flex items-center space-x-2 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
                                    checked: remember_me(),
                                    onchange: move |e| {
                                        let val = e.value() == "true";
                                        remember_me.set(val);
                                        if !val {
                                            auto_login.set(false);
                                        }
                                    },
                                }
                                span { "Lembrar meu email e senha" }
                            }
                            div { class: "flex items-center space-x-4",
                                label { class: "flex items-center space-x-2 cursor-pointer",
                                    input {
                                        r#type: "checkbox",
                                        class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
                                        checked: auto_login(),
                                        onchange: move |e| {
                                            let val = e.value() == "true";
                                            auto_login.set(val);
                                            if val {
                                                remember_me.set(true);
                                            }
                                        },
                                    }
                                    span { "Logar automaticamente" }
                                }
                                span {
                                    class: "text-[#2e83ed] hover:underline cursor-pointer",
                                    onclick: move |_| {
                                        state.show_settings_modal.set(true);
                                    },
                                    "Opções"
                                }
                            }
                        }
 
                        // Botão Entrar
                        div { class: "pt-3 w-full",
                            button {
                                class: "w-full h-[36px] bg-[#cde3f6] hover:bg-[#b8d6f0] text-[#012d93] border border-transparent rounded-[4px] font-bold text-[10px] shadow-sm cursor-pointer transition-colors flex items-center justify-center focus:outline-none",
                                onclick: move |_| do_login(),
                                "Entrar"
                            }
                        }
                    }
                }

                // Footer com link de Criar conta
                div { class: "w-full flex items-center justify-between mt-6 text-[10px] text-slate-500",
                    button {
                        class: "hover:underline text-[#2e83ed] font-semibold cursor-pointer focus:outline-none",
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
