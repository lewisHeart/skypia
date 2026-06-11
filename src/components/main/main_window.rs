use crate::components::main::contact_list::ContactList;
use crate::components::profile::profile_header::ProfileHeader;
use crate::models::{AppTheme, UserStatus};
use crate::sound::play_sound;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn MainWindow(mut state: AppState) -> Element {
    let theme = state.theme();
    // Sinais para os modais locais
    let mut add_contact_email = use_signal(|| String::new());

    let mut search_result = use_signal(|| None::<crate::models::UserProfile>);
    let mut search_error = use_signal(|| None::<String>);
    let mut is_searching = use_signal(|| false);
    let mut active_tab = use_signal(|| "pessoais".to_string());

    let mut temp_name = use_signal(|| String::new());
    let mut temp_msg = use_signal(|| String::new());
    let mut temp_folder = use_signal(|| String::new());

    let mut admin_banner_icon = use_signal(|| "📢".to_string());
    let mut admin_banner_text = use_signal(|| String::new());
    let mut admin_banner_label = use_signal(|| String::new());
    let mut admin_banner_link = use_signal(|| String::new());
    let mut admin_banner_image = use_signal(|| String::new());

    let mut new_cat_input = use_signal(|| String::new());
    let mut show_ad_modal = use_signal(|| false);

    // Sinais para o posicionamento Aero flutuante e arrastável do Modal de Configurações
    let mut settings_pos = use_signal(|| None::<(f64, f64)>);
    let mut settings_dragging = use_signal(|| false);
    let mut settings_drag_offset = use_signal(|| (0.0, 0.0));

    // Sincroniza os valores temporários quando o modal de configurações é aberto
    use_effect(move || {
        if state.show_settings_modal() {
            temp_name.set(state.user_name());
            temp_msg.set(state.user_personal_message());
            temp_folder.set(state.download_folder());
            if let Some(banner) = state.banner_info() {
                admin_banner_icon.set(banner.icon);
                admin_banner_text.set(banner.text);
                admin_banner_label.set(banner.action_label);
                admin_banner_link.set(banner.link);
                admin_banner_image.set(banner.image_url.clone().unwrap_or_default());
            }
        }
    });

    // Reseta o estado de busca quando o modal é fechado
    use_effect(move || {
        if !state.show_add_contact_modal() {
            search_result.set(None);
            search_error.set(None);
            is_searching.set(false);
            add_contact_email.set(String::new());
        }
    });

    let mut handle_search = move || {
        let query = add_contact_email().trim().to_string();
        if query.is_empty() {
            return;
        }

        let token_opt = state.auth_token();
        is_searching.set(true);
        search_error.set(None);
        search_result.set(None);

        spawn(async move {
            if let Some(token) = token_opt {
                match crate::services::api::search_user(&token, &query).await {
                    Ok(user) => {
                        search_result.set(Some(user));
                    }
                    Err(e) => {
                        search_error.set(Some(e));
                    }
                }
            } else {
                search_error.set(Some("Você precisa estar conectado na rede.".to_string()));
            }
            is_searching.set(false);
        });
    };

    rsx! {
        div {
            class: "w-full h-full flex flex-col select-none bg-transparent overflow-hidden",

            // Header do Perfil do usuário
            ProfileHeader { state }

            // Lista de Contatos com pesquisa integrada
            ContactList { state }

            // Banner dinâmico de anúncios do banco de dados
            if let Some(banner) = state.banner_info() {
                div {
                    class: "h-[50px] w-full bg-gradient-to-r {theme.titlebar_gradient()} border-t {theme.titlebar_border()} px-3 flex items-center justify-between text-[11px] shadow-inner flex-shrink-0 cursor-pointer overflow-hidden transition-all hover:brightness-105",
                    onclick: {
                        let banner_clone = banner.clone();
                        move |_| {
                            if banner_clone.image_url.is_some() {
                                show_ad_modal.set(true);
                            } else {
                                let _ = document::eval(&format!("window.open('{}', '_blank')", banner_clone.link));
                            }
                        }
                    },
                    div { class: "flex items-center space-x-2 flex-1 {theme.titlebar_text()} min-w-0",
                        style: "opacity: 0.90;",
                        span { class: "text-base flex-shrink-0", "{banner.icon}" }
                        div { class: "flex flex-col min-w-0 flex-1",
                            span { class: "font-bold {theme.titlebar_text()} truncate", "{banner.text}" }
                            span { class: "text-[10px] text-slate-500 truncate hover:underline", "{banner.action_label}" }
                        }
                    }
                }
            }
        }

        // ==========================================
        // MODAL DE CONFIGURAÇÕES
        // ==========================================
        if state.show_settings_modal() {
            div {
                class: if settings_pos().is_none() {
                    "fixed inset-0 bg-black/15 backdrop-blur-[1px] z-[200] flex items-center justify-center p-4 select-none cursor-default"
                } else {
                    "fixed inset-0 bg-black/15 backdrop-blur-[1px] z-[200] select-none cursor-default"
                },
                onmousemove: move |evt| {
                    if settings_dragging() {
                        let coords = evt.data().page_coordinates();
                        let offset = settings_drag_offset();
                        settings_pos.set(Some((coords.x - offset.0, coords.y - offset.1)));
                    }
                },
                onmouseup: move |_| {
                    settings_dragging.set(false);
                },
                onclick: move |_| state.show_settings_modal.set(false),
                div {
                    class: "w-[92vw] max-w-[350px] sm:max-w-[620px] h-auto max-h-[90vh] sm:h-[480px] border rounded-lg shadow-2xl flex flex-col overflow-hidden pointer-events-auto",
                    style: if let Some((x, y)) = settings_pos() {
                        format!("position: fixed; left: {}px; top: {}px; width: 620px; height: 480px; background: {}; border: 1.5px solid rgba(255, 255, 255, 0.45); box-shadow: 0 10px 30px rgba(0, 0, 0, 0.25), inset 0 1px 0 rgba(255, 255, 255, 0.6); margin: 0; transform: none;", x, y, theme.bg_chat())
                    } else {
                        format!("background: {}; border: 1.5px solid rgba(255, 255, 255, 0.45); box-shadow: 0 10px 30px rgba(0, 0, 0, 0.25), inset 0 1px 0 rgba(255, 255, 255, 0.6);", theme.bg_chat())
                    },
                    onclick: move |e| e.stop_propagation(),

                    div { 
                        class: "h-9 bg-gradient-to-r {theme.titlebar_gradient()} border-b {theme.titlebar_border()} flex items-center justify-between px-3 flex-shrink-0 select-none cursor-move",
                        onmousedown: move |evt| {
                            let coords = evt.data().page_coordinates();
                            let current_pos = settings_pos().unwrap_or_else(|| {
                                (250.0, 120.0)
                            });
                            settings_drag_offset.set((coords.x - current_pos.0, coords.y - current_pos.1));
                            settings_dragging.set(true);
                        },
                        div { class: "flex items-center space-x-1.5 font-bold text-[11px] {theme.titlebar_text()}",
                            img {
                                src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Settings/SVG/ic_fluent_settings_24_color.svg",
                                class: "w-5 h-5 object-contain pointer-events-none"
                            }
                            span { "Configurações do Skypia" }
                        }
                        button {
                            class: "w-[28px] h-[18px] bg-white border border-[#d1d1d1] rounded-[3px] shadow-sm flex items-center justify-center cursor-pointer transition-all hover:bg-[#e81123] hover:border-[#e81123] hover:text-white text-[#6f6f6f] hover:text-white focus:outline-none text-[8px] font-bold",
                            title: "Fechar",
                            onclick: move |_| state.show_settings_modal.set(false),
                            "✕"
                        }
                    }

                    div { class: "flex-1 flex flex-col sm:flex-row overflow-hidden text-xs {theme.titlebar_text()}",
                        // Coluna de Abas (Horizontal com scroll no mobile, Vertical no desktop)
                        div { class: "w-full sm:w-[160px] border-b sm:border-b-0 sm:border-r {theme.titlebar_border()} bg-white/40 flex flex-row sm:flex-col p-1.5 sm:p-2 space-x-1 sm:space-x-0 sm:space-y-1 overflow-x-auto sm:overflow-x-visible sm:overflow-y-auto select-none flex-shrink-0 scrollbar-none",
                            {
                                let is_admin = state.user_email().contains("admin") || state.user_email() == "wk.scbd@skypia.io";
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
                        div { class: "flex-1 p-4 overflow-y-auto flex flex-col space-y-4 bg-white/10 max-h-[50vh] sm:max-h-none",

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
                                                    state.show_settings_modal.set(false);
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
                                            span { "Detectar música do Spotify automaticamente (RPC)" }
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
                                                let cats = state.categories.read();
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
                                                        if !name.is_empty() && !state.categories.read().contains(&name) {
                                                            state.add_category(name);
                                                            new_cat_input.set(String::new());
                                                        }
                                                    }
                                                }
                                            }
                                            button {
                                                class: "px-3 py-1 bg-slate-200 hover:bg-slate-350 border border-slate-350 rounded font-bold text-[10px] cursor-pointer focus:outline-none shadow-sm",
                                                onclick: move |_| {
                                                    let name = new_cat_input().trim().to_string();
                                                    if !name.is_empty() && !state.categories.read().contains(&name) {
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
                                div { class: "flex flex-col space-y-3",
                                    div { class: "flex flex-col space-y-1",
                                        label { class: "font-semibold text-slate-700", "Ícone (Emoji)" }
                                        input {
                                            class: "px-2 py-1.5 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none focus:border-slate-400 bg-white",
                                            value: "{admin_banner_icon}",
                                            oninput: move |e| admin_banner_icon.set(e.value()),
                                        }
                                    }
                                    div { class: "flex flex-col space-y-1",
                                        label { class: "font-semibold text-slate-700", "Texto do Anúncio" }
                                        input {
                                            class: "px-2 py-1.5 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none focus:border-slate-400 bg-white",
                                            value: "{admin_banner_text}",
                                            oninput: move |e| admin_banner_text.set(e.value()),
                                        }
                                    }
                                    div { class: "flex flex-col space-y-1",
                                        label { class: "font-semibold text-slate-700", "Rótulo do Botão" }
                                        input {
                                            class: "px-2 py-1.5 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none focus:border-slate-400 bg-white",
                                            value: "{admin_banner_label}",
                                            oninput: move |e| admin_banner_label.set(e.value()),
                                        }
                                    }
                                    div { class: "flex flex-col space-y-1",
                                        label { class: "font-semibold text-slate-700", "Link de Destino" }
                                        input {
                                            class: "px-2 py-1.5 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none focus:border-slate-400 bg-white",
                                            value: "{admin_banner_link}",
                                            oninput: move |e| admin_banner_link.set(e.value()),
                                        }
                                    }
                                    div { class: "flex flex-col space-y-1",
                                        label { class: "font-semibold text-slate-700", "URL da Imagem (Opcional)" }
                                        input {
                                            class: "px-2 py-1.5 border border-[#d1d1d1] msn-input rounded text-xs w-full focus:outline-none focus:border-slate-400 bg-white",
                                            value: "{admin_banner_image}",
                                            placeholder: "https://site.com/imagem.png",
                                            oninput: move |e| admin_banner_image.set(e.value()),
                                        }
                                    }
                                    button {
                                        class: "px-4 py-1.5 {theme.btn_primary()} rounded font-bold shadow-md cursor-pointer transition-all focus:outline-none self-end text-[10px] disabled:opacity-50 disabled:cursor-not-allowed",
                                        disabled: admin_banner_text().trim().is_empty() || admin_banner_link().trim().is_empty(),
                                        onclick: move |_| {
                                            let img_opt = if admin_banner_image().trim().is_empty() { None } else { Some(admin_banner_image().trim().to_string()) };
                                            let b = crate::models::BannerInfo {
                                                icon: admin_banner_icon(),
                                                text: admin_banner_text().trim().to_string(),
                                                action_label: admin_banner_label().trim().to_string(),
                                                link: admin_banner_link().trim().to_string(),
                                                image_url: img_opt,
                                            };
                                            state.update_banner_admin(b);
                                            state.add_toast("Anúncio Salvo".to_string(), "O banner promocional foi atualizado.".to_string(), None);
                                        },
                                        "Salvar Anúncio"
                                    }
                                }
                            }
                        }
                    }

                    // Rodapé Aero
                    div { class: "h-[50px] bg-slate-50 border-t border-slate-200 px-4 flex items-center justify-end space-x-2 flex-shrink-0",
                        button {
                            class: "px-5 py-1.5 bg-white hover:bg-slate-50 active:bg-slate-100 border border-slate-300 rounded font-semibold transition-all text-[11px] cursor-pointer shadow-sm focus:outline-none text-[#2b3e51]",
                            onclick: move |_| state.show_settings_modal.set(false),
                            "Cancelar"
                        }
                        button {
                            class: "px-5 py-1.5 {theme.btn_primary()} rounded font-bold transition-all text-[11px] cursor-pointer shadow focus:outline-none",
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
                class: "fixed inset-0 bg-black/15 backdrop-blur-[1px] z-[200] flex items-center justify-center p-4",
                onclick: move |_| state.show_add_contact_modal.set(false),
                div {
                    class: "w-[340px] border rounded-lg shadow-2xl flex flex-col overflow-hidden pointer-events-auto",
                    style: "background: {theme.bg_chat()}; border: 1.5px solid rgba(255, 255, 255, 0.45); box-shadow: 0 10px 30px rgba(0, 0, 0, 0.25), inset 0 1px 0 rgba(255, 255, 255, 0.6);",
                    onclick: move |e| e.stop_propagation(),

                    div { class: "h-9 bg-gradient-to-r {theme.titlebar_gradient()} border-b {theme.titlebar_border()} flex items-center justify-between px-3 flex-shrink-0 select-none",
                        div { class: "flex items-center space-x-1.5 font-bold text-[11px] {theme.titlebar_text()}",
                            img {
                                src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Person%20Add/SVG/ic_fluent_person_add_24_color.svg",
                                class: "w-5 h-5 object-contain pointer-events-none"
                            }
                            span { "Adicionar Novo Contato" }
                        }
                        button {
                            class: "w-[28px] h-[18px] bg-white border border-[#d1d1d1] rounded-[3px] shadow-sm flex items-center justify-center cursor-pointer transition-all hover:bg-[#e81123] hover:border-[#e81123] hover:text-white text-[#6f6f6f] hover:text-white focus:outline-none text-[8px] font-bold",
                            title: "Fechar",
                            onclick: move |_| state.show_add_contact_modal.set(false),
                            "✕"
                        }
                    }

                    // Conteúdo do Modal com padding Aero
                    div { class: "p-4 flex flex-col space-y-4 text-xs {theme.titlebar_text()}",

                        // Campo de entrada e botão de busca
                        div { class: "flex flex-col space-y-1.5",
                            label { class: "font-semibold text-slate-700", "Email ou Nome de usuário:" }
                            div { class: "flex space-x-1.5",
                                input {
                                    class: "flex-1 h-[27px] px-2.5 text-xs text-slate-800 bg-white border border-[#d1d1d1] rounded-[4px] msn-input placeholder-[#a5a5a5] placeholder:text-[10px] focus:outline-none focus:border-slate-400",
                                    placeholder: "Joao ou joao@mail.com",
                                    value: "{add_contact_email}",
                                    oninput: move |e| add_contact_email.set(e.value()),
                                    onkeydown: move |e| {
                                        if e.key() == Key::Enter && !add_contact_email().trim().is_empty() && !is_searching() {
                                            handle_search();
                                        }
                                    }
                                }
                                button {
                                    class: "px-3 h-[27px] {theme.btn_primary()} rounded-[4px] font-bold shadow transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center focus:outline-none text-[10px]",
                                    disabled: add_contact_email().trim().is_empty() || is_searching(),
                                    onclick: move |_| handle_search(),
                                    if is_searching() { "Buscando..." } else { "Buscar" }
                                }
                            }
                        }

                        // Painel de Resultados de busca
                        div { class: "min-h-[90px] border {theme.titlebar_border()}/40 bg-white/40 rounded-[6px] p-2.5 flex flex-col justify-center items-center relative overflow-hidden",
                            if is_searching() {
                                div { class: "flex flex-col items-center space-y-2 text-slate-500 py-4",
                                    div { class: "w-5 h-5 border-2 border-sky-600 border-t-transparent rounded-full animate-spin" }
                                    span { "Buscando usuário no servidor..." }
                                }
                            } else if let Some(ref err) = search_error() {
                                div { class: "flex flex-col items-center space-y-1 text-center py-2 text-[#b50a18]",
                                    span { class: "text-lg", "⚠️" }
                                    span { class: "font-semibold", "{err}" }
                                }
                            } else if let Some(ref user) = search_result() {
                                {
                                    let user_for_add = user.clone();
                                    let status_enum = match user_for_add.status.as_str() {
                                        "Online" => crate::models::UserStatus::Online,
                                        "Ocupado" => crate::models::UserStatus::Ocupado,
                                        "Ausente" => crate::models::UserStatus::Ausente,
                                        "Invisivel" => crate::models::UserStatus::Invisivel,
                                        _ => crate::models::UserStatus::Offline,
                                    };
                                    rsx! {
                                        div { class: "w-full flex items-center space-x-3.5",
                                            // Avatar com moldura de status do MSN
                                            div {
                                                class: "flex-shrink-0 p-[2px] rounded-[7px] border {status_enum.avatar_frame_class()} bg-transparent shadow-[inset_0_0.5px_0_rgba(255,255,255,0.4)] flex items-center justify-center shadow-md",
                                                div {
                                                    class: "rounded-[4px] overflow-hidden border border-white/30 bg-white flex-shrink-0 flex items-center justify-center",
                                                    {crate::models::render_avatar(user_for_add.avatar_url.as_deref(), 48)}
                                                }
                                            }
                                            // Detalhes
                                            div { class: "flex-1 min-w-0 flex flex-col space-y-0.5",
                                                span { class: "font-bold text-sm {theme.titlebar_text()} truncate", "{user_for_add.display_name}" }
                                                span { class: "text-[10px] text-slate-500 font-semibold truncate", "{user_for_add.email}" }
                                                span { class: "text-[10px] text-slate-400 truncate italic", "“{user_for_add.personal_message}”" }
                                            }
                                        }
                                    }
                                }
                            } else {
                                // Estado inicial/vazio
                                div { class: "text-center text-slate-400 py-4 font-normal",
                                    "Digite as informações e clique em Buscar para encontrar um amigo."
                                }
                            }
                        }

                        // Botões de controle no rodapé
                        div { class: "flex justify-end space-x-2 pt-2 border-t border-slate-200/50",
                            button {
                                class: "px-4 py-1.5 bg-white hover:bg-slate-100 border border-slate-350 text-slate-700 rounded-[4px] font-bold cursor-pointer transition-colors focus:outline-none text-[10px]",
                                onclick: move |_| state.show_add_contact_modal.set(false),
                                "Cancelar"
                            }
                            if let Some(ref user) = search_result() {
                                {
                                    let user_clone = user.clone();
                                    rsx! {
                                        button {
                                            class: "px-4 py-1.5 {theme.btn_primary()} rounded-[4px] font-bold shadow transition-colors cursor-pointer focus:outline-none text-[10px]",
                                            onclick: move |_| {
                                                state.add_contact_dynamic(
                                                    user_clone.email.clone(),
                                                    user_clone.display_name.clone(),
                                                    UserStatus::Offline,
                                                    user_clone.personal_message.clone()
                                                );
                                                play_sound("online");
                                                state.show_add_contact_modal.set(false);
                                            },
                                            "Adicionar Contato"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ==========================================
        // MODAL DE ANÚNCIO AERO
        // ==========================================
        if show_ad_modal() {
            if let Some(banner) = state.banner_info() {
                div {
                    class: "fixed inset-0 bg-black/30 backdrop-blur-[2px] z-[300] flex items-center justify-center p-4",
                    onclick: move |_| show_ad_modal.set(false),
                    div {
                        class: "w-[90vw] max-w-[420px] border rounded-lg shadow-2xl flex flex-col overflow-hidden pointer-events-auto",
                        style: "background: {theme.bg_chat()}; border: 1.5px solid rgba(255, 255, 255, 0.45); box-shadow: 0 10px 30px rgba(0, 0, 0, 0.35), inset 0 1px 0 rgba(255, 255, 255, 0.6);",
                        onclick: move |e| e.stop_propagation(),

                        div { class: "h-9 bg-gradient-to-r {theme.titlebar_gradient()} border-b {theme.titlebar_border()} flex items-center justify-between px-3 flex-shrink-0 select-none",
                            div { class: "flex items-center space-x-1.5 font-bold text-[11px] {theme.titlebar_text()}",
                                span { class: "text-sm", "{banner.icon}" }
                                span { "Anúncio do Skypia" }
                            }
                            button {
                                class: "w-[28px] h-[18px] bg-white border border-[#d1d1d1] rounded-[3px] shadow-sm flex items-center justify-center cursor-pointer transition-all hover:bg-[#e81123] hover:border-[#e81123] hover:text-white text-[#6f6f6f] hover:text-white focus:outline-none text-[8px] font-bold",
                                title: "Fechar",
                                onclick: move |_| show_ad_modal.set(false),
                                "✕"
                            }
                        }

                        div { class: "p-4 flex flex-col space-y-4 text-xs {theme.titlebar_text()}",
                            if let Some(ref img_url) = banner.image_url {
                                if !img_url.trim().is_empty() {
                                    div { class: "w-full max-h-[200px] overflow-hidden rounded border border-slate-250 shadow-sm flex items-center justify-center bg-black/5",
                                        img {
                                            src: "{img_url}",
                                            class: "max-w-full max-h-[200px] object-contain"
                                        }
                                    }
                                }
                            }

                            div { class: "flex flex-col space-y-1.5 text-center px-2",
                                span { class: "font-bold text-sm text-slate-800", "{banner.text}" }
                                span { class: "text-[10px] text-slate-500", "Clique no botão abaixo para saber mais." }
                            }

                            div { class: "flex justify-end space-x-2 pt-2 border-t border-slate-200/50",
                                button {
                                    class: "px-4 py-1.5 bg-white hover:bg-slate-100 border border-slate-350 text-slate-700 rounded-[4px] font-bold cursor-pointer transition-colors focus:outline-none text-[10px]",
                                    onclick: move |_| show_ad_modal.set(false),
                                    "Fechar"
                                }
                                button {
                                    class: "px-5 py-1.5 {theme.btn_primary()} rounded-[4px] font-bold shadow transition-colors cursor-pointer focus:outline-none text-[10px]",
                                    onclick: {
                                        let link = banner.link.clone();
                                        move |_| {
                                            let _ = document::eval(&format!("window.open('{}', '_blank')", link));
                                            show_ad_modal.set(false);
                                        }
                                    },
                                    "{banner.action_label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
