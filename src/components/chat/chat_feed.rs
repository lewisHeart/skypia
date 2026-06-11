use crate::models::FileTransferState;
use crate::state::AppState;
use dioxus::prelude::*;
use std::collections::HashSet;

#[component]
pub fn ChatFeed(contact_id: String, mut state: AppState) -> Element {
    let theme = state.theme();
    let mut show_images = use_signal(|| HashSet::new());

    let mut limit = use_signal(|| 15);
    let mut last_contact_id = use_signal(|| contact_id.clone());
    if last_contact_id() != contact_id {
        *last_contact_id.write() = contact_id.clone();
        limit.set(15);
    }

    let messages = state.chat_messages();
    let chat_history = messages.get(&contact_id).cloned().unwrap_or_default();

    let total_messages = chat_history.len();
    let show_load_more = total_messages > limit();
    let start_idx = if total_messages > limit() {
        total_messages - limit()
    } else {
        0
    };
    let visible_messages = chat_history[start_idx..].to_vec();

    let last_msg_id = visible_messages.last().map(|m| m.id.clone());
    let feed_id = format!("chat-feed-{}", contact_id);
    let eval_feed_id = feed_id.clone();
    let contact_id_scroll = contact_id.clone();

    use_effect(move || {
        let _last_id = last_msg_id.clone();
        let _cid = contact_id_scroll.clone();
        let id = eval_feed_id.clone();
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let js = format!(
                r#"
                var el = document.getElementById('{}');
                if (el) {{
                    el.scrollTop = el.scrollHeight;
                }}
                "#,
                id
            );
            let _ = document::eval(&js);
        });
    });

    let contact = state.contacts().into_iter().find(|c| c.id == contact_id);
    let group = (state.group_chats)()
        .into_iter()
        .find(|g| g.id == contact_id);
    if contact.is_none() && group.is_none() {
        return rsx! {};
    }

    let display_name = if let Some(ref c) = contact {
        c.display_name.clone()
    } else {
        group
            .as_ref()
            .unwrap()
            .name
            .clone()
            .unwrap_or_else(|| "Grupo sem nome".to_string())
    };

    let format_message_text = |text: &str| -> Element {
        let emoticons = &[
            // Atalhos clássicos
            ("(H)", "smiling-face-with-sunglasses"),
            ("(h)", "smiling-face-with-sunglasses"),
            ("(Y)", "thumbs-up"),
            ("(y)", "thumbs-up"),
            ("(N)", "thumbs-down"),
            ("(n)", "thumbs-down"),
            ("(K)", "kiss-mark"),
            ("(k)", "kiss-mark"),
            ("(A)", "smiling-face-with-halo"),
            ("(a)", "smiling-face-with-halo"),
            ("(L)", "red-heart"),
            ("(l)", "red-heart"),
            ("(O)", "alarm-clock"),
            ("(o)", "alarm-clock"),
            (":-D", "grinning-face-with-big-eyes"),
            (":-d", "grinning-face-with-big-eyes"),
            (":D", "grinning-face-with-big-eyes"),
            (":d", "grinning-face-with-big-eyes"),
            (":-)", "slightly-smiling-face"),
            (":)", "slightly-smiling-face"),
            (";-)", "winking-face"),
            (";)", "winking-face"),
            (":-(", "crying-face"),
            (":(", "crying-face"),
            (":-@", "pouting-face"),
            (":@", "pouting-face"),
            ("(U)", "broken-heart"),
            ("(u)", "broken-heart"),
            ("(G)", "wrapped-gift"),
            ("(g)", "wrapped-gift"),
            ("(F)", "wilted-flower"),
            ("(f)", "wilted-flower"),
            ("(P)", "camera"),
            ("(p)", "camera"),
            ("(M)", "musical-note"),
            ("(m)", "musical-note"),
            ("(S)", "crescent-moon"),
            ("(s)", "crescent-moon"),
            ("(*)", "star"),
            ("(E)", "envelope"),
            ("(e)", "envelope"),
            ("(C)", "hot-beverage"),
            ("(c)", "hot-beverage"),
            // Novos atalhos inseridos via barra de ferramentas (Maiúsculas/Minúsculas)
            ("(HE)", "smiling-face-with-heart-eyes"),
            ("(he)", "smiling-face-with-heart-eyes"),
            ("(BK)", "face-blowing-a-kiss"),
            ("(bk)", "face-blowing-a-kiss"),
            ("(ST)", "squinting-face-with-tongue"),
            ("(st)", "squinting-face-with-tongue"),
            ("(ZF)", "zany-face"),
            ("(zf)", "zany-face"),
            ("(SF)", "shushing-face"),
            ("(sf)", "shushing-face"),
            ("(TF)", "thinking-face"),
            ("(tf)", "thinking-face"),
            ("(EF)", "expressionless-face"),
            ("(ef)", "expressionless-face"),
            ("(SM)", "smirking-face"),
            ("(sm)", "smirking-face"),
            ("(GF)", "grimacing-face"),
            ("(gf)", "grimacing-face"),
            ("(DF)", "drooling-face"),
            ("(df)", "drooling-face"),
            ("(SL)", "sleeping-face"),
            ("(sl)", "sleeping-face"),
            ("(NF)", "nauseated-face"),
            ("(nf)", "nauseated-face"),
            ("(VOM)", "face-vomiting"),
            ("(vom)", "face-vomiting"),
            ("(EH)", "exploding-head"),
            ("(eh)", "exploding-head"),
            ("(PF)", "partying-face"),
            ("(pf)", "partying-face"),
            ("(WF)", "woozy-face"),
            ("(wf)", "woozy-face"),
            ("(;_;)", "crying-face"),
            ("(LCF)", "loudly-crying-face"),
            ("(lcf)", "loudly-crying-face"),
            ("(SCR)", "face-screaming-in-fear"),
            ("(scr)", "face-screaming-in-fear"),
            ("(ANG)", "angry-face"),
            ("(ang)", "angry-face"),
            ("(FSM)", "face-with-symbols-on-mouth"),
            ("(fsm)", "face-with-symbols-on-mouth"),
            ("(SK)", "skull"),
            ("(sk)", "skull"),
            ("(POO)", "pile-of-poo"),
            ("(poo)", "pile-of-poo"),
            ("(CLAP)", "clapping-hands"),
            ("(clap)", "clapping-hands"),
            ("(HS)", "handshake"),
            ("(hs)", "handshake"),
            ("(VIC)", "victory-hand"),
            ("(vic)", "victory-hand"),
            ("(FLEX)", "flexed-biceps"),
            ("(flex)", "flexed-biceps"),
            ("(FOLD)", "folded-hands"),
            ("(fold)", "folded-hands"),
            ("(BR)", "brain"),
            ("(br)", "brain"),
            ("(FIRE)", "fire"),
            ("(fire)", "fire"),
            ("(BOOM)", "collision"),
            ("(boom)", "collision"),
            ("(SPARKS)", "sparkles"),
            ("(sparks)", "sparkles"),
            ("(BAL)", "balloon"),
            ("(bal)", "balloon"),
            ("(POP)", "party-popper"),
            ("(pop)", "party-popper"),
            ("(RAIN)", "rainbow"),
            ("(rain)", "rainbow"),
            ("(SUN)", "sun"),
            ("(sun)", "sun"),
            ("(SNOW)", "snowflake"),
            ("(snow)", "snowflake"),
            ("(UMB)", "umbrella"),
            ("(umb)", "umbrella"),
            ("(DOG)", "dog-face"),
            ("(dog)", "dog-face"),
            ("(CAT)", "cat-face"),
            ("(cat)", "cat-face"),
            ("(PANDA)", "panda"),
            ("(panda)", "panda"),
            ("(ALIEN)", "alien"),
            ("(alien)", "alien"),
            ("(ROCKET)", "rocket"),
            ("(rocket)", "rocket"),
            ("(PLANE)", "airplane"),
            ("(plane)", "airplane"),
            ("(BEER)", "beer-mug"),
            ("(beer)", "beer-mug"),
            ("(PIZZA)", "pizza"),
            ("(pizza)", "pizza"),
            ("(MONEY)", "money-bag"),
            ("(money)", "money-bag"),
            ("(TROPHY)", "trophy"),
            ("(trophy)", "trophy"),
            // Emojis unicode normais
            ("😀", "grinning-face-with-big-eyes"),
            ("😃", "grinning-face-with-big-eyes"),
            ("😄", "grinning-face-with-smiling-eyes"),
            ("😁", "beaming-face-with-smiling-eyes"),
            ("😆", "grinning-squinting-face"),
            ("😅", "grinning-face-with-sweat"),
            ("😂", "face-with-tears-of-joy"),
            ("🤣", "rolling-on-the-floor-laughing"),
            ("😊", "smiling-face-with-smiling-eyes"),
            ("😇", "smiling-face-with-halo"),
            ("🙂", "slightly-smiling-face"),
            ("🙃", "upside-down-face"),
            ("😉", "winking-face"),
            ("😌", "relieved-face"),
            ("😍", "smiling-face-with-heart-eyes"),
            ("🥰", "smiling-face-with-hearts"),
            ("😘", "face-blowing-a-kiss"),
            ("😗", "kissing-face"),
            ("😙", "kissing-face-with-smiling-eyes"),
            ("😚", "kissing-face-with-closed-eyes"),
            ("😋", "face-savoring-food"),
            ("😛", "face-with-tongue"),
            ("😝", "squinting-face-with-tongue"),
            ("😜", "winking-face-with-tongue"),
            ("🤪", "zany-face"),
            ("🤨", "face-with-raised-eyebrow"),
            ("🧐", "face-with-monocle"),
            ("nerd-face", "nerd-face"),
            ("🤓", "nerd-face"),
            ("😎", "smiling-face-with-sunglasses"),
            ("🥸", "disguised-face"),
            ("🤩", "star-struck"),
            ("🥳", "partying-face"),
            ("😏", "smirking-face"),
            ("😒", "unamused-face"),
            ("😞", "disappointed-face"),
            ("😔", "pensive-face"),
            ("😟", "worried-face"),
            ("😕", "confused-face"),
            ("🙁", "slightly-frowning-face"),
            ("☹️", "frowning-face"),
            ("☹", "frowning-face"),
            ("😣", "persevering-face"),
            ("😖", "confounded-face"),
            ("😫", "tired-face"),
            ("😩", "weary-face"),
            ("🥺", "pleading-face"),
            ("😢", "crying-face"),
            ("😭", "loudly-crying-face"),
            ("😤", "face-with-steam-from-nose"),
            ("😠", "angry-face"),
            ("😡", "pouting-face"),
            ("🤬", "face-with-symbols-on-mouth"),
            ("🤯", "exploding-head"),
            ("😳", "flushed-face"),
            ("🥵", "hot-face"),
            ("🥶", "cold-face"),
            ("😱", "face-screaming-in-fear"),
            ("😨", "fearful-face"),
            ("😰", "anxious-face-with-sweat"),
            ("😥", "sad-but-relieved-face"),
            ("😓", "downcast-face-with-sweat"),
            ("🤗", "hugging-face"),
            ("🤔", "thinking-face"),
            ("🤫", "shushing-face"),
            ("melting-face", "melting-face"),
            ("🫠", "melting-face"),
            ("🤥", "lying-face"),
            ("😶", "face-without-mouth"),
            ("neutral-face", "neutral-face"),
            ("😐", "neutral-face"),
            ("😑", "expressionless-face"),
            ("grimacing-face", "grimacing-face"),
            ("😬", "grimacing-face"),
            ("🙄", "face-with-rolling-eyes"),
            ("😯", "hushed-face"),
            ("😦", "frowning-face-with-open-mouth"),
            ("😧", "anguished-face"),
            ("😮", "face-with-open-mouth"),
            ("😲", "astonished-face"),
            ("🥱", "yawning-face"),
            ("😴", "sleeping-face"),
            ("🤤", "drooling-face"),
            ("😪", "sleepy-face"),
            ("😵", "knocked-out-face"),
            ("🤐", "zipper-mouth-face"),
            ("🥴", "woozy-face"),
            ("🤢", "nauseated-face"),
            ("🤮", "face-vomiting"),
            ("sneezing-face", "sneezing-face"),
            ("🤧", "sneezing-face"),
            ("😷", "face-with-medical-mask"),
            ("🤒", "face-with-thermometer"),
            ("🤕", "face-with-head-bandage"),
            ("🤑", "money-mouth-face"),
            ("🤠", "cowboy-hat-face"),
            ("😈", "smiling-face-with-horns"),
            ("👿", "angry-face-with-horns"),
            ("💀", "skull"),
            ("☠️", "skull-and-crossbones"),
            ("☠", "skull-and-crossbones"),
            ("💩", "pile-of-poo"),
            ("🤡", "clown-face"),
            ("👻", "ghost"),
            ("👽", "alien"),
            ("👾", "alien-monster"),
            ("🤖", "robot"),
            // Gestos e mãos
            ("👍", "thumbs-up"),
            ("👎", "thumbs-down"),
            ("👊", "oncoming-fist"),
            ("✊", "raised-fist"),
            ("🤛", "left-facing-fist"),
            ("🤜", "right-facing-fist"),
            ("crossed-fingers", "crossed-fingers"),
            ("🤞", "crossed-fingers"),
            ("✌️", "victory-hand"),
            ("✌", "victory-hand"),
            ("🤟", "love-you-gesture"),
            ("🤘", "sign-of-the-horns"),
            ("👌", "ok-hand"),
            ("👋", "waving-hand"),
            ("🤙", "call-me-hand"),
            ("💪", "flexed-biceps"),
            ("🖕", "middle-finger"),
            ("✍️", "writing-hand"),
            ("✍", "writing-hand"),
            ("🙏", "folded-hands"),
            ("🤝", "handshake"),
            ("heart-hands", "heart-hands"),
            ("🫶", "heart-hands"),
            ("👏", "clapping-hands"),
            ("🙌", "raising-hands"),
            // Outros comuns
            ("🎈", "balloon"),
            ("🎉", "party-popper"),
            ("🕯️", "candle"),
            ("🕯", "candle"),
            ("⏰", "alarm-clock"),
            ("☎️", "telephone"),
            ("☎", "telephone"),
            ("📞", "telephone-receiver"),
            ("✉️", "envelope"),
            ("✉", "envelope"),
            ("📁", "file-folder"),
            ("📂", "open-file-folder"),
            ("💾", "floppy-disk"),
            ("💻", "laptop"),
            ("🖥️", "desktop-computer"),
            ("🖥", "desktop-computer"),
            ("🖨️", "printer"),
            ("🖨", "printer"),
            ("📷", "camera"),
            ("📸", "camera-with-flash"),
            ("📹", "video-camera"),
            ("🎥", "movie-camera"),
            ("🎭", "performing-arts"),
            ("🎫", "ticket"),
            ("🏆", "trophy"),
            ("🥇", "1st-place-medal"),
            ("🥈", "2nd-place-medal"),
            ("🥉", "3rd-place-medal"),
            ("⚽", "soccer-ball"),
            ("🏀", "basketball"),
            ("🏈", "american-football"),
            ("⚾", "baseball"),
            ("🎾", "tennis"),
            ("🏐", "volleyball"),
            ("🎱", "pool-8-ball"),
            ("⛳", "flag-in-hole"),
            ("🎮", "video-game"),
            ("🚗", "automobile"),
            ("🛵", "motor-scooter"),
            ("🏍️", "motorcycle"),
            ("🏍", "motorcycle"),
            ("🚲", "bicycle"),
            ("✈️", "airplane"),
            ("✈", "airplane"),
            ("🚀", "rocket"),
            ("🛸", "flying-saucer"),
            ("🌈", "rainbow"),
            ("☀️", "sun"),
            ("☀", "sun"),
            ("☁️", "cloud"),
            ("☁", "cloud"),
            ("🌧️", "cloud-with-rain"),
            ("🌧", "cloud-with-rain"),
            ("❄️", "snowflake"),
            ("❄", "snowflake"),
            ("🔥", "fire"),
            ("💧", "droplet"),
            ("🌊", "water-wave"),
            ("🍎", "red-apple"),
            ("🍌", "banana"),
            ("🍉", "watermelon"),
            ("🍒", "cherries"),
            ("🍑", "peach"),
            ("🍕", "pizza"),
            ("🍔", "hamburger"),
            ("🍟", "french-fries"),
            ("🌭", "hot-dog"),
            ("🍺", "beer-mug"),
            ("🍻", "clinking-beer-mugs"),
            ("🍷", "wine-glass"),
            ("☕", "hot-beverage"),
            ("❤️", "red-heart"),
            ("❤", "red-heart"),
            ("💔", "broken-heart"),
            ("💕", "two-hearts"),
            ("💖", "sparkling-heart"),
            ("💘", "heart-with-arrow"),
            ("💗", "growing-heart"),
            ("🔔", "bell"),
            ("🎨", "artist-palette"),
            ("hammer", "hammer"),
            ("🔨", "hammer"),
            ("🐷", "pig-face"),
            ("💋", "kiss-mark"),
            ("✨", "sparkles"),
            ("🧠", "brain"),
            ("💥", "collision"),
        ];

        let mut parts = Vec::new();
        let mut current_text = text.to_string();

        while !current_text.is_empty() {
            let mut earliest_match: Option<(usize, usize, &str)> = None;

            for &(code, emoji_name) in emoticons {
                if let Some(idx) = current_text.find(code) {
                    match earliest_match {
                        None => earliest_match = Some((idx, idx + code.len(), emoji_name)),
                        Some((earliest_idx, _, _)) if idx < earliest_idx => {
                            earliest_match = Some((idx, idx + code.len(), emoji_name));
                        }
                        _ => {}
                    }
                }
            }

            if let Some((start, end, emoji_name)) = earliest_match {
                if start > 0 {
                    let prev_text = current_text[..start].to_string();
                    parts.push(rsx! { span { "{prev_text}" } });
                }
                let e_url = crate::models::get_emoji_anim_url(&format!("{}.webp", emoji_name));
                let unicode_char = crate::models::get_emoji_unicode(emoji_name);
                parts.push(rsx! {
                    span { class: "inline-block align-middle mx-0.5 relative",
                        img {
                            src: "{e_url}",
                            class: "w-5 h-5 inline-block align-middle",
                            alt: "{emoji_name}",
                        }
                        span {
                            style: "display: none;",
                            class: "text-base inline-block align-middle leading-none",
                            "{unicode_char}"
                        }
                    }
                });
                current_text = current_text[end..].to_string();
            } else {
                parts.push(rsx! { span { "{current_text}" } });
                break;
            }
        }

        rsx! {
            span {
                for part in parts {
                    {part}
                }
            }
        }
    };

    rsx! {
        div {
            id: "{feed_id}",
            class: "flex-1 overflow-y-auto p-4 space-y-3 bg-transparent min-h-0",

            if chat_history.is_empty() {
                div { class: "h-full flex items-center justify-center text-slate-400 text-xs italic",
                    "Inicie uma conversa em {display_name}!"
                }
            } else {
                if show_load_more {
                    button {
                        class: "w-full py-1.5 text-center hover:bg-black/5 hover:text-sky-600 text-[10px] text-slate-500 font-semibold border border-dashed border-slate-350/70 rounded transition-all cursor-pointer mb-2.5 focus:outline-none",
                        onclick: move |_| {
                            limit.set(limit() + 15);
                        },
                        "Ver mensagens anteriores..."
                    }
                }
                for msg in visible_messages {
                    {
                        let name_color = if msg.sender_id == "0" { theme.titlebar_text() } else { "text-[#e6007e]" };

                        rsx! {
                            div { class: "flex flex-col space-y-0.5 text-xs text-slate-800 select-text",
                                if msg.is_nudge {
                                    div { class: "py-1.5 px-3 bg-red-100 border border-red-200 rounded text-red-700 font-bold flex items-center space-x-2 my-1 animate-pulse shadow-sm",
                                        span { "🔔" }
                                        span { "{msg.text}" }
                                        span { class: "text-[9px] text-red-500 font-normal ml-auto", "{msg.timestamp}" }
                                    }
                                } else if let Some(ref _wink_type) = msg.is_wink {
                                    div { class: "py-1.5 px-3 bg-purple-100 border border-purple-200 rounded text-purple-700 font-bold flex items-center space-x-2 my-1 animate-pulse shadow-sm",
                                        span { "✨" }
                                        span { "{msg.text}" }
                                        span { class: "text-[9px] text-purple-500 font-normal ml-auto", "{msg.timestamp}" }
                                    }
                                } else if msg.is_game_invite {
                                    {
                                        let is_my_invite = msg.sender_id == "0";
                                        let game_states = state.game_states();
                                        let active_game = game_states.get(&contact_id);
                                        let is_accepted = active_game.map(|g| g.accepted).unwrap_or(false);
                                        let cid_accept = contact_id.clone();
                                        let cid_reject = contact_id.clone();
                                        
                                        rsx! {
                                            div { class: "py-2 px-3 bg-emerald-100 border border-emerald-200 rounded text-emerald-700 font-bold flex flex-col space-y-1.5 my-1 shadow-sm",
                                                div { class: "flex items-center space-x-2",
                                                    span { "🎮" }
                                                    if is_accepted {
                                                        span { "Desafio de Jogo da Velha (Iniciado)" }
                                                    } else if is_my_invite {
                                                        span { "Você convidou {display_name} para jogar Jogo da Velha." }
                                                    } else {
                                                        span { "{msg.sender_name} convidou você para jogar Jogo da Velha." }
                                                    }
                                                    span { class: "text-[9px] text-emerald-600 font-normal ml-auto", "{msg.timestamp}" }
                                                }
                                                if !is_accepted && !is_my_invite {
                                                    div { class: "flex items-center space-x-2 text-[11px] font-normal pt-1",
                                                        button {
                                                            class: "px-2 py-0.5 {theme.btn_primary()} rounded font-bold cursor-pointer transition-colors",
                                                            onclick: move |_| state.accept_game_invite(cid_accept.clone()),
                                                            "Aceitar"
                                                        }
                                                        button {
                                                            class: "px-2 py-0.5 bg-white hover:bg-slate-100 border border-slate-350 rounded cursor-pointer transition-colors text-slate-700",
                                                            onclick: move |_| state.reject_game_invite(cid_reject.clone()),
                                                            "Recusar"
                                                        }
                                                    }
                                                } else if !is_accepted && is_my_invite {
                                                    div { class: "text-[10px] font-normal text-emerald-600 italic animate-pulse",
                                                        "Aguardando resposta do contato..."
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else if let Some(ref transfer) = msg.file_transfer {
                                    div { class: "py-2 px-3 bg-slate-100 border border-slate-200 rounded text-slate-700 font-bold flex flex-col space-y-1.5 my-1 shadow-sm",
                                        div { class: "flex items-center space-x-2",
                                            span { "📂" }
                                            span { "{msg.sender_name} enviou um convite de arquivo." }
                                            span { class: "text-[9px] text-slate-500 font-normal ml-auto", "{msg.timestamp}" }
                                        }
                                        {
                                            match transfer {
                                                FileTransferState::Waiting => {
                                                    if msg.sender_id != "0" {
                                                        let cid_accept = contact_id.clone();
                                                        let cid_reject = contact_id.clone();
                                                        let mid = msg.id.clone();
                                                        let mid_reject = msg.id.clone();
                                                        rsx! {
                                                            div { class: "flex items-center space-x-2 text-[11px] font-normal pt-1",
                                                                span { "Arquivo pendente: {msg.text}" }
                                                                button {
                                                                    class: "px-2 py-0.5 {theme.btn_primary()} rounded font-bold cursor-pointer transition-colors",
                                                                    onclick: move |_| state.accept_file_transfer(cid_accept.clone(), mid.clone()),
                                                                    "Aceitar"
                                                                }
                                                                button {
                                                                    class: "px-2 py-0.5 bg-white hover:bg-slate-100 border border-slate-355 rounded cursor-pointer transition-colors",
                                                                    onclick: move |_| state.reject_file_transfer(cid_reject.clone(), mid_reject.clone()),
                                                                    "Recusar"
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        rsx! { span { class: "text-[11px] font-normal text-slate-500", "Aguardando resposta do contato..." } }
                                                    }
                                                }
                                                FileTransferState::Downloading(prog) => {
                                                    rsx! {
                                                        div { class: "flex flex-col space-y-1 pt-1 font-normal text-[11px] w-full",
                                                            span { "Baixando: {prog}%" }
                                                            div { class: "w-full h-2 bg-white rounded-full overflow-hidden border {theme.modal_border()}",
                                                                div { class: "h-full {theme.btn_primary()} transition-all duration-300", style: "width: {prog}%;" }
                                                            }
                                                        }
                                                    }
                                                }
                                                FileTransferState::Completed(filename) => {
                                                    let is_image_visible = show_images().contains(&msg.id);
                                                    let mid = msg.id.clone();
                                                    rsx! {
                                                        div { class: "flex flex-col space-y-1 pt-1 font-normal text-[11px]",
                                                            div { class: "flex items-center space-x-2",
                                                                span { "✓ Transferência Concluída: {filename}" }
                                                                if filename.ends_with(".jpg") {
                                                                    button {
                                                                        class: "{theme.titlebar_text()} hover:underline font-bold cursor-pointer transition-all",
                                                                        onclick: move |_| {
                                                                            if show_images().contains(&mid) {
                                                                                show_images.write().remove(&mid);
                                                                            } else {
                                                                                show_images.write().insert(mid.clone());
                                                                            }
                                                                        },
                                                                        if is_image_visible { "Ocultar Foto" } else { "Visualizar Foto" }
                                                                    }
                                                                }
                                                            }
                                                            if is_image_visible {
                                                                    div { class: "w-32 h-32 border border-slate-300 rounded mt-1 bg-white p-1 shadow flex items-center justify-center overflow-hidden",
                                                                        img {
                                                                            src: "https://picsum.photos/150/150?random={msg.id}",
                                                                            class: "w-full h-full object-cover rounded-sm"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                FileTransferState::Rejected => {
                                                    rsx! { span { class: "text-[11px] font-normal text-red-500/80 italic", "Transferência cancelada ou rejeitada." } }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    div { class: "flex items-baseline space-x-1",
                                        span { class: "font-bold {name_color}", "{msg.sender_name} diz:" }
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
    }
}
