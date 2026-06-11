use crate::models::{AppTheme, UserStatus};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::sync::OnceLock;

mod user;
mod contacts;
mod messages;
mod settings;

static POOL: OnceLock<SqlitePool> = OnceLock::new();

pub(crate) fn get_pool() -> &'static SqlitePool {
    POOL.get().expect("Database pool not initialized. Call DatabaseService::init_pool() first.")
}

fn is_pid_running(pid: u32) -> bool {
    // No Linux, verificamos no diretório /proc
    #[cfg(target_os = "linux")]
    {
        std::path::Path::new(&format!("/proc/{}", pid)).exists()
    }
    #[cfg(not(target_os = "linux"))]
    {
        true
    }
}

#[cfg(target_os = "android")]
fn get_android_files_dir() -> Result<std::path::PathBuf, String> {
    use jni::objects::JObject;
    use jni::JavaVM;
    
    let ctx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }
        .map_err(|e| format!("Failed to get JavaVM: {:?}", e))?;
    
    let mut env = vm.attach_current_thread()
        .map_err(|e| format!("Failed to attach thread: {:?}", e))?;
    
    let context_obj = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };
    
    let files_dir = env.call_method(&context_obj, "getFilesDir", "()Ljava/io/File;", &[])
        .map_err(|e| format!("Failed to call getFilesDir: {:?}", e))?
        .l()
        .map_err(|e| format!("Failed to get getFilesDir object: {:?}", e))?;
        
    let path_obj = env.call_method(&files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])
        .map_err(|e| format!("Failed to call getAbsolutePath: {:?}", e))?
        .l()
        .map_err(|e| format!("Failed to get path string object: {:?}", e))?;
        
    let path_jstr: jni::objects::JString = path_obj.into();
    let path_str: String = env.get_string(&path_jstr)
        .map_err(|e| format!("Failed to convert path string: {:?}", e))?
        .into();
        
    Ok(std::path::PathBuf::from(path_str))
}

pub fn get_app_data_dir() -> std::path::PathBuf {
    #[cfg(target_os = "android")]
    {
        match get_android_files_dir() {
            Ok(path) => path,
            Err(e) => {
                eprintln!("⚠️ Falha ao obter files_dir do Android via JNI: {}. Usando temp_dir.", e);
                std::env::temp_dir()
            }
        }
    }
    #[cfg(not(target_os = "android"))]
    {
        std::path::Path::new(".skypia_data").join("db")
    }
}

fn get_isolated_db_path() -> String {
    let data_dir = get_app_data_dir();
    let _ = std::fs::create_dir_all(&data_dir);

    #[cfg(target_os = "android")]
    {
        data_dir.join("skypia.db").to_string_lossy().to_string()
    }

    #[cfg(not(target_os = "android"))]
    {
        // Tenta encontrar um slot livre de 1 a 10
        for slot in 1..=10 {
            let lock_path = data_dir.join(format!("skypia_{}.lock", slot));
            let mut take_slot = false;
            
            if let Ok(content) = std::fs::read_to_string(&lock_path) {
                if let Ok(pid) = content.trim().parse::<u32>() {
                    if !is_pid_running(pid) {
                        take_slot = true;
                    }
                } else {
                    take_slot = true;
                }
            } else {
                take_slot = true;
            }
            
            if take_slot {
                let my_pid = std::process::id();
                if std::fs::write(&lock_path, my_pid.to_string()).is_ok() {
                    let db_file = if slot == 1 {
                        data_dir.join("skypia.db")
                    } else {
                        data_dir.join(format!("skypia_{}.db", slot))
                    };
                    
                    let db_path_str = db_file.to_string_lossy().to_string();
                    if slot == 1 {
                        println!("🔒 Slot 1 travado (PID {}). Usando {}", my_pid, db_path_str);
                    } else {
                        println!("🔒 Slot {} travado (PID {}). Usando {}", slot, my_pid, db_path_str);
                    }
                    return db_path_str;
                }
            }
        }
        
        data_dir.join("skypia.db").to_string_lossy().to_string()
    }
}


pub struct DatabaseService;

impl DatabaseService {
    /// Inicializa o pool de conexões SQLite e roda migrations + seed.
    /// Deve ser chamado UMA VEZ na inicialização do app.
    pub async fn init_pool() -> Result<(), String> {
        let db_path = get_isolated_db_path();

        let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))
            .map_err(|e| e.to_string())?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .map_err(|e| e.to_string())?;

        // Roda migrations (cria tabelas)
        Self::run_migrations(&pool).await?;

        // Seed de dados iniciais se tabelas estão vazias
        Self::seed_initial_data(&pool).await?;

        POOL.set(pool).map_err(|_| "Pool already initialized".to_string())?;
        Ok(())
    }

    async fn run_migrations(pool: &SqlitePool) -> Result<(), String> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_profile (
                id INTEGER PRIMARY KEY DEFAULT 1,
                name TEXT NOT NULL DEFAULT 'Wellington Skypia',
                email TEXT NOT NULL DEFAULT 'wk.scbd@skypia.io',
                status TEXT NOT NULL DEFAULT 'Online',
                personal_message TEXT NOT NULL DEFAULT '',
                music TEXT,
                avatar_id INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS settings (
                id INTEGER PRIMARY KEY DEFAULT 1,
                interface_scale REAL NOT NULL DEFAULT 1.0,
                use_custom_titlebar INTEGER NOT NULL DEFAULT 1,
                theme TEXT NOT NULL DEFAULT 'AeroBlue',
                chat_mode TEXT NOT NULL DEFAULT 'integrated',
                contact_density TEXT NOT NULL DEFAULT 'medium',
                font_color TEXT NOT NULL DEFAULT '#1e395b',
                font_family TEXT NOT NULL DEFAULT 'Segoe UI',
                spotify_rpc_enabled INTEGER NOT NULL DEFAULT 0,
                show_typing_notification INTEGER NOT NULL DEFAULT 1,
                enable_sounds INTEGER NOT NULL DEFAULT 1,
                enable_toasts INTEGER NOT NULL DEFAULT 1,
                download_folder TEXT NOT NULL DEFAULT '',
                auto_accept_files INTEGER NOT NULL DEFAULT 0,
                remember_password INTEGER NOT NULL DEFAULT 1,
                save_chat_history INTEGER NOT NULL DEFAULT 1,
                saved_email TEXT NOT NULL DEFAULT '',
                saved_password TEXT NOT NULL DEFAULT '',
                auto_login INTEGER NOT NULL DEFAULT 0,
                window_x INTEGER NOT NULL DEFAULT 100,
                window_y INTEGER NOT NULL DEFAULT 100,
                window_width REAL NOT NULL DEFAULT 413.0,
                window_height REAL NOT NULL DEFAULT 735.0,
                fav_collapsed INTEGER NOT NULL DEFAULT 0,
                online_collapsed INTEGER NOT NULL DEFAULT 0,
                offline_collapsed INTEGER NOT NULL DEFAULT 0,
                groups_collapsed INTEGER NOT NULL DEFAULT 0,
                collapsed_categories TEXT NOT NULL DEFAULT '[]'
            );
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS contacts (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL,
                display_name TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'Offline',
                personal_message TEXT NOT NULL DEFAULT '',
                music_listening TEXT,
                avatar_id INTEGER NOT NULL DEFAULT 0,
                is_favorite INTEGER NOT NULL DEFAULT 0,
                relation_status TEXT NOT NULL DEFAULT 'Aceito',
                nickname TEXT,
                avatar_url TEXT
            )
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                contact_id TEXT NOT NULL,
                sender_id TEXT NOT NULL,
                sender_name TEXT NOT NULL,
                text TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                is_nudge INTEGER NOT NULL DEFAULT 0,
                font_color TEXT NOT NULL DEFAULT '#1e395b',
                font_family TEXT NOT NULL DEFAULT 'Segoe UI',
                is_wink TEXT,
                file_transfer TEXT,
                is_game_invite INTEGER NOT NULL DEFAULT 0
            )
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS detached_chats (contact_id TEXT PRIMARY KEY)",
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS auth_token (
                id      INTEGER PRIMARY KEY DEFAULT 1,
                token   TEXT NOT NULL,
                user_id TEXT NOT NULL
            )
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS banners (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                text TEXT NOT NULL,
                action_label TEXT NOT NULL,
                link TEXT NOT NULL,
                icon TEXT NOT NULL
            )
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS recommended_songs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL
            )
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                name TEXT,
                is_group INTEGER NOT NULL DEFAULT 0,
                avatar_url TEXT,
                description TEXT,
                created_at TEXT NOT NULL
            );
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS conversation_members (
                conversation_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                display_name TEXT NOT NULL,
                avatar_url TEXT,
                role TEXT,
                PRIMARY KEY (conversation_id, user_id)
            );
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Tenta adicionar a coluna conversation_id se ela não existir
        let _ = sqlx::query("ALTER TABLE messages ADD COLUMN conversation_id TEXT NOT NULL DEFAULT '1'")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE contacts ADD COLUMN relation_status TEXT NOT NULL DEFAULT 'Aceito'")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE contacts ADD COLUMN nickname TEXT")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN chat_mode TEXT NOT NULL DEFAULT 'integrated'")
            .execute(pool)
            .await;
 
        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN contact_density TEXT NOT NULL DEFAULT 'medium'")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN font_color TEXT NOT NULL DEFAULT '#1e395b'")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN font_family TEXT NOT NULL DEFAULT 'Segoe UI'")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN spotify_rpc_enabled INTEGER NOT NULL DEFAULT 0")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN show_typing_notification INTEGER NOT NULL DEFAULT 1")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN enable_sounds INTEGER NOT NULL DEFAULT 1")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN enable_toasts INTEGER NOT NULL DEFAULT 1")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN download_folder TEXT NOT NULL DEFAULT ''")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN auto_accept_files INTEGER NOT NULL DEFAULT 0")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN remember_password INTEGER NOT NULL DEFAULT 1")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN save_chat_history INTEGER NOT NULL DEFAULT 1")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN saved_email TEXT NOT NULL DEFAULT ''")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN saved_password TEXT NOT NULL DEFAULT ''")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN auto_login INTEGER NOT NULL DEFAULT 0")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN window_x INTEGER NOT NULL DEFAULT 100")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN window_y INTEGER NOT NULL DEFAULT 100")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN window_width REAL NOT NULL DEFAULT 413.0")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN window_height REAL NOT NULL DEFAULT 735.0")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN fav_collapsed INTEGER NOT NULL DEFAULT 0")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN online_collapsed INTEGER NOT NULL DEFAULT 0")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN offline_collapsed INTEGER NOT NULL DEFAULT 0")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN groups_collapsed INTEGER NOT NULL DEFAULT 0")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE settings ADD COLUMN collapsed_categories TEXT NOT NULL DEFAULT '[]'")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE conversations ADD COLUMN avatar_url TEXT")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE conversations ADD COLUMN description TEXT")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE conversation_members ADD COLUMN role TEXT")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE user_profile ADD COLUMN avatar_url TEXT")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE contacts ADD COLUMN avatar_url TEXT")
            .execute(pool)
            .await;

        // Migração para Categorias Personalizadas
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            );
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        let _ = sqlx::query("ALTER TABLE contacts ADD COLUMN category_name TEXT")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE banners ADD COLUMN image_url TEXT")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE conversations ADD COLUMN allow_member_send INTEGER DEFAULT 1")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE conversations ADD COLUMN allow_member_invite INTEGER DEFAULT 1")
            .execute(pool)
            .await;

        Ok(())
    }

    async fn seed_initial_data(pool: &SqlitePool) -> Result<(), String> {
        // Limpa incondicionalmente o banco local de dados legados / cache
        let _ = sqlx::query("DELETE FROM contacts").execute(pool).await;
        let _ = sqlx::query("DELETE FROM messages").execute(pool).await;
        let _ = sqlx::query("DELETE FROM conversations").execute(pool).await;
        let _ = sqlx::query("DELETE FROM conversation_members").execute(pool).await;
        let _ = sqlx::query("DELETE FROM recommended_songs").execute(pool).await;
        println!("🧹 Banco de dados local/cache limpo com sucesso.");

        // Seed user_profile se não existe
        let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM user_profile")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        if user_count == 0 {
            sqlx::query(
                "INSERT INTO user_profile (id, name, email, status, personal_message, music, avatar_id) VALUES (1, '', '', 'Offline', '', NULL, 0)",
            )
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        } else {
            // Garante que o user_profile local do ID 1 não tenha dados estáticos mocks de execuções passadas
            let _ = sqlx::query("UPDATE user_profile SET name = '', email = '', status = 'Offline', personal_message = '', music = NULL WHERE id = 1 AND name = 'Wellington Skypia'")
                .execute(pool)
                .await;
        }

        // Seed settings se não existe
        let settings_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM settings")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        if settings_count == 0 {
            #[cfg(target_os = "android")]
            let query = "INSERT INTO settings (id, interface_scale, use_custom_titlebar, theme, chat_mode, font_color, font_family, spotify_rpc_enabled, show_typing_notification, enable_sounds, enable_toasts, download_folder, auto_accept_files, remember_password, save_chat_history, saved_email, saved_password, auto_login, window_x, window_y, window_width, window_height, fav_collapsed, online_collapsed, offline_collapsed, groups_collapsed, collapsed_categories) VALUES (1, 1.35, 0, 'AeroBlue', 'integrated', '#1e395b', 'Segoe UI', 0, 1, 1, 1, '', 0, 1, 1, '', '', 0, 100, 100, 413.0, 735.0, 0, 0, 0, 0, '[]')";
            #[cfg(not(target_os = "android"))]
            let query = "INSERT INTO settings (id, interface_scale, use_custom_titlebar, theme, chat_mode, font_color, font_family, spotify_rpc_enabled, show_typing_notification, enable_sounds, enable_toasts, download_folder, auto_accept_files, remember_password, save_chat_history, saved_email, saved_password, auto_login, window_x, window_y, window_width, window_height, fav_collapsed, online_collapsed, offline_collapsed, groups_collapsed, collapsed_categories) VALUES (1, 1.0, 1, 'AeroBlue', 'integrated', '#1e395b', 'Segoe UI', 0, 1, 1, 1, '', 0, 1, 1, '', '', 0, 100, 100, 413.0, 735.0, 0, 0, 0, 0, '[]')";

            sqlx::query(query)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        // recommended_songs agora inicia vazia para remover dados estáticos e simulações aleatórias.
        let _ = sqlx::query("DELETE FROM recommended_songs").execute(pool).await;

        Ok(())
    }
}

// ─── Helpers de conversão ───

pub(crate) fn status_to_str(status: &UserStatus) -> &'static str {
    match status {
        UserStatus::Online => "Online",
        UserStatus::Ocupado => "Ocupado",
        UserStatus::Ausente => "Ausente",
        UserStatus::Invisivel => "Invisivel",
        UserStatus::Offline => "Offline",
    }
}

pub(crate) fn str_to_status(s: &str) -> UserStatus {
    match s {
        "Online" => UserStatus::Online,
        "Ocupado" => UserStatus::Ocupado,
        "Ausente" => UserStatus::Ausente,
        "Invisivel" => UserStatus::Invisivel,
        _ => UserStatus::Offline,
    }
}

pub(crate) fn theme_to_str(theme: &AppTheme) -> &'static str {
    match theme {
        AppTheme::AeroBlue => "AeroBlue",
        AppTheme::RubyPink => "RubyPink",
        AppTheme::ForestGreen => "ForestGreen",
        AppTheme::SilverMetallic => "SilverMetallic",
    }
}

pub(crate) fn str_to_theme(s: &str) -> AppTheme {
    match s {
        "RubyPink" => AppTheme::RubyPink,
        "ForestGreen" => AppTheme::ForestGreen,
        "SilverMetallic" => AppTheme::SilverMetallic,
        _ => AppTheme::AeroBlue,
    }
}
