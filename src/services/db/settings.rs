use crate::services::db::{get_pool, DatabaseService};
use sqlx::Row;

impl DatabaseService {
    pub async fn load_settings() -> Result<crate::models::UserSettings, String> {
        let pool = get_pool();
        let row = sqlx::query("SELECT interface_scale, use_custom_titlebar, theme, chat_mode, contact_density, font_color, font_family, spotify_rpc_enabled, show_typing_notification, enable_sounds, enable_toasts, download_folder, auto_accept_files, remember_password, save_chat_history, saved_email, saved_password, auto_login, window_x, window_y, window_width, window_height, fav_collapsed, online_collapsed, offline_collapsed, groups_collapsed, collapsed_categories FROM settings WHERE id = 1")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        let scale: f64 = row.get("interface_scale");
        let custom_bar: i32 = row.get("use_custom_titlebar");
        let theme_str: String = row.get("theme");
        let chat_mode: String = row.get("chat_mode");
        let contact_density: String = row.get("contact_density");
        let font_color: String = row.get("font_color");
        let font_family: String = row.get("font_family");
        let spotify_rpc: i32 = row.get("spotify_rpc_enabled");
        let show_typing: i32 = row.get("show_typing_notification");
        let enable_sounds: i32 = row.get("enable_sounds");
        let enable_toasts: i32 = row.get("enable_toasts");
        let download_folder: String = row.get("download_folder");
        let auto_accept_files: i32 = row.get("auto_accept_files");
        let remember_password: i32 = row.get("remember_password");
        let save_chat_history: i32 = row.get("save_chat_history");
        let saved_email: String = row.get("saved_email");
        let saved_password: String = row.get("saved_password");
        let auto_login: i32 = row.get("auto_login");
        let window_x: i32 = row.get("window_x");
        let window_y: i32 = row.get("window_y");
        let window_width: f64 = row.get("window_width");
        let window_height: f64 = row.get("window_height");
        let fav_collapsed: i32 = row.get("fav_collapsed");
        let online_collapsed: i32 = row.get("online_collapsed");
        let offline_collapsed: i32 = row.get("offline_collapsed");
        let groups_collapsed: i32 = row.get("groups_collapsed");
        let collapsed_categories: String = row.get("collapsed_categories");

        Ok(crate::models::UserSettings {
            interface_scale: scale,
            use_custom_titlebar: custom_bar != 0,
            theme: theme_str,
            chat_mode,
            contact_density,
            font_color,
            font_family,
            spotify_rpc_enabled: spotify_rpc != 0,
            show_typing_notification: show_typing != 0,
            enable_sounds: enable_sounds != 0,
            enable_toasts: enable_toasts != 0,
            download_folder,
            auto_accept_files: auto_accept_files != 0,
            remember_password: remember_password != 0,
            save_chat_history: save_chat_history != 0,
            saved_email,
            saved_password,
            auto_login: auto_login != 0,
            window_x,
            window_y,
            window_width,
            window_height,
            fav_collapsed: fav_collapsed != 0,
            online_collapsed: online_collapsed != 0,
            offline_collapsed: offline_collapsed != 0,
            groups_collapsed: groups_collapsed != 0,
            collapsed_categories,
        })
    }

    pub async fn save_settings(
        settings: &crate::models::UserSettings,
    ) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE settings SET interface_scale = ?, use_custom_titlebar = ?, theme = ?, chat_mode = ?, contact_density = ?, font_color = ?, font_family = ?, spotify_rpc_enabled = ?, show_typing_notification = ?, enable_sounds = ?, enable_toasts = ?, download_folder = ?, auto_accept_files = ?, remember_password = ?, save_chat_history = ?, saved_email = ?, saved_password = ?, auto_login = ?, window_x = ?, window_y = ?, window_width = ?, window_height = ?, fav_collapsed = ?, online_collapsed = ?, offline_collapsed = ?, groups_collapsed = ?, collapsed_categories = ? WHERE id = 1")
            .bind(settings.interface_scale)
            .bind(settings.use_custom_titlebar as i32)
            .bind(&settings.theme)
            .bind(&settings.chat_mode)
            .bind(&settings.contact_density)
            .bind(&settings.font_color)
            .bind(&settings.font_family)
            .bind(settings.spotify_rpc_enabled as i32)
            .bind(settings.show_typing_notification as i32)
            .bind(settings.enable_sounds as i32)
            .bind(settings.enable_toasts as i32)
            .bind(&settings.download_folder)
            .bind(settings.auto_accept_files as i32)
            .bind(settings.remember_password as i32)
            .bind(settings.save_chat_history as i32)
            .bind(&settings.saved_email)
            .bind(&settings.saved_password)
            .bind(settings.auto_login as i32)
            .bind(settings.window_x)
            .bind(settings.window_y)
            .bind(settings.window_width)
            .bind(settings.window_height)
            .bind(settings.fav_collapsed as i32)
            .bind(settings.online_collapsed as i32)
            .bind(settings.offline_collapsed as i32)
            .bind(settings.groups_collapsed as i32)
            .bind(&settings.collapsed_categories)
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
