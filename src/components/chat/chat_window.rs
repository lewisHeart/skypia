use crate::components::chat::chat_feed::ChatFeed;
use crate::components::chat::chat_input::ChatInput;
use crate::components::chat::chat_sidebar::ChatSidebar;
use crate::state::AppState;
use dioxus::prelude::*;

const WINK_STYLES: &str = r#"
@keyframes msnKiss {
    0% { transform: scale(0.1) rotate(0deg); opacity: 0; }
    20% { transform: scale(1.5) rotate(-15deg); opacity: 1; }
    40% { transform: scale(1.3) rotate(15deg); }
    60% { transform: scale(1.6) rotate(-10deg); }
    80% { transform: scale(1.4) rotate(0deg); opacity: 1; }
    100% { transform: scale(2.0); opacity: 0; }
}
.animate-msn-kiss {
    animation: msnKiss 3.5s forwards ease-in-out;
}

@keyframes msnHammer {
    0% { transform: translate(0, -100px) rotate(-45deg); opacity: 0; }
    15% { transform: translate(0, 0) rotate(0deg); opacity: 1; }
    20% { transform: scale(1.1) translate(0, 5px); }
    25% { transform: scale(1.0) translate(0, 0); }
    75% { opacity: 1; }
    100% { opacity: 0; }
}
.animate-msn-hammer {
    animation: msnHammer 2.5s forwards ease-out;
}

@keyframes msnPig {
    0% { transform: translate(-100px, 100px) scale(0.5); }
    25% { transform: translate(0, -50px) scale(1.1); }
    50% { transform: translate(100px, 50px) scale(0.9); }
    75% { transform: translate(0, 0) scale(1.2); }
    100% { transform: translate(-200px, -200px) scale(1.5); opacity: 0; }
}
.animate-msn-pig {
    animation: msnPig 4s forwards linear;
}
"#;

#[component]
pub fn ChatWindow(mut state: AppState, contact_id_prop: Option<String>) -> Element {
    let theme = state.theme();
    let active_chats = state.active_chats();
    let resolved_contact_id = match contact_id_prop.clone() {
        Some(id) => id,
        None => match state.selected_chat_id() {
            Some(id) => id,
            None => return rsx! {},
        },
    };

    let contact_id = resolved_contact_id;
    let contact = state.contacts().into_iter().find(|c| c.id == contact_id);
    let group = (state.group_chats)()
        .into_iter()
        .find(|g| g.id == contact_id);

    if contact.is_none() && group.is_none() {
        return rsx! {};
    }

    let is_group = group.is_some();
    let display_name_to_show = if let Some(ref g) = group {
        g.name
            .clone()
            .unwrap_or_else(|| "Grupo sem nome".to_string())
    } else {
        let c = contact.as_ref().unwrap();
        c.nickname.clone().unwrap_or_else(|| c.display_name.clone())
    };

    let status_text = if let Some(ref g) = group {
        format!("Grupo: {} participantes", g.members.len())
    } else {
        contact.as_ref().unwrap().status.as_str().to_string()
    };

    let display_status = if is_group { "".to_string() } else { format!("({})", status_text) };

    let personal_message_text = if let Some(ref g) = group {
        let member_names = g
            .members
            .iter()
            .map(|m| m.nickname.clone().unwrap_or_else(|| m.display_name.clone()))
            .collect::<Vec<String>>()
            .join(", ");
        format!("Membros: {}", member_names)
    } else {
        contact.as_ref().unwrap().personal_message.clone()
    };



    let active_nudge = state.active_nudge();
    let is_nudge_active = active_nudge.as_ref() == Some(&contact_id);
    let shake_class = if is_nudge_active {
        "animate-nudge-shake"
    } else {
        ""
    };

    rsx! {
        style { "{WINK_STYLES}" }

        // Camada de animação de Winks ativa
        if let Some(wink) = state.active_wink() {
            div {
                class: "absolute inset-0 z-[150] flex flex-col items-center justify-center pointer-events-none select-none overflow-hidden rounded-lg",

                if wink == "kiss" {
                    div { class: "absolute inset-0 bg-pink-400/20 animate-pulse" }
                    div { class: "text-9xl animate-msn-kiss", "💋" }
                } else if wink == "hammer" {
                    div { class: "absolute inset-0 bg-slate-900/10" }
                    div { class: "text-9xl animate-msn-hammer", "🔨" }
                    div { class: "absolute w-40 h-40 border-4 border-dashed border-white/60 rounded-full animate-ping" }
                } else if wink == "pig" {
                    div { class: "absolute inset-0 bg-emerald-400/15" }
                    div { class: "text-9xl animate-msn-pig", "🐷" }
                }
            }
        }

        div {
            class: "w-full h-full flex flex-col select-none bg-transparent {shake_class} overflow-hidden",

            // Abas para chats ativos
            if contact_id_prop.is_none() && active_chats.len() > 1 {
                div { class: "h-8 bg-transparent flex items-center px-2 space-x-1 flex-shrink-0 overflow-x-auto",
                    for chat_id in active_chats {
                        if let Some(c) = state.contacts().into_iter().find(|c| c.id == chat_id) {
                            {
                                let is_active = chat_id == contact_id;
                                let active_tab_style = if is_active {
                                    format!("bg-white border-t border-l border-r border-[#96badb] {theme_text} font-bold z-10", theme_text = theme.titlebar_text())
                                } else {
                                    format!("bg-[#d8e8f6]/70 border border-transparent {theme_text}/80 hover:bg-[#d8e8f6]", theme_text = theme.titlebar_text())
                                };
                                let name_to_show = c.nickname.clone().unwrap_or(c.display_name.clone());
                                let chat_id_select = chat_id.clone();
                                let chat_id_close = chat_id.clone();

                                rsx! {
                                    button {
                                        class: "px-3 h-6 flex items-center space-x-1.5 rounded-t text-[11px] transition-all cursor-pointer truncate max-w-[120px] {active_tab_style}",
                                        onclick: move |_| {
                                            *state.selected_chat_id.write() = Some(chat_id_select.clone());
                                        },
                                        div { class: "w-2 h-2 rounded-full {c.status.color_class()}" }
                                        span { "{name_to_show}" }
                                        span {
                                            class: "text-[9px] text-slate-400 hover:text-red-500 font-bold ml-1.5",
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                state.close_chat(chat_id_close.clone());
                                            },
                                            "x"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Layout Principal do Chat
            div { class: "flex-1 flex min-h-0 w-full relative",

                // Coluna Esquerda: Avatars grandes de perfil ou Jogo da Velha (ChatSidebar)
                ChatSidebar { contact_id: contact_id.clone(), state }

                // Coluna Direita: Cabeçalho de Status, Histórico e Input de texto
                div { class: "flex-1 flex flex-col min-w-0 h-full bg-transparent relative",

                    // Painel Superior de Status (Informações do contato atual)
                    div { class: "p-3 flex items-center bg-transparent flex-shrink-0 justify-between",
                        div { class: "flex items-center space-x-3 min-w-0 flex-1",

                            if contact_id_prop.is_none() {
                                button {
                                    class: "md:hidden px-2 py-1 bg-white/60 hover:bg-white border border-slate-300 {theme.titlebar_text()} text-[11px] rounded font-bold cursor-pointer mr-1 flex items-center space-x-0.5 flex-shrink-0 transition-colors",
                                    title: "Voltar para Lista de Contatos",
                                    onclick: move |_| {
                                        *state.selected_chat_id.write() = None;
                                    },
                                    span { "⬅️" }
                                }
                            }



                            div { class: "flex-1 min-w-0 flex flex-col justify-center",
                                div { class: "flex items-baseline space-x-2.5",
                                    span { class: "font-normal text-[32px] text-[#2d517a] truncate", "{display_name_to_show}" }
                                    span { class: "text-sm text-[#a5a5a5] font-normal flex-shrink-0", "{display_status}" }
                                }
                                p { class: "text-sm text-[#a5a5a5] truncate mt-0.5", "{personal_message_text}" }
                            }
                        }

                        // Ícones de ação clássicos no topo direito (Games, Busca, Voz, Dropdown)
                        div { class: "flex items-center space-x-3 mr-2 flex-shrink-0",
                            button {
                                class: "hover:bg-white/40 p-1.5 rounded transition-all focus:outline-none cursor-pointer flex items-center justify-center",
                                title: "Jogos e Atividades",
                                onclick: move |_| {
                                    state.show_games_modal.set(true);
                                },
                                svg {
                                    class: "w-5 h-5 select-none pointer-events-none",
                                    view_box: "0 0 100 100",
                                    circle { cx: "40", cy: "35", r: "14", fill: "#3a90f0" }
                                    path { d: "M15 75 C15 55, 65 55, 65 75 Z", fill: "#3a90f0" }
                                    circle { cx: "65", cy: "48", r: "11", fill: "#5cd63a" }
                                    path { d: "M45 78 C45 62, 85 62, 85 78 Z", fill: "#5cd63a" }
                                }
                            }
                            button {
                                class: "hover:bg-white/40 p-1.5 rounded transition-all focus:outline-none cursor-pointer flex items-center justify-center",
                                title: "Buscar nesta conversa",
                                svg {
                                    class: "w-5 h-5 text-[#3a90f0] select-none pointer-events-none",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2.5",
                                    stroke_linecap: "round",
                                    circle { cx: "11", cy: "11", r: "5.5" }
                                    path { d: "M15 15l5 5" }
                                }
                            }
                            button {
                                class: "hover:bg-white/40 p-1.5 rounded transition-all focus:outline-none cursor-pointer flex items-center justify-center",
                                title: "Iniciar chamada de áudio",
                                svg {
                                    class: "w-5 h-5 text-[#e81123] select-none pointer-events-none",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    path { d: "M20 15.5c-1.2 0-2.4-.2-3.6-.6-.3-.1-.7 0-1 .2l-2.2 2.2c-2.8-1.4-5.1-3.8-6.6-6.6l2.2-2.2c.3-.3.4-.7.2-1-.3-1.1-.5-2.3-.5-3.5 0-.6-.4-1-1-1H4c-.6 0-1 .4-1 1 0 9.4 7.6 17 17 17 .6 0 1-.4 1-1v-3.5c0-.6-.4-1-1-1z" }
                                }
                            }
                            svg {
                                class: "w-4 h-4 text-slate-500 cursor-pointer hover:text-slate-700",
                                view_box: "0 0 24 24",
                                fill: "currentColor",
                                path { d: "M7 10l5 5 5-5z" }
                            }
                        }

                        if contact_id_prop.is_none() {
                            {
                                let contact_id_detach = contact_id.clone();
                                rsx! {
                                    button {
                                        class: "w-6 h-6 flex items-center justify-center rounded hover:bg-white/45 border border-transparent hover:border-slate-300 {theme.titlebar_text()} cursor-pointer text-xs transition-colors flex-shrink-0",
                                        title: "Desvincular conversa",
                                        onclick: move |_| {
                                            state.detach_chat(contact_id_detach.clone());
                                            let dom = VirtualDom::new_with_props(
                                                DetachedChatWindow,
                                                DetachedChatWindowProps { contact_id: contact_id_detach.clone() }
                                            );

                                            #[cfg(feature = "desktop")]
                                            spawn(async move {
                                                let mut config = dioxus::desktop::Config::new().with_menu(None);
                                                let my_pid = std::process::id();
                                                let db_dir = std::path::Path::new(".skypia_data").join("db");
                                                let mut active_slot = None;
                                                for slot in 1..=10 {
                                                    let lock_path = db_dir.join(format!("skypia_{}.lock", slot));
                                                    if let Ok(content) = std::fs::read_to_string(&lock_path) {
                                                        if let Ok(pid) = content.trim().parse::<u32>() {
                                                            if pid == my_pid {
                                                                active_slot = Some(slot);
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                                if let Some(slot) = active_slot {
                                                    let data_dir = std::env::current_dir()
                                                        .unwrap_or_else(|_| std::path::PathBuf::from("."))
                                                        .join(".skypia_data")
                                                        .join("webview")
                                                        .join(format!("slot_{}", slot));
                                                    let _ = std::fs::create_dir_all(&data_dir);
                                                    config = config.with_data_directory(data_dir);
                                                }
                                                let _ = dioxus::desktop::window().new_window(
                                                    dom,
                                                    config
                                                ).await;
                                            });
                                            #[cfg(not(feature = "desktop"))]
                                            {
                                                let _ = dom;
                                            }
                                        },
                                        "↗"
                                    }
                                }
                            }
                        }
                    }

                    // Histórico do Chat (ChatFeed)
                    ChatFeed { contact_id: contact_id.clone(), state }

                    // Barra de formatação e Entrada de mensagem (ChatInput) ou aviso amarelo
                    if !is_group && contact.as_ref().map(|c| c.relation_status == "Pendente").unwrap_or(false) {
                        div { class: "h-20 bg-[#fffec8] border-t border-[#d8d080] p-4 flex flex-col justify-center items-center text-center text-xs text-[#5c5010] space-y-1 select-text flex-shrink-0",
                            span { class: "text-base", "⚠️" }
                            p { class: "font-semibold", "Esta solicitação de contato ainda não foi aceita." }
                            p { class: "text-[10.5px] text-[#7c7030]", "Você não pode enviar mensagens até que o contato aceite a solicitação." }
                        }
                    } else {
                        ChatInput {
                            contact_id: contact_id.clone(),
                            state,
                            on_nudge: move |_| {}
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DetachedChatWindowProps {
    pub contact_id: String,
}

#[component]
pub fn DetachedChatWindow(props: DetachedChatWindowProps) -> Element {
    #[cfg(feature = "desktop")]
    {
        let mut app_state = use_context_provider(|| AppState::new());
        let desktop = dioxus::desktop::use_window();

        // Garante login e seleção local
        *app_state.logged_in.write() = true;
        *app_state.selected_chat_id.write() = Some(props.contact_id.clone());

        // Adiciona à lista de chats ativos no estado local sem disparar open_chat
        {
            let mut chats = app_state.active_chats.write();
            if !chats.contains(&props.contact_id) {
                chats.push(props.contact_id.clone());
            }
        }

        use_effect(move || {
            let mut state = app_state;
            spawn(async move {
                // 1. Carrega o token de autenticação do SQLite local
                if let Ok(Some((token, user_id))) =
                    crate::services::db::DatabaseService::load_auth_token().await
                {
                    // Define o token e o user_id no estado local imediatamente
                    *state.auth_token.write() = Some(token.clone());
                    *state.server_user_id.write() = Some(user_id.clone());
                    *state.logged_in.write() = true;

                    // 2. Carrega os dados locais e inicia conexões em background
                    state.load_initial_data();
                    state.connect_websocket();

                    // 3. Busca o perfil atual no servidor em background para garantir dados corretos
                    if let Ok(profile) = crate::services::api::get_profile(&token).await {
                        let _ = state.apply_server_profile(profile, token).await;
                    }
                }
            });
        });

        // Sincroniza decorações da janela nativa flutuante
        let desktop_dec = desktop.clone();
        use_effect(move || {
            let use_custom = app_state.use_custom_titlebar();
            desktop_dec.set_decorations(!use_custom);
        });

        // Verificação periódica para fechar janela se o chat for acoplado de volta e para atualizar o tema
        let c_id = props.contact_id.clone();
        let desktop_close = desktop.clone();
        use_effect(move || {
            let cid = c_id.clone();
            let desktop_clone = desktop_close.clone();
            let mut state = app_state;
            spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

                    // 1. Sincroniza o tema e densidade com o banco de dados em tempo real
                    if let Ok((_scale, _custom_bar, db_theme, _chat_mode, db_density)) =
                        crate::services::db::DatabaseService::load_settings().await
                    {
                        if db_theme != state.theme() {
                            *state.theme.write() = db_theme;
                        }
                        if db_density != state.contact_density() {
                            state.update_densities_from_serialized(db_density);
                        }
                    }

                    // 2. Se o chat for acoplado, fecha esta janela
                    if let Ok(detached) =
                        crate::services::db::DatabaseService::get_detached_chats().await
                    {
                        if !detached.contains(&cid) {
                            desktop_clone.close();
                            break;
                        }
                    }
                }
            });
        });

        let theme = app_state.theme();
        let contact_opt = app_state
            .contacts()
            .into_iter()
            .find(|c| c.id == props.contact_id);
        let group_opt = (app_state.group_chats)()
            .into_iter()
            .find(|g| g.id == props.contact_id);

        if contact_opt.is_none() && group_opt.is_none() {
            return rsx! {
                document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
                document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
                div {
                    class: "w-screen h-screen flex flex-col items-center justify-center bg-gradient-to-br {theme.bg_gradient()} text-[#1e395b] font-bold text-xs select-none",
                    span { class: "text-2xl mb-2 animate-bounce", "🦋" }
                    span { class: "animate-pulse", "Carregando conversa..." }
                }
            };
        }

        let contact_name = if let Some(ref c) = contact_opt {
            c.nickname.clone().unwrap_or_else(|| c.display_name.clone())
        } else {
            group_opt
                .as_ref()
                .unwrap()
                .name
                .clone()
                .unwrap_or_else(|| "Grupo sem nome".to_string())
        };

        rsx! {
            document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
            document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

            div {
                class: "w-screen h-screen overflow-hidden flex flex-col bg-gradient-to-br {theme.bg_gradient()} relative font-segoe select-none rounded-t-2xl border border-[#7baad4]/40 shadow-2xl",

                // Bordas e Cantos para Redimensionamento Nativo da Janela Flutuante (Escala 100%!)
                if app_state.use_custom_titlebar() {
                    // Borda Superior
                    div {
                        class: "absolute top-0 left-1.5 right-1.5 h-1.5 cursor-ns-resize z-[999] opacity-0",
                        onmousedown: move |e| {
                            e.stop_propagation();
                            let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::North);
                        }
                    }
                    // Borda Inferior
                    div {
                        class: "absolute bottom-0 left-1.5 right-1.5 h-1.5 cursor-ns-resize z-[999] opacity-0",
                        onmousedown: move |e| {
                            e.stop_propagation();
                            let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::South);
                        }
                    }
                    // Borda Esquerda
                    div {
                        class: "absolute top-1.5 bottom-1.5 left-0 w-1.5 cursor-ew-resize z-[999] opacity-0",
                        onmousedown: move |e| {
                            e.stop_propagation();
                            let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::West);
                        }
                    }
                    // Borda Direita
                    div {
                        class: "absolute top-1.5 bottom-1.5 right-0 w-1.5 cursor-ew-resize z-[999] opacity-0",
                        onmousedown: move |e| {
                            e.stop_propagation();
                            let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::East);
                        }
                    }
                    // Canto Superior Esquerdo
                    div {
                        class: "absolute top-0 left-0 w-2.5 h-2.5 cursor-nwse-resize z-[999] opacity-0",
                        onmousedown: move |e| {
                            e.stop_propagation();
                            let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::NorthWest);
                        }
                    }
                    // Canto Superior Direito
                    div {
                        class: "absolute top-0 right-0 w-2.5 h-2.5 cursor-nesw-resize z-[999] opacity-0",
                        onmousedown: move |e| {
                            e.stop_propagation();
                            let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::NorthEast);
                        }
                    }
                    // Canto Inferior Esquerdo
                    div {
                        class: "absolute bottom-0 left-0 w-2.5 h-2.5 cursor-nesw-resize z-[999] opacity-0",
                        onmousedown: move |e| {
                            e.stop_propagation();
                            let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::SouthWest);
                        }
                    }
                    // Canto Inferior Direito
                    div {
                        class: "absolute bottom-0 right-0 w-2.5 h-2.5 cursor-nwse-resize z-[999] opacity-0",
                        onmousedown: move |e| {
                            e.stop_propagation();
                            let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::SouthEast);
                        }
                    }
                }


                // Subtle theme background bubbles
                div { class: "absolute inset-0 bg-bubbles pointer-events-none opacity-25 z-0" }

                div { class: "w-full h-full flex flex-col pointer-events-auto z-10",

                    // Barra de título Aero personalizada para janela desvinculada
                    if app_state.use_custom_titlebar() {
                        div {
                            class: "w-full h-8 bg-transparent flex items-center justify-between z-50 flex-shrink-0 select-none px-3 relative rounded-t-2xl cursor-default",
                            style: "-webkit-app-region: drag;",
                            onmousedown: move |_| {
                                #[cfg(feature = "desktop")]
                                let _ = dioxus::desktop::use_window().drag_window();
                            },

                            div { class: "flex items-center space-x-1.5 font-bold text-xs pointer-events-none {theme.titlebar_text()} select-none",
                                span { class: "text-base", "💬" }
                                span { "Conversa com {contact_name}" }
                            }

                            // Window control buttons
                            div {
                                class: "flex items-center space-x-2.5",
                                style: "-webkit-app-region: no-drag;",
                                onmousedown: move |e| e.stop_propagation(),

                                button {
                                    class: "px-2.5 h-[22px] flex items-center justify-center bg-white border border-slate-300 rounded font-semibold text-[10px] text-slate-700 hover:underline hover:border-slate-400 shadow-sm cursor-pointer transition-colors focus:outline-none",
                                    title: "Vincular de volta à janela principal do Skypia",
                                    onclick: {
                                        let cid = props.contact_id.clone();
                                        move |e| {
                                            e.stop_propagation();
                                            let cid_spawn = cid.clone();
                                            spawn(async move {
                                                let _ = crate::services::db::DatabaseService::attach_chat(cid_spawn.clone()).await;
                                            });
                                        }
                                    },
                                    span { class: "mr-1 text-xs", "↙" }
                                    span { "Acoplar" }
                                }
                                button {
                                    class: "w-6 h-[22px] flex items-center justify-center rounded hover:bg-black/5 text-slate-600 cursor-pointer transition-colors font-bold text-xs focus:outline-none",
                                    title: "Minimizar",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        #[cfg(feature = "desktop")]
                                        dioxus::desktop::use_window().set_minimized(true);
                                    },
                                    "⎯"
                                }
                                button {
                                    class: "w-6 h-[22px] flex items-center justify-center rounded hover:bg-black/5 text-slate-600 cursor-pointer transition-colors text-[9px] focus:outline-none",
                                    title: "Maximizar",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        #[cfg(feature = "desktop")]
                                        {
                                            let win = dioxus::desktop::use_window();
                                            win.set_maximized(!win.is_maximized());
                                        }
                                    },
                                    "⬜"
                                }
                                button {
                                    class: "h-[22px] px-3.5 bg-white border border-[#f2d3ce] rounded font-bold text-xs text-[#8c2222] shadow-sm cursor-pointer transition-all hover:bg-[#e81123] hover:border-[#e81123] hover:text-white flex items-center justify-center focus:outline-none",
                                    title: "Fechar",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        #[cfg(feature = "desktop")]
                                        dioxus::desktop::use_window().close();
                                    },
                                    "X"
                                }
                            }
                        }
                    } else {
                        // Barra de Controle simplificada caso use decorações do sistema nativo
                        div {
                            class: "h-8 bg-transparent px-3 flex items-center justify-between {theme.titlebar_text()} font-bold text-xs select-none",
                            div { class: "flex items-center space-x-1.5",
                                span { "💬" }
                                span { "Conversa com {contact_name}" }
                            }
                            button {
                                class: "px-2.5 h-[22px] flex items-center justify-center bg-white border border-slate-300 rounded font-semibold text-[10px] text-slate-700 hover:underline hover:border-slate-400 shadow-sm cursor-pointer transition-colors focus:outline-none",
                                title: "Vincular de volta à janela principal do Skypia",
                                onclick: {
                                    let cid = props.contact_id.clone();
                                    move |e| {
                                        e.stop_propagation();
                                        let cid_spawn = cid.clone();
                                        spawn(async move {
                                            let _ = crate::services::db::DatabaseService::attach_chat(cid_spawn.clone()).await;
                                        });
                                    }
                                },
                                span { class: "mr-1 text-xs", "↙" }
                                span { "Acoplar" }
                            }
                        }
                    }

                    // Área do chat propriamente dita (Escalonada!)
                    div {
                        class: "flex-1 min-h-0 w-full relative",

                        div {
                            class: "absolute inset-0 overflow-hidden",

                            div {
                                class: "w-full h-full relative",
                                style: "transform: scale({app_state.interface_scale()}); transform-origin: top left; width: {100.0 / app_state.interface_scale()}%; height: {100.0 / app_state.interface_scale()}%;",

                                ChatWindow { state: app_state, contact_id_prop: Some(props.contact_id.clone()) }
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(not(feature = "desktop"))]
    {
        let _ = props;
        rsx! {
            div { class: "p-4 text-xs text-slate-500", "Recurso disponível apenas no Desktop." }
        }
    }
}
