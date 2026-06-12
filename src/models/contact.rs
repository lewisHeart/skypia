use serde::{Deserialize, Serialize};
use super::user::UserStatus;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
