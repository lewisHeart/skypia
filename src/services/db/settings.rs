use crate::models::AppTheme;
use crate::services::db::{get_pool, DatabaseService, theme_to_str, str_to_theme};
use sqlx::Row;

impl DatabaseService {
    pub async fn load_settings() -> Result<(f64, bool, AppTheme, String), String> {
        let pool = get_pool();
        let row = sqlx::query("SELECT interface_scale, use_custom_titlebar, theme, chat_mode FROM settings WHERE id = 1")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        let scale: f64 = row.get("interface_scale");
        let custom_bar: i32 = row.get("use_custom_titlebar");
        let theme_str: String = row.get("theme");
        let chat_mode: String = row.get("chat_mode");

        Ok((scale, custom_bar != 0, str_to_theme(&theme_str), chat_mode))
    }

    pub async fn save_settings(
        scale: f64,
        custom_bar: bool,
        theme: AppTheme,
        chat_mode: String,
    ) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE settings SET interface_scale = ?, use_custom_titlebar = ?, theme = ?, chat_mode = ? WHERE id = 1")
            .bind(scale)
            .bind(custom_bar as i32)
            .bind(theme_to_str(&theme))
            .bind(chat_mode)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn detach_chat(contact_id: String) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("INSERT OR IGNORE INTO detached_chats (contact_id) VALUES (?)")
            .bind(contact_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn attach_chat(contact_id: String) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("DELETE FROM detached_chats WHERE contact_id = ?")
            .bind(contact_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_detached_chats() -> Result<Vec<String>, String> {
        let pool = get_pool();
        let rows = sqlx::query("SELECT contact_id FROM detached_chats")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(rows.iter().map(|r| {
            let id: String = r.get("contact_id");
            id
        }).collect())
    }

    pub async fn get_recommended_songs() -> Result<Vec<String>, String> {
        let pool = get_pool();
        let rows = sqlx::query("SELECT title FROM recommended_songs ORDER BY id")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(rows.iter().map(|r| r.get("title")).collect())
    }
}
