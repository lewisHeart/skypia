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
    Downloading(u8), // progresso de 0 a 100
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
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [TicTacToeCell::Empty; 9],
            turn: TicTacToeCell::X,
            winner: None,
            is_draw: false,
            active: true,
        }
    }
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
    pub font_color: String,  // hex code, e.g., "#0000ff"
    pub font_family: String, // e.g., "Segoe UI", "Comic Sans MS"
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
            AppTheme::AeroBlue => "from-[#d3e5f5] via-[#aed2f2] to-[#7eb5e6]",
            AppTheme::RubyPink => "from-[#fde2e4] via-[#ffb5a7] to-[#fcd5ce]",
            AppTheme::ForestGreen => "from-[#e8f5e9] via-[#c8e6c9] to-[#a5d6a7]",
            AppTheme::SilverMetallic => "from-[#f5f5f7] via-[#e5e5ea] to-[#d1d1d6]",
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
            AppTheme::AeroBlue => "text-[#1b324d]",
            AppTheme::RubyPink => "text-[#5a2024]",
            AppTheme::ForestGreen => "text-[#1d3d20]",
            AppTheme::SilverMetallic => "text-[#333333]",
        }
    }
}

pub fn render_avatar(url_opt: Option<&str>, size_px: usize) -> Element {
    let final_url = match url_opt {
        Some(url) if url.starts_with("http") => url.to_string(),
        Some(url) => format!("{}{}", crate::services::api::SERVER_BASE_URL, url),
        None => "".to_string(),
    };

    if !final_url.is_empty() {
        rsx! {
            img {
                src: "{final_url}",
                width: "{size_px}px",
                height: "{size_px}px",
                class: "rounded-md object-cover flex-shrink-0 border border-slate-350 shadow-inner",
                alt: "Avatar",
                onerror: move |_| {
                    // Se falhar o carregamento, renderiza o fallback (nada a fazer pois final_url não é editável na hora, mas é um bom placeholder)
                }
            }
        }
    } else {
        rsx! {
            svg {
                view_box: "0 0 100 100",
                width: "{size_px}px",
                height: "{size_px}px",
                class: "rounded-md flex-shrink-0 border border-slate-300 shadow-sm",
                defs {
                    linearGradient { id: "msnGrad", x1: "0%", y1: "0%", x2: "100%", y2: "100%",
                        stop { offset: "0%", stop_color: "#e6f2ff" }
                        stop { offset: "100%", stop_color: "#bcd6f7" }
                    }
                }
                rect { width: "100", height: "100", rx: "10", fill: "url(#msnGrad)" }
                // Boneco clássico do MSN azul/verde
                // Cabeça azul
                circle { cx: "44", cy: "38", r: "13", fill: "#3b82f6" }
                // Corpo azul
                path { d: "M20 76 C20 58, 68 58, 68 76 Z", fill: "#3b82f6" }
                // Cabeça verde (parceiro clássico)
                circle { cx: "66", cy: "48", r: "10", fill: "#22c55e" }
                // Corpo verde
                path { d: "M48 76 C48 64, 84 64, 84 76 Z", fill: "#22c55e" }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Conversation {
    pub id: String,
    pub name: Option<String>,
    pub is_group: bool,
    pub created_at: String,
    pub members: Vec<UserProfile>,
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
}

