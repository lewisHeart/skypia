use crate::models::{
    AppTheme, BannerInfo, Contact, FileTransferState, Message, TicTacToe, TicTacToeCell, UserStatus,
    ClientAction,
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    pub id: usize,
    pub title: String,
    pub message: String,
    pub avatar_id: usize,
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
    pub active_chats: Signal<Vec<usize>>,        // contact_ids
    pub selected_chat_id: Signal<Option<usize>>, // selected contact_id
    pub chat_messages: Signal<HashMap<usize, Vec<Message>>>,
    pub toasts: Signal<Vec<Toast>>,
    pub theme: Signal<AppTheme>,
    pub toast_counter: Signal<usize>,
    pub message_counter: Signal<usize>,
    pub detached_chats: Signal<Vec<usize>>, // contact_ids desvinculados em janelas nativas
    pub use_custom_titlebar: Signal<bool>,  // barra de título personalizada ativa
    pub interface_scale: Signal<f64>,       // fator de escala (zoom) do aplicativo

    // Novos estados para Skypia completo e dinâmico
    pub banner_info: Signal<Option<BannerInfo>>,
    pub active_wink: Signal<Option<String>>, // wink sendo executado na tela ("kiss", "hammer", "pig")
    pub game_states: Signal<HashMap<usize, TicTacToe>>, // jogo da velha por contato
    pub show_settings_modal: Signal<bool>,
    pub show_add_contact_modal: Signal<bool>,
    pub show_music_player_modal: Signal<bool>,
    pub recommended_songs: Signal<Vec<String>>,

    // Autenticação real com o servidor
    pub auth_token: Signal<Option<String>>,
    pub server_user_id: Signal<Option<i64>>,
    pub user_avatar_url: Signal<Option<String>>, // URL da foto real (do servidor)
    pub show_register_modal: Signal<bool>,
    pub server_error: Signal<Option<String>>,    // Último erro da API
    pub show_avatar_picker: Signal<bool>,

    // WebSocket e digitação em tempo real
    pub ws_tx: Signal<Option<tokio::sync::mpsc::UnboundedSender<crate::models::ClientAction>>>,
    pub typing_contacts: Signal<std::collections::HashMap<usize, Vec<usize>>>,

    // Solicitações pendentes e inatividade
    pub pending_requests: Signal<Vec<Contact>>,
    pub last_activity_time: Signal<u64>,
    pub was_automatically_away: Signal<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            logged_in: Signal::new(false),
            signing_in: Signal::new(false),
            user_name: Signal::new("Wellington Skypia".to_string()),
            user_email: Signal::new("wk.scbd@skypia.io".to_string()),
            user_status: Signal::new(UserStatus::Online),
            user_personal_message: Signal::new("Tô cagando".to_string()),
            user_music: Signal::new(Some("Linkin Park - In The End".to_string())),
            user_avatar_id: Signal::new(0),

            contacts: Signal::new(Vec::new()),
            active_chats: Signal::new(Vec::new()),
            selected_chat_id: Signal::new(None),
            chat_messages: Signal::new(HashMap::new()),
            toasts: Signal::new(Vec::new()),
            theme: Signal::new(AppTheme::AeroBlue),
            toast_counter: Signal::new(1),
            message_counter: Signal::new(10),
            detached_chats: Signal::new(Vec::new()),
            use_custom_titlebar: Signal::new(true),
            interface_scale: Signal::new(1.0),

            banner_info: Signal::new(None),
            active_wink: Signal::new(None),
            game_states: Signal::new(HashMap::new()),
            show_settings_modal: Signal::new(false),
            show_add_contact_modal: Signal::new(false),
            show_music_player_modal: Signal::new(false),
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
        }
    }

    // Carrega dados dinâmicos do banco assincronamente
    pub fn load_initial_data(&mut self) {
        let mut contacts_sig = self.contacts;
        let mut chat_messages_sig = self.chat_messages;
        let mut message_counter_sig = self.message_counter;
        let mut detached_sig = self.detached_chats;
        let mut name_sig = self.user_name;
        let mut music_sig = self.user_music;
        let mut banner_sig = self.banner_info;
        let mut songs_sig = self.recommended_songs;
        let mut scale_sig = self.interface_scale;
        let mut custom_bar_sig = self.use_custom_titlebar;
        let mut theme_sig = self.theme;
        let mut pending_sig = self.pending_requests;

        let token_opt = self.auth_token();

        spawn(async move {
            if let Ok((scale, custom_bar, theme)) =
                crate::services::db::DatabaseService::load_settings().await
            {
                *scale_sig.write() = scale;
                *custom_bar_sig.write() = custom_bar;
                *theme_sig.write() = theme;
            }

            // Sincronização de rede se autenticado
            if let Some(token) = token_opt {
                // 1. Busca contatos do servidor e salva localmente
                if let Ok(srv_contacts) = crate::services::api::get_contacts(&token).await {
                    for profile in srv_contacts {
                        let status_enum = match profile.status.as_str() {
                            "Online" => UserStatus::Online,
                            "Ocupado" => UserStatus::Ocupado,
                            "Ausente" => UserStatus::Ausente,
                            "Invisivel" => UserStatus::Invisivel,
                            _ => UserStatus::Offline,
                        };
                        let _ = crate::services::db::DatabaseService::add_contact(
                            profile.email,
                            profile.display_name,
                            status_enum,
                            profile.personal_message,
                            profile.relation_status.unwrap_or_else(|| "Aceito".to_string()),
                            profile.nickname,
                        ).await;
                    }
                }

                // 1.1 Busca solicitações pendentes do servidor
                if let Ok(pending_srv) = crate::services::api::get_pending_requests(&token).await {
                    let contacts_mapped: Vec<Contact> = pending_srv.into_iter().enumerate().map(|(idx, profile)| {
                        let status_enum = match profile.status.as_str() {
                            "Online" => UserStatus::Online,
                            "Ocupado" => UserStatus::Ocupado,
                            "Ausente" => UserStatus::Ausente,
                            "Invisivel" => UserStatus::Invisivel,
                            _ => UserStatus::Offline,
                        };
                        Contact {
                            id: profile.id as usize,
                            email: profile.email,
                            display_name: profile.display_name,
                            status: status_enum,
                            personal_message: profile.personal_message,
                            music_listening: profile.music,
                            avatar_id: idx % 7,
                            is_favorite: false,
                            relation_status: "Pendente".to_string(),
                            nickname: None,
                        }
                    }).collect();
                    *pending_sig.write() = contacts_mapped;
                }

                // 2. Busca conversas do servidor e salva localmente
                if let Ok(srv_conversations) = crate::services::api::get_conversations(&token).await {
                    let _ = crate::services::db::DatabaseService::save_conversations(srv_conversations).await;
                }
            }

            if let Ok(loaded_contacts) = crate::services::db::DatabaseService::load_contacts().await
            {
                *contacts_sig.write() = loaded_contacts.clone();

                // Carrega mensagens de todos os contatos carregados
                let mut all_messages = HashMap::new();
                let mut max_msg_id = 0;

                for contact in loaded_contacts {
                    if let Ok(msgs) =
                        crate::services::db::DatabaseService::load_messages(contact.id).await
                    {
                        for m in &msgs {
                            if m.id > max_msg_id {
                                max_msg_id = m.id;
                            }
                        }
                        all_messages.insert(contact.id, msgs);
                    }
                }

                *chat_messages_sig.write() = all_messages;
                if max_msg_id > 0 {
                    *message_counter_sig.write() = max_msg_id + 1;
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

            if let Ok(songs) = crate::services::db::DatabaseService::get_recommended_songs().await {
                *songs_sig.write() = songs;
            }

            if let Ok(banner) = crate::services::db::DatabaseService::get_banner_info().await {
                *banner_sig.write() = Some(banner);
            }
        });
    }

    // Sincroniza a lista de chats destacados nativamente
    pub fn sync_detached_chats(&mut self) {
        let mut detached_sig = self.detached_chats;
        let mut selected_chat_sig = self.selected_chat_id;
        let active = self.active_chats();

        spawn(async move {
            if let Ok(detached) = crate::services::db::DatabaseService::get_detached_chats().await {
                let old_detached = detached_sig.read().clone();
                if old_detached != detached {
                    *detached_sig.write() = detached.clone();

                    // Se o chat selecionado atualmente na janela integrada foi desvinculado por outra ação,
                    // removemos a seleção na janela principal para mostrar a tela de boas-vindas.
                    let current_selected = (selected_chat_sig)();
                    if let Some(sel_id) = current_selected {
                        if detached.contains(&sel_id) {
                            *selected_chat_sig.write() =
                                active.iter().copied().find(|id| !detached.contains(id));
                        }
                    }
                }
            }
        });
    }

    pub fn set_user_name(&mut self, name: String) {
        *self.user_name.write() = name.clone();
        let token_opt = self.auth_token();
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_user_name(name.clone()).await;
            if let Some(token) = token_opt {
                let _ = crate::services::api::update_profile(&token, crate::services::api::UpdateProfileRequest {
                    display_name: Some(name),
                    personal_message: None,
                    status: None,
                    music: None,
                }).await;
            }
        });
    }

    pub fn set_user_status(&mut self, status: UserStatus) {
        *self.user_status.write() = status;
        if status == UserStatus::Offline {
            *self.logged_in.write() = false;
        }
        let token_opt = self.auth_token();
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_user_status(status).await;
            if let Some(token) = token_opt {
                let status_str = match status {
                    UserStatus::Online => "Online",
                    UserStatus::Ocupado => "Ocupado",
                    UserStatus::Ausente => "Ausente",
                    UserStatus::Invisivel => "Invisivel",
                    UserStatus::Offline => "Offline",
                };
                let _ = crate::services::api::update_profile(&token, crate::services::api::UpdateProfileRequest {
                    display_name: None,
                    personal_message: None,
                    status: Some(status_str.to_string()),
                    music: None,
                }).await;
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
        let token_opt = self.auth_token();
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_personal_message(msg.clone()).await;
            if let Some(token) = token_opt {
                let _ = crate::services::api::update_profile(&token, crate::services::api::UpdateProfileRequest {
                    display_name: None,
                    personal_message: Some(msg),
                    status: None,
                    music: None,
                }).await;
            }
        });
    }

    pub fn set_user_music(&mut self, music: Option<String>) {
        *self.user_music.write() = music.clone();
        let token_opt = self.auth_token();
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_user_music(music.clone()).await;
            if let Some(token) = token_opt {
                let _ = crate::services::api::update_profile(&token, crate::services::api::UpdateProfileRequest {
                    display_name: None,
                    personal_message: None,
                    status: None,
                    music: Some(music),
                }).await;
            }
        });
    }

    pub fn add_toast(&mut self, title: String, message: String, avatar_id: usize) {
        let id = self.toast_counter();
        *self.toast_counter.write() += 1;

        let toast = Toast {
            id,
            title,
            message,
            avatar_id,
        };

        self.toasts.write().push(toast);
    }

    pub fn remove_toast(&mut self, id: usize) {
        self.toasts.write().retain(|t| t.id != id);
    }

    pub fn toggle_favorite(&mut self, contact_id: usize) {
        let mut list = self.contacts.write();
        if let Some(contact) = list.iter_mut().find(|c| c.id == contact_id) {
            contact.is_favorite = !contact.is_favorite;
            let is_fav = contact.is_favorite;
            spawn(async move {
                let _ =
                    crate::services::db::DatabaseService::save_contact_favorite(contact_id, is_fav)
                        .await;
            });
        }
    }

    pub fn open_chat(&mut self, contact_id: usize) {
        let is_detached = self.detached_chats().contains(&contact_id);

        {
            let mut chats = self.active_chats.write();
            if !chats.contains(&contact_id) {
                chats.push(contact_id);
            }
        }

        // Se o chat estiver desvinculado em uma janela nativa, dar duplo clique
        // na barra lateral reata o chat e acopla de volta na janela principal!
        if is_detached {
            self.attach_chat(contact_id);
        } else {
            *self.selected_chat_id.write() = Some(contact_id);
        }
    }

    pub fn close_chat(&mut self, contact_id: usize) {
        // Remove das janelas desvinculadas se estiver lá
        spawn(async move {
            let _ = crate::services::db::DatabaseService::attach_chat(contact_id).await;
        });
        self.detached_chats.write().retain(|&id| id != contact_id);

        let is_selected = (self.selected_chat_id)() == Some(contact_id);

        {
            let mut chats = self.active_chats.write();
            chats.retain(|&id| id != contact_id);
        }

        if is_selected {
            // Apenas seleciona o próximo chat que NÃO esteja desvinculado
            let detached = self.detached_chats();
            let chats = self.active_chats();
            *self.selected_chat_id.write() =
                chats.iter().copied().find(|id| !detached.contains(&id));
        }
    }

    pub fn send_message(
        &mut self,
        contact_id: usize,
        text: String,
        font_color: String,
        font_family: String,
    ) {
        if text.trim().is_empty() {
            return;
        }

        // Se o WebSocket estiver ativo, envia para o servidor
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(ClientAction::SendMessage {
                conversation_id: contact_id as i64,
                text,
                font_color,
                font_family,
            });
            return;
        }

        let msg_id = self.message_counter();
        *self.message_counter.write() += 1;

        let now = chrono::Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        let new_msg = Message {
            id: msg_id,
            conversation_id: contact_id,
            sender_id: 0,
            sender_name: self.user_name(),
            text,
            timestamp,
            is_nudge: false,
            font_color,
            font_family,
            is_wink: None,
            file_transfer: None,
            is_game_invite: false,
        };

        let mut messages = self.chat_messages.write();
        messages
            .entry(contact_id)
            .or_insert_with(Vec::new)
            .push(new_msg.clone());

        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_message(contact_id, new_msg).await;
        });
    }

    pub fn send_nudge(&mut self, contact_id: usize) {
        // Se o WebSocket estiver ativo, envia para o servidor
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(ClientAction::SendNudge {
                conversation_id: contact_id as i64,
            });
            return;
        }

        let msg_id = self.message_counter();
        *self.message_counter.write() += 1;

        let now = chrono::Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        let nudge_msg = Message {
            id: msg_id,
            conversation_id: contact_id,
            sender_id: 0,
            sender_name: self.user_name(),
            text: "Você enviou um Chamar a Atenção.".to_string(),
            timestamp: timestamp.clone(),
            is_nudge: true,
            font_color: "#e81123".to_string(),
            font_family: "Segoe UI".to_string(),
            is_wink: None,
            file_transfer: None,
            is_game_invite: false,
        };

        let mut messages = self.chat_messages.write();
        messages
            .entry(contact_id)
            .or_insert_with(Vec::new)
            .push(nudge_msg.clone());

        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_message(contact_id, nudge_msg).await;
        });
    }

    pub fn set_typing(&mut self, contact_id: usize, is_typing: bool) {
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(ClientAction::SetTyping {
                conversation_id: contact_id as i64,
                is_typing,
            });
        }
    }

    pub fn receive_nudge(&mut self, contact_id: usize) {
        let contact_name = if let Some(c) = self.contacts().iter().find(|c| c.id == contact_id) {
            c.display_name.clone()
        } else {
            "Contato".to_string()
        };

        let msg_id = self.message_counter();
        *self.message_counter.write() += 1;

        let now = chrono::Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        let nudge_msg = Message {
            id: msg_id,
            conversation_id: contact_id,
            sender_id: contact_id,
            sender_name: contact_name.clone(),
            text: format!("{} enviou um Chamar a Atenção.", contact_name),
            timestamp,
            is_nudge: true,
            font_color: "#e81123".to_string(),
            font_family: "Segoe UI".to_string(),
            is_wink: None,
            file_transfer: None,
            is_game_invite: false,
        };

        {
            let mut messages = self.chat_messages.write();
            messages
                .entry(contact_id)
                .or_insert_with(Vec::new)
                .push(nudge_msg.clone());
        }

        // Se o chat não estiver desvinculado, abre e seleciona na tela principal
        self.open_chat(contact_id);

        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_message(contact_id, nudge_msg).await;
        });
    }

    pub fn receive_reply(
        &mut self,
        contact_id: usize,
        text: String,
        font_color: String,
        font_family: String,
    ) {
        let contact_name = if let Some(c) = self.contacts().iter().find(|c| c.id == contact_id) {
            c.display_name.clone()
        } else {
            "Contato".to_string()
        };

        let msg_id = self.message_counter();
        *self.message_counter.write() += 1;

        let now = chrono::Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        let new_msg = Message {
            id: msg_id,
            conversation_id: contact_id,
            sender_id: contact_id,
            sender_name: contact_name,
            text,
            timestamp,
            is_nudge: false,
            font_color,
            font_family,
            is_wink: None,
            file_transfer: None,
            is_game_invite: false,
        };

        let mut messages = self.chat_messages.write();
        messages
            .entry(contact_id)
            .or_insert_with(Vec::new)
            .push(new_msg.clone());

        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_message(contact_id, new_msg).await;
        });
    }

    // Gerenciamento de Janelas Desvinculadas (Janelas nativas do SO)
    pub fn detach_chat(&mut self, contact_id: usize) {
        // Salva no banco de dados compartilhado
        spawn(async move {
            let _ = crate::services::db::DatabaseService::detach_chat(contact_id).await;
        });

        // Adiciona ao estado local
        if !self.detached_chats().contains(&contact_id) {
            self.detached_chats.write().push(contact_id);
        }

        // Se este era o chat selecionado na janela principal, limpa a seleção para exibir o placeholder de boas-vindas
        if (self.selected_chat_id)() == Some(contact_id) {
            let active = self.active_chats();
            let detached = self.detached_chats();
            *self.selected_chat_id.write() =
                active.iter().copied().find(|id| !detached.contains(&id));
        }
    }

    pub fn attach_chat(&mut self, contact_id: usize) {
        // Salva no banco de dados compartilhado (o loop na janela flutuante vai detectar e fechar ela)
        spawn(async move {
            let _ = crate::services::db::DatabaseService::attach_chat(contact_id).await;
        });

        // Remove do estado local
        self.detached_chats.write().retain(|&id| id != contact_id);
        *self.selected_chat_id.write() = Some(contact_id);
    }

    // Adiciona contato dinâmico de verdade
    pub fn add_contact_dynamic(
        &mut self,
        email: String,
        display_name: String,
        status: UserStatus,
        personal_message: String,
    ) {
        let mut list = self.contacts;
        let mut state_clone = *self;
        let token_opt = self.auth_token();

        spawn(async move {
            let mut added_contact = None;

            if let Some(token) = token_opt {
                match crate::services::api::add_contact(&token, email.clone()).await {
                    Ok(profile) => {
                        let status_enum = match profile.status.as_str() {
                            "Online" => UserStatus::Online,
                            "Ocupado" => UserStatus::Ocupado,
                            "Ausente" => UserStatus::Ausente,
                            "Invisivel" => UserStatus::Invisivel,
                            _ => UserStatus::Offline,
                        };
                        if let Ok(c) = crate::services::db::DatabaseService::add_contact(
                            profile.email,
                            profile.display_name,
                            status_enum,
                            profile.personal_message,
                            profile.relation_status.unwrap_or_else(|| "Aceito".to_string()),
                            profile.nickname,
                        ).await {
                            added_contact = Some(c);
                        }
                    }
                    Err(e) => {
                        state_clone.add_toast(
                            "Erro ao Adicionar".to_string(),
                            format!("Não foi possível adicionar o contato: {}", e),
                            0,
                        );
                    }
                }
            } else {
                if let Ok(c) = crate::services::db::DatabaseService::add_contact(
                    email,
                    display_name,
                    status,
                    personal_message,
                    "Aceito".to_string(),
                    None,
                ).await {
                    added_contact = Some(c);
                }
            }

            if let Some(c) = added_contact {
                {
                    let mut list_w = list.write();
                    if let Some(existing) = list_w.iter_mut().find(|item| item.email == c.email) {
                        *existing = c.clone();
                    } else {
                        list_w.push(c.clone());
                    }
                }
                state_clone.add_toast(
                    "Contato Adicionado".to_string(),
                    format!("{} foi adicionado ou atualizado.", c.display_name),
                    c.avatar_id,
                );
                state_clone.load_initial_data();
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

    pub fn active_chats(&self) -> Vec<usize> {
        self.active_chats.read().clone()
    }

    pub fn selected_chat_id(&self) -> Option<usize> {
        (self.selected_chat_id)()
    }

    pub fn chat_messages(&self) -> HashMap<usize, Vec<Message>> {
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

    pub fn detached_chats(&self) -> Vec<usize> {
        self.detached_chats.read().clone()
    }

    pub fn use_custom_titlebar(&self) -> bool {
        (self.use_custom_titlebar)()
    }

    pub fn interface_scale(&self) -> f64 {
        (self.interface_scale)()
    }

    pub fn set_settings(&mut self, scale: f64, custom_bar: bool, theme: AppTheme) {
        *self.interface_scale.write() = scale;
        *self.use_custom_titlebar.write() = custom_bar;
        *self.theme.write() = theme;
        spawn(async move {
            let _ =
                crate::services::db::DatabaseService::save_settings(scale, custom_bar, theme).await;
        });
    }

    pub fn banner_info(&self) -> Option<BannerInfo> {
        self.banner_info.read().clone()
    }

    pub fn active_wink(&self) -> Option<String> {
        self.active_wink.read().clone()
    }

    pub fn game_states(&self) -> HashMap<usize, TicTacToe> {
        self.game_states.read().clone()
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

    pub fn recommended_songs(&self) -> Vec<String> {
        self.recommended_songs.read().clone()
    }

    pub fn auth_token(&self) -> Option<String> {
        self.auth_token.read().clone()
    }

    pub fn server_user_id(&self) -> Option<i64> {
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

    pub fn typing_contacts(&self) -> HashMap<usize, Vec<usize>> {
        self.typing_contacts.read().clone()
    }

    /// Aplica o perfil vindo do servidor ao estado local
    pub fn apply_server_profile(&mut self, profile: crate::models::UserProfile, token: String) {
        *self.auth_token.write() = Some(token.clone());
        *self.server_user_id.write() = Some(profile.id);
        *self.user_name.write() = profile.display_name.clone();
        *self.user_email.write() = profile.email.clone();
        *self.user_personal_message.write() = profile.personal_message.clone();
        if let Some(music) = profile.music.clone() {
            *self.user_music.write() = Some(music);
        }
        if let Some(url) = profile.avatar_url.clone() {
            *self.user_avatar_url.write() = Some(url);
        }
        // Salva token no SQLite local para auto-login
        let user_id = profile.id;
        let mut self_clone = *self;
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_auth_token(token, user_id).await;
            self_clone.connect_websocket();
        });
    }

    /// Estabelece a conexão com o WebSocket do servidor
    pub fn connect_websocket(&mut self) {
        if let Some(token) = self.auth_token() {
            crate::services::ws::connect_ws(*self, token);
        }
    }

    /// Faz logout completo
    pub fn logout(&mut self) {
        *self.logged_in.write() = false;
        *self.auth_token.write() = None;
        *self.server_user_id.write() = None;
        *self.user_avatar_url.write() = None;
        *self.ws_tx.write() = None;
        spawn(async move {
            let _ = crate::services::db::DatabaseService::clear_auth_token().await;
        });
    }

    pub fn send_wink(&mut self, contact_id: usize, wink_name: String) {
        let msg_id = self.message_counter();
        *self.message_counter.write() += 1;

        let now = chrono::Local::now().format("%H:%M:%S").to_string();
        let new_wink_msg = Message {
            id: msg_id,
            conversation_id: contact_id,
            sender_id: 0,
            sender_name: self.user_name(),
            text: format!(
                "Você enviou uma Piscadela: {}.",
                match wink_name.as_str() {
                    "kiss" => "Beijo de Batom",
                    "hammer" => "Martelada na Tela",
                    "pig" => "Porco Dançarino",
                    _ => "Piscadela",
                }
            ),
            timestamp: now.clone(),
            is_nudge: false,
            font_color: "#0066cc".to_string(),
            font_family: "Segoe UI".to_string(),
            is_wink: Some(wink_name.clone()),
            file_transfer: None,
            is_game_invite: false,
        };

        // Dispara o Wink local
        *self.active_wink.write() = Some(wink_name.clone());
        let mut app_state = *self;
        let wink_name_clone = wink_name.clone();
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
            let current = app_state.active_wink.read().clone();
            if current.as_ref() == Some(&wink_name_clone) {
                *app_state.active_wink.write() = None;
            }
        });

        // Envia mensagem
        self.chat_messages
            .write()
            .entry(contact_id)
            .or_default()
            .push(new_wink_msg.clone());
        spawn(async move {
            let _ =
                crate::services::db::DatabaseService::save_message(contact_id, new_wink_msg).await;
        });

    }

    pub fn send_file_transfer(&mut self, contact_id: usize, filename: String) {
        let msg_id = self.message_counter();
        *self.message_counter.write() += 1;
        let now = chrono::Local::now().format("%H:%M:%S").to_string();

        let new_msg = Message {
            id: msg_id,
            conversation_id: contact_id,
            sender_id: 0,
            sender_name: self.user_name(),
            text: format!("Você enviou um convite de arquivo: '{}'.", filename),
            timestamp: now,
            is_nudge: false,
            font_color: "#7a7a7a".to_string(),
            font_family: "Segoe UI".to_string(),
            is_wink: None,
            file_transfer: Some(FileTransferState::Waiting),
            is_game_invite: false,
        };

        self.chat_messages
            .write()
            .entry(contact_id)
            .or_default()
            .push(new_msg.clone());


    }

    pub fn accept_file_transfer(&mut self, contact_id: usize, msg_id: usize) {
        let mut state_clone = *self;
        let mut messages = self.chat_messages.write();
        if let Some(list) = messages.get_mut(&contact_id) {
            if let Some(msg) = list.iter_mut().find(|m| m.id == msg_id) {
                if let Some(FileTransferState::Waiting) = msg.file_transfer {
                    msg.file_transfer = Some(FileTransferState::Downloading(0));

                    // Inicia o progresso
                    let filename = msg
                        .text
                        .split('\'')
                        .nth(1)
                        .unwrap_or("foto.jpg")
                        .to_string();
                    spawn(async move {
                        for progress in (1..=10).map(|x| x * 10) {
                            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                            let mut msgs_write = state_clone.chat_messages.write();
                            if let Some(l) = msgs_write.get_mut(&contact_id) {
                                if let Some(m) = l.iter_mut().find(|m| m.id == msg_id) {
                                    if progress == 100 {
                                        m.file_transfer =
                                            Some(FileTransferState::Completed(filename.clone()));
                                        m.text = format!(
                                            "Transferência do arquivo '{}' concluída.",
                                            filename
                                        );
                                    } else {
                                        m.file_transfer =
                                            Some(FileTransferState::Downloading(progress));
                                    }
                                }
                            }
                        }
                    });
                }
            }
        }
    }

    pub fn reject_file_transfer(&mut self, contact_id: usize, msg_id: usize) {
        let mut messages = self.chat_messages.write();
        if let Some(list) = messages.get_mut(&contact_id) {
            if let Some(msg) = list.iter_mut().find(|m| m.id == msg_id) {
                msg.file_transfer = Some(FileTransferState::Rejected);
                msg.text = "O envio do arquivo foi cancelado ou rejeitado.".to_string();
            }
        }
    }

    pub fn start_game(&mut self, contact_id: usize) {
        let msg_id = self.message_counter();
        let u_name = self.user_name();

        let mut games = self.game_states.write();
        games.insert(contact_id, TicTacToe::new());

        *self.message_counter.write() += 1;
        let now = chrono::Local::now().format("%H:%M:%S").to_string();
        let new_msg = Message {
            id: msg_id,
            conversation_id: contact_id,
            sender_id: 0,
            sender_name: u_name,
            text: "Iniciou uma partida de Jogo da Velha.".to_string(),
            timestamp: now,
            is_nudge: false,
            font_color: "#2e6930".to_string(),
            font_family: "Segoe UI".to_string(),
            is_wink: None,
            file_transfer: None,
            is_game_invite: true,
        };
        self.chat_messages
            .write()
            .entry(contact_id)
            .or_default()
            .push(new_msg);
    }

    pub fn make_game_move(&mut self, contact_id: usize, cell_idx: usize) {

        let mut games = self.game_states.write();
        if let Some(game) = games.get_mut(&contact_id) {
            if !game.active
                || game.board[cell_idx] != TicTacToeCell::Empty
                || game.turn != TicTacToeCell::X
            {
                return;
            }

            game.board[cell_idx] = TicTacToeCell::X;

            if check_game_over(game) {
                return;
            }

            game.turn = TicTacToeCell::O;
        }
    }

    pub fn pending_requests(&self) -> Vec<Contact> {
        self.pending_requests.read().clone()
    }

    pub fn accept_friend_request(&mut self, contact_id: usize) {
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        spawn(async move {
            if let Some(token) = token_opt {
                match crate::services::api::accept_friend(&token, contact_id as i64).await {
                    Ok(profile) => {
                        let status_enum = match profile.status.as_str() {
                            "Online" => UserStatus::Online,
                            "Ocupado" => UserStatus::Ocupado,
                            "Ausente" => UserStatus::Ausente,
                            "Invisivel" => UserStatus::Invisivel,
                            _ => UserStatus::Offline,
                        };
                        let friend_name = profile.display_name.clone();
                        let _ = crate::services::db::DatabaseService::add_contact(
                            profile.email,
                            profile.display_name,
                            status_enum,
                            profile.personal_message,
                            "Aceito".to_string(),
                            profile.nickname,
                        ).await;
                        state_clone.add_toast(
                            "Solicitação Aceita".to_string(),
                            format!("Você agora é amigo de {}.", friend_name),
                            0,
                        );
                    }
                    Err(e) => {
                        state_clone.add_toast("Erro ao Aceitar".to_string(), e, 0);
                    }
                }
            }
            state_clone.load_initial_data();
        });
    }

    pub fn reject_friend_request(&mut self, contact_id: usize) {
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        spawn(async move {
            if let Some(token) = token_opt {
                match crate::services::api::reject_friend(&token, contact_id as i64).await {
                    Ok(_) => {
                        state_clone.add_toast(
                            "Solicitação Recusada".to_string(),
                            "A solicitação de amizade foi removida.".to_string(),
                            0,
                        );
                    }
                    Err(e) => {
                        state_clone.add_toast("Erro ao Recusar".to_string(), e, 0);
                    }
                }
            }
            state_clone.load_initial_data();
        });
    }

    pub fn block_contact(&mut self, contact_id: usize, block: bool) {
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        spawn(async move {
            if let Some(token) = token_opt {
                match crate::services::api::block_friend(&token, contact_id as i64, block).await {
                    Ok(_) => {
                        let rel = if block { "Bloqueado".to_string() } else { "Aceito".to_string() };
                        let _ = crate::services::db::DatabaseService::update_contact_relation(contact_id, rel).await;
                        state_clone.add_toast(
                            if block { "Contato Bloqueado".to_string() } else { "Contato Desbloqueado".to_string() },
                            if block { "Você bloqueou o contato.".to_string() } else { "Você desbloqueou o contato.".to_string() },
                            0,
                        );
                    }
                    Err(e) => {
                        state_clone.add_toast("Erro ao Atualizar".to_string(), e, 0);
                    }
                }
            } else {
                let rel = if block { "Bloqueado".to_string() } else { "Aceito".to_string() };
                let _ = crate::services::db::DatabaseService::update_contact_relation(contact_id, rel).await;
            }
            state_clone.load_initial_data();
        });
    }

    pub fn rename_contact(&mut self, contact_id: usize, nickname: Option<String>) {
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        spawn(async move {
            if let Some(token) = token_opt {
                match crate::services::api::update_contact_nickname(&token, contact_id as i64, nickname.clone()).await {
                    Ok(_) => {
                        let _ = crate::services::db::DatabaseService::update_contact_nickname(contact_id, nickname).await;
                    }
                    Err(e) => {
                        state_clone.add_toast("Erro ao Renomear".to_string(), e, 0);
                    }
                }
            } else {
                let _ = crate::services::db::DatabaseService::update_contact_nickname(contact_id, nickname).await;
            }
            state_clone.load_initial_data();
        });
    }

    pub fn record_activity(&mut self) {
        let now_sec = chrono::Utc::now().timestamp() as u64;
        *self.last_activity_time.write() = now_sec;
        if self.was_automatically_away() {
            *self.was_automatically_away.write() = false;
            if self.user_status() == UserStatus::Online {
                self.set_user_status(UserStatus::Online);
                self.add_toast(
                    "Status Online".to_string(),
                    "Você voltou e seu status foi definido para Online.".to_string(),
                    self.user_avatar_id(),
                );
            }
        }
    }

    pub fn check_inactivity_and_update(&mut self) {
        let now_sec = chrono::Utc::now().timestamp() as u64;
        let last = self.last_activity_time();
        if now_sec > last + 300 { // 5 minutos = 300 segundos
            if self.user_status() == UserStatus::Online {
                *self.was_automatically_away.write() = true;
                self.set_user_status(UserStatus::Ausente);
                self.add_toast(
                    "Status Alterado".to_string(),
                    "Você está ausente por inatividade.".to_string(),
                    self.user_avatar_id(),
                );
            }
        }
    }

    pub fn last_activity_time(&self) -> u64 {
        (self.last_activity_time)()
    }

    pub fn was_automatically_away(&self) -> bool {
        (self.was_automatically_away)()
    }
}

fn check_game_over(game: &mut TicTacToe) -> bool {
    let win_patterns = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for p in &win_patterns {
        if game.board[p[0]] != TicTacToeCell::Empty
            && game.board[p[0]] == game.board[p[1]]
            && game.board[p[0]] == game.board[p[2]]
        {
            game.winner = Some(game.board[p[0]]);
            game.active = false;
            return true;
        }
    }

    if game.board.iter().all(|c| *c != TicTacToeCell::Empty) {
        game.is_draw = true;
        game.active = false;
        return true;
    }

    false
}
