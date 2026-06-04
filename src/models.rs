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
    pub id: usize,
    pub email: String,
    pub display_name: String,
    pub status: UserStatus,
    pub personal_message: String,
    pub music_listening: Option<String>,
    pub avatar_id: usize, // 0-9
    pub is_favorite: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub id: usize,
    pub conversation_id: usize,
    pub sender_id: usize, // 0 for local user, others for contacts
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

pub fn render_avatar(id: usize, size_px: usize) -> Element {
    match id {
        1 => {
            rsx! {
                svg { view_box: "0 0 100 100", width: "{size_px}px", height: "{size_px}px", class: "rounded-md",
                    rect { width: "100", height: "100", rx: "10", fill: "#1a1a2e" }
                    path { d: "M20 30 L40 10 L50 35 L65 5 L75 35 L90 20 L80 60 L20 60 Z", fill: "#ff007f" }
                    path { d: "M15 35 Q40 50 65 30 Q75 60 85 35 L80 65 L20 65 Z", fill: "#111111" }
                    polygon { points: "50,25 55,40 70,40 58,50 62,65 50,55 38,65 42,50 30,40 45,40", fill: "#00fff0" }
                }
            }
        }
        2 => {
            rsx! {
                svg { view_box: "0 0 100 100", width: "{size_px}px", height: "{size_px}px", class: "rounded-md",
                    rect { width: "100", height: "100", rx: "10", fill: "#ffe5ec" }
                    circle { cx: "25", cy: "45", r: "14", fill: "#e07a5f" }
                    circle { cx: "75", cy: "45", r: "14", fill: "#e07a5f" }
                    circle { cx: "50", cy: "50", r: "28", fill: "#f4f1de" }
                    path { d: "M22 50 Q50 30 78 50 C70 32 30 32 22 50 Z", fill: "#e07a5f" }
                    circle { cx: "42", cy: "52", r: "3", fill: "#3d405b" }
                    circle { cx: "58", cy: "52", r: "3", fill: "#3d405b" }
                    path { d: "M45 62 Q50 67 55 62", stroke: "#e07a5f", stroke_width: "2.5", fill: "none" }
                }
            }
        }
        3 => {
            rsx! {
                svg { view_box: "0 0 100 100", width: "{size_px}px", height: "{size_px}px", class: "rounded-md",
                    rect { width: "100", height: "100", rx: "10", fill: "#2a9d8f" }
                    rect { x: "20", y: "35", width: "60", height: "30", rx: "12", fill: "#e76f51" }
                    circle { cx: "35", cy: "50", r: "5", fill: "#264653" }
                    circle { cx: "60", cy: "50", r: "4", fill: "#e9c46a" }
                    circle { cx: "70", cy: "50", r: "4", fill: "#e9c46a" }
                }
            }
        }
        4 => {
            rsx! {
                svg { view_box: "0 0 100 100", width: "{size_px}px", height: "{size_px}px", class: "rounded-md",
                    rect { width: "100", height: "100", rx: "10", fill: "#ffccd5" }
                    path { d: "M12,38 C12,22 35,15 50,35 C65,15 88,22 88,38 C88,60 50,85 50,85 C50,85 12,60 12,38 Z", fill: "#ff4d6d" }
                    polygon { points: "25,20 28,26 34,27 29,32 30,38 25,34 20,38 21,32 16,27 22,26", fill: "#fff" }
                    polygon { points: "75,65 77,69 82,70 78,74 79,79 75,76 71,79 72,74 68,70 73,69", fill: "#fff" }
                }
            }
        }
        5 => {
            rsx! {
                svg { view_box: "0 0 100 100", width: "{size_px}px", height: "{size_px}px", class: "rounded-md",
                    rect { width: "100", height: "100", rx: "10", fill: "#1c1c1e" }
                    circle { cx: "32", cy: "68", r: "10", fill: "#00d2ff" }
                    circle { cx: "68", cy: "58", r: "10", fill: "#00d2ff" }
                    rect { x: "37", y: "25", width: "5", height: "43", fill: "#00d2ff" }
                    rect { x: "73", y: "15", width: "5", height: "43", fill: "#00d2ff" }
                    path { d: "M37 25 L78 15 L78 25 L37 35 Z", fill: "#00d2ff" }
                }
            }
        }
        6 => {
            rsx! {
                svg { view_box: "0 0 100 100", width: "{size_px}px", height: "{size_px}px", class: "rounded-md",
                    rect { width: "100", height: "100", rx: "10", fill: "#e9c46a" }
                    path { d: "M10 65 Q 30 55, 50 65 T 90 65 L 90 90 L 10 90 Z", fill: "#264653" }
                    path { d: "M10 75 Q 30 68, 50 75 T 90 75 L 90 90 L 10 90 Z", fill: "#2a9d8f" }
                    circle { cx: "50", cy: "40", r: "18", fill: "#f4a261" }
                }
            }
        }
        _ => {
            rsx! {
                svg { view_box: "0 0 100 100", width: "{size_px}px", height: "{size_px}px", class: "rounded-md",
                    rect { width: "100", height: "100", rx: "10", fill: "#aed2f2" }
                    circle { cx: "50", cy: "35", r: "18", fill: "#0078d7" }
                    path { d: "M15 85 C15 62, 85 62, 85 85 Z", fill: "#0078d7" }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Conversation {
    pub id: usize,
    pub name: Option<String>,
    pub is_group: bool,
    pub created_at: String,
    pub members: Vec<UserProfile>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserProfile {
    pub id: i64,
    pub email: String,
    pub display_name: String,
    pub personal_message: String,
    pub status: String,
    pub music: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsEvent {
    ChatMessage(Message),
    PresenceUpdate {
        user_id: i64,
        status: String,
        personal_message: String,
        music: Option<String>,
        avatar_url: Option<String>,
    },
    Nudge {
        conversation_id: i64,
        sender_id: i64,
        sender_name: String,
    },
    Typing {
        conversation_id: i64,
        user_id: i64,
        is_typing: bool,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ClientAction {
    SendMessage {
        conversation_id: i64,
        text: String,
        font_color: String,
        font_family: String,
    },
    SendNudge {
        conversation_id: i64,
    },
    SetTyping {
        conversation_id: i64,
        is_typing: bool,
    },
}

