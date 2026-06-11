use crate::components::main::contact_list::ContactList;
use crate::components::profile::profile_header::ProfileHeader;
use crate::models::UserStatus;
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

    let mut show_ad_modal = use_signal(|| false);

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
                if banner.icon == "BANNER" {
                    if let Some(ref img_url) = banner.image_url {
                        a {
                            class: "h-[50px] w-full border-t {theme.titlebar_border()} cursor-pointer overflow-hidden transition-all hover:brightness-105 flex items-center justify-center bg-black/5 flex-shrink-0",
                            href: "{banner.link}",
                            target: "_blank",
                            img {
                                src: "{img_url}",
                                class: "w-full h-full object-cover select-none pointer-events-none"
                            }
                        }
                    } else {
                        div {}
                    }
                } else {
                    if banner.image_url.is_some() {
                        div {
                            class: "h-[50px] w-full bg-gradient-to-r {theme.titlebar_gradient()} border-t {theme.titlebar_border()} px-3 flex items-center justify-between text-[11px] shadow-inner flex-shrink-0 cursor-pointer overflow-hidden transition-all hover:brightness-105",
                            onclick: move |_| {
                                show_ad_modal.set(true);
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
                    } else {
                        a {
                            class: "h-[50px] w-full bg-gradient-to-r {theme.titlebar_gradient()} border-t {theme.titlebar_border()} px-3 flex items-center justify-between text-[11px] shadow-inner flex-shrink-0 cursor-pointer overflow-hidden transition-all hover:brightness-105",
                            href: "{banner.link}",
                            target: "_blank",
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
