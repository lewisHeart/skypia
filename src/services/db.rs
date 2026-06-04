use crate::models::{Contact, Message, UserStatus, BannerInfo, FileTransferState, AppTheme};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::sync::OnceLock;

static DB: OnceLock<Mutex<MockDatabase>> = OnceLock::new();

fn get_db() -> &'static Mutex<MockDatabase> {
    DB.get_or_init(|| Mutex::new(MockDatabase::new()))
}

fn trigger_save(db: &MockDatabase) {
    if let Ok(data) = serde_json::to_string_pretty(db) {
        tokio::spawn(async move {
            let _ = tokio::fs::write("skypia_db.json", data).await;
        });
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MockDatabase {
    contacts: Vec<Contact>,
    messages: HashMap<usize, Vec<Message>>,
    user_name: String,
    user_status: UserStatus,
    personal_message: String,
    user_music: Option<String>,
    avatar_id: usize,
    detached_chats: HashSet<usize>,
    interface_scale: f64,
    use_custom_titlebar: bool,
    theme: AppTheme,
}

impl MockDatabase {
    fn new() -> Self {
        // Tenta carregar os dados persistidos do arquivo JSON
        if let Ok(content) = std::fs::read_to_string("skypia_db.json") {
            if let Ok(db) = serde_json::from_str::<MockDatabase>(&content) {
                return db;
            }
        }

        // Se o arquivo não existir ou for inválido, cria o estado inicial padrão
        let db = Self::default_mock();
        // Salva o estado inicial no disco
        if let Ok(data) = serde_json::to_string_pretty(&db) {
            let _ = std::fs::write("skypia_db.json", data);
        }
        db
    }

    fn default_mock() -> Self {
        let contacts = vec![
            Contact {
                id: 1,
                email: "lucas_heavy@hotmail.com".to_string(),
                display_name: "Lucas [Emo Core]".to_string(),
                status: UserStatus::Online,
                personal_message: "Sei que o amanhã trará esperança... 🎧 NX Zero".to_string(),
                music_listening: Some("NX Zero - Cedo Ou Tarde".to_string()),
                avatar_id: 1,
                is_favorite: true,
            },
            Contact {
                id: 2,
                email: "gabi_sz@live.com".to_string(),
                display_name: "Gabii *_* (ausente)".to_string(),
                status: UserStatus::Ausente,
                personal_message: "Estudando para a prova de física... nao perturbe".to_string(),
                music_listening: None,
                avatar_id: 2,
                is_favorite: true,
            },
            Contact {
                id: 3,
                email: "felipe.games@skypia.io".to_string(),
                display_name: "Felipe [Jogando CS 1.6]".to_string(),
                status: UserStatus::Ocupado,
                personal_message: "Dando HS no de_dust2! Sem convite p/ call".to_string(),
                music_listening: None,
                avatar_id: 3,
                is_favorite: false,
            },
            Contact {
                id: 4,
                email: "mari_ballet@gmail.com".to_string(),
                display_name: "Mariana ✨".to_string(),
                status: UserStatus::Offline,
                personal_message: "Offline é mais legal... tchau!".to_string(),
                music_listening: None,
                avatar_id: 4,
                is_favorite: false,
            },
            Contact {
                id: 5,
                email: "thiago_rock@hotmail.com".to_string(),
                display_name: "Thiago [Linkin Park fan]".to_string(),
                status: UserStatus::Online,
                personal_message: "In the end, it doesn't even matter...".to_string(),
                music_listening: Some("Linkin Park - In The End".to_string()),
                avatar_id: 5,
                is_favorite: false,
            },
            Contact {
                id: 6,
                email: "aninha_loves@skypia.io".to_string(),
                display_name: "Ana Carolina ♥".to_string(),
                status: UserStatus::Offline,
                personal_message: "Sorria, mesmo sem motivos!".to_string(),
                music_listening: None,
                avatar_id: 6,
                is_favorite: false,
            },
        ];

        let mut messages = HashMap::new();
        messages.insert(1, vec![
            Message {
                id: 1,
                sender_id: 1,
                sender_name: "Lucas [Emo Core]".to_string(),
                text: "Eae cara! blz?".to_string(),
                timestamp: "02:10:15".to_string(),
                is_nudge: false,
                font_color: "#e6007e".to_string(),
                font_family: "Comic Sans MS".to_string(),
                is_wink: None,
                file_transfer: None,
                is_game_invite: false,
            },
            Message {
                id: 2,
                sender_id: 0,
                sender_name: "Você".to_string(),
                text: "Fala Lucas! Tudo ótimo por aqui. E contigo?".to_string(),
                timestamp: "02:11:00".to_string(),
                is_nudge: false,
                font_color: "#0066cc".to_string(),
                font_family: "Segoe UI".to_string(),
                is_wink: None,
                file_transfer: None,
                is_game_invite: false,
            },
            Message {
                id: 3,
                sender_id: 1,
                sender_name: "Lucas [Emo Core]".to_string(),
                text: "Tranquilo, escutando o novo cd do NX Zero, mto bom (Y)".to_string(),
                timestamp: "02:11:32".to_string(),
                is_nudge: false,
                font_color: "#e6007e".to_string(),
                font_family: "Comic Sans MS".to_string(),
                is_wink: None,
                file_transfer: None,
                is_game_invite: false,
            },
        ]);

        messages.insert(2, vec![
            Message {
                id: 4,
                sender_id: 2,
                sender_name: "Gabii *_* (ausente)".to_string(),
                text: "Oi, quando voltar a gente se fala!".to_string(),
                timestamp: "23:45:10".to_string(),
                is_nudge: false,
                font_color: "#bb00cc".to_string(),
                font_family: "Arial".to_string(),
                is_wink: None,
                file_transfer: None,
                is_game_invite: false,
            },
        ]);

        Self {
            contacts,
            messages,
            user_name: "Wellington Skypia".to_string(),
            user_status: UserStatus::Online,
            personal_message: "Codando meu próprio clone do Skypia em Dioxus! (H)".to_string(),
            user_music: Some("Coldplay - Viva La Vida".to_string()),
            avatar_id: 0,
            detached_chats: HashSet::new(),
            interface_scale: 1.0,
            use_custom_titlebar: true,
            theme: AppTheme::AeroBlue,
        }
    }
}

pub struct DatabaseService;

impl DatabaseService {
    // Carrega nome do usuário
    pub async fn load_user_name() -> Result<String, String> {
        let db = get_db().lock().map_err(|e| e.to_string())?;
        Ok(db.user_name.clone())
    }

    // Carrega música no perfil do usuário
    pub async fn load_user_music() -> Result<Option<String>, String> {
        let db = get_db().lock().map_err(|e| e.to_string())?;
        Ok(db.user_music.clone())
    }

    // Carrega contatos
    pub async fn load_contacts() -> Result<Vec<Contact>, String> {
        let db = get_db().lock().map_err(|e| e.to_string())?;
        Ok(db.contacts.clone())
    }

    // Carrega histórico de mensagens de um contato
    pub async fn load_messages(contact_id: usize) -> Result<Vec<Message>, String> {
        let db = get_db().lock().map_err(|e| e.to_string())?;
        Ok(db.messages.get(&contact_id).cloned().unwrap_or_default())
    }

    // Salva uma mensagem no histórico
    pub async fn save_message(contact_id: usize, message: Message) -> Result<(), String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.messages.entry(contact_id).or_default().push(message);
        trigger_save(&db);
        Ok(())
    }

    // Atualiza nome do usuário
    pub async fn save_user_name(name: String) -> Result<(), String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.user_name = name;
        trigger_save(&db);
        Ok(())
    }

    // Atualiza música no perfil do usuário
    pub async fn save_user_music(music: Option<String>) -> Result<(), String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.user_music = music;
        trigger_save(&db);
        Ok(())
    }

    // Atualiza mensagem pessoal do usuário logado
    pub async fn save_personal_message(msg: String) -> Result<(), String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.personal_message = msg;
        trigger_save(&db);
        Ok(())
    }

    // Atualiza status do usuário
    pub async fn save_user_status(status: UserStatus) -> Result<(), String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.user_status = status;
        trigger_save(&db);
        Ok(())
    }

    // Atualiza avatar do usuário
    pub async fn save_user_avatar(avatar_id: usize) -> Result<(), String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.avatar_id = avatar_id;
        trigger_save(&db);
        Ok(())
    }

    // Adiciona contato
    pub async fn add_contact(email: String, display_name: String, status: UserStatus, personal_message: String) -> Result<Contact, String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        let next_id = db.contacts.len() + 1;
        let c = Contact {
            id: next_id,
            email,
            display_name,
            status,
            personal_message,
            music_listening: None,
            avatar_id: (next_id % 7),
            is_favorite: false,
        };
        db.contacts.push(c.clone());
        trigger_save(&db);
        Ok(c)
    }

    // Atualiza favorito de um contato
    pub async fn save_contact_favorite(contact_id: usize, is_favorite: bool) -> Result<(), String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        if let Some(c) = db.contacts.iter_mut().find(|c| c.id == contact_id) {
            c.is_favorite = is_favorite;
        }
        trigger_save(&db);
        Ok(())
    }

    // Gerenciamento de Janelas de Chat nativas desvinculadas
    pub async fn detach_chat(contact_id: usize) -> Result<(), String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.detached_chats.insert(contact_id);
        trigger_save(&db);
        Ok(())
    }

    pub async fn attach_chat(contact_id: usize) -> Result<(), String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.detached_chats.remove(&contact_id);
        trigger_save(&db);
        Ok(())
    }

    pub async fn get_detached_chats() -> Result<Vec<usize>, String> {
        let db = get_db().lock().map_err(|e| e.to_string())?;
        Ok(db.detached_chats.iter().copied().collect())
    }

    // Carrega configurações gerais (escala, barra personalizada, tema)
    pub async fn load_settings() -> Result<(f64, bool, AppTheme), String> {
        let db = get_db().lock().map_err(|e| e.to_string())?;
        Ok((db.interface_scale, db.use_custom_titlebar, db.theme))
    }

    // Salva configurações gerais
    pub async fn save_settings(scale: f64, custom_bar: bool, theme: AppTheme) -> Result<(), String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.interface_scale = scale;
        db.use_custom_titlebar = custom_bar;
        db.theme = theme;
        trigger_save(&db);
        Ok(())
    }

    pub async fn get_banner_info() -> Result<BannerInfo, String> {
        let banners = vec![
            BannerInfo {
                text: "Navegue na web com muito mais velocidade e segurança!".to_string(),
                action_label: "Instalar Skypia Browser".to_string(),
                link: "https://skypia.io/browser".to_string(),
                icon: "🌐".to_string(),
            },
            BannerInfo {
                text: "Ouça as melhores músicas retrô com alta fidelidade!".to_string(),
                action_label: "Skypia Music Premium".to_string(),
                link: "https://skypia.io/music".to_string(),
                icon: "🎵".to_string(),
            },
            BannerInfo {
                text: "Seus e-mails e arquivos protegidos em um só lugar.".to_string(),
                action_label: "Acessar Skypia Mail".to_string(),
                link: "https://skypia.io/mail".to_string(),
                icon: "📧".to_string(),
            },
            BannerInfo {
                text: "Espaço gratuito ilimitado para suas fotos e dados na nuvem.".to_string(),
                action_label: "Conhecer Skypia Drive".to_string(),
                link: "https://skypia.io/drive".to_string(),
                icon: "💾".to_string(),
            },
        ];
        
        // Rotaciona a cada 15 segundos baseado no tempo atual
        use std::time::SystemTime;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let idx = (now / 15) as usize % banners.len();
        Ok(banners[idx].clone())
    }

    pub async fn get_recommended_songs() -> Result<Vec<String>, String> {
        Ok(vec![
            "NX Zero - Cedo Ou Tarde".to_string(),
            "Coldplay - Viva La Vida".to_string(),
            "Linkin Park - In The End".to_string(),
            "Green Day - Boulevard of Broken Dreams".to_string(),
            "Blink-182 - I Miss You".to_string(),
            "Evanescence - Bring Me To Life".to_string(),
            "Simple Plan - Welcome to My Life".to_string(),
            "Fresno - Alguém Que Te Faz Sorrir".to_string(),
            "Paramore - Decode".to_string(),
            "Pitty - Admirável Chip Novo".to_string(),
        ])
    }
}
