use crate::models::{AppTheme, BannerInfo, Contact, FileTransferState, Message, UserStatus};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Row, SqlitePool};
use std::str::FromStr;
use std::sync::OnceLock;

static POOL: OnceLock<SqlitePool> = OnceLock::new();

fn get_pool() -> &'static SqlitePool {
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

fn get_isolated_db_path() -> String {
    // Tenta encontrar um slot livre de 1 a 10
    for slot in 1..=10 {
        let lock_path = format!("skypia_{}.lock", slot);
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
                if slot == 1 {
                    println!("🔒 Slot 1 travado (PID {}). Usando skypia.db", my_pid);
                    return "skypia.db".to_string();
                } else {
                    println!("🔒 Slot {} travado (PID {}). Usando skypia_{}.db", slot, my_pid, slot);
                    return format!("skypia_{}.db", slot);
                }
            }
        }
    }
    
    "skypia.db".to_string()
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
                theme TEXT NOT NULL DEFAULT 'AeroBlue'
            );
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // SQLite não suporta múltiplos statements em um único query facilmente via sqlx,
        // então rodamos cada tabela separadamente
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS contacts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT NOT NULL,
                display_name TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'Offline',
                personal_message TEXT NOT NULL DEFAULT '',
                music_listening TEXT,
                avatar_id INTEGER NOT NULL DEFAULT 0,
                is_favorite INTEGER NOT NULL DEFAULT 0,
                relation_status TEXT NOT NULL DEFAULT 'Aceito',
                nickname TEXT
            )
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                contact_id INTEGER NOT NULL,
                sender_id INTEGER NOT NULL,
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
            "CREATE TABLE IF NOT EXISTS detached_chats (contact_id INTEGER PRIMARY KEY)",
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS auth_token (
                id      INTEGER PRIMARY KEY DEFAULT 1,
                token   TEXT NOT NULL,
                user_id INTEGER NOT NULL
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
                id INTEGER PRIMARY KEY,
                name TEXT,
                is_group INTEGER NOT NULL DEFAULT 0,
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
                conversation_id INTEGER NOT NULL,
                user_id INTEGER NOT NULL,
                display_name TEXT NOT NULL,
                avatar_url TEXT,
                PRIMARY KEY (conversation_id, user_id)
            );
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Tenta adicionar a coluna conversation_id se ela não existir
        let _ = sqlx::query("ALTER TABLE messages ADD COLUMN conversation_id INTEGER NOT NULL DEFAULT 1")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE contacts ADD COLUMN relation_status TEXT NOT NULL DEFAULT 'Aceito'")
            .execute(pool)
            .await;

        let _ = sqlx::query("ALTER TABLE contacts ADD COLUMN nickname TEXT")
            .execute(pool)
            .await;

        Ok(())
    }

    async fn seed_initial_data(pool: &SqlitePool) -> Result<(), String> {
        // Se houver dados mocks legados (ex. o e-mail de mock lucas_heavy@hotmail.com), limpa o banco local
        let has_mock: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM contacts WHERE email = 'lucas_heavy@hotmail.com'")
            .fetch_one(pool)
            .await
            .unwrap_or(0);
        if has_mock > 0 {
            let _ = sqlx::query("DELETE FROM contacts").execute(pool).await;
            let _ = sqlx::query("DELETE FROM messages").execute(pool).await;
            let _ = sqlx::query("DELETE FROM conversations").execute(pool).await;
            let _ = sqlx::query("DELETE FROM conversation_members").execute(pool).await;
            println!("🧹 Banco de dados local limpo de dados mocks legados.");
        }

        // Seed user_profile se não existe
        let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM user_profile")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        if user_count == 0 {
            sqlx::query(
                "INSERT INTO user_profile (id, name, email, status, personal_message, music, avatar_id) VALUES (1, ?, ?, 'Online', 'Tô cagando', 'Linkin Park - In The End', 0)",
            )
            .bind("Wellington Skypia")
            .bind("wk.scbd@skypia.io")
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        // Seed settings se não existe
        let settings_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM settings")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        if settings_count == 0 {
            sqlx::query("INSERT INTO settings (id, interface_scale, use_custom_titlebar, theme) VALUES (1, 1.0, 1, 'AeroBlue')")
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        // Seed banners se tabela vazia
        let banner_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM banners")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        if banner_count == 0 {
            let banners = vec![
                ("Navegue na web com muito mais velocidade e segurança!", "Instalar Skypia Browser", "https://skypia.io/browser", "🌐"),
                ("Ouça as melhores músicas retrô com alta fidelidade!", "Skypia Music Premium", "https://skypia.io/music", "🎵"),
                ("Seus e-mails e arquivos protegidos em um só lugar.", "Acessar Skypia Mail", "https://skypia.io/mail", "📧"),
                ("Espaço gratuito ilimitado para suas fotos e dados na nuvem.", "Conhecer Skypia Drive", "https://skypia.io/drive", "💾"),
            ];

            for (text, action, link, icon) in banners {
                sqlx::query("INSERT INTO banners (text, action_label, link, icon) VALUES (?, ?, ?, ?)")
                    .bind(text)
                    .bind(action)
                    .bind(link)
                    .bind(icon)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        // Seed recommended songs se tabela vazia
        let song_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM recommended_songs")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        if song_count == 0 {
            let songs = vec![
                "NX Zero - Cedo Ou Tarde",
                "Coldplay - Viva La Vida",
                "Linkin Park - In The End",
                "Green Day - Boulevard of Broken Dreams",
                "Blink-182 - I Miss You",
                "Evanescence - Bring Me To Life",
                "Simple Plan - Welcome to My Life",
                "Fresno - Alguém Que Te Faz Sorrir",
                "Paramore - Decode",
                "Pitty - Admirável Chip Novo",
            ];

            for title in songs {
                sqlx::query("INSERT INTO recommended_songs (title) VALUES (?)")
                    .bind(title)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    // ─── User Profile ───

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

    // ─── Contacts ───

    pub async fn load_contacts() -> Result<Vec<Contact>, String> {
        let pool = get_pool();
        let rows = sqlx::query(
            "SELECT id, email, display_name, status, personal_message, music_listening, avatar_id, is_favorite, relation_status, nickname FROM contacts ORDER BY id",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let contacts = rows
            .iter()
            .map(|row| {
                let id: i64 = row.get("id");
                let status_str: String = row.get("status");
                let is_fav: i32 = row.get("is_favorite");
                let avatar: i64 = row.get("avatar_id");

                Contact {
                    id: id as usize,
                    email: row.get("email"),
                    display_name: row.get("display_name"),
                    status: str_to_status(&status_str),
                    personal_message: row.get("personal_message"),
                    music_listening: row.get("music_listening"),
                    avatar_id: avatar as usize,
                    is_favorite: is_fav != 0,
                    relation_status: row.get("relation_status"),
                    nickname: row.get("nickname"),
                }
            })
            .collect();

        Ok(contacts)
    }

    pub async fn add_contact(
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
            let existing_id: i64 = row.get("id");
            let is_fav: i32 = row.get("is_favorite");
            let avatar: i64 = row.get("avatar_id");

            sqlx::query("UPDATE contacts SET display_name = ?, status = ?, personal_message = ?, relation_status = ?, nickname = ? WHERE id = ?")
                .bind(&display_name)
                .bind(status_to_str(&status))
                .bind(&personal_message)
                .bind(&relation_status)
                .bind(&nickname)
                .bind(existing_id)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;

            return Ok(Contact {
                id: existing_id as usize,
                email,
                display_name,
                status,
                personal_message,
                music_listening: None,
                avatar_id: avatar as usize,
                is_favorite: is_fav != 0,
                relation_status,
                nickname,
            });
        }

        let result = sqlx::query(
            "INSERT INTO contacts (email, display_name, status, personal_message, music_listening, avatar_id, is_favorite, relation_status, nickname) VALUES (?, ?, ?, ?, NULL, ?, 0, ?, ?)",
        )
        .bind(&email)
        .bind(&display_name)
        .bind(status_to_str(&status))
        .bind(&personal_message)
        .bind(0_i64)
        .bind(&relation_status)
        .bind(&nickname)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        let new_id = result.last_insert_rowid() as usize;
        let avatar_id = new_id % 7;

        // Atualiza avatar baseado no ID
        sqlx::query("UPDATE contacts SET avatar_id = ? WHERE id = ?")
            .bind(avatar_id as i64)
            .bind(new_id as i64)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Contact {
            id: new_id,
            email,
            display_name,
            status,
            personal_message,
            music_listening: None,
            avatar_id,
            is_favorite: false,
            relation_status,
            nickname,
        })
    }

    pub async fn update_contact_nickname(contact_id: usize, nickname: Option<String>) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE contacts SET nickname = ? WHERE id = ?")
            .bind(nickname)
            .bind(contact_id as i64)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn update_contact_relation(contact_id: usize, relation_status: String) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE contacts SET relation_status = ? WHERE id = ?")
            .bind(relation_status)
            .bind(contact_id as i64)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn delete_contact(contact_id: usize) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("DELETE FROM contacts WHERE id = ?")
            .bind(contact_id as i64)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn save_contact_favorite(
        contact_id: usize,
        is_favorite: bool,
    ) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE contacts SET is_favorite = ? WHERE id = ?")
            .bind(is_favorite as i32)
            .bind(contact_id as i64)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // ─── Messages ───

    pub async fn load_messages(conversation_id: usize) -> Result<Vec<Message>, String> {
        let pool = get_pool();
        let rows = sqlx::query(
            "SELECT id, conversation_id, sender_id, sender_name, text, timestamp, is_nudge, font_color, font_family, is_wink, file_transfer, is_game_invite FROM messages WHERE conversation_id = ? ORDER BY id",
        )
        .bind(conversation_id as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let messages = rows
            .iter()
            .map(|row| {
                let id: i64 = row.get("id");
                let conv_id: i64 = row.get("conversation_id");
                let sender_id: i64 = row.get("sender_id");
                let is_nudge: i32 = row.get("is_nudge");
                let is_game_invite: i32 = row.get("is_game_invite");
                let is_wink: Option<String> = row.get("is_wink");
                let file_transfer_str: Option<String> = row.get("file_transfer");

                let file_transfer = file_transfer_str.and_then(|s| {
                    serde_json::from_str::<FileTransferState>(&s).ok()
                });

                Message {
                    id: id as usize,
                    conversation_id: conv_id as usize,
                    sender_id: sender_id as usize,
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

    pub async fn save_message(conversation_id: usize, message: Message) -> Result<(), String> {
        let pool = get_pool();

        let file_transfer_str = message
            .file_transfer
            .as_ref()
            .and_then(|ft| serde_json::to_string(ft).ok());

        sqlx::query(
            "INSERT INTO messages (conversation_id, sender_id, sender_name, text, timestamp, is_nudge, font_color, font_family, is_wink, file_transfer, is_game_invite) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(conversation_id as i64)
        .bind(message.sender_id as i64)
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

    // ─── Settings ───

    pub async fn load_settings() -> Result<(f64, bool, AppTheme), String> {
        let pool = get_pool();
        let row = sqlx::query("SELECT interface_scale, use_custom_titlebar, theme FROM settings WHERE id = 1")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        let scale: f64 = row.get("interface_scale");
        let custom_bar: i32 = row.get("use_custom_titlebar");
        let theme_str: String = row.get("theme");

        Ok((scale, custom_bar != 0, str_to_theme(&theme_str)))
    }

    pub async fn save_settings(
        scale: f64,
        custom_bar: bool,
        theme: AppTheme,
    ) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("UPDATE settings SET interface_scale = ?, use_custom_titlebar = ?, theme = ? WHERE id = 1")
            .bind(scale)
            .bind(custom_bar as i32)
            .bind(theme_to_str(&theme))
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // ─── Detached Chats ───

    pub async fn detach_chat(contact_id: usize) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("INSERT OR IGNORE INTO detached_chats (contact_id) VALUES (?)")
            .bind(contact_id as i64)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn attach_chat(contact_id: usize) -> Result<(), String> {
        let pool = get_pool();
        sqlx::query("DELETE FROM detached_chats WHERE contact_id = ?")
            .bind(contact_id as i64)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_detached_chats() -> Result<Vec<usize>, String> {
        let pool = get_pool();
        let rows = sqlx::query("SELECT contact_id FROM detached_chats")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(rows.iter().map(|r| {
            let id: i64 = r.get("contact_id");
            id as usize
        }).collect())
    }

    // ─── Banners ───

    pub async fn get_banner_info() -> Result<BannerInfo, String> {
        let pool = get_pool();
        let rows = sqlx::query("SELECT text, action_label, link, icon FROM banners ORDER BY id")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

        if rows.is_empty() {
            return Err("Nenhum banner cadastrado".to_string());
        }

        // Rotaciona baseado no tempo atual
        let now = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let idx = (now / 15) as usize % rows.len();
        let row = &rows[idx];

        Ok(BannerInfo {
            text: row.get("text"),
            action_label: row.get("action_label"),
            link: row.get("link"),
            icon: row.get("icon"),
        })
    }

    // ─── Recommended Songs ───

    pub async fn get_recommended_songs() -> Result<Vec<String>, String> {
        let pool = get_pool();
        let rows = sqlx::query("SELECT title FROM recommended_songs ORDER BY id")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(rows.iter().map(|r| r.get("title")).collect())
    }

    // ─── Auth Token (auto-login) ───

    pub async fn save_auth_token(token: String, user_id: i64) -> Result<(), String> {
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

    pub async fn load_auth_token() -> Result<Option<(String, i64)>, String> {
        let pool = get_pool();
        let row: Option<(String, i64)> =
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

    pub async fn save_conversations(conversations: Vec<crate::models::Conversation>) -> Result<(), String> {
        let pool = get_pool();
        
        let _ = sqlx::query("DELETE FROM conversations").execute(pool).await;
        let _ = sqlx::query("DELETE FROM conversation_members").execute(pool).await;

        for conv in conversations {
            sqlx::query("INSERT INTO conversations (id, name, is_group, created_at) VALUES (?, ?, ?, ?)")
                .bind(conv.id as i64)
                .bind(&conv.name)
                .bind(conv.is_group as i32)
                .bind(&conv.created_at)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;

            for member in conv.members {
                sqlx::query("INSERT INTO conversation_members (conversation_id, user_id, display_name, avatar_url) VALUES (?, ?, ?, ?)")
                    .bind(conv.id as i64)
                    .bind(member.id)
                    .bind(&member.display_name)
                    .bind(&member.avatar_url)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    pub async fn load_conversations() -> Result<Vec<crate::models::Conversation>, String> {
        let pool = get_pool();
        
        let rows = sqlx::query("SELECT id, name, is_group, created_at FROM conversations ORDER BY id DESC")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut conversations = Vec::new();

        for row in rows {
            let id: i64 = row.get("id");
            let name: Option<String> = row.get("name");
            let is_group: i32 = row.get("is_group");
            let created_at: String = row.get("created_at");

            let member_rows = sqlx::query("SELECT user_id, display_name, avatar_url FROM conversation_members WHERE conversation_id = ?")
                .bind(id)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut members = Vec::new();
            for m_row in member_rows {
                let u_id: i64 = m_row.get("user_id");
                let display_name: String = m_row.get("display_name");
                let avatar_url: Option<String> = m_row.get("avatar_url");

                members.push(crate::models::UserProfile {
                    id: u_id,
                    email: "".to_string(),
                    display_name,
                    personal_message: "".to_string(),
                    status: "Offline".to_string(),
                    music: None,
                    avatar_url,
                    relation_status: None,
                    nickname: None,
                });
            }

            conversations.push(crate::models::Conversation {
                id: id as usize,
                name,
                is_group: is_group != 0,
                created_at,
                members,
            });
        }

        Ok(conversations)
    }
}

// ─── Helpers de conversão ───

fn status_to_str(status: &UserStatus) -> &'static str {
    match status {
        UserStatus::Online => "Online",
        UserStatus::Ocupado => "Ocupado",
        UserStatus::Ausente => "Ausente",
        UserStatus::Invisivel => "Invisivel",
        UserStatus::Offline => "Offline",
    }
}

fn str_to_status(s: &str) -> UserStatus {
    match s {
        "Online" => UserStatus::Online,
        "Ocupado" => UserStatus::Ocupado,
        "Ausente" => UserStatus::Ausente,
        "Invisivel" => UserStatus::Invisivel,
        _ => UserStatus::Offline,
    }
}

fn theme_to_str(theme: &AppTheme) -> &'static str {
    match theme {
        AppTheme::AeroBlue => "AeroBlue",
        AppTheme::RubyPink => "RubyPink",
        AppTheme::ForestGreen => "ForestGreen",
        AppTheme::SilverMetallic => "SilverMetallic",
    }
}

fn str_to_theme(s: &str) -> AppTheme {
    match s {
        "RubyPink" => AppTheme::RubyPink,
        "ForestGreen" => AppTheme::ForestGreen,
        "SilverMetallic" => AppTheme::SilverMetallic,
        _ => AppTheme::AeroBlue,
    }
}
