use crate::models::{Message, FileTransferState};
use crate::services::db::{get_pool, DatabaseService};
use sqlx::Row;

impl DatabaseService {
    pub async fn load_messages(conversation_id: String) -> Result<Vec<Message>, String> {
        let pool = get_pool();
        let rows = sqlx::query(
            "SELECT id, conversation_id, sender_id, sender_name, text, timestamp, is_nudge, font_color, font_family, is_wink, file_transfer, is_game_invite FROM messages WHERE conversation_id = ? ORDER BY id",
        )
        .bind(&conversation_id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let messages = rows
            .iter()
            .map(|row| {
                let id: String = row.get("id");
                let conv_id: String = row.get("conversation_id");
                let sender_id: String = row.get("sender_id");
                let is_nudge: i32 = row.get("is_nudge");
                let is_game_invite: i32 = row.get("is_game_invite");
                let is_wink: Option<String> = row.get("is_wink");
                let file_transfer_str: Option<String> = row.get("file_transfer");

                let file_transfer = file_transfer_str.and_then(|s| {
                    serde_json::from_str::<FileTransferState>(&s).ok()
                });

                Message {
                    id,
                    conversation_id: conv_id,
                    sender_id,
                    sender_name: row.get("sender_name"),
                    text: row.get("text"),
                    timestamp: row.get("timestamp"),
                    is_nudge: is_nudge != 0,
                    font_color: row.get("font_color"),
                    font_family: row.get("font_family"),
                    is_wink,
                    file_transfer,
                    is_game_invite: is_game_invite != 0,
                }
            })
            .collect();

        Ok(messages)
    }

    pub async fn save_message(conversation_id: String, message: Message) -> Result<(), String> {
        let pool = get_pool();

        let file_transfer_str = message
            .file_transfer
            .as_ref()
            .and_then(|ft| serde_json::to_string(ft).ok());

        sqlx::query(
            "INSERT INTO messages (id, conversation_id, sender_id, sender_name, text, timestamp, is_nudge, font_color, font_family, is_wink, file_transfer, is_game_invite) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&message.id)
        .bind(&conversation_id)
        .bind(&message.sender_id)
        .bind(&message.sender_name)
        .bind(&message.text)
        .bind(&message.timestamp)
        .bind(message.is_nudge as i32)
        .bind(&message.font_color)
        .bind(&message.font_family)
        .bind(&message.is_wink)
        .bind(&file_transfer_str)
        .bind(message.is_game_invite as i32)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn save_conversations(conversations: Vec<crate::models::Conversation>) -> Result<(), String> {
        let pool = get_pool();
        
        let _ = sqlx::query("DELETE FROM conversations").execute(pool).await;
        let _ = sqlx::query("DELETE FROM conversation_members").execute(pool).await;

        for conv in conversations {
            sqlx::query("INSERT INTO conversations (id, name, is_group, avatar_url, description, created_at) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(&conv.id)
                .bind(&conv.name)
                .bind(conv.is_group as i32)
                .bind(&conv.avatar_url)
                .bind(&conv.description)
                .bind(&conv.created_at)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;

            for member in conv.members {
                sqlx::query("INSERT INTO conversation_members (conversation_id, user_id, display_name, avatar_url, role) VALUES (?, ?, ?, ?, ?)")
                    .bind(&conv.id)
                    .bind(&member.id)
                    .bind(&member.display_name)
                    .bind(&member.avatar_url)
                    .bind(&member.role)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    pub async fn load_conversations() -> Result<Vec<crate::models::Conversation>, String> {
        let pool = get_pool();
        
        let rows = sqlx::query("SELECT id, name, is_group, avatar_url, description, created_at FROM conversations ORDER BY id DESC")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut conversations = Vec::new();

        for row in rows {
            let id: String = row.get("id");
            let name: Option<String> = row.get("name");
            let is_group: i32 = row.get("is_group");
            let avatar_url: Option<String> = row.get("avatar_url");
            let description: Option<String> = row.get("description");
            let created_at: String = row.get("created_at");

            let member_rows = sqlx::query("SELECT user_id, display_name, avatar_url, role FROM conversation_members WHERE conversation_id = ?")
                .bind(&id)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut members = Vec::new();
            for m_row in member_rows {
                let u_id: String = m_row.get("user_id");
                let display_name: String = m_row.get("display_name");
                let avatar_url: Option<String> = m_row.get("avatar_url");
                let role: Option<String> = m_row.get("role");

                members.push(crate::models::UserProfile {
                    id: u_id,
                    email: "".to_string(),
                    username: "".to_string(),
                    full_name: "".to_string(),
                    display_name,
                    personal_message: "".to_string(),
                    status: "Offline".to_string(),
                    music: None,
                    avatar_url,
                    relation_status: None,
                    nickname: None,
                    role,
                });
            }

            conversations.push(crate::models::Conversation {
                id,
                name,
                is_group: is_group != 0,
                avatar_url,
                description,
                created_at,
                members,
            });
        }

        Ok(conversations)
    }
}
