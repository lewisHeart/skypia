use serde::{Deserialize, Serialize};
use super::user::UserProfile;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
