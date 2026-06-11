#![allow(dead_code)]
use crate::models::{AppTheme, BannerInfo, Contact, Message, TicTacToe, UserStatus};
use dioxus::prelude::*;
use std::collections::HashMap;

mod auth;
mod chat;
mod contact;
mod game;

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
    pub categories: Signal<Vec<String>>,
    pub show_friend_requests_modal: Signal<bool>,
    pub show_group_profile_modal: Signal<bool>,
    pub group_profile_id: Signal<Option<String>>,
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
            categories: Signal::new(Vec::new()),
            show_friend_requests_modal: Signal::new(false),
            show_group_profile_modal: Signal::new(false),
            group_profile_id: Signal::new(None),
        }
    }

    // Carrega dados dinâmicos do banco assincronamente
    pub fn load_initial_data(&mut self) {
        let mut contacts_sig = self.contacts;
        let mut chat_messages_sig = self.chat_messages;
        let mut name_sig = self.user_name;
        let mut music_sig = self.user_music;
        let mut banner_sig = self.banner_info;
        let mut songs_sig = self.recommended_songs;
        let mut scale_sig = self.interface_scale;
        let mut custom_bar_sig = self.use_custom_titlebar;
        let mut theme_sig = self.theme;
        let mut chat_mode_sig = self.chat_mode;
        let mut pending_sig = self.pending_requests;
        let mut group_chats_sig = self.group_chats;
        let mut chat_font_color_sig = self.chat_font_color;
        let mut chat_font_family_sig = self.chat_font_family;
        let mut detached_sig = self.detached_chats;
        let mut avatar_url_sig = self.user_avatar_url;
        let mut spotify_rpc_sig = self.spotify_rpc_enabled;
        let mut show_typing_sig = self.show_typing_notification;
        let mut enable_sounds_sig = self.enable_sounds;
        let mut enable_toasts_sig = self.enable_toasts;
        let mut download_folder_sig = self.download_folder;
        let mut auto_accept_sig = self.auto_accept_files;
        let mut remember_password_sig = self.remember_password;
        let mut save_history_sig = self.save_chat_history;
        let mut categories_sig = self.categories;

        let token_opt = self.auth_token();
        let self_user_id = self.server_user_id();
        let mut self_clone = *self;

        spawn(async move {
            // Carrega categorias locais
            if let Ok(cats) = crate::services::db::DatabaseService::get_categories().await {
                *categories_sig.write() = cats;
            }

            // 0. Carrega contatos e conversas locais do SQLite imediatamente (Offline-first!)
            if let Ok(local_contacts) = crate::services::db::DatabaseService::load_contacts().await {
                if !local_contacts.is_empty() {
                    *contacts_sig.write() = local_contacts;
                }
            }
            if let Ok(local_conversations) = crate::services::db::DatabaseService::load_conversations().await {
                let mut all_messages = std::collections::HashMap::new();
                let mut groups = Vec::new();
                for conv in local_conversations {
                    if conv.is_group {
                        groups.push(conv.clone());
                    }
                    if let Ok(local_messages) = crate::services::db::DatabaseService::load_messages(conv.id.clone()).await {
                        let mut normalized_messages = Vec::new();
                        for mut msg in local_messages {
                            if let Some(ref s_id) = self_user_id {
                                if &msg.sender_id == s_id {
                                    msg.sender_id = "0".to_string();
                                }
                            }
                            normalized_messages.push(msg);
                        }
                        all_messages.insert(conv.id.clone(), normalized_messages);
                    }
                }
                *chat_messages_sig.write() = all_messages;
                *group_chats_sig.write() = groups;
            }

            if let Ok(db_settings) = crate::services::db::DatabaseService::load_settings().await {
                *scale_sig.write() = db_settings.interface_scale;
                *custom_bar_sig.write() = db_settings.use_custom_titlebar;
                *theme_sig.write() = crate::services::db::str_to_theme(&db_settings.theme);
                *chat_mode_sig.write() = db_settings.chat_mode;
                self_clone.update_densities_from_serialized(db_settings.contact_density);
                *chat_font_color_sig.write() = db_settings.font_color;
                *chat_font_family_sig.write() = db_settings.font_family;
                *spotify_rpc_sig.write() = db_settings.spotify_rpc_enabled;
                *show_typing_sig.write() = db_settings.show_typing_notification;
                *enable_sounds_sig.write() = db_settings.enable_sounds;
                *enable_toasts_sig.write() = db_settings.enable_toasts;
                *download_folder_sig.write() = db_settings.download_folder;
                *auto_accept_sig.write() = db_settings.auto_accept_files;
                *remember_password_sig.write() = db_settings.remember_password;
                *save_history_sig.write() = db_settings.save_chat_history;
            }

            // Sincronização de rede se autenticado
            if let Some(token) = token_opt {
                // 1. Busca contatos do servidor e salva em memória
                if let Ok(srv_contacts) = crate::services::api::get_contacts(&token).await {
                    // Carrega favoritos e categorias locais do SQLite
                    let (local_favorites, local_categories_map) = if let Ok(local_list) =
                        crate::services::db::DatabaseService::load_contacts().await
                    {
                        let mut favs = std::collections::HashSet::new();
                        let mut cats = std::collections::HashMap::new();
                        for c in local_list {
                            if c.is_favorite {
                                favs.insert(c.id.clone());
                            }
                            if let Some(cat) = c.category_name {
                                cats.insert(c.id.clone(), cat);
                            }
                        }
                        (favs, cats)
                    } else {
                        (std::collections::HashSet::new(), std::collections::HashMap::new())
                    };

                    let mut contacts_mapped = Vec::new();
                    for profile in srv_contacts {
                        let status_enum = match profile.status.as_str() {
                            "Online" => UserStatus::Online,
                            "Ocupado" => UserStatus::Ocupado,
                            "Ausente" => UserStatus::Ausente,
                            "Invisivel" => UserStatus::Invisivel,
                            _ => UserStatus::Offline,
                        };
                        let is_fav = local_favorites.contains(&profile.id);
                        let cat_name = local_categories_map.get(&profile.id).cloned();
                        contacts_mapped.push(Contact {
                            id: profile.id.clone(),
                            email: profile.email,
                            display_name: profile.display_name,
                            status: status_enum,
                            personal_message: profile.personal_message,
                            music_listening: profile.music,
                            avatar_url: profile.avatar_url,
                            is_favorite: is_fav,
                            relation_status: profile
                                .relation_status
                                .unwrap_or_else(|| "Aceito".to_string()),
                            nickname: profile.nickname,
                            category_name: cat_name,
                        });
                    }
                    *contacts_sig.write() = contacts_mapped.clone();
                    for contact in contacts_mapped {
                        spawn(async move {
                            let _ = crate::services::db::DatabaseService::save_contact(&contact).await;
                        });
                    }
                }

                // 1.1 Busca solicitações pendentes do servidor
                if let Ok(pending_srv) = crate::services::api::get_pending_requests(&token).await {
                    let contacts_mapped: Vec<Contact> = pending_srv
                        .into_iter()
                        .map(|profile| {
                            let status_enum = match profile.status.as_str() {
                                "Online" => UserStatus::Online,
                                "Ocupado" => UserStatus::Ocupado,
                                "Ausente" => UserStatus::Ausente,
                                "Invisivel" => UserStatus::Invisivel,
                                _ => UserStatus::Offline,
                            };
                            Contact {
                                id: profile.id,
                                email: profile.email,
                                display_name: profile.display_name,
                                status: status_enum,
                                personal_message: profile.personal_message,
                                music_listening: profile.music,
                                avatar_url: profile.avatar_url,
                                is_favorite: false,
                                relation_status: "Pendente".to_string(),
                                nickname: None,
                                category_name: None,
                            }
                        })
                        .collect();
                    *pending_sig.write() = contacts_mapped;
                }

                // 2. Busca histórico de mensagens das conversas do servidor
                if let Ok(srv_conversations) = crate::services::api::get_conversations(&token).await
                {
                    let _ = crate::services::db::DatabaseService::save_conversations(srv_conversations.clone()).await;
                    let mut all_messages = HashMap::new();
                    let mut groups = Vec::new();

                    for conv in srv_conversations {
                        if conv.is_group {
                            groups.push(conv.clone());
                            if let Ok(srv_messages) =
                                crate::services::api::get_conversation_messages(&token, &conv.id)
                                    .await
                            {
                                for msg in &srv_messages {
                                    let _ = crate::services::db::DatabaseService::save_message(conv.id.clone(), msg.clone()).await;
                                }
                                let mut normalized_messages = Vec::new();
                                for mut msg in srv_messages {
                                    if let Some(ref s_id) = self_user_id {
                                        if &msg.sender_id == s_id {
                                            msg.sender_id = "0".to_string();
                                        }
                                    }
                                    normalized_messages.push(msg);
                                }
                                all_messages.insert(conv.id.clone(), normalized_messages);
                            }
                        } else {
                            // Encontra o contato parceiro na conversa
                            let partner_opt = conv.members.iter().find(|member| {
                                if let Some(ref s_id) = self_user_id {
                                    &member.id != s_id
                                } else {
                                    true
                                }
                            });

                            if let Some(partner) = partner_opt {
                                let partner_id = partner.id.clone();
                                if let Ok(srv_messages) =
                                    crate::services::api::get_conversation_messages(&token, &conv.id)
                                        .await
                                {
                                    for msg in &srv_messages {
                                        let _ = crate::services::db::DatabaseService::save_message(conv.id.clone(), msg.clone()).await;
                                    }
                                    let mut normalized_messages = Vec::new();
                                    for mut msg in srv_messages {
                                        // Se a mensagem foi enviada pelo próprio usuário local, muda o sender_id para "0"
                                        if let Some(ref s_id) = self_user_id {
                                            if &msg.sender_id == s_id {
                                                msg.sender_id = "0".to_string();
                                            }
                                        }
                                        // Seta a conversation_id no front para ser o partner_id
                                        msg.conversation_id = partner_id.clone();
                                        normalized_messages.push(msg);
                                    }
                                    all_messages.insert(partner_id, normalized_messages);
                                }
                            }
                        }
                    }
                    *chat_messages_sig.write() = all_messages;
                    *group_chats_sig.write() = groups;
                }
            }

            if let Ok(detached) = crate::services::db::DatabaseService::get_detached_chats().await {
                *detached_sig.write() = detached;
            }

            if let Ok(name) = crate::services::db::DatabaseService::load_user_name().await {
                *name_sig.write() = name;
            }

            if let Ok(music) = crate::services::db::DatabaseService::load_user_music().await {
                *music_sig.write() = music;
            }

            if let Ok(avatar_url) = crate::services::db::DatabaseService::load_user_avatar_url().await {
                *avatar_url_sig.write() = avatar_url;
            }

            if let Ok(songs) = crate::services::db::DatabaseService::get_recommended_songs().await {
                *songs_sig.write() = songs;
            }

            if let Ok(Some(local_banner)) = crate::services::db::DatabaseService::load_banner().await {
                *banner_sig.write() = Some(local_banner);
            } else if let Ok(banner) = crate::services::api::get_banner().await {
                *banner_sig.write() = Some(banner);
            } else {
                *banner_sig.write() = None;
            }
        });
    }

    pub fn set_user_name(&mut self, name: String) {
        *self.user_name.write() = name.clone();

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
            let _ = crate::services::db::DatabaseService::save_user_name(name.clone()).await;
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
            let _ = crate::services::db::DatabaseService::save_user_status(status).await;
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
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_user_avatar(avatar_id).await;
        });
    }

    pub fn set_user_personal_message(&mut self, msg: String) {
        *self.user_personal_message.write() = msg.clone();

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
            let _ = crate::services::db::DatabaseService::save_personal_message(msg.clone()).await;
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
            let _ = crate::services::db::DatabaseService::save_user_music(music.clone()).await;
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

    pub fn add_toast(&mut self, title: String, message: String, avatar_url: Option<String>) {
        let id = self.toast_counter();
        *self.toast_counter.write() += 1;

        let toast = Toast {
            id,
            title,
            message,
            avatar_url,
        };

        self.toasts.write().push(toast);
    }

    pub fn remove_toast(&mut self, id: usize) {
        self.toasts.write().retain(|t| t.id != id);
    }

    pub fn save_current_settings(&self) {
        let settings = crate::models::UserSettings {
            interface_scale: self.interface_scale(),
            use_custom_titlebar: self.use_custom_titlebar(),
            theme: crate::services::db::theme_to_str(&self.theme()).to_string(),
            chat_mode: self.chat_mode(),
            contact_density: self.contact_density(),
            font_color: self.chat_font_color(),
            font_family: self.chat_font_family(),
            spotify_rpc_enabled: self.spotify_rpc_enabled(),
            show_typing_notification: self.show_typing_notification(),
            enable_sounds: self.enable_sounds(),
            enable_toasts: self.enable_toasts(),
            download_folder: self.download_folder(),
            auto_accept_files: self.auto_accept_files(),
            remember_password: self.remember_password(),
            save_chat_history: self.save_chat_history(),
        };
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_settings(&settings).await;
        });
    }

    pub fn set_settings(&mut self, scale: f64, custom_bar: bool, theme: AppTheme) {
        *self.interface_scale.write() = scale;
        *self.use_custom_titlebar.write() = custom_bar;
        *self.theme.write() = theme;
        self.save_current_settings();
    }

    pub fn set_chat_mode(&mut self, mode: String) {
        *self.chat_mode.write() = mode;
        self.save_current_settings();
    }

    pub fn chat_font_color(&self) -> String {
        self.chat_font_color.read().clone()
    }

    pub fn chat_font_family(&self) -> String {
        self.chat_font_family.read().clone()
    }

    pub fn set_chat_font_color(&mut self, color: String) {
        *self.chat_font_color.write() = color;
        self.save_current_settings();
    }

    pub fn set_chat_font_family(&mut self, font_family: String) {
        *self.chat_font_family.write() = font_family;
        self.save_current_settings();
    }

    pub fn spotify_rpc_enabled(&self) -> bool {
        (self.spotify_rpc_enabled)()
    }

    pub fn set_spotify_rpc_enabled(&mut self, enabled: bool) {
        *self.spotify_rpc_enabled.write() = enabled;
        self.save_current_settings();
    }

    pub fn show_typing_notification(&self) -> bool {
        (self.show_typing_notification)()
    }

    pub fn set_show_typing_notification(&mut self, show: bool) {
        *self.show_typing_notification.write() = show;
        self.save_current_settings();
    }

    pub fn enable_sounds(&self) -> bool {
        (self.enable_sounds)()
    }

    pub fn set_enable_sounds(&mut self, enable: bool) {
        *self.enable_sounds.write() = enable;
        self.save_current_settings();
    }

    pub fn enable_toasts(&self) -> bool {
        (self.enable_toasts)()
    }

    pub fn set_enable_toasts(&mut self, enable: bool) {
        *self.enable_toasts.write() = enable;
        self.save_current_settings();
    }

    pub fn download_folder(&self) -> String {
        self.download_folder.read().clone()
    }

    pub fn set_download_folder(&mut self, folder: String) {
        *self.download_folder.write() = folder;
        self.save_current_settings();
    }

    pub fn auto_accept_files(&self) -> bool {
        (self.auto_accept_files)()
    }

    pub fn set_auto_accept_files(&mut self, auto: bool) {
        *self.auto_accept_files.write() = auto;
        self.save_current_settings();
    }

    pub fn remember_password(&self) -> bool {
        (self.remember_password)()
    }

    pub fn set_remember_password(&mut self, remember: bool) {
        *self.remember_password.write() = remember;
        self.save_current_settings();
    }

    pub fn save_chat_history(&self) -> bool {
        (self.save_chat_history)()
    }

    pub fn set_save_chat_history(&mut self, save: bool) {
        *self.save_chat_history.write() = save;
        self.save_current_settings();
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

    pub fn show_games_modal(&self) -> bool {
        (self.show_games_modal)()
    }

    pub fn show_settings_modal(&self) -> bool {
        (self.show_settings_modal)()
    }

    pub fn show_add_contact_modal(&self) -> bool {
        (self.show_add_contact_modal)()
    }

    pub fn show_music_player_modal(&self) -> bool {
        (self.show_music_player_modal)()
    }

    pub fn show_profile_modal(&self) -> bool {
        (self.show_profile_modal)()
    }

    pub fn show_about(&self) -> bool {
        (self.show_about)()
    }

    pub fn profile_modal_contact_id(&self) -> Option<String> {
        self.profile_modal_contact_id.read().clone()
    }

    pub fn open_my_profile(&mut self) {
        *self.profile_modal_contact_id.write() = None;
        self.show_profile_modal.set(true);
    }

    pub fn open_contact_profile(&mut self, contact_id: String) {
        *self.profile_modal_contact_id.write() = Some(contact_id);
        self.show_profile_modal.set(true);
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

    pub fn add_category(&mut self, name: String) {
        let mut cats = self.categories.write();
        if !cats.contains(&name) {
            cats.push(name.clone());
            cats.sort();
            spawn(async move {
                let _ = crate::services::db::DatabaseService::add_category(name).await;
            });
        }
    }

    pub fn delete_category(&mut self, name: String) {
        self.categories.write().retain(|c| c != &name);
        for contact in self.contacts.write().iter_mut() {
            if contact.category_name.as_ref() == Some(&name) {
                contact.category_name = None;
            }
        }
        spawn(async move {
            let _ = crate::services::db::DatabaseService::delete_category(name).await;
        });
    }

    pub fn update_contact_category(&mut self, contact_id: String, category: Option<String>) {
        for contact in self.contacts.write().iter_mut() {
            if contact.id == contact_id {
                contact.category_name = category.clone();
                break;
            }
        }
        spawn(async move {
            let _ = crate::services::db::DatabaseService::update_contact_category(contact_id, category).await;
        });
    }

    pub fn update_banner_admin(&mut self, banner: crate::models::BannerInfo) {
        *self.banner_info.write() = Some(banner.clone());
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

    pub fn categories(&self) -> Vec<String> {
        self.categories.read().clone()
    }

    pub fn show_friend_requests_modal(&self) -> bool {
        (self.show_friend_requests_modal)()
    }

    pub fn show_group_profile_modal(&self) -> bool {
        (self.show_group_profile_modal)()
    }

    pub fn group_profile_id(&self) -> Option<String> {
        self.group_profile_id.read().clone()
    }

    pub fn group_chats(&self) -> Vec<crate::models::Conversation> {
        self.group_chats.read().clone()
    }
}
