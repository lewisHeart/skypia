#![allow(dead_code)]
use crate::models::{AppTheme, BannerInfo, Contact, Message, TicTacToe, UserStatus};
use dioxus::prelude::*;
use std::collections::HashMap;

mod auth;
mod chat;
mod contact;
mod game;
mod settings;
mod ui;
mod loader;
pub mod version;

#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    pub id: usize,
    pub title: String,
    pub message: String,
    pub avatar_url: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AppState {
    pub logged_in: Signal<bool>,
    pub signing_in: Signal<bool>,
    pub user_name: Signal<String>,
    pub user_email: Signal<String>,
    pub user_status: Signal<UserStatus>,
    pub user_personal_message: Signal<String>,
    pub user_music: Signal<Option<String>>,
    pub user_avatar_id: Signal<usize>,

    pub contacts: Signal<Vec<Contact>>,
    pub active_chats: Signal<Vec<String>>,        // contact_ids
    pub selected_chat_id: Signal<Option<String>>, // selected contact_id
    pub chat_messages: Signal<HashMap<String, Vec<Message>>>,
    pub toasts: Signal<Vec<Toast>>,
    pub theme: Signal<AppTheme>,
    pub toast_counter: Signal<usize>,
    pub message_counter: Signal<usize>,
    pub detached_chats: Signal<Vec<String>>, // contact_ids desvinculados em janelas nativas
    pub use_custom_titlebar: Signal<bool>,   // barra de título personalizada ativa
    pub interface_scale: Signal<f64>,        // fator de escala (zoom) do aplicativo
    pub chat_mode: Signal<String>,           // modo de chat (integrated ou detached)

    // Novos estados para Skypia completo e dinâmico
    pub banner_info: Signal<Option<BannerInfo>>,
    pub active_wink: Signal<Option<String>>, // wink sendo executado na tela ("kiss", "hammer", "pig")
    pub game_states: Signal<HashMap<String, TicTacToe>>, // jogo da velha por contato
    pub show_games_modal: Signal<bool>,
    pub show_settings_modal: Signal<bool>,
    pub show_add_contact_modal: Signal<bool>,
    pub show_music_player_modal: Signal<bool>,
    pub show_profile_modal: Signal<bool>,
    pub show_about: Signal<bool>,
    pub profile_modal_contact_id: Signal<Option<String>>,
    pub recommended_songs: Signal<Vec<String>>,

    // Autenticação real com o servidor
    pub auth_token: Signal<Option<String>>,
    pub server_user_id: Signal<Option<String>>,
    pub user_avatar_url: Signal<Option<String>>, // URL da foto real (do servidor)
    pub is_admin: Signal<bool>,
    pub show_register_modal: Signal<bool>,
    pub server_error: Signal<Option<String>>, // Último erro da API
    pub show_avatar_picker: Signal<bool>,

    // WebSocket e digitação em tempo real
    pub ws_tx: Signal<Option<tokio::sync::mpsc::UnboundedSender<crate::models::ClientAction>>>,
    pub typing_contacts: Signal<std::collections::HashMap<String, Vec<String>>>,

    // Solicitações pendentes e inatividade
    pub pending_requests: Signal<Vec<Contact>>,
    pub last_activity_time: Signal<u64>,
    pub was_automatically_away: Signal<bool>,
    pub active_nudge: Signal<Option<String>>,
    pub contact_density: Signal<String>,
    pub fav_density: Signal<String>,
    pub online_density: Signal<String>,
    pub offline_density: Signal<String>,
    pub groups_density: Signal<String>,
    pub unread_counts: Signal<HashMap<String, usize>>,
    pub group_chats: Signal<Vec<crate::models::Conversation>>,
    pub dragged_contact_id: Signal<Option<String>>,
    pub chat_font_color: Signal<String>,
    pub chat_font_family: Signal<String>,
    pub spotify_rpc_enabled: Signal<bool>,
    pub show_typing_notification: Signal<bool>,
    pub enable_sounds: Signal<bool>,
    pub enable_toasts: Signal<bool>,
    pub download_folder: Signal<String>,
    pub auto_accept_files: Signal<bool>,
    pub remember_password: Signal<bool>,
    pub save_chat_history: Signal<bool>,
    pub saved_email: Signal<String>,
    pub saved_password: Signal<String>,
    pub auto_login: Signal<bool>,
    pub categories: Signal<Vec<String>>,
    pub show_friend_requests_modal: Signal<bool>,
    pub show_group_profile_modal: Signal<bool>,
    pub group_profile_id: Signal<Option<String>>,
    pub window_x: Signal<i32>,
    pub window_y: Signal<i32>,
    pub window_width: Signal<f64>,
    pub window_height: Signal<f64>,
    pub fav_collapsed: Signal<bool>,
    pub online_collapsed: Signal<bool>,
    pub offline_collapsed: Signal<bool>,
    pub groups_collapsed: Signal<bool>,
    pub collapsed_categories: Signal<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            logged_in: Signal::new(false),
            signing_in: Signal::new(false),
            user_name: Signal::new(String::new()),
            user_email: Signal::new(String::new()),
            user_status: Signal::new(UserStatus::Offline),
            user_personal_message: Signal::new(String::new()),
            user_music: Signal::new(None),
            user_avatar_id: Signal::new(0),

            contacts: Signal::new(Vec::new()),
            active_chats: Signal::new(Vec::new()),
            selected_chat_id: Signal::new(None),
            chat_messages: Signal::new(HashMap::new()),
            toasts: Signal::new(Vec::new()),
            theme: Signal::new(AppTheme::AeroBlue),
            toast_counter: Signal::new(1),
            detached_chats: Signal::new(Vec::new()),
            #[cfg(target_os = "android")]
            use_custom_titlebar: Signal::new(false),
            #[cfg(not(target_os = "android"))]
            use_custom_titlebar: Signal::new(true),
            #[cfg(target_os = "android")]
            interface_scale: Signal::new(1.35),
            #[cfg(not(target_os = "android"))]
            interface_scale: Signal::new(1.0),
            chat_mode: Signal::new("integrated".to_string()),

            banner_info: Signal::new(None),
            active_wink: Signal::new(None),
            game_states: Signal::new(HashMap::new()),
            show_games_modal: Signal::new(false),
            show_settings_modal: Signal::new(false),
            show_add_contact_modal: Signal::new(false),
            show_music_player_modal: Signal::new(false),
            show_profile_modal: Signal::new(false),
            show_about: Signal::new(false),
            profile_modal_contact_id: Signal::new(None),
            recommended_songs: Signal::new(Vec::new()),

            auth_token: Signal::new(None),
            server_user_id: Signal::new(None),
            user_avatar_url: Signal::new(None),
            is_admin: Signal::new(false),
            show_register_modal: Signal::new(false),
            server_error: Signal::new(None),
            show_avatar_picker: Signal::new(false),
            ws_tx: Signal::new(None),
            typing_contacts: Signal::new(std::collections::HashMap::new()),

            pending_requests: Signal::new(Vec::new()),
            last_activity_time: Signal::new(chrono::Utc::now().timestamp() as u64),
            was_automatically_away: Signal::new(false),
            active_nudge: Signal::new(None),
            contact_density: Signal::new("medium".to_string()),
            fav_density: Signal::new("medium".to_string()),
            online_density: Signal::new("medium".to_string()),
            offline_density: Signal::new("medium".to_string()),
            groups_density: Signal::new("medium".to_string()),
            unread_counts: Signal::new(HashMap::new()),
            group_chats: Signal::new(Vec::new()),
            dragged_contact_id: Signal::new(None),
            chat_font_color: Signal::new("#1e395b".to_string()),
            chat_font_family: Signal::new("Segoe UI".to_string()),
            message_counter: Signal::new(1),
            spotify_rpc_enabled: Signal::new(false),
            show_typing_notification: Signal::new(true),
            enable_sounds: Signal::new(true),
            enable_toasts: Signal::new(true),
            download_folder: Signal::new(String::new()),
            auto_accept_files: Signal::new(false),
            remember_password: Signal::new(true),
            save_chat_history: Signal::new(true),
            saved_email: Signal::new(String::new()),
            saved_password: Signal::new(String::new()),
            auto_login: Signal::new(false),
            categories: Signal::new(Vec::new()),
            show_friend_requests_modal: Signal::new(false),
            show_group_profile_modal: Signal::new(false),
            group_profile_id: Signal::new(None),
            window_x: Signal::new(100),
            window_y: Signal::new(100),
            window_width: Signal::new(413.0),
            window_height: Signal::new(735.0),
            fav_collapsed: Signal::new(false),
            online_collapsed: Signal::new(false),
            offline_collapsed: Signal::new(false),
            groups_collapsed: Signal::new(false),
            collapsed_categories: Signal::new("[]".to_string()),
        }
    }



    pub fn set_user_name(&mut self, name: String) {
        *self.user_name.write() = name.clone();
        crate::state::version::increment_state_version();

        // Atualiza via WebSocket em tempo real se conectado
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(crate::models::ClientAction::UpdatePresence {
                status: None,
                personal_message: None,
                music: None,
                display_name: Some(name.clone()),
            });
        }

        let token_opt = self.auth_token();
        let has_ws = self.ws_tx.read().is_some();
        spawn(async move {
            if let Err(e) = crate::services::db::DatabaseService::save_user_name(name.clone()).await {
                eprintln!("❌ Erro ao salvar nome de usuário no SQLite: {}", e);
            }
            if !has_ws {
                if let Some(token) = token_opt {
                    let _ = crate::services::api::update_profile(
                        &token,
                        crate::services::api::UpdateProfileRequest {
                            display_name: Some(name),
                            personal_message: None,
                            status: None,
                            music: None,
                        },
                    )
                    .await;
                }
            }
        });
    }

    pub fn set_user_status(&mut self, status: UserStatus) {
        *self.user_status.write() = status;
        crate::state::version::increment_state_version();
        if status == UserStatus::Offline {
            self.logout();
        }

        let status_str = match status {
            UserStatus::Online => "Online",
            UserStatus::Ocupado => "Ocupado",
            UserStatus::Ausente => "Ausente",
            UserStatus::Invisivel => "Invisivel",
            UserStatus::Offline => "Offline",
        };

        // Atualiza via WebSocket em tempo real se conectado
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(crate::models::ClientAction::UpdatePresence {
                status: Some(status_str.to_string()),
                personal_message: None,
                music: None,
                display_name: None,
            });
        }

        let token_opt = self.auth_token();
        let has_ws = self.ws_tx.read().is_some();
        spawn(async move {
            if let Err(e) = crate::services::db::DatabaseService::save_user_status(status).await {
                eprintln!("❌ Erro ao salvar status de usuário no SQLite: {}", e);
            }
            if !has_ws {
                if let Some(token) = token_opt {
                    let _ = crate::services::api::update_profile(
                        &token,
                        crate::services::api::UpdateProfileRequest {
                            display_name: None,
                            personal_message: None,
                            status: Some(status_str.to_string()),
                            music: None,
                        },
                    )
                    .await;
                }
            }
        });
    }

    pub fn set_user_avatar(&mut self, avatar_id: usize) {
        *self.user_avatar_id.write() = avatar_id;
        crate::state::version::increment_state_version();
        spawn(async move {
            if let Err(e) = crate::services::db::DatabaseService::save_user_avatar(avatar_id).await {
                eprintln!("❌ Erro ao salvar avatar de usuário no SQLite: {}", e);
            }
        });
    }

    pub fn set_user_personal_message(&mut self, msg: String) {
        *self.user_personal_message.write() = msg.clone();
        crate::state::version::increment_state_version();

        // Atualiza via WebSocket em tempo real se conectado
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(crate::models::ClientAction::UpdatePresence {
                status: None,
                personal_message: Some(msg.clone()),
                music: None,
                display_name: None,
            });
        }

        let token_opt = self.auth_token();
        let has_ws = self.ws_tx.read().is_some();
        spawn(async move {
            if let Err(e) = crate::services::db::DatabaseService::save_personal_message(msg.clone()).await {
                eprintln!("❌ Erro ao salvar recado pessoal no SQLite: {}", e);
            }
            if !has_ws {
                if let Some(token) = token_opt {
                    let _ = crate::services::api::update_profile(
                        &token,
                        crate::services::api::UpdateProfileRequest {
                            display_name: None,
                            personal_message: Some(msg),
                            status: None,
                            music: None,
                        },
                    )
                    .await;
                }
            }
        });
    }

    pub fn set_user_music(&mut self, music: Option<String>) {
        *self.user_music.write() = music.clone();
        crate::state::version::increment_state_version();

        // Atualiza via WebSocket em tempo real se conectado
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(crate::models::ClientAction::UpdatePresence {
                status: None,
                personal_message: None,
                music: Some(music.clone()),
                display_name: None,
            });
        }

        let token_opt = self.auth_token();
        let has_ws = self.ws_tx.read().is_some();
        spawn(async move {
            if let Err(e) = crate::services::db::DatabaseService::save_user_music(music.clone()).await {
                eprintln!("❌ Erro ao salvar recado de música no SQLite: {}", e);
            }
            if !has_ws {
                if let Some(token) = token_opt {
                    let _ = crate::services::api::update_profile(
                        &token,
                        crate::services::api::UpdateProfileRequest {
                            display_name: None,
                            personal_message: None,
                            status: None,
                            music: Some(music),
                        },
                    )
                    .await;
                }
            }
        });
    }



    // Getters convenientes
    pub fn logged_in(&self) -> bool {
        (self.logged_in)()
    }

    pub fn signing_in(&self) -> bool {
        (self.signing_in)()
    }

    pub fn user_name(&self) -> String {
        self.user_name.read().clone()
    }

    pub fn user_email(&self) -> String {
        self.user_email.read().clone()
    }

    pub fn user_status(&self) -> UserStatus {
        (self.user_status)()
    }

    pub fn user_personal_message(&self) -> String {
        self.user_personal_message.read().clone()
    }

    pub fn user_music(&self) -> Option<String> {
        self.user_music.read().clone()
    }

    pub fn user_avatar_id(&self) -> usize {
        (self.user_avatar_id)()
    }

    pub fn contacts(&self) -> Vec<Contact> {
        self.contacts.read().clone()
    }

    pub fn active_chats(&self) -> Vec<String> {
        self.active_chats.read().clone()
    }

    pub fn selected_chat_id(&self) -> Option<String> {
        (self.selected_chat_id)()
    }

    pub fn chat_messages(&self) -> HashMap<String, Vec<Message>> {
        self.chat_messages.read().clone()
    }

    pub fn toasts(&self) -> Vec<Toast> {
        self.toasts.read().clone()
    }

    pub fn theme(&self) -> AppTheme {
        (self.theme)()
    }

    pub fn toast_counter(&self) -> usize {
        (self.toast_counter)()
    }

    pub fn message_counter(&self) -> usize {
        (self.message_counter)()
    }

    pub fn detached_chats(&self) -> Vec<String> {
        self.detached_chats.read().clone()
    }

    pub fn use_custom_titlebar(&self) -> bool {
        #[cfg(target_os = "android")]
        {
            false
        }
        #[cfg(not(target_os = "android"))]
        {
            (self.use_custom_titlebar)()
        }
    }

    pub fn interface_scale(&self) -> f64 {
        (self.interface_scale)()
    }

    pub fn chat_mode(&self) -> String {
        #[cfg(target_os = "android")]
        {
            "integrated".to_string()
        }
        #[cfg(not(target_os = "android"))]
        {
            self.chat_mode.read().clone()
        }
    }

    pub fn banner_info(&self) -> Option<BannerInfo> {
        self.banner_info.read().clone()
    }

    pub fn active_wink(&self) -> Option<String> {
        self.active_wink.read().clone()
    }

    pub fn game_states(&self) -> HashMap<String, TicTacToe> {
        self.game_states.read().clone()
    }

    pub fn recommended_songs(&self) -> Vec<String> {
        self.recommended_songs.read().clone()
    }

    pub fn auth_token(&self) -> Option<String> {
        self.auth_token.read().clone()
    }

    pub fn server_user_id(&self) -> Option<String> {
        (self.server_user_id)()
    }

    pub fn user_avatar_url(&self) -> Option<String> {
        self.user_avatar_url.read().clone()
    }

    pub fn show_register_modal(&self) -> bool {
        (self.show_register_modal)()
    }

    pub fn server_error(&self) -> Option<String> {
        self.server_error.read().clone()
    }

    pub fn show_avatar_picker(&self) -> bool {
        (self.show_avatar_picker)()
    }

    pub fn typing_contacts(&self) -> HashMap<String, Vec<String>> {
        self.typing_contacts.read().clone()
    }

    pub fn pending_requests(&self) -> Vec<Contact> {
        self.pending_requests.read().clone()
    }

    pub fn last_activity_time(&self) -> u64 {
        (self.last_activity_time)()
    }

    pub fn was_automatically_away(&self) -> bool {
        (self.was_automatically_away)()
    }

    pub fn contact_density(&self) -> String {
        self.contact_density.read().clone()
    }

    pub fn fav_density(&self) -> String {
        self.fav_density.read().clone()
    }

    pub fn online_density(&self) -> String {
        self.online_density.read().clone()
    }

    pub fn offline_density(&self) -> String {
        self.offline_density.read().clone()
    }

    pub fn groups_density(&self) -> String {
        self.groups_density.read().clone()
    }

    pub fn is_admin(&self) -> bool {
        (self.is_admin)()
    }

    pub fn set_contact_density(&mut self, density: String) {
        *self.contact_density.write() = density;
        self.save_current_settings();
    }

    pub fn set_category_density(&mut self, category: &str, density: String) {
        match category {
            "fav" => *self.fav_density.write() = density,
            "online" => *self.online_density.write() = density,
            "offline" => *self.offline_density.write() = density,
            "groups" => *self.groups_density.write() = density,
            _ => {}
        }

        let fav = self.fav_density.read().clone();
        let online = self.online_density.read().clone();
        let offline = self.offline_density.read().clone();
        let groups = self.groups_density.read().clone();
        let serialized = format!("fav:{},online:{},offline:{},groups:{}", fav, online, offline, groups);
        
        *self.contact_density.write() = serialized;
        self.save_current_settings();
    }

    pub fn update_densities_from_serialized(&mut self, density: String) {
        *self.contact_density.write() = density.clone();
        let (fav, online, offline, groups) = if density.contains(':') {
            let mut f = "medium".to_string();
            let mut o = "medium".to_string();
            let mut off = "medium".to_string();
            let mut g = "medium".to_string();
            for part in density.split(',') {
                let subparts: Vec<&str> = part.split(':').collect();
                if subparts.len() == 2 {
                    match subparts[0] {
                        "fav" => f = subparts[1].to_string(),
                        "online" => o = subparts[1].to_string(),
                        "offline" => off = subparts[1].to_string(),
                        "groups" => g = subparts[1].to_string(),
                        _ => {}
                    }
                }
            }
            (f, o, off, g)
        } else {
            let d = density.as_str();
            let f = if d == "large" { "large".to_string() } else { "medium".to_string() };
            let o = if d == "small" { "small".to_string() } else { "medium".to_string() };
            let off = if d == "small" { "small".to_string() } else { "medium".to_string() };
            let g = "medium".to_string();
            (f, o, off, g)
        };

        *self.fav_density.write() = fav;
        *self.online_density.write() = online;
        *self.offline_density.write() = offline;
        *self.groups_density.write() = groups;
    }

    pub fn active_nudge(&self) -> Option<String> {
        self.active_nudge.read().clone()
    }

    pub fn unread_counts(&self) -> HashMap<String, usize> {
        self.unread_counts.read().clone()
    }

    pub fn unread_count_for(&self, contact_id: &str) -> usize {
        self.unread_counts.read().get(contact_id).copied().unwrap_or(0)
    }

    pub fn increment_unread(&mut self, contact_id: &str) {
        let mut counts = self.unread_counts.write();
        let count = counts.entry(contact_id.to_string()).or_insert(0);
        *count += 1;
    }

    pub fn mark_chat_read(&mut self, contact_id: &str) {
        self.unread_counts.write().remove(contact_id);
    }

    pub fn update_banner_admin(&mut self, banner: crate::models::BannerInfo) {
        *self.banner_info.write() = Some(banner.clone());
        crate::state::version::increment_state_version();
        let mut state_clone = *self;
        let token_opt = self.auth_token();
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_banner(&banner).await;
            if let Some(token) = token_opt {
                match crate::services::api::update_banner(&token, &banner).await {
                    Ok(_) => {
                        println!("Banner atualizado com sucesso no servidor");
                    }
                    Err(e) => {
                        state_clone.add_toast(
                            "Erro no Servidor".to_string(),
                            format!("Não foi possível salvar o anúncio no servidor: {}", e),
                            None,
                        );
                    }
                }
            }
        });
    }

    pub fn group_chats(&self) -> Vec<crate::models::Conversation> {
        self.group_chats.read().clone()
    }
}
