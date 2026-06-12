use crate::models::{AppTheme, UserStatus};
use crate::state::AppState;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SettingsContentProps {
    pub state: AppState,
    pub is_native_window: bool,
}

#[component]
pub fn SettingsContent(props: SettingsContentProps) -> Element {
    let mut state = props.state;
    let theme = state.theme();

    let mut active_tab = use_signal(|| "pessoais".to_string());
    let mut temp_name = use_signal(|| state.user_name());
    let mut temp_msg = use_signal(|| state.user_personal_message());
    let mut temp_folder = use_signal(|| state.download_folder());

    let mut admin_banner_icon = use_signal(|| "📢".to_string());
    let mut admin_banner_text = use_signal(|| String::new());
    let mut admin_banner_label = use_signal(|| String::new());
    let mut admin_banner_link = use_signal(|| String::new());
    let mut admin_banner_image = use_signal(|| String::new());
    let mut admin_banner_type = use_signal(|| "classic".to_string());
    let mut new_cat_input = use_signal(|| String::new());

    let _is_uploading_ad = use_signal(|| false);
    let _ad_upload_error = use_signal(|| Option::<String>::None);

    let mut admin_banner_title = use_signal(|| String::new());
    let mut admin_banner_active = use_signal(|| true);
    let mut selected_banner_id = use_signal(|| Option::<i32>::None);
    let mut reload_banners_trigger = use_signal(|| 0);

    // Sincroniza os valores do perfil quando o componente é montado
    use_effect(move || {
        temp_name.set(state.user_name());
        temp_msg.set(state.user_personal_message());
        temp_folder.set(state.download_folder());
    });

    let mut close_action = move || {
        if props.is_native_window {
            #[cfg(feature = "desktop")]
            {
                dioxus::desktop::use_window().close();
            }
        } else {
            state.show_settings_modal.set(false);
        }
    };

    rsx! {
        div { class: "flex-1 flex flex-col sm:flex-row overflow-hidden text-xs {theme.titlebar_text()} h-full w-full bg-[#eff5fb]",
            // Coluna de Abas (Horizontal com scroll no mobile, Vertical no desktop)
            div { class: "w-full sm:w-[160px] border-b sm:border-b-0 sm:border-r {theme.titlebar_border()} bg-white/40 flex flex-row sm:flex-col p-1.5 sm:p-2 space-x-1 sm:space-x-0 sm:space-y-1 overflow-x-auto sm:overflow-x-visible sm:overflow-y-auto select-none flex-shrink-0 scrollbar-none",
                {
                    let is_admin = state.is_admin();
                    let mut tab_list = vec![
                        ("pessoais", "Pessoal", "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Person/SVG/ic_fluent_person_20_color.svg"),
                        ("gerais", "Geral", "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Settings/SVG/ic_fluent_settings_20_color.svg"),
                        ("mensagens", "Mensagens", "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Mail/SVG/ic_fluent_mail_20_color.svg"),
                        ("sons", "Sons/Alertas", "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Alert/SVG/ic_fluent_alert_20_color.svg"),
                        ("transferencias", "Arquivos", "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Document%20Folder/SVG/ic_fluent_document_folder_20_color.svg"),
                        ("privacidade", "Privacidade", "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Shield/SVG/ic_fluent_shield_20_color.svg"),
                        ("seguranca", "Segurança", "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Lock%20Closed/SVG/ic_fluent_lock_closed_20_color.svg"),
                        ("conexao", "Conexão", "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Router/SVG/ic_fluent_router_20_color.svg"),
                    ];
                    if is_admin {
                        tab_list.push(("admin_banners", "Anúncios", "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Alert%20Urgent/SVG/ic_fluent_alert_urgent_20_color.svg"));
                    }
                    tab_list.into_iter().map(move |(tab_id, tab_label, icon_url)| {
                        let is_active = active_tab() == tab_id;
                        let tab_class = if is_active { "bg-white shadow-sm border border-slate-200/85 font-extrabold" } else { "hover:bg-white/50 border border-transparent" };
                        rsx! {
                            button {
                                class: "px-2.5 py-1.5 rounded flex items-center space-x-1.5 text-left cursor-pointer transition-colors focus:outline-none text-[10px] sm:text-[11px] font-semibold flex-shrink-0 sm:w-full {tab_class}",
                                onclick: move |_| active_tab.set(tab_id.to_string()),
                                img {
                                    src: "{icon_url}",
                                    class: "w-4 h-4 object-contain pointer-events-none"
                                }
                                span { "{tab_label}" }
                            }
                        }
                    })
                }
            }

            // Coluna de Conteúdo (Scrollable)
            div { class: "flex-1 p-4 overflow-y-auto flex flex-col justify-between bg-white/15 h-full",
                div { class: "flex-1 flex flex-col space-y-4 min-h-0 overflow-y-auto pb-4",

                    if active_tab() == "pessoais" {
                        div { class: "flex flex-col space-y-3",
                            div { class: "flex flex-col space-y-1",
                                label { class: "font-semibold text-slate-700", "Apelido (Nome de exibição)" }
                                input {
                                    class: "px-2 py-1.5 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none focus:border-slate-400 bg-white",
                                    value: "{temp_name}",
                                    oninput: move |e| temp_name.set(e.value()),
                                    onblur: move |_| state.set_user_name(temp_name())
                                }
                            }

                            div { class: "flex flex-col space-y-1",
                                label { class: "font-semibold text-slate-700", "Mensagem pessoal (Substatus)" }
                                input {
                                    class: "px-2 py-1.5 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none focus:border-slate-400 bg-white",
                                    value: "{temp_msg}",
                                    oninput: move |e| temp_msg.set(e.value()),
                                    onblur: move |_| state.set_user_personal_message(temp_msg())
                                }
                            }

                            div { class: "flex flex-col space-y-2 pt-2 border-t border-slate-200/50",
                                label { class: "font-semibold text-slate-700", "Imagem de perfil (Avatar)" }
                                div { class: "flex items-center space-x-3",
                                    button {
                                        class: "px-3 py-1.5 bg-white/60 hover:bg-white border border-slate-300 rounded font-semibold text-[10px] cursor-pointer focus:outline-none transition-colors",
                                        onclick: move |_| {
                                            if !props.is_native_window {
                                                state.show_settings_modal.set(false);
                                            }
                                            state.show_avatar_picker.set(true);
                                        },
                                        "Alterar imagem de perfil..."
                                    }
                                }
                            }
                        }
                    }

                    if active_tab() == "gerais" {
                        div { class: "flex flex-col space-y-3",
                            div { class: "flex flex-col space-y-1",
                                label { class: "font-semibold text-slate-700", "Escala da Interface" }
                                select {
                                    class: "w-full h-[27px] px-2 border border-[#d1d1d1] rounded bg-white text-slate-800 focus:outline-none text-xs",
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

                            div { class: "flex flex-col space-y-1",
                                label { class: "font-semibold text-slate-700", "Aparência (Skins)" }
                                select {
                                    class: "w-full h-[27px] px-2 border border-[#d1d1d1] rounded bg-white text-slate-800 focus:outline-none text-xs",
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

                            div { class: "flex flex-col space-y-1 pt-1.5 border-t border-slate-200/50",
                                label { class: "font-semibold text-slate-700", "Sincronização de Música" }
                                label { class: "flex items-center space-x-2 cursor-pointer mt-1",
                                    input {
                                        r#type: "checkbox",
                                        class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
                                        checked: state.spotify_rpc_enabled(),
                                        onchange: move |e| {
                                            state.set_spotify_rpc_enabled(e.value() == "true");
                                        }
                                    }
                                    span { "Detectar música do Spotify automaticamente" }
                                }
                            }

                            if cfg!(not(target_os = "android")) {
                                div { class: "flex flex-col space-y-3 pt-2 border-t border-slate-200/50",
                                    div { class: "flex flex-col space-y-1.5",
                                        label { class: "font-semibold text-slate-700", "Estilo de Decorações da Janela" }
                                        label { class: "flex items-center space-x-2 cursor-pointer",
                                            input {
                                                r#type: "checkbox",
                                                class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
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
                                        label { class: "font-semibold text-slate-700", "Modo de Chat" }
                                        select {
                                            class: "w-full h-[27px] px-2 border border-[#d1d1d1] rounded bg-white text-slate-800 focus:outline-none text-xs",
                                            onchange: move |e| {
                                                state.set_chat_mode(e.value());
                                            },
                                            option { value: "integrated", selected: state.chat_mode() == "integrated", "Chat Conectado" }
                                            option { value: "detached", selected: state.chat_mode() == "detached", "Janela Separada" }
                                        }
                                    }
                                }
                            }

                            // Gerenciamento de Categorias de Contatos
                            div { class: "flex flex-col space-y-2 pt-2 border-t border-slate-200/50",
                                label { class: "font-semibold text-slate-700", "Categorias de Contatos" }
                                div { class: "flex flex-col space-y-1 max-h-[100px] overflow-y-auto border border-[#d1d1d1] rounded p-1 bg-white/50",
                                    {
                                        let cats = state.categories();
                                        if cats.is_empty() {
                                            rsx! {
                                                span { class: "text-[10px] text-slate-400 italic p-1", "Nenhuma categoria personalizada." }
                                            }
                                        } else {
                                            rsx! {
                                                for cat in cats.iter() {
                                                    div { class: "flex items-center justify-between py-0.5 px-1.5 hover:bg-slate-100/60 rounded",
                                                        span { class: "text-[11px] text-slate-700 font-medium", "{cat}" }
                                                        button {
                                                            class: "text-rose-500 hover:text-rose-700 font-semibold text-[10px] cursor-pointer focus:outline-none",
                                                            onclick: {
                                                                let cat_clone = cat.clone();
                                                                move |_| {
                                                                    let _token = state.auth_token();
                                                                    state.delete_category(cat_clone.clone());
                                                                }
                                                            },
                                                            "Remover"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                div { class: "flex items-center space-x-1.5 mt-1",
                                    input {
                                        class: "px-2 py-1 border border-[#d1d1d1] msn-input rounded text-xs flex-1 focus:outline-none bg-white",
                                        placeholder: "Nova categoria...",
                                        value: "{new_cat_input}",
                                        oninput: move |e| new_cat_input.set(e.value()),
                                        onkeydown: move |e| {
                                            if e.key() == Key::Enter {
                                                let name = new_cat_input().trim().to_string();
                                                if !name.is_empty() && !state.categories().contains(&name) {
                                                    state.add_category(name);
                                                    new_cat_input.set(String::new());
                                                }
                                            }
                                        }
                                    }
                                    button {
                                        class: "px-3 py-1 bg-white/60 hover:bg-white border border-[#d1d1d1] rounded text-[10px] font-semibold cursor-pointer focus:outline-none transition-colors",
                                        onclick: move |_| {
                                            let name = new_cat_input().trim().to_string();
                                            if !name.is_empty() && !state.categories().contains(&name) {
                                                state.add_category(name);
                                                new_cat_input.set(String::new());
                                            }
                                        },
                                        "Adicionar"
                                    }
                                }
                            }
                        }
                    }

                    if active_tab() == "mensagens" {
                        div { class: "flex flex-col space-y-3",
                            div { class: "flex flex-col space-y-1",
                                label { class: "font-semibold text-slate-700", "Fonte padrão das mensagens" }
                                select {
                                    class: "w-full h-[27px] px-2 border border-[#d1d1d1] rounded bg-white text-slate-800 focus:outline-none text-xs",
                                    onchange: move |e| state.set_chat_font_family(e.value()),
                                    option { value: "Segoe UI", selected: state.chat_font_family() == "Segoe UI", "Segoe UI" }
                                    option { value: "Comic Sans MS", selected: state.chat_font_family() == "Comic Sans MS", "Comic Sans" }
                                    option { value: "Arial", selected: state.chat_font_family() == "Arial", "Arial" }
                                    option { value: "Courier New", selected: state.chat_font_family() == "Courier New", "Courier New" }
                                }
                            }

                            div { class: "flex flex-col space-y-1",
                                label { class: "font-semibold text-slate-700", "Cor padrão do texto" }
                                div { class: "grid grid-cols-8 gap-1.5 w-full pt-1",
                                    {
                                        ["#000000", "#0066cc", "#e6007e", "#2e6930", "#e81123", "#ffb900", "#7a7a7a", "#8e24aa"].iter().map(|&color| {
                                            let is_selected = state.chat_font_color() == color;
                                            let border_color = if is_selected { "#3b82f6" } else { "#d1d1d1" };
                                            rsx! {
                                                div {
                                                    class: "w-6 h-6 rounded cursor-pointer border hover:scale-110 hover:shadow transition-all flex items-center justify-center relative",
                                                    style: "background-color: {color}; border-color: {border_color};",
                                                    onclick: move |_| state.set_chat_font_color(color.to_string()),
                                                    if is_selected {
                                                        span { class: "text-white text-[9px] font-bold drop-shadow", "✓" }
                                                    }
                                                }
                                            }
                                        })
                                    }
                                }
                            }

                            div { class: "flex flex-col pt-2 border-t border-slate-200/50",
                                label { class: "flex items-center space-x-2 cursor-pointer",
                                    input {
                                        r#type: "checkbox",
                                        class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
                                        checked: state.show_typing_notification(),
                                        onchange: move |e| state.set_show_typing_notification(e.value() == "true")
                                    }
                                    span { "Exibir notificação 'digitando...' para meus contatos" }
                                }
                            }
                        }
                    }

                    if active_tab() == "sons" {
                        div { class: "flex flex-col space-y-3",
                            label { class: "font-semibold text-slate-700", "Sons e Alertas do Sistema" }
                            label { class: "flex items-center space-x-2 cursor-pointer mt-1",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
                                    checked: state.enable_sounds(),
                                    onchange: move |e| state.set_enable_sounds(e.value() == "true")
                                }
                                span { "Habilitar efeitos sonoros (nudge, chamadas, mensagens)" }
                            }

                            label { class: "flex items-center space-x-2 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
                                    checked: state.enable_toasts(),
                                    onchange: move |e| state.set_enable_toasts(e.value() == "true")
                                }
                                span { "Exibir popups de notificações (toasts) no canto inferior" }
                            }
                        }
                    }

                    if active_tab() == "transferencias" {
                        div { class: "flex flex-col space-y-3",
                            div { class: "flex flex-col space-y-1",
                                label { class: "font-semibold text-slate-700", "Pasta de Downloads padrão" }
                                input {
                                    class: "px-2 py-1.5 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none focus:border-slate-400 bg-white",
                                    value: "{temp_folder}",
                                    placeholder: "Ex: /home/usuario/Downloads/Skypia",
                                    oninput: move |e| temp_folder.set(e.value()),
                                    onblur: move |_| state.set_download_folder(temp_folder())
                                }
                            }

                            label { class: "flex items-center space-x-2 cursor-pointer pt-2 border-t border-slate-200/50",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
                                    checked: state.auto_accept_files(),
                                    onchange: move |e| state.set_auto_accept_files(e.value() == "true")
                                }
                                span { "Aceitar automaticamente arquivos recebidos de contatos comuns" }
                            }
                        }
                    }

                    if active_tab() == "privacidade" {
                        div { class: "flex flex-col space-y-3",
                            label { class: "font-semibold text-slate-700", "Privacidade da Conta" }
                            label { class: "flex items-center space-x-2 cursor-pointer mt-1",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
                                    checked: state.user_status() == UserStatus::Invisivel,
                                    onchange: move |e| {
                                        let status = if e.value() == "true" { UserStatus::Invisivel } else { UserStatus::Online };
                                        state.set_user_status(status);
                                    }
                                }
                                span { "Ficar invisível por padrão na rede" }
                            }

                            label { class: "flex items-center space-x-2 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
                                    checked: state.user_status() == UserStatus::Ausente,
                                    onchange: move |e| {
                                        let status = if e.value() == "true" { UserStatus::Ausente } else { UserStatus::Online };
                                        state.set_user_status(status);
                                    }
                                }
                                span { "Mostrar-me como Ausente temporariamente" }
                            }
                        }
                    }

                    if active_tab() == "seguranca" {
                        div { class: "flex flex-col space-y-3",
                            label { class: "font-semibold text-slate-700", "Segurança e Histórico" }
                            label { class: "flex items-center space-x-2 cursor-pointer mt-1",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
                                    checked: state.remember_password(),
                                    onchange: move |e| state.set_remember_password(e.value() == "true")
                                }
                                span { "Lembrar minhas credenciais (Email e Senha)" }
                            }

                            label { class: "flex items-center space-x-2 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
                                    checked: state.save_chat_history(),
                                    onchange: move |e| state.set_save_chat_history(e.value() == "true")
                                }
                                span { "Salvar mensagens localmente no histórico (SQLite)" }
                            }
                        }
                    }

                    if active_tab() == "conexao" {
                        div { class: "flex flex-col space-y-3",
                            div { class: "flex flex-col space-y-1",
                                label { class: "font-semibold text-slate-700", "Endereço do Servidor (WebSocket)" }
                                input {
                                    class: "px-2 py-1.5 border border-[#d1d1d1] bg-slate-50 text-slate-500 rounded text-xs w-full cursor-not-allowed focus:outline-none",
                                    value: "ws://192.168.1.16:8082/ws",
                                    disabled: true
                                }
                            }

                            div { class: "flex items-center space-x-2 pt-2 border-t border-slate-200/50",
                                span { class: "font-semibold text-slate-700", "Status de Rede:" }
                                if state.ws_tx.read().is_some() {
                                    span { class: "px-2.5 py-0.5 bg-emerald-100 text-emerald-700 rounded-full font-bold text-[9px]", "Conectado" }
                                } else {
                                    span { class: "px-2.5 py-0.5 bg-rose-100 text-rose-700 rounded-full font-bold text-[9px]", "Desconectado" }
                                }
                            }
                        }
                    }

                                  if active_tab() == "admin_banners" {
                        {
                            let is_admin = state.is_admin();
                            if !is_admin {
                                rsx! {
                                    div { class: "p-4 text-red-500 font-bold flex items-center space-x-2 bg-red-50 border border-red-200 rounded",
                                        span { "⚠️" }
                                        span { "Acesso Negado: Apenas administradores podem gerenciar anúncios." }
                                    }
                                }
                            } else {
                                let banners_res = use_resource(move || {
                                    let token = state.auth_token();
                                    async move {
                                        if let Some(t) = token {
                                            crate::services::api::list_banners_admin(&t).await.ok()
                                        } else {
                                            None
                                        }
                                    }
                                });

                                rsx! {
                                    div { class: "flex flex-col space-y-4 h-full",
                                        div { class: "flex flex-col sm:flex-row space-y-4 sm:space-y-0 sm:space-x-4 min-h-0 flex-1",
                                            // Lista de Anúncios (Esquerda)
                                            div { class: "flex-1 flex flex-col space-y-2 border border-slate-200 bg-white/50 p-2.5 rounded-lg overflow-y-auto max-h-[220px] sm:max-h-none",
                                                div { class: "flex items-center justify-between border-b border-slate-200/60 pb-1.5 mb-1.5",
                                                    span { class: "font-bold text-[11px] text-slate-600", "Anúncios Cadastrados" }
                                                    button {
                                                        class: "px-2 py-0.5 bg-sky-500 hover:bg-sky-600 text-white font-bold text-[9px] rounded shadow cursor-pointer focus:outline-none transition-colors",
                                                        onclick: move |_| {
                                                            selected_banner_id.set(None);
                                                            admin_banner_title.set(String::new());
                                                            admin_banner_icon.set("📢".to_string());
                                                            admin_banner_text.set(String::new());
                                                            admin_banner_label.set(String::new());
                                                            admin_banner_link.set(String::new());
                                                            admin_banner_image.set(String::new());
                                                            admin_banner_active.set(true);
                                                        },
                                                        "+ Novo"
                                                    }
                                                }
                                                match *banners_res.value().read() {
                                                    Some(Some(ref list)) if !list.is_empty() => {
                                                        rsx! {
                                                            div { class: "flex flex-col space-y-1.5",
                                                                for banner in list {
                                                                    {
                                                                        let b_id = banner.id;
                                                                        let b_title = banner.title.clone();
                                                                        let b_icon = banner.icon.clone();
                                                                        let b_text = banner.text.clone();
                                                                        let b_label = banner.action_label.clone();
                                                                        let b_link = banner.link.clone();
                                                                        let b_img = banner.image_url.clone().unwrap_or_default();
                                                                        let b_active = banner.is_active;

                                                                        rsx! {
                                                                            div { class: "p-2 bg-white rounded border border-slate-200 shadow-sm flex items-center justify-between text-[11px]",
                                                                                div { class: "flex flex-col min-w-0 flex-1 pr-2",
                                                                                    span { class: "font-bold text-slate-800 truncate", "{b_title}" }
                                                                                    span { class: "text-[10px] text-slate-500 truncate", "{b_text} ({b_link})" }
                                                                                }
                                                                                div { class: "flex items-center space-x-1 flex-shrink-0",
                                                                                    // Toggle Ativo/Inativo
                                                                                    button {
                                                                                        class: if b_active { "px-1.5 py-0.5 bg-emerald-100 hover:bg-emerald-200 text-emerald-700 font-bold text-[9px] rounded border border-emerald-200 cursor-pointer" } else { "px-1.5 py-0.5 bg-slate-100 hover:bg-slate-200 text-slate-500 font-bold text-[9px] rounded border border-slate-200 cursor-pointer" },
                                                                                        onclick: move |_| {
                                                                                            let token = state.auth_token();
                                                                                            spawn(async move {
                                                                                                if let Some(t) = token {
                                                                                                    let _ = crate::services::api::toggle_banner_admin_req(&t, b_id).await;
                                                                                                    reload_banners_trigger.set(reload_banners_trigger() + 1);
                                                                                                }
                                                                                            });
                                                                                        },
                                                                                        if b_active { "Ativo" } else { "Inativo" }
                                                                                    }
                                                                                    // Editar
                                                                                    button {
                                                                                        class: "px-1.5 py-0.5 bg-sky-50 hover:bg-sky-100 text-sky-700 font-bold text-[9px] rounded border border-sky-200 cursor-pointer",
                                                                                        onclick: move |_| {
                                                                                            selected_banner_id.set(Some(b_id));
                                                                                            admin_banner_title.set(b_title.clone());
                                                                                            admin_banner_icon.set(b_icon.clone());
                                                                                            admin_banner_text.set(b_text.clone());
                                                                                            admin_banner_label.set(b_label.clone());
                                                                                            admin_banner_link.set(b_link.clone());
                                                                                            admin_banner_image.set(b_img.clone());
                                                                                            admin_banner_active.set(b_active);
                                                                                            if b_icon == "BANNER" {
                                                                                                admin_banner_type.set("full".to_string());
                                                                                            } else {
                                                                                                admin_banner_type.set("classic".to_string());
                                                                                            }
                                                                                        },
                                                                                        "Editar"
                                                                                    }
                                                                                    // Excluir
                                                                                    button {
                                                                                        class: "px-1.5 py-0.5 bg-rose-50 hover:bg-rose-100 text-rose-600 font-bold text-[9px] rounded border border-rose-200 cursor-pointer",
                                                                                        onclick: move |_| {
                                                                                            let token = state.auth_token();
                                                                                            spawn(async move {
                                                                                                if let Some(t) = token {
                                                                                                    let _ = crate::services::api::delete_banner_admin_req(&t, b_id).await;
                                                                                                    reload_banners_trigger.set(reload_banners_trigger() + 1);
                                                                                                }
                                                                                            });
                                                                                        },
                                                                                        "✕"
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Some(Some(_)) => {
                                                        rsx! { span { class: "text-slate-400 text-[10px] italic p-2 block text-center", "Nenhum anúncio cadastrado." } }
                                                    }
                                                    _ => {
                                                        rsx! { span { class: "text-slate-400 text-[10px] italic p-2 block text-center animate-pulse", "Carregando anúncios..." } }
                                                    }
                                                }
                                            }

                                            // Formulário de Edição (Direita)
                                            div { class: "flex-1 flex flex-col space-y-2.5 border border-slate-200 bg-white/50 p-3 rounded-lg overflow-y-auto",
                                                span { class: "font-bold text-[11px] text-slate-600 border-b border-slate-200 pb-1.5",
                                                    if selected_banner_id().is_some() { "Editar Anúncio" } else { "Cadastrar Anúncio" }
                                                }

                                                // Título do Anúncio (Uso administrativo)
                                                div { class: "flex flex-col space-y-1",
                                                    label { class: "font-semibold text-slate-700", "Título do Anúncio (Uso Interno)" }
                                                    input {
                                                        class: "px-2 py-1 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none bg-white",
                                                        value: "{admin_banner_title}",
                                                        placeholder: "Ex: Banner Promoção de Inverno",
                                                        oninput: move |e| admin_banner_title.set(e.value()),
                                                    }
                                                }

                                                // Tipo de Anúncio
                                                div { class: "flex flex-col space-y-1",
                                                    label { class: "font-semibold text-slate-700", "Tipo de Anúncio" }
                                                    div { class: "flex space-x-4 py-0.5",
                                                        label { class: "flex items-center space-x-1.5 cursor-pointer text-xs font-semibold text-slate-700",
                                                            input {
                                                                r#type: "radio",
                                                                name: "banner_type",
                                                                checked: admin_banner_type() == "classic",
                                                                onchange: move |_| admin_banner_type.set("classic".to_string()),
                                                            }
                                                            span { "Texto e Ícones" }
                                                        }
                                                        label { class: "flex items-center space-x-1.5 cursor-pointer text-xs font-semibold text-slate-700",
                                                            input {
                                                                r#type: "radio",
                                                                name: "banner_type",
                                                                checked: admin_banner_type() == "full",
                                                                onchange: move |_| admin_banner_type.set("full".to_string()),
                                                            }
                                                            span { "Imagem Completa (50px)" }
                                                        }
                                                    }
                                                }

                                                if admin_banner_type() == "classic" {
                                                    div { class: "flex flex-col space-y-2.5",
                                                        div { class: "flex flex-col space-y-1",
                                                            label { class: "font-semibold text-slate-700", "Ícone / Emoji" }
                                                            input {
                                                                class: "px-2 py-1 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none bg-white",
                                                                value: "{admin_banner_icon}",
                                                                placeholder: "📢",
                                                                oninput: move |e| admin_banner_icon.set(e.value()),
                                                            }
                                                        }
                                                        div { class: "flex flex-col space-y-1",
                                                            label { class: "font-semibold text-slate-700", "Texto do Anúncio" }
                                                            input {
                                                                class: "px-2 py-1 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none bg-white",
                                                                value: "{admin_banner_text}",
                                                                placeholder: "Promoção imperdível do MSN!",
                                                                oninput: move |e| admin_banner_text.set(e.value()),
                                                            }
                                                        }
                                                        div { class: "flex flex-col space-y-1",
                                                            label { class: "font-semibold text-slate-700", "Rótulo do Botão (Ação)" }
                                                            input {
                                                                class: "px-2 py-1 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none bg-white",
                                                                value: "{admin_banner_label}",
                                                                placeholder: "Clique Aqui",
                                                                oninput: move |e| admin_banner_label.set(e.value()),
                                                            }
                                                        }
                                                    }
                                                }

                                                div { class: "flex flex-col space-y-1",
                                                    label { class: "font-semibold text-slate-700", "Link do Anúncio" }
                                                    input {
                                                        class: "px-2 py-1 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none bg-white",
                                                        value: "{admin_banner_link}",
                                                        placeholder: "http://site-promocional.com",
                                                        oninput: move |e| admin_banner_link.set(e.value()),
                                                    }
                                                }

                                                div { class: "flex flex-col space-y-1",
                                                    label { class: "font-semibold text-slate-700", "URL da Imagem (Opcional no clássico)" }
                                                    input {
                                                        class: "px-2 py-1 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none bg-white",
                                                        value: "{admin_banner_image}",
                                                        placeholder: "http://site.com/foto.jpg",
                                                        oninput: move |e| admin_banner_image.set(e.value()),
                                                    }
                                                }

                                                label { class: "flex items-center space-x-2 cursor-pointer py-1.5 text-slate-750 font-semibold",
                                                    input {
                                                        r#type: "checkbox",
                                                        class: "rounded-none border-[#a0a0a0] bg-[#e5e0ea] text-sky-600 focus:ring-0 focus:outline-none w-3.5 h-3.5",
                                                        checked: admin_banner_active(),
                                                        onchange: move |e| admin_banner_active.set(e.value() == "true")
                                                    }
                                                    span { "Deixar este anúncio ativo imediatamente" }
                                                }

                                                button {
                                                    class: "px-4 py-1.5 {theme.btn_primary()} rounded font-bold shadow-md cursor-pointer transition-all focus:outline-none self-end text-[10px] disabled:opacity-50 disabled:cursor-not-allowed mt-2",
                                                    disabled: if admin_banner_type() == "full" {
                                                        admin_banner_title().trim().is_empty() || admin_banner_link().trim().is_empty()
                                                    } else {
                                                        admin_banner_title().trim().is_empty() || admin_banner_text().trim().is_empty() || admin_banner_link().trim().is_empty()
                                                    },
                                                    onclick: move |_| {
                                                        let token = match state.auth_token() {
                                                            Some(t) => t,
                                                            None => return,
                                                        };
                                                        let title = admin_banner_title().trim().to_string();
                                                        let text = if admin_banner_type() == "full" { "".to_string() } else { admin_banner_text().trim().to_string() };
                                                        let action_label = if admin_banner_type() == "full" { "".to_string() } else { admin_banner_label().trim().to_string() };
                                                        let icon = if admin_banner_type() == "full" { "BANNER".to_string() } else { admin_banner_icon().clone() };
                                                        let link = admin_banner_link().trim().to_string();
                                                        let img_opt = if admin_banner_image().trim().is_empty() { None } else { Some(admin_banner_image().trim().to_string()) };
                                                        let is_active = admin_banner_active();

                                                        let mut state_clone = state;
                                                        spawn(async move {
                                                            if let Some(id) = selected_banner_id() {
                                                                if crate::services::api::update_banner_admin_req(&token, id, &title, &text, &action_label, &link, &icon, img_opt, is_active).await.is_ok() {
                                                                    state_clone.add_toast("Anúncio Salvo".to_string(), "O banner foi editado e atualizado.".to_string(), None);
                                                                }
                                                            } else {
                                                                if crate::services::api::create_banner_admin(&token, &title, &text, &action_label, &link, &icon, img_opt, is_active).await.is_ok() {
                                                                    state_clone.add_toast("Anúncio Criado".to_string(), "Novo banner cadastrado com sucesso.".to_string(), None);
                                                                }
                                                            }
                                                            selected_banner_id.set(None);
                                                            admin_banner_title.set(String::new());
                                                            admin_banner_text.set(String::new());
                                                            admin_banner_label.set(String::new());
                                                            admin_banner_link.set(String::new());
                                                            admin_banner_icon.set("📢".to_string());
                                                            admin_banner_image.set(String::new());
                                                            admin_banner_active.set(true);
                                                            reload_banners_trigger.set(reload_banners_trigger() + 1);
                                                        });
                                                    },
                                                    if selected_banner_id().is_some() { "Atualizar Anúncio" } else { "Salvar Anúncio" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

                // Rodapé Aero
                div { class: "h-[50px] bg-slate-50 border-t border-slate-200 px-4 flex items-center justify-end space-x-2 flex-shrink-0 pt-2",
                    button {
                        class: "px-5 py-1.5 bg-white hover:bg-slate-50 active:bg-slate-100 border border-slate-300 rounded font-semibold transition-all text-[11px] cursor-pointer shadow-sm focus:outline-none text-[#2b3e51]",
                        onclick: move |_| close_action(),
                        "Cancelar"
                    }
                    button {
                        class: "px-5 py-1.5 {theme.btn_primary()} rounded font-bold transition-all text-[11px] cursor-pointer shadow focus:outline-none",
                        onclick: move |_| close_action(),
                        "Ok"
                    }
                }
            }
        }
    }
