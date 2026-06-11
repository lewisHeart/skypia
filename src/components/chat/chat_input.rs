use crate::sound::play_sound;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn ChatInput(contact_id: String, mut state: AppState, on_nudge: EventHandler<()>) -> Element {
    let theme = state.theme();
    let mut input_text = use_signal(|| String::new());
    let mut selected_font = use_signal(|| "Segoe UI".to_string());
    let mut selected_color = use_signal(|| "#1e395b".to_string());

    // UI Popovers
    let mut show_emoticon_panel = use_signal(|| false);
    let mut show_font_panel = use_signal(|| false);
    let mut show_wink_panel = use_signal(|| false);
    let mut show_file_panel = use_signal(|| false);

    let typings = state.typing_contacts();
    let users_typing = typings.get(&contact_id).cloned().unwrap_or_default();
    let is_typing_srv = !users_typing.is_empty();

    let typing_name = if is_typing_srv {
        let first_user_id = &users_typing[0];
        if let Some(c) = state.contacts().iter().find(|c| c.id == *first_user_id) {
            c.nickname.clone().unwrap_or_else(|| c.display_name.clone())
        } else {
            "Alguém".to_string()
        }
    } else {
        String::new()
    };

    // Efeito de debounce para notificar o servidor que o usuário está digitando
    let contact_id_clone = contact_id.clone();
    use_effect(move || {
        let txt = input_text();
        let mut state = state;
        let cid = contact_id_clone.clone();
        if !txt.trim().is_empty() {
            state.set_typing(cid.clone(), true);
            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
                state.set_typing(cid, false);
            });
        } else {
            state.set_typing(cid, false);
        }
    });

    let contact = state.contacts().into_iter().find(|c| c.id == contact_id);
    let group = (state.group_chats)()
        .into_iter()
        .find(|g| g.id == contact_id);
    if contact.is_none() && group.is_none() {
        return rsx! {};
    }
    let is_group = group.is_some();

    // Send nudge handler
    let contact_id_nudge = contact_id.clone();
    let handle_send_nudge = move |_| {
        state.send_nudge(contact_id_nudge.clone());
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
            // Overlay invisível para fechar popovers ao clicar fora
            if show_font_panel() || show_emoticon_panel() || show_wink_panel() || show_file_panel() {
                div {
                    class: "fixed inset-0 z-40 bg-transparent cursor-default",
                    onclick: move |e| {
                        e.stop_propagation();
                        show_font_panel.set(false);
                        show_emoticon_panel.set(false);
                        show_wink_panel.set(false);
                        show_file_panel.set(false);
                    }
                }
            }

            // Barra de Ações e Formatação Clássica do MSN
            div { class: "h-8 bg-transparent px-3 flex items-center justify-between text-xs {theme.titlebar_text()} relative select-none",
                // Lado Esquerdo: Emojis, Winks, Compartilhamento, Sino de Atenção
                div { class: "flex items-center space-x-2.5",
                    // Emojis
                    button {
                        class: "hover:bg-white/80 p-1 rounded cursor-pointer flex items-center space-x-1.5 transition-colors focus:outline-none",
                        title: "Emojis",
                        onclick: move |_| {
                            show_emoticon_panel.set(!show_emoticon_panel());
                            show_font_panel.set(false);
                            show_wink_panel.set(false);
                            show_file_panel.set(false);
                        },
                        img {
                            src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Emoji/SVG/ic_fluent_emoji_24_regular.svg",
                            class: "w-5 h-5 select-none pointer-events-none"
                        }
                        span { class: "text-[8px] text-slate-600 select-none", "▼" }
                    }

                    // Piscadelas (Winks)
                    button {
                        class: "hover:bg-white/80 p-1 rounded cursor-pointer flex items-center space-x-1.5 transition-colors focus:outline-none",
                        title: "Piscadelas (Winks)",
                        onclick: move |_| {
                            show_wink_panel.set(!show_wink_panel());
                            show_font_panel.set(false);
                            show_emoticon_panel.set(false);
                            show_file_panel.set(false);
                        },
                        img {
                            src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Gift/SVG/ic_fluent_gift_24_color.svg",
                            class: "w-5 h-5 select-none pointer-events-none"
                        }
                        span { class: "text-[8px] text-slate-600 select-none", "▼" }
                    }

                    // Enviar Arquivo (Pasta)
                    button {
                        class: "hover:bg-white/80 p-1 rounded cursor-pointer flex items-center space-x-1.5 transition-colors focus:outline-none",
                        title: "Enviar Arquivo",
                        onclick: move |_| {
                            show_file_panel.set(!show_file_panel());
                            show_wink_panel.set(false);
                            show_font_panel.set(false);
                            show_emoticon_panel.set(false);
                        },
                        img {
                            src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Document%20Folder/SVG/ic_fluent_document_folder_24_color.svg",
                            class: "w-5 h-5 select-none pointer-events-none"
                        }
                        span { class: "text-[8px] text-slate-600 select-none", "▼" }
                    }

                    // Chamar a Atenção (Nudge)
                    button {
                        class: "hover:bg-white/80 p-1 rounded cursor-pointer flex items-center transition-colors focus:outline-none",
                        title: "Chamar a Atenção (Nudge)",
                        onclick: handle_send_nudge,
                        img {
                            src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Alert/SVG/ic_fluent_alert_24_color.svg",
                            class: "w-5 h-5 select-none pointer-events-none"
                        }
                        span { class: "text-[11px] text-[#2b3e51] font-semibold ml-1.5", "Chamar atenção" }
                    }

                    // Skypia Jogos
                    if !is_group {
                        button {
                            class: "hover:bg-white/80 p-1 rounded cursor-pointer flex items-center transition-colors focus:outline-none",
                            title: "Skypia Jogos",
                            onclick: move |_| {
                                state.show_games_modal.set(true);
                                show_file_panel.set(false);
                                show_wink_panel.set(false);
                                show_font_panel.set(false);
                                show_emoticon_panel.set(false);
                            },
                            img {
                                src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Game%20Chat/SVG/ic_fluent_game_chat_20_color.svg",
                                class: "w-5 h-5 select-none pointer-events-none"
                            }
                        }
                    }
                }

                // Lado Direito: Estilo de Fonte
                div { class: "flex items-center",
                    button {
                        class: "hover:bg-white/80 p-1 rounded cursor-pointer flex items-center space-x-1.5 transition-colors focus:outline-none",
                        title: "Fonte e Cor",
                        onclick: move |_| {
                            show_font_panel.set(!show_font_panel());
                            show_emoticon_panel.set(false);
                            show_wink_panel.set(false);
                            show_file_panel.set(false);
                        },
                        svg {
                            class: "w-5 h-5 select-none pointer-events-none",
                            view_box: "0 0 24 24",
                            text { x: "4", y: "15", font_family: "Arial", font_weight: "bold", font_size: "14", fill: "#2b3e51", "A" }
                            rect { x: "3", y: "17", width: "18", height: "3", fill: "url(#textGradient)" }
                            defs {
                                linearGradient { id: "textGradient", x1: "0", y1: "0", x2: "1", y2: "0",
                                    stop { offset: "0%", stop_color: "#eb4824" }
                                    stop { offset: "50%", stop_color: "#ffcd0f" }
                                    stop { offset: "100%", stop_color: "#3b82f6" }
                                }
                            }
                        }
                        span { class: "text-[8px] text-slate-600 select-none", "▼" }
                    }
                }

                // POPOVERS RENDER
                if show_font_panel() {
                    div { class: "absolute left-2 bottom-9 w-44 bg-white border {theme.titlebar_border()} rounded shadow-lg z-50 p-2.5 flex flex-col space-y-2.5 text-xs text-slate-700",
                        div { class: "flex flex-col space-y-1",
                            span { class: "font-bold text-[10px] text-slate-400", "Fonte" }
                            div { class: "flex flex-col space-y-0.5",
                                for font_name in &["Segoe UI", "Comic Sans MS", "Arial", "Courier New"] {
                                    button {
                                        class: "px-2 py-1 text-left hover:bg-black/5 rounded transition-colors cursor-pointer {theme.titlebar_text()}",
                                        style: "font-family: {font_name};",
                                        onclick: move |_| {
                                            selected_font.set(font_name.to_string());
                                        },
                                        span {
                                            class: if selected_font() == *font_name { "font-bold" } else { "" },
                                            "{font_name}"
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "border-t border-slate-100" }

                        div { class: "flex flex-col space-y-1",
                            span { class: "font-bold text-[10px] text-slate-400", "Cor" }
                            div { class: "grid grid-cols-4 gap-1.5",
                                for color in &["#000000", "#0066cc", "#e6007e", "#2e6930", "#e81123", "#ffb900", "#7a7a7a", "#8e24aa"] {
                                    div {
                                        class: "w-6 h-6 rounded cursor-pointer border {theme.titlebar_border()} hover:scale-110 hover:shadow transition-all flex items-center justify-center relative",
                                        style: "background-color: {color};",
                                        onclick: move |_| {
                                            selected_color.set(color.to_string());
                                        },
                                        if selected_color() == *color {
                                            span { class: "text-white text-[9px] font-bold drop-shadow", "✓" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if show_emoticon_panel() {
                    div {
                        class: "absolute left-10 bottom-9 w-60 max-h-52 overflow-y-auto bg-white border {theme.titlebar_border()} rounded shadow-lg z-50 p-2 grid grid-cols-6 gap-1 text-base scrollbar-thin scrollbar-thumb-slate-300",
                        for (code, emoji_name) in &[
                            (":-)", "slightly-smiling-face"),
                            (":-D", "grinning-face-with-big-eyes"),
                            (";-)", "winking-face"),
                            (":-P", "face-with-tongue"),
                            (":-O", "face-with-open-mouth"),
                            (":-$", "flushed-face"),
                            (":-@", "pouting-face"),
                            (":-S", "confused-face"),
                            ("(H)", "smiling-face-with-sunglasses"),
                            ("(Y)", "thumbs-up"),
                            ("(N)", "thumbs-down"),
                            ("(K)", "kiss-mark"),
                            ("(A)", "smiling-face-with-halo"),
                            ("(L)", "red-heart"),
                            ("(U)", "broken-heart"),
                            ("(O)", "alarm-clock"),
                            ("(G)", "wrapped-gift"),
                            ("(F)", "wilted-flower"),
                            ("(P)", "camera"),
                            ("(M)", "musical-note"),
                            ("(S)", "crescent-moon"),
                            ("(*)", "star"),
                            ("(E)", "envelope"),
                            ("(C)", "hot-beverage"),
                            ("(HE)", "smiling-face-with-heart-eyes"),
                            ("(BK)", "face-blowing-a-kiss"),
                            ("(ST)", "squinting-face-with-tongue"),
                            ("(ZF)", "zany-face"),
                            ("(SF)", "shushing-face"),
                            ("(TF)", "thinking-face"),
                            ("(EF)", "expressionless-face"),
                            ("(SM)", "smirking-face"),
                            ("(GF)", "grimacing-face"),
                            ("(DF)", "drooling-face"),
                            ("(SL)", "sleeping-face"),
                            ("(NF)", "nauseated-face"),
                            ("(VOM)", "face-vomiting"),
                            ("(EH)", "exploding-head"),
                            ("(PF)", "partying-face"),
                            ("(WF)", "woozy-face"),
                            ("(;_;)", "crying-face"),
                            ("(LCF)", "loudly-crying-face"),
                            ("(SCR)", "face-screaming-in-fear"),
                            ("(ANG)", "angry-face"),
                            ("(FSM)", "face-with-symbols-on-mouth"),
                            ("(SK)", "skull"),
                            ("(POO)", "pile-of-poo"),
                            ("(CLAP)", "clapping-hands"),
                            ("(HS)", "handshake"),
                            ("(VIC)", "victory-hand"),
                            ("(FLEX)", "flexed-biceps"),
                            ("(FOLD)", "folded-hands"),
                            ("(BR)", "brain"),
                            ("(FIRE)", "fire"),
                            ("(BOOM)", "collision"),
                            ("(SPARKS)", "sparkles"),
                            ("(BAL)", "balloon"),
                            ("(POP)", "party-popper"),
                            ("(RAIN)", "rainbow"),
                            ("(SUN)", "sun"),
                            ("(SNOW)", "snowflake"),
                            ("(UMB)", "umbrella"),
                            ("(DOG)", "dog-face"),
                            ("(CAT)", "cat-face"),
                            ("(PANDA)", "panda"),
                            ("(ALIEN)", "alien"),
                            ("(ROCKET)", "rocket"),
                            ("(PLANE)", "airplane"),
                            ("(BEER)", "beer-mug"),
                            ("(PIZZA)", "pizza"),
                            ("(MONEY)", "money-bag"),
                            ("(TROPHY)", "trophy"),
                        ] {
                            button {
                                class: "hover:bg-slate-100 p-0.5 rounded flex items-center justify-center transition-colors cursor-pointer",
                                title: code,
                                onclick: move |_| insert_emoticon(code),
                                img {
                                    src: "https://registry.npmmirror.com/@lobehub/assets-emoji-anim/latest/files/assets/{emoji_name}.webp",
                                    class: "w-6 h-6 object-contain pointer-events-none"
                                }
                            }
                        }
                    }
                }

                if show_wink_panel() {
                    div { class: "absolute left-20 bottom-9 w-40 bg-white border {theme.titlebar_border()} rounded shadow-lg z-50 p-1 flex flex-col text-xs text-slate-700",
                        button {
                            class: "px-2 py-1 text-left hover:bg-black/5 rounded transition-colors flex items-center space-x-1.5 cursor-pointer {theme.titlebar_text()}",
                            onclick: {
                                let cid = contact_id.clone();
                                move |_| {
                                    state.send_wink(cid.clone(), "kiss".to_string());
                                    show_wink_panel.set(false);
                                }
                            },
                            img {
                                src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/kiss-mark.webp",
                                class: "w-4 h-4 object-contain pointer-events-none"
                            }
                            span { "Beijo de Batom" }
                        }
                        button {
                            class: "px-2 py-1 text-left hover:bg-black/5 rounded transition-colors flex items-center space-x-1.5 cursor-pointer {theme.titlebar_text()}",
                            onclick: {
                                let cid = contact_id.clone();
                                move |_| {
                                    state.send_wink(cid.clone(), "hammer".to_string());
                                    show_wink_panel.set(false);
                                }
                            },
                            img {
                                src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/hammer.webp",
                                class: "w-4 h-4 object-contain pointer-events-none"
                            }
                            span { "Martelada na Tela" }
                        }
                        button {
                            class: "px-2 py-1 text-left hover:bg-black/5 rounded transition-colors flex items-center space-x-1.5 cursor-pointer {theme.titlebar_text()}",
                            onclick: {
                                let cid = contact_id.clone();
                                move |_| {
                                    state.send_wink(cid.clone(), "pig".to_string());
                                    show_wink_panel.set(false);
                                }
                            },
                            img {
                                src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/pig-face.webp",
                                class: "w-4 h-4 object-contain pointer-events-none"
                            }
                            span { "Porco Dançarino" }
                        }
                    }
                }

                if show_file_panel() {
                    div { class: "absolute left-32 bottom-9 w-40 bg-white border {theme.titlebar_border()} rounded shadow-lg z-50 p-1 flex flex-col text-xs text-slate-700",
                        button {
                            class: "px-2 py-1 text-left hover:bg-black/5 rounded transition-colors flex items-center space-x-1.5 cursor-pointer {theme.titlebar_text()}",
                            onclick: {
                                let cid = contact_id.clone();
                                move |_| {
                                    state.send_file_transfer(cid.clone(), "foto_flogao_2010.jpg".to_string());
                                    show_file_panel.set(false);
                                }
                            },
                            img {
                                src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/framed-picture.webp",
                                class: "w-4 h-4 object-contain pointer-events-none"
                            }
                            span { "Enviar Foto" }
                        }
                        button {
                            class: "px-2 py-1 text-left hover:bg-black/5 rounded transition-colors flex items-center space-x-1.5 cursor-pointer {theme.titlebar_text()}",
                            onclick: {
                                let cid = contact_id.clone();
                                move |_| {
                                    state.send_file_transfer(cid.clone(), "musica_emo.mp3".to_string());
                                    show_file_panel.set(false);
                                }
                            },
                            img {
                                src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/musical-note.webp",
                                class: "w-4 h-4 object-contain pointer-events-none"
                            }
                            span { "Enviar Música" }
                        }
                        button {
                            class: "px-2 py-1 text-left hover:bg-black/5 rounded transition-colors flex items-center space-x-1.5 cursor-pointer {theme.titlebar_text()}",
                            onclick: {
                                let cid = contact_id.clone();
                                move |_| {
                                    state.send_file_transfer(cid.clone(), "jogo_habbo.exe".to_string());
                                    show_file_panel.set(false);
                                }
                            },
                            img {
                                src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/floppy-disk.webp",
                                class: "w-4 h-4 object-contain pointer-events-none"
                            }
                            span { "Enviar Arquivo" }
                        }
                    }
                }
            }

            // Chat message input area
            div { class: "h-[85px] bg-transparent py-[12.5px] px-[14px] flex flex-col justify-between relative",
                if is_typing_srv {
                    div { class: "absolute -top-5 left-2 h-5 text-[10px] text-slate-500 italic flex items-center space-x-1 animate-pulse z-10 bg-[#eff5fb] px-2 rounded-t border-t border-l border-r border-[#96badb]",
                        span { "✍️" }
                        span { "{typing_name} está digitando..." }
                    }
                }
                 div { class: "flex-1 flex space-x-2.5 w-full items-center",
                    textarea {
                        class: "flex-1 h-[60px] resize-none p-1.5 text-xs msn-input rounded-none border-2 border-[#d1d1d1] placeholder-[#a5a5a5] placeholder:text-[10px] focus:outline-none focus:border-slate-400",
                        style: "font-family: {selected_font()}; color: {selected_color()};",
                        placeholder: "Digite sua mensagem aqui...",
                        value: "{input_text}",
                        oninput: move |e| input_text.set(e.value()),
                        onkeydown: {
                            let cid = contact_id.clone();
                            move |e| {
                                if e.key() == Key::Enter && !e.modifiers().shift() {
                                    e.prevent_default();
                                    let txt = input_text();
                                    if !txt.trim().is_empty() {
                                        state.send_message(cid.clone(), txt.clone(), selected_color(), selected_font());
                                        input_text.set(String::new());
                                        play_sound("message");
                                    }
                                }
                            }
                        }
                    }

                    button {
                        class: "w-[60px] h-[60px] bg-[#5cb2ff] hover:bg-[#4ba2ef] active:bg-[#3992df] transition-colors flex items-center justify-center cursor-pointer text-white focus:outline-none flex-shrink-0 rounded-none border-none",
                        title: "Enviar (Mensagem de Voz)",
                        onclick: {
                            let cid = contact_id.clone();
                            move |_| {
                                let txt = input_text();
                                if !txt.trim().is_empty() {
                                    state.send_message(cid.clone(), txt.clone(), selected_color(), selected_font());
                                    input_text.set(String::new());
                                    play_sound("message");
                                }
                            }
                        },
                        img {
                            src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Mic/SVG/ic_fluent_mic_24_color.svg",
                            class: "w-6 h-6 select-none pointer-events-none brightness-0 invert"
                        }
                    }
                }
            }
        }
    }
}
