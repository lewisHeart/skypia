#![allow(dead_code)]
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowPos {
    pub x: i32,
    pub y: i32,
    pub is_dragging: bool,
    pub drag_offset_x: i32,
    pub drag_offset_y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UserStatus {
    Online,
    Ocupado,
    Ausente,
    Invisivel,
    Offline,
}

impl UserStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserStatus::Online => "Disponível",
            UserStatus::Ocupado => "Ocupado",
            UserStatus::Ausente => "Ausente",
            UserStatus::Invisivel => "Invisível",
            UserStatus::Offline => "Offline",
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            UserStatus::Online => "bg-[#3cd070] border-[#2fa558]", // Green
            UserStatus::Ocupado => "bg-[#e81123] border-[#b50a18]", // Red
            UserStatus::Ausente => "bg-[#ffb900] border-[#c99200]", // Orange/Yellow
            UserStatus::Invisivel => "bg-[#7a7a7a] border-[#5a5a5a]", // Gray
            UserStatus::Offline => "bg-[#7a7a7a] border-[#5a5a5a]", // Gray
        }
    }

    pub fn avatar_frame_class(&self) -> &'static str {
        match self {
            UserStatus::Online => "bg-gradient-to-b from-[#a9f54b] via-[#85e028] to-[#5db30e] border-[#5db30e] shadow-[0_0_3px_rgba(133,224,40,0.35)]",
            UserStatus::Ocupado => "bg-gradient-to-b from-[#ff8c8c] via-[#ff4747] to-[#d61818] border-[#d61818] shadow-[0_0_3px_rgba(255,71,71,0.35)]",
            UserStatus::Ausente => "bg-gradient-to-b from-[#ffeb8c] via-[#ffcb47] to-[#e69d00] border-[#e69d00] shadow-[0_0_3px_rgba(255,203,71,0.35)]",
            UserStatus::Invisivel => "bg-gradient-to-b from-[#fafafa] via-[#e3e3e3] to-[#c2c2c2] border-[#b5b5b5] shadow-sm",
            UserStatus::Offline => "bg-gradient-to-b from-[#fafafa] via-[#e3e3e3] to-[#c2c2c2] border-[#b5b5b5] shadow-sm",
        }
    }

    pub fn text_color(&self) -> &'static str {
        match self {
            UserStatus::Online => "text-green-600",
            UserStatus::Ocupado => "text-red-500",
            UserStatus::Ausente => "text-amber-500",
            UserStatus::Invisivel => "text-gray-500",
            UserStatus::Offline => "text-gray-400",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FileTransferState {
    Waiting,
    Downloading(u8),   // progresso de 0 a 100
    Completed(String), // nome da imagem/arquivo
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TicTacToeCell {
    Empty,
    X,
    O,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TicTacToe {
    pub board: [TicTacToeCell; 9],
    pub turn: TicTacToeCell,
    pub winner: Option<TicTacToeCell>,
    pub is_draw: bool,
    pub active: bool,
    pub accepted: bool,
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [TicTacToeCell::Empty; 9],
            turn: TicTacToeCell::X,
            winner: None,
            is_draw: false,
            active: true,
            accepted: false,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct UserSettings {
    pub interface_scale: f64,
    pub use_custom_titlebar: bool,
    pub theme: String,
    pub chat_mode: String,
    pub contact_density: String,
    pub font_color: String,
    pub font_family: String,
    pub spotify_rpc_enabled: bool,
    pub show_typing_notification: bool,
    pub enable_sounds: bool,
    pub enable_toasts: bool,
    pub download_folder: String,
    pub auto_accept_files: bool,
    pub remember_password: bool,
    pub save_chat_history: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Contact {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub status: UserStatus,
    pub personal_message: String,
    pub music_listening: Option<String>,
    pub avatar_url: Option<String>,
    pub is_favorite: bool,
    pub relation_status: String,
    pub nickname: Option<String>,
    pub category_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BannerInfo {
    pub text: String,
    pub action_label: String,
    pub link: String,
    pub icon: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AppTheme {
    AeroBlue,
    RubyPink,
    ForestGreen,
    SilverMetallic,
}

impl AppTheme {
    pub fn name(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "Azul Aero",
            AppTheme::RubyPink => "Rosa Choque",
            AppTheme::ForestGreen => "Verde Natureza",
            AppTheme::SilverMetallic => "Prata Metálico",
        }
    }

    pub fn bg_gradient(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "from-[#c2ddf4] via-[#ffffff] to-[#eff8fa]",
            AppTheme::RubyPink => "from-[#fcd5ce] via-[#ffffff] to-[#fde2e4]",
            AppTheme::ForestGreen => "from-[#c8e6c9] via-[#ffffff] to-[#e8f5e9]",
            AppTheme::SilverMetallic => "from-[#e5e5ea] via-[#ffffff] to-[#f5f5f7]",
        }
    }

    pub fn glass_bg(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "bg-[#ffffff]/50 border-[#8ab8e6]/60 shadow-[#4b7cb6]/20",
            AppTheme::RubyPink => "bg-[#ffffff]/55 border-[#f5b3b5]/60 shadow-[#e07a7f]/20",
            AppTheme::ForestGreen => "bg-[#ffffff]/50 border-[#a2cfab]/60 shadow-[#5b9666]/20",
            AppTheme::SilverMetallic => "bg-[#ffffff]/60 border-[#c7c7cc]/60 shadow-[#8e8e93]/20",
        }
    }

    pub fn accent_color(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "text-[#1d528f] bg-[#1d528f]",
            AppTheme::RubyPink => "text-[#a81c43] bg-[#a81c43]",
            AppTheme::ForestGreen => "text-[#2e6930] bg-[#2e6930]",
            AppTheme::SilverMetallic => "text-[#3a3a3c] bg-[#3a3a3c]",
        }
    }
    pub fn glass_border_color(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "border-[#5c98d6]",
            AppTheme::RubyPink => "border-[#ea888e]",
            AppTheme::ForestGreen => "border-[#85c290]",
            AppTheme::SilverMetallic => "border-[#b0b0b8]",
        }
    }

    pub fn titlebar_gradient(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "from-[#e3effa] via-[#edf5fc] to-[#f4f9fd]",
            AppTheme::RubyPink => "from-[#fae6e8] via-[#fcf0f1] to-[#fdf7f7]",
            AppTheme::ForestGreen => "from-[#e7f4e9] via-[#f1f9f2] to-[#f7fcf8]",
            AppTheme::SilverMetallic => "from-[#f2f2f4] via-[#f7f7f9] to-[#fafafb]",
        }
    }

    pub fn titlebar_border(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "border-[#a6b9cd]/45",
            AppTheme::RubyPink => "border-[#dfacb0]/45",
            AppTheme::ForestGreen => "border-[#accbad]/45",
            AppTheme::SilverMetallic => "border-[#bcbcbc]/45",
        }
    }

    pub fn titlebar_text(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "text-[#2d517a]",
            AppTheme::RubyPink => "text-[#5a2024]",
            AppTheme::ForestGreen => "text-[#1d3d20]",
            AppTheme::SilverMetallic => "text-[#333333]",
        }
    }

    pub fn bg_chat(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "linear-gradient(180deg, #c2ddf4 0%, #ffffff 15%, #ffffff 89%, #eff8fa 100%)",
            AppTheme::RubyPink => "linear-gradient(180deg, rgba(254, 230, 232, 0.95) 0%, rgba(253, 203, 196, 0.9) 100%)",
            AppTheme::ForestGreen => "linear-gradient(180deg, rgba(232, 245, 233, 0.95) 0%, rgba(175, 224, 177, 0.9) 100%)",
            AppTheme::SilverMetallic => "linear-gradient(180deg, rgba(245, 245, 247, 0.95) 0%, rgba(209, 209, 214, 0.9) 100%)",
        }
    }

    pub fn modal_gradient(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "from-[#e6f1fc] to-[#c8def5]",
            AppTheme::RubyPink => "from-[#fae6e8] to-[#f5b3b5]",
            AppTheme::ForestGreen => "from-[#e7f4e9] to-[#a2cfab]",
            AppTheme::SilverMetallic => "from-[#f2f2f4] to-[#d1d1d6]",
        }
    }

    pub fn modal_border(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "border-[#7ba9d4]",
            AppTheme::RubyPink => "border-[#ea888e]",
            AppTheme::ForestGreen => "border-[#85c290]",
            AppTheme::SilverMetallic => "border-[#b0b0b8]",
        }
    }

    pub fn btn_primary(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "bg-gradient-to-b from-[#8fc1e9] via-[#5c98d6] to-[#4585c5] hover:from-[#9bd0fa] hover:via-[#70abeb] hover:to-[#579adf] text-white border-[#4074a8]",
            AppTheme::RubyPink => "bg-gradient-to-b from-[#f5b3b5] via-[#ea888e] to-[#a81c43] hover:from-[#ffc4c6] hover:via-[#f59e9f] hover:to-[#c82255] text-white border-[#a81c43]",
            AppTheme::ForestGreen => "bg-gradient-to-b from-[#a2cfab] via-[#85c290] to-[#2e6930] hover:from-[#b5dec0] hover:via-[#9cd0ab] hover:to-[#387e3a] text-white border-[#2e6930]",
            AppTheme::SilverMetallic => "bg-gradient-to-b from-[#d1d1d6] via-[#aeaea2] to-[#3a3a3c] hover:from-[#e5e5ea] hover:via-[#c7c7cc] hover:to-[#48484a] text-white border-[#3a3a3c]",
        }
    }

    pub fn tooltip_bg(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "from-sky-50 to-sky-100/95 border-[#a6b9cd]",
            AppTheme::RubyPink => "from-[#fff3f3] to-[#ffd7db]/95 border-[#ea888e]",
            AppTheme::ForestGreen => "from-[#f4fbf5] to-[#dcf5e1]/95 border-[#85c290]",
            AppTheme::SilverMetallic => "from-[#fafafb] to-[#e5e5ea]/95 border-[#b0b0b8]",
        }
    }

    pub fn toast_gradient(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "linear-gradient(135deg, rgba(240, 248, 255, 0.95) 0%, rgba(215, 235, 252, 0.95) 100%)",
            AppTheme::RubyPink => "linear-gradient(135deg, rgba(255, 243, 243, 0.95) 0%, rgba(255, 215, 219, 0.95) 100%)",
            AppTheme::ForestGreen => "linear-gradient(135deg, rgba(244, 251, 245, 0.95) 0%, rgba(220, 245, 225, 0.95) 100%)",
            AppTheme::SilverMetallic => "linear-gradient(135deg, rgba(250, 250, 251, 0.95) 0%, rgba(229, 229, 234, 0.95) 100%)",
        }
    }
}

use std::sync::{Mutex, LazyLock};
use std::collections::HashMap;

static AVATAR_CACHE: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

#[component]
pub fn Avatar(url: Option<String>, size: usize) -> Element {
    let final_url = match url {
        Some(ref u) if u.starts_with("http") => u.to_string(),
        Some(ref u)
            if u.starts_with("/assets/")
                || u.starts_with("assets/")
                || u.starts_with("/_assets/")
                || u.starts_with("_assets/")
                || u.starts_with("dioxus-asset://") =>
        {
            u.to_string()
        }
        Some(ref u) if !u.is_empty() => format!("{}{}", crate::services::api::SERVER_BASE_URL, u),
        _ => "".to_string(),
    };

    // Obter do cache de forma síncrona
    let cached = if final_url.is_empty() {
        None
    } else {
        AVATAR_CACHE.lock().unwrap().get(&final_url).cloned()
    };

    let url_to_fetch = final_url.clone();
    let has_cache = cached.is_some();

    // Recurso reativo para fazer o fetch assíncrono em segundo plano se não estiver em cache
    let avatar_resource = use_resource(move || {
        let url = url_to_fetch.clone();
        async move {
            if url.is_empty() || has_cache {
                return None;
            }
            if !url.starts_with("http") {
                return Some(url);
            }
            match reqwest::get(&url).await {
                Ok(resp) => {
                    if let Ok(bytes) = resp.bytes().await {
                        use base64::Engine;
                        let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                        let mime = if url.contains(".gif") {
                            "image/gif"
                        } else if url.contains(".png") {
                            "image/png"
                        } else {
                            "image/jpeg"
                        };
                        let data_uri = format!("data:{};base64,{}", mime, b64);
                        AVATAR_CACHE.lock().unwrap().insert(url.clone(), data_uri.clone());
                        Some(data_uri)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
    });

    let display_url = if let Some(ref c) = cached {
        c.clone()
    } else if let Some(Some(ref res_url)) = *avatar_resource.value().read() {
        res_url.clone()
    } else {
        final_url.clone()
    };

    let is_loading = cached.is_none() && final_url.starts_with("http") && avatar_resource.value().read().is_none();
    
    // Se o fetch assíncrono terminou e retornou None, consideramos falha e exibimos o fallback.
    let show_fallback = display_url.is_empty() || (!is_loading && cached.is_none() && final_url.starts_with("http") && avatar_resource.value().read().as_ref().map(|x| x.is_none()).unwrap_or(true));

    if !show_fallback {
        rsx! {
            img {
                src: "{display_url}",
                width: "{size}px",
                height: "{size}px",
                class: "rounded-[4px] object-cover flex-shrink-0 border border-slate-350 shadow-inner",
                alt: "Avatar"
            }
        }
    } else {
        rsx! {
            svg {
                view_box: "0 0 100 100",
                width: "{size}px",
                height: "{size}px",
                class: "rounded-[4px] flex-shrink-0 border border-slate-300 shadow-sm",
                defs {
                    linearGradient { id: "msnGrad", x1: "0%", y1: "0%", x2: "100%", y2: "100%",
                        stop { offset: "0%", stop_color: "#e6f2ff" }
                        stop { offset: "100%", stop_color: "#bcd6f7" }
                    }
                }
                rect { width: "100", height: "100", rx: "4", fill: "url(#msnGrad)" }
                // Boneco clássico do MSN azul/verde
                circle { cx: "44", cy: "38", r: "13", fill: "#3b82f6" }
                path { d: "M20 76 C20 58, 68 58, 68 76 Z", fill: "#3b82f6" }
                circle { cx: "66", cy: "48", r: "10", fill: "#22c55e" }
                path { d: "M48 76 C48 64, 84 64, 84 76 Z", fill: "#22c55e" }
            }
        }
    }
}

pub fn render_avatar(url_opt: Option<&str>, size_px: usize) -> Element {
    rsx! {
        Avatar {
            url: url_opt.map(|s| s.to_string()),
            size: size_px,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Conversation {
    pub id: String,
    pub name: Option<String>,
    pub is_group: bool,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub members: Vec<UserProfile>,
    pub allow_member_send: Option<bool>,
    pub allow_member_invite: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub username: String,
    pub full_name: String,
    pub display_name: String,
    pub personal_message: String,
    pub status: String,
    pub music: Option<String>,
    pub avatar_url: Option<String>,
    pub relation_status: Option<String>,
    pub nickname: Option<String>,
    pub role: Option<String>,
    pub is_favorite: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    #[cfg(feature = "desktop")]
    {
        format!("emojis://{}", name)
    }
    #[cfg(not(feature = "desktop"))]
    {
        format!("/emojis/{}", name)
    }
}

pub fn get_emoji_anim_url(name: &str) -> String {
    #[cfg(feature = "desktop")]
    {
        format!("emojis-anim://{}", name)
    }
    #[cfg(not(feature = "desktop"))]
    {
        format!("/emojis_anim/{}", name)
    }
}

