use serde::{Deserialize, Serialize};
use dioxus::prelude::*;
use super::user::UserProfile;
use super::group::Conversation;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileTransferState {
    Waiting,
    Downloading(u8),   // progresso de 0 a 100
    Completed(String), // nome da imagem/arquivo
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub sender_id: String, // "0" for local user, others for contacts
    pub sender_name: String,
    pub text: String,
    pub timestamp: String,
    pub is_nudge: bool,
    pub font_color: String,      // hex code, e.g., "#0000ff"
    pub font_family: String,     // e.g., "Segoe UI", "Comic Sans MS"
    pub is_wink: Option<String>, // Some("kiss", "hammer", "pig")
    pub file_transfer: Option<FileTransferState>,
    pub is_game_invite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsEvent {
    ChatMessage(Message),
    PresenceUpdate {
        user_id: String,
        status: String,
        personal_message: String,
        music: Option<String>,
        avatar_url: Option<String>,
        display_name: String,
    },
    Nudge {
        conversation_id: String,
        sender_id: String,
        sender_name: String,
    },
    Typing {
        conversation_id: String,
        user_id: String,
        is_typing: bool,
    },
    ContactRequestReceived {
        requester: UserProfile,
    },
    ContactRequestAccepted {
        contact: UserProfile,
    },
    ContactAdded {
        contact: UserProfile,
    },
    ContactBlocked {
        contact_id: String,
        blocked: bool,
    },
    ContactRemoved {
        contact_id: String,
    },
    NicknameUpdated {
        contact_id: String,
        nickname: Option<String>,
    },
    FavoriteUpdated {
        contact_id: String,
        is_favorite: bool,
    },
    ConversationJoined(Conversation),
    BannerUpdated {
        text: String,
        action_label: String,
        link: String,
        icon: String,
        image_url: Option<String>,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ClientAction {
    SendMessage {
        conversation_id: String,
        text: String,
        font_color: String,
        font_family: String,
    },
    SendNudge {
        conversation_id: String,
    },
    SetTyping {
        conversation_id: String,
        is_typing: bool,
    },
    UpdatePresence {
        status: Option<String>,
        personal_message: Option<String>,
        music: Option<Option<String>>,
        display_name: Option<String>,
    },
    AddContact {
        email_or_username: String,
    },
    AcceptContact {
        contact_id: String,
    },
    RejectContact {
        contact_id: String,
    },
    BlockContact {
        contact_id: String,
        block: bool,
    },
    SetNickname {
        contact_id: String,
        nickname: Option<String>,
    },
    SetFavorite {
        contact_id: String,
        is_favorite: bool,
    },
}

pub fn parse_emoticons_inline(text: &str, size_class: &str) -> Element {
    let emoticons = &[
        (":)", "smiling-face"),
        (":-)", "smiling-face"),
        (":(", "frowning-face"),
        (":-(", "frowning-face"),
        (";)", "winking-face"),
        (";-)", "winking-face"),
        (":P", "face-with-tongue"),
        (":p", "face-with-tongue"),
        (":-P", "face-with-tongue"),
        (":-p", "face-with-tongue"),
        ("(H)", "smiling-face-with-sunglasses"),
        ("(h)", "smiling-face-with-sunglasses"),
        ("(A)", "smiling-face-with-halo"),
        ("(a)", "smiling-face-with-halo"),
        (":@", "angry-face"),
        ("(6)", "smiling-face-with-horns"),
        ("(L)", "red-heart"),
        ("(l)", "red-heart"),
        ("(U)", "broken-heart"),
        ("(u)", "broken-heart"),
        ("(M)", "musical-note"),
        ("(m)", "musical-note"),
        ("(F)", "floppy-disk"),
        ("(f)", "floppy-disk"),
        ("(I)", "framed-picture"),
        ("(i)", "framed-picture"),
        ("(S)", "sparkles"),
        ("(s)", "sparkles"),
        ("(B)", "brain"),
        ("(b)", "brain"),
        ("(C)", "collision"),
        ("(c)", "collision"),
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
            let e_url = get_emoji_url(&format!("{}.svg", emoji_name));
            parts.push(rsx! {
                img {
                     src: "{e_url}",
                     class: "{size_class} inline-block align-middle mx-0.5",
                     alt: "{emoji_name}"
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
}

pub fn get_emoji_url(name: &str) -> String {
    let base_name = name.trim_end_matches(".svg").trim_end_matches(".webp").trim_end_matches(".png").trim_end_matches(".gif");
    let anim_url = crate::emoji_assets::get_emoji_anim_asset(base_name);
    if !anim_url.is_empty() {
        anim_url
    } else {
        crate::emoji_assets::get_emoji_static_asset(base_name)
    }
}

pub fn get_emoji_anim_url(name: &str) -> String {
    get_emoji_url(name)
}

pub fn get_emoji_unicode(name: &str) -> &'static str {
    match name {
        "slightly-smiling-face" => "😊",
        "grinning-face-with-big-eyes" => "😃",
        "winking-face" => "😉",
        "face-with-tongue" => "😜",
        "face-with-open-mouth" => "😮",
        "flushed-face" => "😳",
        "pouting-face" => "😡",
        "confused-face" => "😕",
        "smiling-face-with-sunglasses" => "😎",
        "thumbs-up" => "👍",
        "thumbs-down" => "👎",
        "kiss-mark" => "💋",
        "smiling-face-with-halo" => "😇",
        "red-heart" => "❤️",
        "broken-heart" => "💔",
        "alarm-clock" => "⏰",
        "wrapped-gift" => "🎁",
        "wilted-flower" => "🥀",
        "camera" => "📷",
        "musical-note" => "🎵",
        "crescent-moon" => "🌙",
        "star" => "⭐",
        "envelope" => "✉️",
        "hot-beverage" => "☕",
        "smiling-face-with-heart-eyes" => "😍",
        "face-blowing-a-kiss" => "😘",
        "squinting-face-with-tongue" => "😝",
        "zany-face" => "🤪",
        "shushing-face" => "🤫",
        "thinking-face" => "🤔",
        "expressionless-face" => "😑",
        "smirking-face" => "😏",
        "grimacing-face" => "😬",
        "drooling-face" => "🤤",
        "sleeping-face" => "😴",
        "nauseated-face" => "🤢",
        "face-vomiting" => "🤮",
        "exploding-head" => "🤯",
        "partying-face" => "🥳",
        "woozy-face" => "🥴",
        "crying-face" => "😢",
        "loudly-crying-face" => "😭",
        "face-screaming-in-fear" => "😱",
        "angry-face" => "😠",
        "face-with-symbols-on-mouth" => "🤬",
        "skull" => "💀",
        "pile-of-poo" => "💩",
        "clapping-hands" => "👏",
        "handshake" => "🤝",
        "victory-hand" => "✌️",
        "flexed-biceps" => "💪",
        "folded-hands" => "🙏",
        "brain" => "🧠",
        "fire" => "🔥",
        "collision" => "💥",
        "sparkles" => "✨",
        "balloon" => "🎈",
        "party-popper" => "🎉",
        "rainbow" => "🌈",
        "sun" => "☀️",
        "snowflake" => "❄️",
        "umbrella" => "☔",
        "dog-face" => "🐶",
        "cat-face" => "🐱",
        "panda" => "🐼",
        "alien" => "👽",
        "rocket" => "🚀",
        "airplane" => "✈️",
        "beer-mug" => "🍺",
        "pizza" => "🍕",
        "money-bag" => "💰",
        "trophy" => "🏆",
        _ => "😊",
    }
}
