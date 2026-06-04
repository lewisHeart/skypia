use dioxus::prelude::*;
use crate::models::render_avatar;
use crate::state::AppState;
use crate::sound::play_sound;

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

    let messages = state.chat_messages();
    let chat_history = messages.get(&contact_id).cloned().unwrap_or_default();

    let mut input_text = use_signal(|| String::new());
    let mut selected_font = use_signal(|| "Segoe UI".to_string());
    let mut selected_color = use_signal(|| "#000000".to_string());
    let mut is_shaking = use_signal(|| false);
    
    // UI Popovers
    let mut show_emoticon_panel = use_signal(|| false);
    let mut show_color_panel = use_signal(|| false);
    let mut show_font_panel = use_signal(|| false);

    // Parse MSN emoticon codes in message rendering
    let format_message_text = |text: &str| -> Element {
        let mut parsed = text.to_string();
        parsed = parsed.replace("(H)", "😎");
        parsed = parsed.replace("(Y)", "👍");
        parsed = parsed.replace("(N)", "👎");
        parsed = parsed.replace("(K)", "💋");
        parsed = parsed.replace("(A)", "😇");
        parsed = parsed.replace("(L)", "❤️");
        parsed = parsed.replace("(O)", "⏰");
        parsed = parsed.replace(":-D", "😀");
        parsed = parsed.replace(":-)", "🙂");
        parsed = parsed.replace(";-)", "😉");
        parsed = parsed.replace(":-(", "😢");
        parsed = parsed.replace(":-@", "😡");
        
        rsx! { span { "{parsed}" } }
    };

    // Simulated Auto-Reply Generator
    let generate_auto_reply = move |c_id: usize| {
        let replies = vec![
            "blz cara! dps entra no meu flogao pra ver as fotos da festa: flogao.com.br/goth_emo_2010",
            "pera ai, vo ali comer um trakinas e ja volto rsrs",
            "vc viu o video do jeremias na TV? mto engraçado kkkk o cão foi quem buto pra nois bebe!",
            "me cutuca dnv ae, gostei da tremida kkkkk",
            "vc tem o cd do linkin park ou do slipknot pra me passar dps por bluetooth?",
            "nossa minha net discada ta mto lenta hj, se eu cair eh pq minha mae tiro o telefone do gancho :(",
            "add meu orkut dps! procure por 'Gabii_Sz' q se me acha (L)",
            "mandei um winky ai p vc ver, mas acho q seu pc antigo n carrega kkkk",
            "vamos jogar habbo hotel ou tibia hj mais tarde?",
            "ok (Y)",
        ];
        
        let now_ms = chrono::Utc::now().timestamp_millis() as usize;
        let reply_idx = now_ms % replies.len();
        let text = replies[reply_idx].to_string();
        
        let mut app_state = state;
        spawn(async move {
            // Typing delay
            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
            app_state.receive_reply(
                c_id, 
                text, 
                "#e6007e".to_string(), // Nostalgic pink reply color
                "Comic Sans MS".to_string()
            );
            play_sound("message");
        });
    };

    // Trigger Nudge Shake
    let mut perform_shake = move || {
        is_shaking.set(true);
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(420)).await;
            is_shaking.set(false);
        });
    };

    // Send nudge handler
    let handle_send_nudge = move |_| {
        state.send_nudge(contact_id);
        play_sound("nudge");
        perform_shake();
        
        // Simulates contact replying with a nudge back after 2 seconds
        let mut app_state = state;
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
            app_state.receive_nudge(contact_id);
            play_sound("nudge");
            
            // Trigger window shaking again!
            is_shaking.set(true);
            tokio::time::sleep(std::time::Duration::from_millis(420)).await;
            is_shaking.set(false);
            
            // Send text reply following nudge
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            app_state.receive_reply(
                contact_id, 
                "Para de me tremer cara! KKKK custou pra eu arrumar meu monitor de tubo".to_string(), 
                "#e6007e".to_string(), 
                "Comic Sans MS".to_string()
            );
            play_sound("message");
        });
    };

    // Send Message Handler
    let mut handle_send = move || {
        let txt = input_text();
        if txt.trim().is_empty() {
            return;
        }
        
        state.send_message(contact_id, txt.clone(), selected_color(), selected_font());
        input_text.set(String::new());
        play_sound("message");
        
        // Trigger auto reply simulation
        generate_auto_reply(contact_id);
    };

    // Emoticon insertion helper
    let mut insert_emoticon = move |code: &str| {
        let current = input_text();
        input_text.set(format!("{}{}", current, code));
        show_emoticon_panel.set(false);
    };

    // Shake class dynamic resolution
    let shake_class = if is_shaking() { "nudge-shake" } else { "" };

    rsx! {
        div {
            class: "w-full h-full flex flex-col select-none bg-bubbles {shake_class}",
            style: "background: linear-gradient(180deg, rgba(230, 241, 252, 0.9) 0%, rgba(190, 215, 240, 0.85) 100%);",
            

            // Tab bar for active chats
            if contact_id_prop.is_none() && active_chats.len() > 1 {
                div { class: "h-8 bg-white/20 border-b border-white/10 flex items-center px-2 space-x-1 flex-shrink-0 overflow-x-auto",
                    for chat_id in active_chats {
                        {
                            if let Some(c) = state.contacts().into_iter().find(|c| c.id == chat_id) {
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
                            } else {
                                rsx! {}
                            }
                        }
                    }
                }
            }

            // Top Status Panel for current contact
            div { class: "p-3 flex items-center space-x-3 bg-white/10 border-b border-white/20 flex-shrink-0 justify-between",
                div { class: "flex items-center space-x-3 min-w-0 flex-1",
                    
                    // Botão Voltar (apenas visível em telas pequenas/mobile e se estiver integrado)
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

                    div { class: "avatar-frame bg-white flex-shrink-0 shadow",
                        {render_avatar(contact.avatar_id, 36)}
                    }
                    div { class: "flex-1 min-w-0 flex flex-col space-y-0.5",
                        div { class: "flex items-center space-x-2",
                            span { class: "font-bold text-sm text-[#1b324d] truncate", "{contact.display_name}" }
                            span { class: "text-[10px] px-1 py-0.1 {contact.status.color_class()} text-white rounded font-medium", "{contact.status.as_str()}" }
                        }
                        p { class: "text-xs text-[#3a5879]/85 italic truncate", "“{contact.personal_message}”" }
                    }
                }
                
                // Botão de desvincular chat (apenas no modo integrado)
                if contact_id_prop.is_none() {
                    button {
                        class: "w-6 h-6 flex items-center justify-center rounded hover:bg-white/40 border border-transparent hover:border-white/50 text-[#1e395b] cursor-pointer text-xs transition-colors flex-shrink-0",
                        title: "Desvincular conversa",
                        onclick: move |_| {
                            state.detach_chat(contact_id);
                            
                            // Cria o VirtualDom da nova janela
                            let dom = VirtualDom::new_with_props(
                                DetachedChatWindow,
                                DetachedChatWindowProps { contact_id }
                            );
                            
                            // Abre a nova janela nativa de desktop de forma assíncrona
                            spawn(async move {
                                let _ = dioxus::desktop::window().new_window(
                                    dom,
                                    dioxus::desktop::Config::default()
                                ).await;
                            });
                        },
                        "↗"
                    }
                }
            }

            // Main chat layout (History, Toolbar, Inputs on left; Avatars on right)
            div { class: "flex-1 flex min-h-0",
                
                // Left Column: Chat area
                div { class: "flex-1 flex flex-col sm:border-r border-white/20 min-w-0",
                    
                    // Messages History Log
                    div { 
                        class: "flex-1 overflow-y-auto p-4 space-y-3 bg-white/40",
                        
                        if chat_history.is_empty() {
                            div { class: "h-full flex items-center justify-center text-slate-400 text-xs italic",
                                "Inicie uma conversa nostálgica com {contact.display_name}!"
                            }
                        } else {
                            for msg in chat_history {
                                {
                                    let name_color = if msg.sender_id == 0 { "text-[#0066cc]" } else { "text-[#e6007e]" };
                                    
                                    rsx! {
                                        div { class: "flex flex-col space-y-0.5 text-xs text-slate-800 select-text",
                                            if msg.is_nudge {
                                                div { class: "py-1.5 px-3 bg-red-100/70 border border-red-200 rounded text-red-700 font-bold flex items-center space-x-2 my-1 animate-pulse shadow-sm",
                                                    span { "🔔" }
                                                    span { "{msg.text}" }
                                                    span { class: "text-[9px] text-red-500 font-normal ml-auto", "{msg.timestamp}" }
                                                }
                                            } else {
                                                div { class: "flex items-baseline space-x-1.5",
                                                    span { class: "font-bold {name_color}", "{msg.sender_name}" }
                                                    span { class: "text-[9px] text-slate-400 font-normal", "[{msg.timestamp}] diz:" }
                                                }
                                                p {
                                                    class: "pl-2 select-text",
                                                    style: "font-family: {msg.font_family}; color: {msg.font_color}; font-size: 13px;",
                                                    {format_message_text(&msg.text)}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Text Formatting & Action Toolbar
                    div { class: "h-8 bg-white/50 border-t border-b border-white/20 px-3 flex items-center justify-between text-xs text-[#2f4b6c] flex-shrink-0 relative",
                        div { class: "flex items-center space-x-3.5",
                            // Font selector
                            button { 
                                class: "hover:text-[#0066cc] cursor-pointer flex items-center space-x-0.5",
                                onclick: move |_| {
                                    show_font_panel.set(!show_font_panel());
                                    show_color_panel.set(false);
                                    show_emoticon_panel.set(false);
                                },
                                span { "A" }
                                span { class: "text-[8px]", "▼" }
                            }
                            
                            // Color selector
                            button { 
                                class: "hover:text-[#0066cc] cursor-pointer flex items-center space-x-0.5",
                                onclick: move |_| {
                                    show_color_panel.set(!show_color_panel());
                                    show_font_panel.set(false);
                                    show_emoticon_panel.set(false);
                                },
                                div { class: "w-3 h-3 border border-slate-400 rounded-sm bg-gradient-to-r from-red-500 via-green-500 to-blue-500" }
                                span { class: "text-[8px]", "▼" }
                            }

                            // Emoticons
                            button { 
                                class: "hover:text-[#0066cc] cursor-pointer flex items-center space-x-0.5",
                                onclick: move |_| {
                                    show_emoticon_panel.set(!show_emoticon_panel());
                                    show_font_panel.set(false);
                                    show_color_panel.set(false);
                                },
                                span { "☺" }
                                span { class: "text-[8px]", "▼" }
                            }

                            // Chamar atenção (Nudge)
                            button { 
                                class: "px-2 py-0.5 rounded hover:bg-slate-200 border border-slate-300 text-[10px] font-bold bg-white/70 shadow-sm text-red-600 flex items-center space-x-1 cursor-pointer nudge-btn-hover active:scale-95 transition-transform",
                                title: "Chamar a Atenção",
                                onclick: handle_send_nudge,
                                span { "🔔" }
                                span { "Chamar Atenção" }
                            }
                        }

                        // Floating Font Selector Panel
                        if show_font_panel() {
                            div { class: "absolute left-2 bottom-9 w-36 bg-white border border-slate-300 rounded shadow-lg z-50 p-1 flex flex-col text-xs",
                                for font_name in &["Segoe UI", "Comic Sans MS", "Arial", "Courier New"] {
                                    button {
                                        class: "px-2 py-1 text-left hover:bg-sky-100 rounded transition-colors",
                                        style: "font-family: {font_name};",
                                        onclick: move |_| {
                                            selected_font.set(font_name.to_string());
                                            show_font_panel.set(false);
                                        },
                                        "{font_name}"
                                    }
                                }
                            }
                        }

                        // Floating Color Selector Panel
                        if show_color_panel() {
                            div { class: "absolute left-8 bottom-9 w-32 bg-white border border-slate-300 rounded shadow-lg z-50 p-2 grid grid-cols-4 gap-1.5",
                                for color in &["#000000", "#0066cc", "#e6007e", "#2e6930", "#e81123", "#ffb900", "#7a7a7a", "#8e24aa"] {
                                    div {
                                        class: "w-5 h-5 rounded cursor-pointer border border-slate-300 hover:scale-110 hover:shadow transition-transform",
                                        style: "background-color: {color};",
                                        onclick: move |_| {
                                            selected_color.set(color.to_string());
                                            show_color_panel.set(false);
                                        }
                                    }
                                }
                            }
                        }

                        // Floating Emoticon Panel
                        if show_emoticon_panel() {
                            div { class: "absolute left-16 bottom-9 w-44 bg-white border border-slate-300 rounded shadow-lg z-50 p-2 grid grid-cols-4 gap-2 text-base",
                                for (code, icon) in &[
                                    ("(H)", "😎"), ("(Y)", "👍"), ("(N)", "👎"), ("(K)", "💋"),
                                    ("(A)", "😇"), ("(L)", "❤️"), ("(O)", "⏰"), (":-D", "😀"),
                                    (":-)", "🙂"), (";-)", "😉"), (":-(", "😢"), (":-@", "😡")
                                ] {
                                    button {
                                        class: "hover:bg-slate-100 p-1 rounded flex items-center justify-center transition-colors cursor-pointer",
                                        title: code,
                                        onclick: move |_| insert_emoticon(code),
                                        "{icon}"
                                    }
                                }
                            }
                        }
                    }

                    // Chat message input area
                    div { class: "h-20 bg-white border-t border-white/20 p-2 flex space-x-2 flex-shrink-0",
                        textarea {
                            class: "flex-1 resize-none p-1.5 text-xs msn-input rounded",
                            style: "font-family: {selected_font()}; color: {selected_color()};",
                            placeholder: "Digite sua mensagem aqui...",
                            value: "{input_text}",
                            oninput: move |e| input_text.set(e.value()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter && !e.modifiers().shift() {
                                    e.prevent_default();
                                    handle_send();
                                }
                            }
                        }
                        
                        button {
                            class: "w-16 h-full bg-gradient-to-b from-[#8fc1e9] via-[#5c98d6] to-[#4585c5] hover:from-[#9bd0fa] hover:via-[#70abeb] hover:to-[#579adf] text-white border border-[#4074a8] rounded font-bold text-xs shadow cursor-pointer flex items-center justify-center active:scale-95 transition-transform",
                            onclick: move |_| handle_send(),
                            "Enviar"
                        }
                    }
                }

                // Right Column: Avatars display panel (nostalgic layouts!)
                div { class: "hidden sm:flex w-28 flex-col items-center justify-between p-3 bg-white/10 flex-shrink-0",
                    
                    // Contact's avatar frame
                    div { class: "flex flex-col items-center space-y-1.5",
                        div { class: "avatar-frame bg-white shadow-md relative",
                            {render_avatar(contact.avatar_id, 64)}
                            // Overlay status dot
                            div { class: "absolute -bottom-1 -right-1 w-4 h-4 rounded-full bg-white border border-[#a6b9cd] flex items-center justify-center shadow-sm",
                                div { class: "w-2.5 h-2.5 rounded-full {contact.status.color_class()} border border-black/10" }
                            }
                        }
                        span { class: "text-[10px] text-slate-500 font-bold max-w-[85px] truncate text-center", "{contact.display_name}" }
                    }

                    // User's own avatar frame
                    div { class: "flex flex-col items-center space-y-1.5",
                        div { class: "avatar-frame bg-white shadow-md relative",
                            {render_avatar(state.user_avatar_id(), 64)}
                            div { class: "absolute -bottom-1 -right-1 w-4 h-4 rounded-full bg-white border border-[#a6b9cd] flex items-center justify-center shadow-sm",
                                div { class: "w-2.5 h-2.5 rounded-full {state.user_status().color_class()} border border-black/10" }
                            }
                        }
                        span { class: "text-[10px] text-slate-500 font-bold max-w-[85px] truncate text-center", "Você" }
                    }
                }
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
    let mut app_state = use_context_provider(|| AppState::new());
    let desktop = dioxus::desktop::use_window();
    
    // Garante login e seleção local
    *app_state.logged_in.write() = true;
    *app_state.selected_chat_id.write() = Some(props.contact_id);
    
    // Abre o chat localmente no estado da nova janela
    app_state.open_chat(props.contact_id);

    // Carrega os dados iniciais assincronamente
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
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                
                // Se o chat for acoplado (removido do banco compartilhado de destacados), fecha esta janela
                if let Ok(detached) = crate::services::db::DatabaseService::get_detached_chats().await {
                    if !detached.contains(&c_id) {
                        desktop_clone.close();
                        break;
                    }
                }
                
                // Recarrega contatos
                if let Ok(contacts) = crate::services::db::DatabaseService::load_contacts().await {
                    *state.contacts.write() = contacts;
                }
                
                // Recarrega mensagens
                if let Ok(msgs) = crate::services::db::DatabaseService::load_messages(c_id).await {
                    let mut chat_msgs = state.chat_messages.write();
                    chat_msgs.insert(c_id, msgs);
                }
            }
        });
    });

    let theme = app_state.theme();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        
        div { 
            class: "w-screen h-screen overflow-hidden flex bg-gradient-to-br {theme.bg_gradient()} relative font-segoe select-none",
            
            // Subtle theme background bubbles
            div { class: "absolute inset-0 bg-bubbles pointer-events-none opacity-25 z-0" }

            div { class: "w-full h-full flex flex-col pointer-events-auto z-10",
                
                // Barra de Controle personalizada para Acoplar de volta
                div { 
                    class: "h-9 bg-gradient-to-r from-[#8fc1e9] via-[#5c98d6] to-[#4585c5] px-3 flex items-center justify-between text-white font-bold text-xs select-none border-b border-[#4074a8]/50",
                    div { class: "flex items-center space-x-1.5",
                        span { "💬" }
                        span { "Conversa Desvinculada" }
                    }
                    button {
                        class: "px-2 h-6 flex items-center justify-center bg-white/20 hover:bg-white/35 rounded text-[10px] text-white border border-white/20 cursor-pointer transition-all space-x-1",
                        title: "Vincular de volta à janela principal do MSN",
                        onclick: move |_| {
                            spawn(async move {
                                let _ = crate::services::db::DatabaseService::attach_chat(props.contact_id).await;
                            });
                        },
                        span { "↙" }
                        span { "Acoplar" }
                    }
                }
                
                div { class: "flex-1 min-h-0 min-w-0",
                    ChatWindow { state: app_state, contact_id_prop: Some(props.contact_id) }
                }
            }
        }
    }
}
