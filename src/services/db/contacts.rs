use crate::models::{Contact, UserStatus};
use crate::services::db::{get_pool, DatabaseService, status_to_str, str_to_status};
use sqlx::Row;

impl DatabaseService {
    pub async fn load_contacts() -> Result<Vec<Contact>, String> {
        let pool = get_pool();
        let rows = sqlx::query(
            "SELECT id, email, display_name, status, personal_message, music_listening, avatar_id, is_favorite, relation_status, nickname, avatar_url FROM contacts ORDER BY id",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let contacts = rows
            .iter()
            .map(|row| {
                let id: String = row.get("id");
                let status_str: String = row.get("status");
                let is_fav: i32 = row.get("is_favorite");
                let _avatar: i64 = row.get("avatar_id");

                Contact {
                    id,
                    email: row.get("email"),
                    display_name: row.get("display_name"),
                    status: str_to_status(&status_str),
                    personal_message: row.get("personal_message"),
                    music_listening: row.get("music_listening"),
                    avatar_url: row.get("avatar_url"),
                    is_favorite: is_fav != 0,
                    relation_status: row.get("relation_status"),
                    nickname: row.get("nickname"),
                }
            })
            .collect();

        Ok(contacts)
    }

    pub async fn add_contact(
        id: String,
        email: String,
        display_name: String,
        status: UserStatus,
        personal_message: String,
        relation_status: String,
        nickname: Option<String>,
    ) -> Result<Contact, String> {
        let pool = get_pool();

        // Verifica se já existe um contato com o mesmo e-mail para evitar duplicação local
        let existing = sqlx::query("SELECT id, is_favorite, avatar_id FROM contacts WHERE email = ?")
            .bind(&email)
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = existing {
            let existing_id: String = row.get("id");
            let is_fav: i32 = row.get("is_favorite");
            let _avatar: i64 = row.get("avatar_id");

            sqlx::query("UPDATE contacts SET id = ?, display_name = ?, status = ?, personal_message = ?, relation_status = ?, nickname = ? WHERE id = ?")
                .bind(&id)
                .bind(&display_name)
                .bind(status_to_str(&status))
                .bind(&personal_message)
                .bind(&relation_status)
                .bind(&nickname)
                .bind(&existing_id)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;

            return Ok(Contact {
                id,
                email,
                display_name,
                status,
                personal_message,
                music_listening: None,
                avatar_url: None,
                is_favorite: is_fav != 0,
                relation_status,
                nickname,
            });
        }

        let avatar_id = (id.as_bytes().iter().map(|&b| b as usize).sum::<usize>()) % 7;
        sqlx::query(
            "INSERT INTO contacts (id, email, display_name, status, personal_message, music_listening, avatar_id, is_favorite, relation_status, nickname) VALUES (?, ?, ?, ?, ?, NULL, ?, 0, ?, ?)",
        )
        .bind(&id)
        .bind(&email)
        .bind(&display_name)
        .bind(status_to_str(&status))
        .bind(&personal_message)
        .bind(avatar_id as i64)
        .bind(&relation_status)
        .bind(&nickname)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Contact {
            id,
            email,
            display_name,
            status,
            personal_message,
            music_listening: None,
            avatar_url: None,
            is_favorite: false,
            relation_status,
            nickname,
        })
    }

    pub async fn update_contact_nickname(contact_id: String, nickname: Option<String>) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE contacts SET nickname = ? WHERE id = ?")
            .bind(nickname)
            .bind(contact_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn update_contact_relation(contact_id: String, relation_status: String) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE contacts SET relation_status = ? WHERE id = ?")
            .bind(relation_status)
            .bind(contact_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn delete_contact(contact_id: String) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("DELETE FROM contacts WHERE id = ?")
            .bind(contact_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn save_contact_favorite(
        contact_id: String,
        email: String,
        display_name: String,
        is_favorite: bool,
    ) -> Result<(), String> {
        let pool = get_pool();
        let result = sqlx::query("UPDATE contacts SET is_favorite = ? WHERE id = ?")
            .bind(is_favorite as i32)
            .bind(&contact_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            sqlx::query(
                "INSERT INTO contacts (id, email, display_name, status, personal_message, is_favorite, relation_status) VALUES (?, ?, ?, 'Offline', '', ?, 'Aceito')",
            )
            .bind(&contact_id)
            .bind(&email)
            .bind(&display_name)
            .bind(is_favorite as i32)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn save_contact(contact: &Contact) -> Result<(), String> {
        let pool = get_pool();
        let status_str = status_to_str(&contact.status);
        
        let result = sqlx::query("UPDATE contacts SET email = ?, display_name = ?, status = ?, personal_message = ?, music_listening = ?, is_favorite = ?, relation_status = ?, nickname = ?, avatar_url = ? WHERE id = ?")
            .bind(&contact.email)
            .bind(&contact.display_name)
            .bind(&status_str)
            .bind(&contact.personal_message)
            .bind(&contact.music_listening)
            .bind(contact.is_favorite as i32)
            .bind(&contact.relation_status)
            .bind(&contact.nickname)
            .bind(&contact.avatar_url)
            .bind(&contact.id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            let avatar_id = (contact.id.as_bytes().iter().map(|&b| b as usize).sum::<usize>()) % 7;
            sqlx::query("INSERT INTO contacts (id, email, display_name, status, personal_message, music_listening, avatar_id, is_favorite, relation_status, nickname, avatar_url) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(&contact.id)
                .bind(&contact.email)
                .bind(&contact.display_name)
                .bind(&status_str)
                .bind(&contact.personal_message)
                .bind(&contact.music_listening)
                .bind(avatar_id as i64)
                .bind(contact.is_favorite as i32)
                .bind(&contact.relation_status)
                .bind(&contact.nickname)
                .bind(&contact.avatar_url)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
