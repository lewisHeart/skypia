use crate::models::{Contact, Message, UserStatus};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::sync::OnceLock;

static DB: OnceLock<Mutex<MockDatabase>> = OnceLock::new();

fn get_db() -> &'static Mutex<MockDatabase> {
    DB.get_or_init(|| Mutex::new(MockDatabase::new()))
}

struct MockDatabase {
    contacts: Vec<Contact>,
    messages: HashMap<usize, Vec<Message>>,
    user_status: UserStatus,
    personal_message: String,
    avatar_id: usize,
    detached_chats: HashSet<usize>,
}

impl MockDatabase {
    fn new() -> Self {
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
                email: "felipe.games@msn.com".to_string(),
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
                email: "aninha_loves@msn.com".to_string(),
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
            },
        ]);

        Self {
            contacts,
            messages,
            user_status: UserStatus::Online,
            personal_message: "Codando meu próprio clone do MSN em Dioxus! (H)".to_string(),
            avatar_id: 0,
            detached_chats: HashSet::new(),
        }
    }
}

pub struct DatabaseService;

impl DatabaseService {
    // Carrega contatos
    pub async fn load_contacts() -> Result<Vec<Contact>, String> {
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        let db = get_db().lock().map_err(|e| e.to_string())?;
        Ok(db.contacts.clone())
    }

    // Carrega histórico de mensagens de um contato
    pub async fn load_messages(contact_id: usize) -> Result<Vec<Message>, String> {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let db = get_db().lock().map_err(|e| e.to_string())?;
        Ok(db.messages.get(&contact_id).cloned().unwrap_or_default())
    }

    // Salva uma mensagem no histórico
    pub async fn save_message(contact_id: usize, message: Message) -> Result<(), String> {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.messages.entry(contact_id).or_default().push(message);
        Ok(())
    }

    // Atualiza mensagem pessoal do usuário logado
    pub async fn save_personal_message(msg: String) -> Result<(), String> {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.personal_message = msg;
        Ok(())
    }

    // Atualiza status do usuário
    pub async fn save_user_status(status: UserStatus) -> Result<(), String> {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.user_status = status;
        Ok(())
    }

    // Atualiza avatar do usuário
    pub async fn save_user_avatar(avatar_id: usize) -> Result<(), String> {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.avatar_id = avatar_id;
        Ok(())
    }

    // Atualiza favorito de um contato
    pub async fn save_contact_favorite(contact_id: usize, is_favorite: bool) -> Result<(), String> {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        if let Some(c) = db.contacts.iter_mut().find(|c| c.id == contact_id) {
            c.is_favorite = is_favorite;
        }
        Ok(())
    }

    // Gerenciamento de Janelas de Chat nativas desvinculadas
    pub async fn detach_chat(contact_id: usize) -> Result<(), String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.detached_chats.insert(contact_id);
        Ok(())
    }

    pub async fn attach_chat(contact_id: usize) -> Result<(), String> {
        let mut db = get_db().lock().map_err(|e| e.to_string())?;
        db.detached_chats.remove(&contact_id);
        Ok(())
    }

    pub async fn get_detached_chats() -> Result<Vec<usize>, String> {
        let db = get_db().lock().map_err(|e| e.to_string())?;
        Ok(db.detached_chats.iter().copied().collect())
    }
}
