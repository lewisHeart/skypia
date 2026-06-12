use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowPos {
    pub x: i32,
    pub y: i32,
    pub is_dragging: bool,
    pub drag_offset_x: i32,
    pub drag_offset_y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub saved_email: String,
    pub saved_password: String,
    pub auto_login: bool,
    pub window_x: i32,
    pub window_y: i32,
    pub window_width: f64,
    pub window_height: f64,
    pub fav_collapsed: bool,
    pub online_collapsed: bool,
    pub offline_collapsed: bool,
    pub groups_collapsed: bool,
    pub collapsed_categories: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub is_admin: Option<bool>,
}
