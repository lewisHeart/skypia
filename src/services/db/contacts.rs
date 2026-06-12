#![allow(dead_code)]
use crate::models::{Contact, UserStatus};
use crate::services::db::{get_pool, DatabaseService, status_to_str, str_to_status};
use sqlx::Row;

impl DatabaseService {
    pub async fn load_contacts() -> Result<Vec<Contact>, String> {
        let pool = get_pool();
        let rows = sqlx::query(
            "SELECT id, email, display_name, status, personal_message, music_listening, avatar_id, is_favorite, relation_status, nickname, avatar_url, category_name FROM contacts ORDER BY id",
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
                    category_name: row.get("category_name"),
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
        let existing = sqlx::query("SELECT id, is_favorite, avatar_id, category_name FROM contacts WHERE email = ?")
            .bind(&email)
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = existing {
            let existing_id: String = row.get("id");
            let is_fav: i32 = row.get("is_favorite");
            let _avatar: i64 = row.get("avatar_id");
            let category_name: Option<String> = row.get("category_name");

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
                category_name,
            });
        }

        let avatar_id = (id.as_bytes().iter().map(|&b| b as usize).sum::<usize>()) % 7;
        sqlx::query(
            "INSERT INTO contacts (id, email, display_name, status, personal_message, music_listening, avatar_id, is_favorite, relation_status, nickname, category_name) VALUES (?, ?, ?, ?, ?, NULL, ?, 0, ?, ?, NULL)",
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
            category_name: None,
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
                "INSERT INTO contacts (id, email, display_name, status, personal_message, is_favorite, relation_status, category_name) VALUES (?, ?, ?, 'Offline', '', ?, 'Aceito', NULL)",
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
        
        let result = sqlx::query("UPDATE contacts SET email = ?, display_name = ?, status = ?, personal_message = ?, music_listening = ?, is_favorite = ?, relation_status = ?, nickname = ?, avatar_url = ?, category_name = ? WHERE id = ?")
            .bind(&contact.email)
            .bind(&contact.display_name)
            .bind(&status_str)
            .bind(&contact.personal_message)
            .bind(&contact.music_listening)
            .bind(contact.is_favorite as i32)
            .bind(&contact.relation_status)
            .bind(&contact.nickname)
            .bind(&contact.avatar_url)
            .bind(&contact.category_name)
            .bind(&contact.id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            let avatar_id = (contact.id.as_bytes().iter().map(|&b| b as usize).sum::<usize>()) % 7;
            sqlx::query("INSERT INTO contacts (id, email, display_name, status, personal_message, music_listening, avatar_id, is_favorite, relation_status, nickname, avatar_url, category_name) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
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
                .bind(&contact.category_name)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn save_contacts_bulk(contacts: Vec<Contact>) -> Result<(), String> {
        let pool = get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        for contact in contacts {
            let status_str = status_to_str(&contact.status);
            
            let result = sqlx::query("UPDATE contacts SET email = ?, display_name = ?, status = ?, personal_message = ?, music_listening = ?, is_favorite = ?, relation_status = ?, nickname = ?, avatar_url = ?, category_name = ? WHERE id = ?")
                .bind(&contact.email)
                .bind(&contact.display_name)
                .bind(&status_str)
                .bind(&contact.personal_message)
                .bind(&contact.music_listening)
                .bind(contact.is_favorite as i32)
                .bind(&contact.relation_status)
                .bind(&contact.nickname)
                .bind(&contact.avatar_url)
                .bind(&contact.category_name)
                .bind(&contact.id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            if result.rows_affected() == 0 {
                let avatar_id = (contact.id.as_bytes().iter().map(|&b| b as usize).sum::<usize>()) % 7;
                sqlx::query("INSERT INTO contacts (id, email, display_name, status, personal_message, music_listening, avatar_id, is_favorite, relation_status, nickname, avatar_url, category_name) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
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
                    .bind(&contact.category_name)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn add_category(name: String) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("INSERT INTO categories (name) VALUES (?)")
            .bind(name)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn delete_category(name: String) -> Result<(), String> {
        let pool = get_pool();
        let _ = sqlx::query("DELETE FROM categories WHERE name = ?")
            .bind(&name)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        let _ = sqlx::query("UPDATE contacts SET category_name = NULL WHERE category_name = ?")
            .bind(&name)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_categories() -> Result<Vec<String>, String> {
        let pool = get_pool();
        let rows = sqlx::query("SELECT name FROM categories ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;
        let list = rows.iter().map(|r| r.get("name")).collect();
        Ok(list)
    }

    pub async fn update_contact_category(contact_id: String, category: Option<String>) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE contacts SET category_name = ? WHERE id = ?")
            .bind(category)
            .bind(contact_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
