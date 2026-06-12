use crate::sound::play_sound;
use crate::state::AppState;
use dioxus::prelude::*;
use base64::Engine;

#[component]
pub fn ChatInput(contact_id: String, mut state: AppState, on_nudge: EventHandler<()>) -> Element {
    let theme = state.theme();
    let theme_titlebar_border = theme.titlebar_border();
    let theme_titlebar_text = theme.titlebar_text();
    let mut input_text = use_signal(|| String::new());
    let mut selected_font = use_signal(|| state.chat_font_family());
    let mut selected_color = use_signal(|| state.chat_font_color());

    // UI Popovers
    let mut show_emoticon_panel = use_signal(|| false);
    let mut show_font_panel = use_signal(|| false);
    let mut show_wink_panel = use_signal(|| false);
    let mut show_file_panel = use_signal(|| false);

    let is_recording = use_signal(|| false);
    let mut recording_seconds = use_signal(|| 0);
    let mut recording_error = use_signal(|| Option::<String>::None);

    // Efeito para contar os segundos de gravação
    use_effect(move || {
        let is_rec = is_recording();
        if is_rec {
            recording_seconds.set(0);
            spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    if !is_recording() {
                        break;
                    }
                    recording_seconds.set(recording_seconds() + 1);
                }
            });
        }
    });

    let format_duration = |sec: usize| -> String {
        let m = sec / 60;
        let s = sec % 60;
        format!("{:02}:{:02}", m, s)
    };

    let custom_stickers = use_signal(|| Vec::<(i64, String, String)>::new());

    use_effect(move || {
        if show_wink_panel() {
            let mut custom_stickers_clone = custom_stickers;
            spawn(async move {
                if let Ok(st) = crate::services::db::DatabaseService::get_stickers().await {
                    custom_stickers_clone.set(st);
                }
            });
        }
    });

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

    let self_id = state.server_user_id();
    let is_local_user_admin = group.as_ref().map(|g| {
        g.members.iter().any(|m| Some(m.id.clone()) == self_id && m.role.as_deref() == Some("admin"))
    }).unwrap_or(false);

    let is_send_disabled = if let Some(ref g) = group {
        let allow_send = g.allow_member_send.unwrap_or(true);
        !allow_send && !is_local_user_admin
    } else {
        false
    };

    // Send nudge handler
    let contact_id_nudge = contact_id.clone();
    let handle_send_nudge = move |_| {
        if is_send_disabled { return; }
        state.send_nudge(contact_id_nudge.clone());
        play_sound("nudge");
        on_nudge.call(());
    };

    // Gravação de Mensagem de Voz (Microfone)
    let start_recording = {
        let cid = contact_id.clone();
        let state = state;
        let mut recording_error = recording_error;
        let mut is_recording = is_recording;
        move || {
            if is_send_disabled { return; }
            if let Some(token) = state.auth_token() {
                let mut state_clone = state;
                let cid_clone = cid.clone();
                
                recording_error.set(None);
                
                let init_script = r#"
                    (async function() {
                        try {
                            if (!navigator.mediaDevices || !navigator.mediaDevices.getUserMedia) {
                                dioxus.send("error:Microfone não suportado nesta plataforma.");
                                return;
                            }
                            const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
                            window.mediaStream = stream;
                            window.audioChunks = [];
                            
                            let options = { mimeType: 'audio/webm' };
                            if (!MediaRecorder.isTypeSupported(options.mimeType)) {
                                if (MediaRecorder.isTypeSupported('audio/mp4')) {
                                    options = { mimeType: 'audio/mp4' };
                                } else if (MediaRecorder.isTypeSupported('audio/ogg')) {
                                    options = { mimeType: 'audio/ogg' };
                                } else {
                                    options = {};
                                }
                            }
                            
                            window.mediaRecorder = new MediaRecorder(stream, options);
                            window.mediaRecorder.ondataavailable = e => {
                                if (e.data && e.data.size > 0) {
                                    window.audioChunks.push(e.data);
                                }
                            };
                            
                            window.mediaRecorder.onstop = () => {
                                const mimeType = window.mediaRecorder.mimeType || 'audio/webm';
                                const blob = new Blob(window.audioChunks, { type: mimeType });
                                
                                let ext = 'webm';
                                if (mimeType.includes('mp4')) {
                                    ext = 'mp4';
                                } else if (mimeType.includes('ogg')) {
                                    ext = 'ogg';
                                } else if (mimeType.includes('wav')) {
                                    ext = 'wav';
                                } else if (mimeType.includes('mpeg') || mimeType.includes('mp3')) {
                                    ext = 'mp3';
                                }
                                
                                const reader = new FileReader();
                                reader.readAsDataURL(blob);
                                reader.onloadend = () => {
                                    const base64data = reader.result;
                                    dioxus.send("audio:" + ext + ":" + base64data);
                                };
                            };
                            
                            window.mediaRecorder.start(100);
                            dioxus.send("started");
                        } catch (err) {
                            console.error("Microphone access error:", err);
                            dioxus.send("error:" + err.message);
                        }
                    })();
                "#;
                
                let mut eval = document::eval(init_script);
                
                spawn(async move {
                    if let Ok(serde_json::Value::String(response)) = eval.recv().await {
                        if response == "started" {
                            is_recording.set(true);
                            
                            if let Ok(serde_json::Value::String(final_response)) = eval.recv().await {
                                if final_response.starts_with("audio:") {
                                    let parts: Vec<&str> = final_response.splitn(3, ':').collect();
                                    if parts.len() == 3 {
                                        let ext = parts[1];
                                        let data_uri = parts[2];
                                        if let Some(comma_idx) = data_uri.find(',') {
                                            let base64_str = &data_uri[comma_idx + 1..];
                                            if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(base64_str) {
                                                let filename = format!("voice-message.{}", ext);
                                                let mime = match ext {
                                                    "webm" => "audio/webm",
                                                    "mp4" => "audio/mp4",
                                                    "ogg" => "audio/ogg",
                                                    "wav" => "audio/wav",
                                                    _ => "audio/mpeg",
                                                };
                                                match crate::services::api::upload_generic_file(&token, bytes, &filename, mime).await {
                                                    Ok(res) => {
                                                        if let Some(url) = res["url"].as_str() {
                                                            let md_text = format!("[Áudio: Mensagem de Voz]({})", url);
                                                            state_clone.send_message(cid_clone, md_text, "#000000".to_string(), "Segoe UI".to_string());
                                                        }
                                                    }
                                                    Err(e) => {
                                                        eprintln!("Erro no upload de áudio: {}", e);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else if final_response == "cancelled" {
                                    println!("Gravação cancelada.");
                                } else if final_response.starts_with("error:") {
                                    let err_msg = final_response.trim_start_matches("error:").to_string();
                                    recording_error.set(Some(err_msg));
                                }
                            }
                        } else if response.starts_with("error:") {
                            let err_msg = response.trim_start_matches("error:").to_string();
                            recording_error.set(Some(err_msg));
                        }
                    }
                    is_recording.set(false);
                });
            }
        }
    };

    // Helper to insert emoticons at text cursor
    let mut insert_emoticon = move |code: &str| {
        input_text.set(format!("{}{}", input_text(), code));
        show_emoticon_panel.set(false);
    };

    let emoticons_list: Vec<(&str, String)> = [
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
    ]
    .into_iter()
    .map(|(code, emoji_name)| {
        let url = crate::models::get_emoji_anim_url(&format!("{}.webp", emoji_name));
        (code, url)
    })
    .collect();

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
            div { class: "h-8 bg-transparent px-3 flex items-center justify-between text-xs {theme_titlebar_text} relative select-none",
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

                    // Gravar Mensagem de Voz (Microfone)
                    button {
                        class: "hover:bg-white/80 p-1 rounded cursor-pointer flex items-center transition-colors focus:outline-none",
                        title: "Gravar Mensagem de Voz",
                        onclick: {
                            let mut start_recording = start_recording.clone();
                            move |_| {
                                start_recording();
                            }
                        },
                        img {
                            src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Mic/SVG/ic_fluent_mic_24_color.svg",
                            class: "w-5 h-5 select-none pointer-events-none"
                        }
                        span { class: "text-[11px] text-[#2b3e51] font-semibold ml-1.5 hidden sm:inline", "Gravar voz" }
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
                        span { class: "text-[11px] text-[#2b3e51] font-semibold ml-1.5 hidden sm:inline", "Chamar atenção" }
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
                    div { class: "absolute left-2 bottom-9 w-44 bg-white border {theme_titlebar_border} rounded shadow-lg z-50 p-2.5 flex flex-col space-y-2.5 text-xs text-slate-700",
                        div { class: "flex flex-col space-y-1",
                            span { class: "font-bold text-[10px] text-slate-400", "Fonte" }
                            div { class: "flex flex-col space-y-0.5",
                                for font_name in &["Segoe UI", "Comic Sans MS", "Arial", "Courier New"] {
                                    button {
                                        class: "px-2 py-1 text-left hover:bg-black/5 rounded transition-colors cursor-pointer {theme_titlebar_text}",
                                        style: "font-family: {font_name};",
                                        onclick: move |_| {
                                            selected_font.set(font_name.to_string());
                                            state.set_chat_font_family(font_name.to_string());
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
                                        class: "w-6 h-6 rounded cursor-pointer border {theme_titlebar_border} hover:scale-110 hover:shadow transition-all flex items-center justify-center relative",
                                        style: "background-color: {color};",
                                        onclick: move |_| {
                                            selected_color.set(color.to_string());
                                            state.set_chat_font_color(color.to_string());
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
                        class: "absolute left-10 bottom-9 w-60 max-h-52 overflow-y-auto bg-white border {theme_titlebar_border} rounded shadow-lg z-50 p-2 grid grid-cols-6 gap-1 text-base scrollbar-thin scrollbar-thumb-slate-300",
                        for (code, emoji_url) in emoticons_list {
                            button {
                                class: "hover:bg-slate-100 p-0.5 rounded flex items-center justify-center transition-colors cursor-pointer",
                                title: code,
                                onclick: move |_| insert_emoticon(code),
                                img {
                                    src: emoji_url,
                                    class: "w-6 h-6 object-contain pointer-events-none"
                                }
                            }
                        }
                    }
                }

                if show_wink_panel() {
                    div { class: "absolute left-20 bottom-9 w-64 bg-white border {theme_titlebar_border} rounded shadow-lg z-50 p-2 flex flex-col text-xs text-slate-700",
                        div { class: "flex justify-between items-center mb-2 pb-1 border-b border-slate-100",
                            span { class: "font-bold {theme_titlebar_text}", "Meus Stickers e Winks" }
                            label {
                                class: "cursor-pointer text-[10px] bg-slate-100 hover:bg-slate-200 px-2 py-0.5 rounded font-semibold text-slate-600 transition-colors",
                                input {
                                    r#type: "file",
                                    class: "hidden",
                                    accept: "image/*",
                                    onchange: {
                                        let state = state;
                                        let token_opt = state.auth_token();
                                        let mut custom_stickers_clone = custom_stickers;
                                        move |e| {
                                            if let Some(token) = token_opt.clone() {
                                                let files = e.files();
                                                if let Some(file) = files.into_iter().next() {
                                                    spawn(async move {
                                                        if let Ok(bytes) = file.read_bytes().await {
                                                            match crate::services::api::upload_generic_file(&token, bytes.to_vec(), "sticker.gif", "image/gif").await {
                                                                Ok(url_val) => {
                                                                    let url_str = if let Some(s) = url_val.as_str() {
                                                                        s.to_string()
                                                                    } else {
                                                                        url_val.to_string()
                                                                    };
                                                                    let _ = crate::services::db::DatabaseService::add_sticker("Sticker".to_string(), url_str).await;
                                                                    if let Ok(st) = crate::services::db::DatabaseService::get_stickers().await {
                                                                        custom_stickers_clone.set(st);
                                                                    }
                                                                }
                                                                Err(e) => eprintln!("Upload sticker error: {}", e),
                                                            }
                                                        }
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                                "+ Adicionar"
                            }
                        }
                        div { class: "grid grid-cols-4 gap-2 max-h-48 overflow-y-auto scrollbar-thin scrollbar-thumb-slate-300 pr-1",
                            // Winks Padrão
                            button {
                                class: "p-1.5 hover:bg-black/5 rounded transition-colors flex flex-col items-center justify-center cursor-pointer {theme_titlebar_text} border border-transparent hover:border-slate-200",
                                onclick: {
                                    let cid = contact_id.clone();
                                    move |_| {
                                        state.send_wink(cid.clone(), "kiss".to_string());
                                        show_wink_panel.set(false);
                                    }
                                },
                                img {
                                    src: crate::models::get_emoji_url("kiss-mark.svg"),
                                    class: "w-6 h-6 object-contain pointer-events-none mb-1"
                                }
                                span { class: "text-[9px] text-center truncate w-full", "Beijo" }
                            }
                            button {
                                class: "p-1.5 hover:bg-black/5 rounded transition-colors flex flex-col items-center justify-center cursor-pointer {theme_titlebar_text} border border-transparent hover:border-slate-200",
                                onclick: {
                                    let cid = contact_id.clone();
                                    move |_| {
                                        state.send_wink(cid.clone(), "hammer".to_string());
                                        show_wink_panel.set(false);
                                    }
                                },
                                img {
                                    src: crate::models::get_emoji_url("hammer.svg"),
                                    class: "w-6 h-6 object-contain pointer-events-none mb-1"
                                }
                                span { class: "text-[9px] text-center truncate w-full", "Martelo" }
                            }
                            button {
                                class: "p-1.5 hover:bg-black/5 rounded transition-colors flex flex-col items-center justify-center cursor-pointer {theme_titlebar_text} border border-transparent hover:border-slate-200",
                                onclick: {
                                    let cid = contact_id.clone();
                                    move |_| {
                                        state.send_wink(cid.clone(), "pig".to_string());
                                        show_wink_panel.set(false);
                                    }
                                },
                                img {
                                    src: crate::models::get_emoji_url("pig-face.svg"),
                                    class: "w-6 h-6 object-contain pointer-events-none mb-1"
                                }
                                span { class: "text-[9px] text-center truncate w-full", "Porco" }
                            }
                            
                            // Custom Stickers
                            for (id, _name, url) in custom_stickers() {
                                {
                                    let url_clone = url.clone();
                                    let cid = contact_id.clone();
                                    let sid = id;
                                    let mut custom_stickers_del = custom_stickers;
                                    rsx! {
                                        div { class: "relative group",
                                            button {
                                                class: "w-full p-1.5 hover:bg-black/5 rounded transition-colors flex flex-col items-center justify-center cursor-pointer border border-transparent hover:border-slate-200",
                                                onclick: move |_| {
                                                    state.send_wink(cid.clone(), url_clone.clone());
                                                    show_wink_panel.set(false);
                                                },
                                                img {
                                                    src: "{url}",
                                                    class: "w-8 h-8 object-contain pointer-events-none"
                                                }
                                            }
                                            button {
                                                class: "absolute -top-1 -right-1 bg-white rounded-full w-4 h-4 flex items-center justify-center text-[10px] border border-slate-300 text-red-500 opacity-0 group-hover:opacity-100 transition-opacity hover:bg-red-50",
                                                onclick: move |e| {
                                                    e.stop_propagation();
                                                    spawn(async move {
                                                        let _ = crate::services::db::DatabaseService::delete_sticker(sid).await;
                                                        if let Ok(st) = crate::services::db::DatabaseService::get_stickers().await {
                                                            custom_stickers_del.set(st);
                                                        }
                                                    });
                                                },
                                                "×"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if show_file_panel() {
                    div { class: "absolute left-32 bottom-9 w-40 bg-white border {theme_titlebar_border} rounded shadow-lg z-50 p-1 flex flex-col text-xs text-slate-700",
                        label {
                            class: "px-2 py-1 text-left hover:bg-black/5 rounded transition-colors flex items-center space-x-1.5 cursor-pointer {theme_titlebar_text}",
                            input {
                                r#type: "file",
                                class: "hidden",
                                accept: "image/*",
                                onchange: {
                                    let cid = contact_id.clone();
                                    let state = state;
                                    let token_opt = state.auth_token();
                                    move |e| {
                                        show_file_panel.set(false);
                                        if let Some(token) = token_opt.clone() {
                                            let files = e.files();
                                            if let Some(file) = files.into_iter().next() {
                                                let cid_clone = cid.clone();
                                                let filename_clone = file.name();
                                                let mut st = state;
                                                spawn(async move {
                                                    if let Ok(bytes) = file.read_bytes().await {
                                                        let ext = std::path::Path::new(&filename_clone)
                                                            .extension()
                                                            .and_then(|e| e.to_str())
                                                            .unwrap_or("bin");
                                                        let mime = match ext.to_lowercase().as_str() {
                                                            "png" => "image/png",
                                                            "jpg" | "jpeg" => "image/jpeg",
                                                            "gif" => "image/gif",
                                                            "webp" => "image/webp",
                                                            _ => "application/octet-stream"
                                                        };
                                                        
                                                        match crate::services::api::upload_generic_file(&token, bytes.to_vec(), &filename_clone, mime).await {
                                                            Ok(res) => {
                                                                if let Some(url) = res["url"].as_str() {
                                                                    let md_text = format!("![{}]({})", filename_clone, url);
                                                                    st.send_message(cid_clone, md_text, "#000000".to_string(), "Segoe UI".to_string());
                                                                }
                                                            }
                                                            Err(e) => {
                                                                eprintln!("Upload error: {}", e);
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                            img {
                                src: crate::models::get_emoji_url("framed-picture.svg"),
                                class: "w-4 h-4 object-contain pointer-events-none"
                            }
                            span { "Enviar Foto" }
                        }
                        label {
                            class: "px-2 py-1 text-left hover:bg-black/5 rounded transition-colors flex items-center space-x-1.5 cursor-pointer {theme_titlebar_text}",
                            input {
                                r#type: "file",
                                class: "hidden",
                                accept: "audio/*",
                                onchange: {
                                    let cid = contact_id.clone();
                                    let state = state;
                                    let token_opt = state.auth_token();
                                    move |e| {
                                        show_file_panel.set(false);
                                        if let Some(token) = token_opt.clone() {
                                            let files = e.files();
                                            if let Some(file) = files.into_iter().next() {
                                                let cid_clone = cid.clone();
                                                let filename_clone = file.name();
                                                let mut st = state;
                                                spawn(async move {
                                                    if let Ok(bytes) = file.read_bytes().await {
                                                        match crate::services::api::upload_generic_file(&token, bytes.to_vec(), &filename_clone, "audio/mpeg").await {
                                                            Ok(res) => {
                                                                if let Some(url) = res["url"].as_str() {
                                                                    let md_text = format!("[Áudio: {}]({})", filename_clone, url);
                                                                    st.send_message(cid_clone, md_text, "#000000".to_string(), "Segoe UI".to_string());
                                                                }
                                                            }
                                                            Err(e) => {
                                                                eprintln!("Upload error: {}", e);
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                            img {
                                src: crate::models::get_emoji_url("musical-note.svg"),
                                class: "w-4 h-4 object-contain pointer-events-none"
                            }
                            span { "Enviar Música" }
                        }
                        label {
                            class: "px-2 py-1 text-left hover:bg-black/5 rounded transition-colors flex items-center space-x-1.5 cursor-pointer {theme_titlebar_text}",
                            input {
                                r#type: "file",
                                class: "hidden",
                                onchange: {
                                    let cid = contact_id.clone();
                                    let state = state;
                                    let token_opt = state.auth_token();
                                    move |e| {
                                        show_file_panel.set(false);
                                        if let Some(token) = token_opt.clone() {
                                            let files = e.files();
                                            if let Some(file) = files.into_iter().next() {
                                                let cid_clone = cid.clone();
                                                let filename_clone = file.name();
                                                let mut st = state;
                                                spawn(async move {
                                                    if let Ok(bytes) = file.read_bytes().await {
                                                        match crate::services::api::upload_generic_file(&token, bytes.to_vec(), &filename_clone, "application/octet-stream").await {
                                                            Ok(res) => {
                                                                if let Some(url) = res["url"].as_str() {
                                                                    let md_text = format!("[Arquivo: {}]({})", filename_clone, url);
                                                                    st.send_message(cid_clone, md_text, "#000000".to_string(), "Segoe UI".to_string());
                                                                }
                                                            }
                                                            Err(e) => {
                                                                eprintln!("Upload error: {}", e);
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                            img {
                                src: crate::models::get_emoji_url("floppy-disk.svg"),
                                class: "w-4 h-4 object-contain pointer-events-none"
                            }
                            span { "Enviar Arquivo" }
                        }
                    }
                }
            }

            // Banner de erro de microfone/gravação
            if let Some(ref err) = *recording_error.read() {
                div { class: "bg-red-50 border-b border-red-200 px-3 py-1.5 text-[10px] text-red-650 flex items-center justify-between select-none animate-fade-in relative z-20",
                    span { class: "font-semibold", "Erro de Microfone: {err}" }
                    button {
                        class: "text-red-800 hover:text-red-950 font-bold focus:outline-none cursor-pointer",
                        onclick: move |_| recording_error.set(None),
                        "✕"
                    }
                }
            }

            // Chat message input area
            div { class: "h-[85px] bg-transparent py-[12.5px] px-[14px] flex flex-col justify-between relative",
                if is_typing_srv && !is_recording() {
                    div { class: "absolute -top-5 left-2 h-5 text-[10px] text-slate-500 italic flex items-center space-x-1 animate-pulse z-10 bg-[#eff5fb] px-2 rounded-t border-t border-l border-r border-[#96badb]",
                        span { "✍️" }
                        span { "{typing_name} está digitando..." }
                    }
                }
                
                if is_recording() {
                    div { class: "flex-1 flex space-x-3 w-full items-center bg-[#fdf2f2] border-2 border-red-200 p-2 shadow-inner justify-between",
                        div { class: "flex items-center space-x-2.5 text-xs text-red-700 font-bold select-none",
                            span { class: "w-2.5 h-2.5 rounded-full bg-red-600 animate-pulse" }
                            span { "Gravando Mensagem de Voz..." }
                            span { class: "px-2 py-0.5 bg-red-100 text-red-800 rounded font-semibold text-[11px] font-mono",
                                "{format_duration(recording_seconds())}"
                            }
                        }
                        div { class: "flex items-center space-x-2",
                            button {
                                class: "px-3 py-1.5 bg-red-600 hover:bg-red-700 text-white font-bold text-[10px] rounded shadow cursor-pointer transition-colors focus:outline-none flex items-center space-x-1",
                                onclick: move |_| {
                                    let stop_script = r#"
                                        if (window.mediaRecorder && window.mediaRecorder.state !== "inactive") {
                                            window.mediaRecorder.stop();
                                            if (window.mediaStream) {
                                                window.mediaStream.getTracks().forEach(t => t.stop());
                                            }
                                        }
                                    "#;
                                    let _ = document::eval(stop_script);
                                },
                                span { "⏹" }
                                span { "Parar e Enviar" }
                            }
                            button {
                                class: "px-3 py-1.5 bg-white hover:bg-slate-100 text-slate-700 border border-slate-300 font-bold text-[10px] rounded shadow cursor-pointer transition-colors focus:outline-none",
                                onclick: move |_| {
                                    let cancel_script = r#"
                                        if (window.mediaRecorder) {
                                            if (window.mediaRecorder.state !== "inactive") {
                                                window.mediaRecorder.stop();
                                            }
                                            if (window.mediaStream) {
                                                window.mediaStream.getTracks().forEach(t => t.stop());
                                            }
                                        }
                                        dioxus.send("cancelled");
                                    "#;
                                    let _ = document::eval(cancel_script);
                                },
                                "Cancelar"
                            }
                        }
                    }
                } else {
                    div { class: "flex-1 flex space-x-2.5 w-full items-center",
                        textarea {
                            class: if is_send_disabled { "flex-1 h-[60px] resize-none p-1.5 text-xs msn-input rounded-none border-2 border-[#d1d1d1] placeholder-[#a5a5a5] placeholder:text-[10px] focus:outline-none focus:border-slate-400 bg-slate-100 text-slate-400" } else { "flex-1 h-[60px] resize-none p-1.5 text-xs msn-input rounded-none border-2 border-[#d1d1d1] placeholder-[#a5a5a5] placeholder:text-[10px] focus:outline-none focus:border-slate-400" },
                            style: "font-family: {selected_font()}; color: {selected_color()};",
                            placeholder: if is_send_disabled { "O envio de mensagens foi desativado por um administrador." } else { "Digite sua mensagem aqui..." },
                            disabled: is_send_disabled,
                            value: "{input_text}",
                            oninput: move |e| input_text.set(e.value()),
                            onkeydown: {
                                let cid = contact_id.clone();
                                move |e| {
                                    if is_send_disabled { return; }
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
                            class: if is_send_disabled { "w-[60px] h-[60px] flex items-center justify-center focus:outline-none flex-shrink-0 rounded-none border-none bg-slate-300 text-slate-500 cursor-not-allowed opacity-60" } else { "w-[60px] h-[60px] bg-[#5cb2ff] hover:bg-[#4ba2ef] active:bg-[#3992df] transition-colors flex items-center justify-center cursor-pointer text-white focus:outline-none flex-shrink-0 rounded-none border-none" },
                            title: if is_send_disabled {
                                "Envio desativado".to_string()
                            } else if input_text().trim().is_empty() {
                                "Gravar Mensagem de Voz".to_string()
                            } else {
                                "Enviar".to_string()
                            },
                            disabled: is_send_disabled,
                            onclick: {
                                let cid = contact_id.clone();
                                let mut start_recording = start_recording.clone();
                                move |_| {
                                    if is_send_disabled { return; }
                                    let txt = input_text();
                                    if txt.trim().is_empty() {
                                        start_recording();
                                    } else {
                                        state.send_message(cid.clone(), txt.clone(), selected_color(), selected_font());
                                        input_text.set(String::new());
                                        play_sound("message");
                                    }
                                }
                            },
                            if input_text().trim().is_empty() {
                                img {
                                    src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Mic/SVG/ic_fluent_mic_24_color.svg",
                                    class: "w-6 h-6 select-none pointer-events-none brightness-0 invert",
                                    alt: "Gravar Voz"
                                }
                            } else {
                                img {
                                    src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Send/SVG/ic_fluent_send_24_color.svg",
                                    class: "w-6 h-6 select-none pointer-events-none brightness-0 invert",
                                    alt: "Enviar"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
