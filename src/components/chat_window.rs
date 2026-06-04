use dioxus::prelude::*;
use crate::models::{render_avatar, UserStatus, AppTheme};
use crate::state::AppState;
use crate::components::chat_feed::ChatFeed;
use crate::components::chat_input::ChatInput;
use crate::components::chat_sidebar::ChatSidebar;

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
pub fn ChatWindow(mut state: AppState, contact_id_prop: Option<usize>) -> Element {
    let active_chats = state.active_chats();
    let resolved_contact_id = match contact_id_prop {
        Some(id) => id,
        None => match state.selected_chat_id() {
            Some(id) => id,
            None => return rsx! {},
        }
    };

    let contact_id = resolved_contact_id;
    let contact = state.contacts().into_iter().find(|c| c.id == contact_id);
    if contact.is_none() {
        return rsx! {};
    }
    let contact = contact.unwrap();

    let mut is_shaking = use_signal(|| false);
    let shake_class = if is_shaking() { "nudge-shake" } else { "" };

    let status_color = match contact.status {
        UserStatus::Online => "border-[#3cd070]",
        UserStatus::Ocupado => "border-[#e81123]",
        UserStatus::Ausente => "border-[#ffb900]",
        UserStatus::Offline | UserStatus::Invisivel => "border-slate-400",
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
            class: "w-full h-full flex flex-col select-none bg-bubbles {shake_class} overflow-hidden",
            style: "background: linear-gradient(180deg, rgba(230, 241, 252, 0.9) 0%, rgba(190, 215, 240, 0.85) 100%);",
            
            // Abas para chats ativos
            if contact_id_prop.is_none() && active_chats.len() > 1 {
                div { class: "h-8 bg-white/20 border-b border-white/10 flex items-center px-2 space-x-1 flex-shrink-0 overflow-x-auto",
                    for chat_id in active_chats {
                        if let Some(c) = state.contacts().into_iter().find(|c| c.id == chat_id) {
                            {
                                let is_active = chat_id == contact_id;
                                let active_tab_style = if is_active {
                                    "bg-white/80 border-[#7ba9d4] text-[#1e395b] font-bold"
                                } else {
                                    "bg-white/30 border-transparent text-[#2f4b6c]/80 hover:bg-white/50"
                                };
                                
                                rsx! {
                                    button {
                                        class: "px-3 h-6 flex items-center space-x-1.5 border rounded-t text-[11px] transition-all cursor-pointer truncate max-w-[120px] {active_tab_style}",
                                        onclick: move |_| {
                                            *state.selected_chat_id.write() = Some(chat_id);
                                        },
                                        div { class: "w-2 h-2 rounded-full {c.status.color_class()}" }
                                        span { "{c.display_name}" }
                                        span { 
                                            class: "text-[9px] text-slate-400 hover:text-red-500 font-bold ml-1.5",
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                state.close_chat(chat_id);
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

            // Painel Superior de Status (Informações do contato atual)
            div { class: "p-3 flex items-center space-x-3 bg-white/10 border-b border-white/20 flex-shrink-0 justify-between",
                div { class: "flex items-center space-x-3 min-w-0 flex-1",
                    
                    if contact_id_prop.is_none() {
                        button {
                            class: "md:hidden px-2 py-1 bg-white/30 hover:bg-white/50 border border-white/20 text-[#1e395b] text-[11px] rounded font-bold cursor-pointer mr-1 flex items-center space-x-0.5 flex-shrink-0 transition-colors",
                            title: "Voltar para Lista de Contatos",
                            onclick: move |_| {
                                *state.selected_chat_id.write() = None;
                            },
                            span { "←" }
                            span { "Contatos" }
                        }
                    }

                    // Avatar do cabeçalho com moldura fixa e badge de status clássico do MSN
                    div { 
                        class: "relative p-[2.5px] flex-shrink-0 shadow rounded-[8px] border border-[#a1c6e7] bg-white transition-all",
                        {render_avatar(contact.avatar_id, 36)}
                        
                        // Status Badge overlay
                        div { 
                            class: "absolute -bottom-0.5 -right-0.5 w-[13px] h-[13px] rounded-full bg-white border border-[#a1c6e7] flex items-center justify-center pointer-events-none z-10 shadow-sm",
                            div { class: "w-[7px] h-[7px] rounded-full {contact.status.color_class()} border border-black/10" }
                        }
                    }
                    
                    div { class: "flex-1 min-w-0 flex flex-col space-y-0.5",
                        div { class: "flex items-center space-x-2",
                            span { class: "font-bold text-sm text-[#1b324d] truncate", "{contact.display_name}" }
                            span { class: "text-[10px] px-1 py-0.1 {contact.status.color_class()} text-white rounded font-medium", "{contact.status.as_str()}" }
                        }
                        p { class: "text-xs text-[#3a5879]/85 italic truncate", "“{contact.personal_message}”" }
                    }
                }
                
                if contact_id_prop.is_none() {
                    button {
                        class: "w-6 h-6 flex items-center justify-center rounded hover:bg-white/40 border border-transparent hover:border-white/50 text-[#1e395b] cursor-pointer text-xs transition-colors flex-shrink-0",
                        title: "Desvincular conversa",
                        onclick: move |_| {
                            state.detach_chat(contact_id);
                            let dom = VirtualDom::new_with_props(
                                DetachedChatWindow,
                                DetachedChatWindowProps { contact_id }
                            );
                            
                            #[cfg(feature = "desktop")]
                            spawn(async move {
                                let _ = dioxus::desktop::window().new_window(
                                    dom,
                                    dioxus::desktop::Config::default().with_menu(None)
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

            // Layout Principal do Chat
            div { class: "flex-1 flex min-h-0 w-full",
                
                // Coluna Esquerda: Histórico e Input de texto
                div { class: "flex-1 flex flex-col sm:border-r border-white/20 min-w-0 h-full",
                    // Histórico do Chat (ChatFeed)
                    ChatFeed { contact_id, state }
                    
                    // Barra de formatação e Entrada de mensagem (ChatInput)
                    ChatInput {
                        contact_id,
                        state,
                        on_nudge: move |_| {
                            is_shaking.set(true);
                            spawn(async move {
                                tokio::time::sleep(std::time::Duration::from_millis(420)).await;
                                is_shaking.set(false);
                            });
                        }
                    }
                }
                
                // Coluna Direita: Avatars grandes de perfil ou Jogo da Velha (ChatSidebar)
                ChatSidebar { contact_id, state }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DetachedChatWindowProps {
    pub contact_id: usize,
}

#[component]
pub fn DetachedChatWindow(props: DetachedChatWindowProps) -> Element {
    #[cfg(feature = "desktop")]
    {
        let mut app_state = use_context_provider(|| AppState::new());
        let desktop = dioxus::desktop::use_window();
        
        // Garante login e seleção local
        *app_state.logged_in.write() = true;
        *app_state.selected_chat_id.write() = Some(props.contact_id);
        
        // Abre o chat localmente no estado da nova janela
        app_state.open_chat(props.contact_id);

        use_effect(move || {
            let mut state = app_state;
            state.load_initial_data();
        });

        // Sincronização periódica das mensagens com o banco de dados compartilhado
        use_effect(move || {
            let mut state = app_state;
            let c_id = props.contact_id;
            let desktop_clone = desktop.clone();
            spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                    
                    // Se o chat for acoplado, fecha esta janela
                    if let Ok(detached) = crate::services::db::DatabaseService::get_detached_chats().await {
                        if !detached.contains(&c_id) {
                            desktop_clone.close();
                            break;
                        }
                    }
                    
                    // Recarrega contatos se mudou
                    if let Ok(contacts) = crate::services::db::DatabaseService::load_contacts().await {
                        let current = state.contacts.read().clone();
                        if current != contacts {
                            *state.contacts.write() = contacts;
                        }
                    }
                    
                    // Recarrega mensagens se mudou
                    if let Ok(msgs) = crate::services::db::DatabaseService::load_messages(c_id).await {
                        let current = state.chat_messages.read().get(&c_id).cloned().unwrap_or_default();
                        if current != msgs {
                            let mut chat_msgs = state.chat_messages.write();
                            chat_msgs.insert(c_id, msgs);
                        }
                    }
                }
            });
        });

        let theme = app_state.theme();
        let contact = app_state.contacts().into_iter().find(|c| c.id == props.contact_id);
        let contact_name = contact.map(|c| c.display_name).unwrap_or_else(|| "Contato".to_string());

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
                            class: "w-full h-8 bg-gradient-to-b {theme.titlebar_gradient()} flex items-center justify-between z-50 flex-shrink-0 select-none border-b {theme.titlebar_border()} px-3 relative rounded-t-2xl shadow-sm cursor-default",
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
                                    class: "px-2.5 h-[22px] flex items-center justify-center bg-white border border-slate-300 rounded font-semibold text-[10px] text-slate-700 hover:text-sky-600 shadow-sm cursor-pointer transition-colors focus:outline-none",
                                    title: "Vincular de volta à janela principal do Skypia",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        spawn(async move {
                                            let _ = crate::services::db::DatabaseService::attach_chat(props.contact_id).await;
                                        });
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
                            class: "h-8 bg-gradient-to-b {theme.titlebar_gradient()} px-3 flex items-center justify-between text-[#1b324d] font-bold text-xs select-none border-b {theme.titlebar_border()}",
                            div { class: "flex items-center space-x-1.5 {theme.titlebar_text()}",
                                span { "💬" }
                                span { "Conversa com {contact_name}" }
                            }
                            button {
                                class: "px-2.5 h-[22px] flex items-center justify-center bg-white border border-slate-300 rounded font-semibold text-[10px] text-slate-700 hover:text-sky-600 shadow-sm cursor-pointer transition-colors focus:outline-none",
                                title: "Vincular de volta à janela principal do Skypia",
                                onclick: move |e| {
                                    e.stop_propagation();
                                    spawn(async move {
                                        let _ = crate::services::db::DatabaseService::attach_chat(props.contact_id).await;
                                    });
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
                                
                                ChatWindow { state: app_state, contact_id_prop: Some(props.contact_id) }
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
