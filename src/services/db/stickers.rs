use crate::services::db::{get_pool, DatabaseService};
use sqlx::Row;

impl DatabaseService {
    pub async fn add_sticker(name: String, url: String) -> Result<i64, String> {
        let pool = get_pool();
        let result = sqlx::query("INSERT INTO stickers (name, url) VALUES (?, ?)")
            .bind(name)
            .bind(url)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_stickers() -> Result<Vec<(i64, String, String)>, String> {
        let pool = get_pool();
        let rows = sqlx::query("SELECT id, name, url FROM stickers ORDER BY id DESC")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(rows.into_iter().map(|row| {
            let id: i64 = row.get("id");
            let name: String = row.get("name");
            let url: String = row.get("url");
            (id, name, url)
        }).collect())
    }

    pub async fn delete_sticker(id: i64) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("DELETE FROM stickers WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
