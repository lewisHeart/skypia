#![allow(dead_code)]
use crate::models::UserStatus;
use crate::services::db::{get_pool, DatabaseService, status_to_str};
use sqlx::Row;

impl DatabaseService {
    pub async fn load_user_name() -> Result<String, String> {
        let pool = get_pool();
        let name: String = sqlx::query_scalar("SELECT name FROM user_profile WHERE id = 1")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(name)
    }

    pub async fn save_user_name(name: String) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE user_profile SET name = ? WHERE id = 1")
            .bind(&name)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn load_user_music() -> Result<Option<String>, String> {
        let pool = get_pool();
        let music: Option<String> =
            sqlx::query_scalar("SELECT music FROM user_profile WHERE id = 1")
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;
        Ok(music)
    }

    pub async fn save_user_music(music: Option<String>) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE user_profile SET music = ? WHERE id = 1")
            .bind(&music)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn save_user_status(status: UserStatus) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE user_profile SET status = ? WHERE id = 1")
            .bind(status_to_str(&status))
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn save_user_avatar(avatar_id: usize) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE user_profile SET avatar_id = ? WHERE id = 1")
            .bind(avatar_id as i64)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn save_personal_message(msg: String) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE user_profile SET personal_message = ? WHERE id = 1")
            .bind(&msg)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn save_auth_token(token: String, user_id: String) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query(
            "INSERT INTO auth_token (id, token, user_id) VALUES (1, ?, ?) ON CONFLICT(id) DO UPDATE SET token = excluded.token, user_id = excluded.user_id"
        )
        .bind(&token)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn load_auth_token() -> Result<Option<(String, String)>, String> {
        let pool = get_pool();
        let row: Option<(String, String)> =
            sqlx::query_as("SELECT token, user_id FROM auth_token WHERE id = 1")
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?;
        Ok(row)
    }

    pub async fn clear_auth_token() -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("DELETE FROM auth_token WHERE id = 1")
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn save_user_profile_data(
        name: String,
        email: String,
        personal_message: String,
        music: Option<String>,
    ) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE user_profile SET name = ?, email = ?, personal_message = ?, music = ? WHERE id = 1")
            .bind(&name)
            .bind(&email)
            .bind(&personal_message)
            .bind(&music)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn load_user_avatar_url() -> Result<Option<String>, String> {
        let pool = get_pool();
        let url: Option<String> = sqlx::query_scalar("SELECT avatar_url FROM user_profile WHERE id = 1")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(url)
    }

    pub async fn save_user_avatar_url(url: Option<String>) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE user_profile SET avatar_url = ? WHERE id = 1")
            .bind(&url)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn save_banner(banner: &crate::models::BannerInfo) -> Result<(), String> {
        let pool = get_pool();
        let _ = sqlx::query("DELETE FROM banners").execute(pool).await;
        sqlx::query("INSERT INTO banners (text, action_label, link, icon) VALUES (?, ?, ?, ?)")
            .bind(&banner.text)
            .bind(&banner.action_label)
            .bind(&banner.link)
            .bind(&banner.icon)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn load_banner() -> Result<Option<crate::models::BannerInfo>, String> {
        let pool = get_pool();
        let row = sqlx::query("SELECT text, action_label, link, icon FROM banners ORDER BY id DESC LIMIT 1")
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            Ok(Some(crate::models::BannerInfo {
                text: r.get("text"),
                action_label: r.get("action_label"),
                link: r.get("link"),
                icon: r.get("icon"),
            }))
        } else {
            Ok(None)
        }
    }
}
