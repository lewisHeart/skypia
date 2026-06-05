use dioxus::prelude::*;
use crate::state::AppState;
use crate::sound::play_sound;

#[component]
pub fn ChatInput(
    contact_id: String,
    mut state: AppState,
    on_nudge: EventHandler<()>,
) -> Element {
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
    let group = (state.group_chats)().into_iter().find(|g| g.id == contact_id);
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

            // Formatting & Action Toolbar
            div { class: "h-8 bg-white/50 border-t border-b {theme.titlebar_border()} px-3 flex items-center justify-between text-xs {theme.titlebar_text()} relative",
                div { class: "flex items-center space-x-3.5",
                    button { 
                        class: "hover:bg-black/5 p-1 rounded cursor-pointer flex items-center space-x-0.5 transition-colors",
                        title: "Fonte e Cor",
                        onclick: move |_| {
                            show_font_panel.set(!show_font_panel());
                            show_emoticon_panel.set(false);
                            show_wink_panel.set(false);
                            show_file_panel.set(false);
                        },
                        img {
                            src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/input-latin-uppercase.webp",
                            class: "w-4.5 h-4.5 object-contain pointer-events-none"
                        }
                        span { class: "text-[7px] text-slate-500", "▼" }
                    }

                    button { 
                        class: "hover:bg-black/5 p-1 rounded cursor-pointer flex items-center space-x-0.5 transition-colors",
                        title: "Emojis",
                        onclick: move |_| {
                            show_emoticon_panel.set(!show_emoticon_panel());
                            show_font_panel.set(false);
                            show_wink_panel.set(false);
                            show_file_panel.set(false);
                        },
                        img {
                            src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/grinning-face-with-big-eyes.webp",
                            class: "w-4.5 h-4.5 object-contain pointer-events-none"
                        }
                        span { class: "text-[7px] text-slate-500", "▼" }
                    }

                    button { 
                        class: "hover:bg-black/5 p-1 rounded cursor-pointer flex items-center space-x-0.5 transition-colors",
                        title: "Piscadelas (Winks)",
                        onclick: move |_| {
                            show_wink_panel.set(!show_wink_panel());
                            show_font_panel.set(false);
                            show_emoticon_panel.set(false);
                            show_file_panel.set(false);
                        },
                        img {
                            src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/sparkles.webp",
                            class: "w-4.5 h-4.5 object-contain pointer-events-none"
                        }
                        span { class: "text-[7px] text-slate-500", "▼" }
                    }

                    button { 
                        class: "hover:bg-black/5 p-1 rounded cursor-pointer flex items-center space-x-0.5 transition-colors",
                        title: "Enviar Arquivo",
                        onclick: move |_| {
                            show_file_panel.set(!show_file_panel());
                            show_wink_panel.set(false);
                            show_font_panel.set(false);
                            show_emoticon_panel.set(false);
                        },
                        img {
                            src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/file-folder.webp",
                            class: "w-4.5 h-4.5 object-contain pointer-events-none"
                        }
                        span { class: "text-[7px] text-slate-500", "▼" }
                    }

                    if !is_group {
                        button { 
                            class: "hover:bg-black/5 p-1 rounded cursor-pointer flex items-center transition-colors",
                            title: "Desafiar para Jogo da Velha",
                            onclick: {
                                let cid = contact_id.clone();
                                move |_| {
                                    state.start_game(cid.clone());
                                    show_file_panel.set(false);
                                    show_wink_panel.set(false);
                                    show_font_panel.set(false);
                                    show_emoticon_panel.set(false);
                                }
                            },
                            img {
                                src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/video-game.webp",
                                class: "w-4.5 h-4.5 object-contain pointer-events-none"
                            }
                        }
                    }

                    button { 
                        class: "hover:bg-black/5 p-1 rounded cursor-pointer flex items-center transition-colors active:scale-90",
                        title: "Chamar a Atenção (Nudge)",
                        onclick: handle_send_nudge,
                        img {
                            src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/bell.webp",
                            class: "w-4.5 h-4.5 object-contain pointer-events-none animate-bounce"
                        }
                    }
                }

                // POPOVERS RENDER
                if show_font_panel() {
                    div { class: "absolute left-2 bottom-9 w-44 bg-white border {theme.titlebar_border()} rounded shadow-lg z-50 p-2.5 flex flex-col space-y-2.5 text-xs text-slate-700",
                        div { class: "flex flex-col space-y-1",
                            span { class: "font-bold text-[10px] text-slate-400 uppercase tracking-wider", "Fonte" }
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
                            span { class: "font-bold text-[10px] text-slate-400 uppercase tracking-wider", "Cor" }
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
                            span { "Enviar Foto (.jpg)" }
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
                            span { "Enviar Música (.mp3)" }
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
                            span { "Enviar Programa (.exe)" }
                        }
                    }
                }
            }

            // Chat message input area
            div { class: "h-20 bg-white border-t {theme.modal_border()} p-2 flex flex-col justify-between relative",
                if is_typing_srv {
                    div { class: "absolute -top-5 left-2 h-5 text-[10px] text-slate-500 italic flex items-center space-x-1 animate-pulse z-10 bg-white/60 px-2 rounded-t border-t border-l border-r {theme.modal_border()}",
                        span { "✍️" }
                        span { "{typing_name} está digitando..." }
                    }
                }
                 div { class: "flex-1 flex space-x-2 w-full",
                    textarea {
                        class: "flex-1 resize-none p-1.5 text-xs msn-input rounded border {theme.modal_border()}",
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
                        class: "w-16 h-full {theme.btn_primary()} rounded font-bold text-xs shadow cursor-pointer flex items-center justify-center active:scale-95 transition-transform",
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
                        "Enviar"
                    }
                }
            }
        }
    }
}
