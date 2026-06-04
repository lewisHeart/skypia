use dioxus::prelude::*;
use crate::state::AppState;
use crate::sound::play_sound;

#[component]
pub fn ChatInput(
    contact_id: usize,
    mut state: AppState,
    on_nudge: EventHandler<()>,
) -> Element {
    let mut input_text = use_signal(|| String::new());
    let mut selected_font = use_signal(|| "Segoe UI".to_string());
    let mut selected_color = use_signal(|| "#1e395b".to_string());
    
    // UI Popovers
    let mut show_emoticon_panel = use_signal(|| false);
    let mut show_color_panel = use_signal(|| false);
    let mut show_font_panel = use_signal(|| false);
    let mut show_wink_panel = use_signal(|| false);
    let mut show_file_panel = use_signal(|| false);
    
    let typings = state.typing_contacts();
    let is_typing_srv = typings.get(&contact_id).map(|list: &Vec<usize>| list.contains(&contact_id)).unwrap_or(false);

    // Efeito de debounce para notificar o servidor que o usuário está digitando
    use_effect(move || {
        let txt = input_text();
        let mut state = state;
        if !txt.trim().is_empty() {
            state.set_typing(contact_id, true);
            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
                state.set_typing(contact_id, false);
            });
        } else {
            state.set_typing(contact_id, false);
        }
    });
 
    let contact = state.contacts().into_iter().find(|c| c.id == contact_id);
    if contact.is_none() {
        return rsx! {};
    }
    let contact = contact.unwrap();

    // Send Message Handler
    let mut handle_send = move || {
        let txt = input_text();
        if txt.trim().is_empty() {
            return;
        }
        
        state.send_message(contact_id, txt.clone(), selected_color(), selected_font());
        input_text.set(String::new());
        play_sound("message");
    };

    // Send nudge handler
    let handle_send_nudge = move |_| {
        state.send_nudge(contact_id);
        play_sound("nudge");
        on_nudge.call(());
    };


    // Helper to insert emoticons at text cursor
    let mut insert_emoticon = move |code: &str| {
        input_text.set(format!("{}{}", input_text(), code));
        show_emoticon_panel.set(false);
    };

    rsx! {
        div { class: "flex flex-col flex-shrink-0 relative z-20",
            // Formatting & Action Toolbar
            div { class: "h-8 bg-white/50 border-t border-b border-white/20 px-3 flex items-center justify-between text-xs text-[#2f4b6c] relative",
                div { class: "flex items-center space-x-3.5",
                    button { 
                        class: "hover:text-[#0066cc] cursor-pointer flex items-center space-x-0.5 transition-colors",
                        onclick: move |_| {
                            show_font_panel.set(!show_font_panel());
                            show_color_panel.set(false);
                            show_emoticon_panel.set(false);
                            show_wink_panel.set(false);
                            show_file_panel.set(false);
                        },
                        span { "A" }
                        span { class: "text-[8px]", "▼" }
                    }
                    
                    button { 
                        class: "hover:text-[#0066cc] cursor-pointer flex items-center space-x-0.5 transition-colors",
                        onclick: move |_| {
                            show_color_panel.set(!show_color_panel());
                            show_font_panel.set(false);
                            show_emoticon_panel.set(false);
                            show_wink_panel.set(false);
                            show_file_panel.set(false);
                        },
                        div { class: "w-3 h-3 border border-slate-400 rounded-sm bg-gradient-to-r from-red-500 via-green-500 to-blue-500" }
                        span { class: "text-[8px]", "▼" }
                    }

                    button { 
                        class: "hover:text-[#0066cc] cursor-pointer flex items-center space-x-0.5 transition-colors",
                        onclick: move |_| {
                            show_emoticon_panel.set(!show_emoticon_panel());
                            show_font_panel.set(false);
                            show_color_panel.set(false);
                            show_wink_panel.set(false);
                            show_file_panel.set(false);
                        },
                        span { "☺" }
                        span { class: "text-[8px]", "▼" }
                    }

                    button { 
                        class: "hover:text-[#0066cc] cursor-pointer flex items-center space-x-0.5 font-semibold transition-colors",
                        onclick: move |_| {
                            show_wink_panel.set(!show_wink_panel());
                            show_font_panel.set(false);
                            show_color_panel.set(false);
                            show_emoticon_panel.set(false);
                            show_file_panel.set(false);
                        },
                        span { "✨" }
                        span { "Piscadelas" }
                        span { class: "text-[8px]", "▼" }
                    }

                    button { 
                        class: "hover:text-[#0066cc] cursor-pointer flex items-center space-x-0.5 font-semibold transition-colors",
                        onclick: move |_| {
                            show_file_panel.set(!show_file_panel());
                            show_wink_panel.set(false);
                            show_font_panel.set(false);
                            show_color_panel.set(false);
                            show_emoticon_panel.set(false);
                        },
                        span { "📂" }
                        span { "Arquivos" }
                        span { class: "text-[8px]", "▼" }
                    }

                    button { 
                        class: "hover:text-[#0066cc] cursor-pointer flex items-center space-x-0.5 font-semibold transition-colors",
                        onclick: move |_| {
                            state.start_game(contact_id);
                            show_file_panel.set(false);
                            show_wink_panel.set(false);
                            show_font_panel.set(false);
                            show_color_panel.set(false);
                            show_emoticon_panel.set(false);
                        },
                        span { "🎮" }
                        span { "Jogos" }
                    }

                    button { 
                        class: "px-2 py-0.5 rounded hover:bg-slate-200 border border-slate-300 text-[10px] font-bold bg-white/70 shadow-sm text-red-600 flex items-center space-x-1 cursor-pointer nudge-btn-hover active:scale-95 transition-transform",
                        title: "Chamar a Atenção",
                        onclick: handle_send_nudge,
                        span { "🔔" }
                        span { "Chamar Atenção" }
                    }
                }

                // POPOVERS RENDER
                if show_font_panel() {
                    div { class: "absolute left-2 bottom-9 w-36 bg-white border border-slate-300 rounded shadow-lg z-50 p-1 flex flex-col text-xs",
                        for font_name in &["Segoe UI", "Comic Sans MS", "Arial", "Courier New"] {
                            button {
                                class: "px-2 py-1 text-left hover:bg-sky-100 rounded transition-colors cursor-pointer",
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

                if show_wink_panel() {
                    div { class: "absolute left-24 bottom-9 w-40 bg-white border border-slate-300 rounded shadow-lg z-50 p-1 flex flex-col text-xs text-slate-700",
                        button {
                            class: "px-2 py-1 text-left hover:bg-purple-100 rounded transition-colors flex items-center space-x-1.5 cursor-pointer",
                            onclick: move |_| {
                                state.send_wink(contact_id, "kiss".to_string());
                                show_wink_panel.set(false);
                            },
                            span { "💋" }
                            span { "Beijo de Batom" }
                        }
                        button {
                            class: "px-2 py-1 text-left hover:bg-purple-100 rounded transition-colors flex items-center space-x-1.5 cursor-pointer",
                            onclick: move |_| {
                                state.send_wink(contact_id, "hammer".to_string());
                                show_wink_panel.set(false);
                            },
                            span { "🔨" }
                            span { "Martelada na Tela" }
                        }
                        button {
                            class: "px-2 py-1 text-left hover:bg-purple-100 rounded transition-colors flex items-center space-x-1.5 cursor-pointer",
                            onclick: move |_| {
                                state.send_wink(contact_id, "pig".to_string());
                                show_wink_panel.set(false);
                            },
                            span { "🐷" }
                            span { "Porco Dançarino" }
                        }
                    }
                }

                if show_file_panel() {
                    div { class: "absolute left-36 bottom-9 w-40 bg-white border border-slate-300 rounded shadow-lg z-50 p-1 flex flex-col text-xs text-slate-700",
                        button {
                            class: "px-2 py-1 text-left hover:bg-sky-100 rounded transition-colors flex items-center space-x-1.5 cursor-pointer",
                            onclick: move |_| {
                                state.send_file_transfer(contact_id, "foto_flogao_2010.jpg".to_string());
                                show_file_panel.set(false);
                            },
                            span { "🖼️" }
                            span { "Enviar Foto (.jpg)" }
                        }
                        button {
                            class: "px-2 py-1 text-left hover:bg-sky-100 rounded transition-colors flex items-center space-x-1.5 cursor-pointer",
                            onclick: move |_| {
                                state.send_file_transfer(contact_id, "musica_emo.mp3".to_string());
                                show_file_panel.set(false);
                            },
                            span { "🎵" }
                            span { "Enviar Música (.mp3)" }
                        }
                        button {
                            class: "px-2 py-1 text-left hover:bg-sky-100 rounded transition-colors flex items-center space-x-1.5 cursor-pointer",
                            onclick: move |_| {
                                state.send_file_transfer(contact_id, "jogo_habbo.exe".to_string());
                                show_file_panel.set(false);
                            },
                            span { "💾" }
                            span { "Enviar Programa (.exe)" }
                        }
                    }
                }
            }

            // Chat message input area
            div { class: "h-20 bg-white border-t border-white/20 p-2 flex flex-col justify-between relative",
                if is_typing_srv {
                    div { class: "absolute -top-5 left-2 h-5 text-[10px] text-slate-500 italic flex items-center space-x-1 animate-pulse z-10 bg-white/60 px-2 rounded-t border-t border-l border-r border-slate-200/50",
                        span { "✍️" }
                        span { "{contact.display_name} está digitando..." }
                    }
                }
                div { class: "flex-1 flex space-x-2 w-full",
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
        }
    }
}
