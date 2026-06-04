use std::collections::HashMap;
use dioxus::prelude::*;
use crate::models::{Contact, Message, UserStatus, AppTheme, BannerInfo, FileTransferState, TicTacToe, TicTacToeCell};

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
    pub active_chats: Signal<Vec<usize>>, // contact_ids
    pub selected_chat_id: Signal<Option<usize>>, // selected contact_id
    pub chat_messages: Signal<HashMap<usize, Vec<Message>>>,
    pub toasts: Signal<Vec<Toast>>,
    pub theme: Signal<AppTheme>,
    pub toast_counter: Signal<usize>,
    pub message_counter: Signal<usize>,
    pub detached_chats: Signal<Vec<usize>>, // contact_ids desvinculados em janelas nativas
    pub use_custom_titlebar: Signal<bool>, // barra de título personalizada ativa
    pub interface_scale: Signal<f64>, // fator de escala (zoom) do aplicativo
    
    // Novos estados para Skypia completo e dinâmico
    pub banner_info: Signal<Option<BannerInfo>>,
    pub active_wink: Signal<Option<String>>, // wink sendo executado na tela ("kiss", "hammer", "pig")
    pub game_states: Signal<HashMap<usize, TicTacToe>>, // jogo da velha por contato
    pub show_settings_modal: Signal<bool>,
    pub show_add_contact_modal: Signal<bool>,
    pub show_music_player_modal: Signal<bool>,
    pub recommended_songs: Signal<Vec<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            logged_in: Signal::new(false),
            signing_in: Signal::new(false),
            user_name: Signal::new("Wellington Skypia".to_string()),
            user_email: Signal::new("wk.scbd@skypia.io".to_string()),
            user_status: Signal::new(UserStatus::Online),
            user_personal_message: Signal::new("Codando meu próprio clone do Skypia em Dioxus! (H)".to_string()),
            user_music: Signal::new(Some("Coldplay - Viva La Vida".to_string())),
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
        
        spawn(async move {
            if let Ok((scale, custom_bar, theme)) = crate::services::db::DatabaseService::load_settings().await {
                *scale_sig.write() = scale;
                *custom_bar_sig.write() = custom_bar;
                *theme_sig.write() = theme;
            }

            if let Ok(loaded_contacts) = crate::services::db::DatabaseService::load_contacts().await {
                *contacts_sig.write() = loaded_contacts;
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
            
            // Loop periódico para rotacionar o banner dinamicamente a cada 12 segundos
            let mut banner_loop_sig = banner_sig;
            spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(12)).await;
                    if let Ok(banner) = crate::services::db::DatabaseService::get_banner_info().await {
                        let current_banner = banner_loop_sig.read().clone();
                        if current_banner.as_ref() != Some(&banner) {
                            *banner_loop_sig.write() = Some(banner);
                        }
                    }
                }
            });
            
            // Carrega mensagens de contatos iniciais
            let mut all_messages = HashMap::new();
            let mut max_msg_id = 0;
            
            for contact_id in &[1, 2] {
                if let Ok(msgs) = crate::services::db::DatabaseService::load_messages(*contact_id).await {
                    for m in &msgs {
                        if m.id > max_msg_id {
                            max_msg_id = m.id;
                        }
                    }
                    all_messages.insert(*contact_id, msgs);
                }
            }
            
            *chat_messages_sig.write() = all_messages;
            if max_msg_id > 0 {
                *message_counter_sig.write() = max_msg_id + 1;
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
                            *selected_chat_sig.write() = active.iter().copied().find(|id| !detached.contains(id));
                        }
                    }
                }
            }
        });
    }

    pub fn set_user_name(&mut self, name: String) {
        *self.user_name.write() = name.clone();
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_user_name(name).await;
        });
    }

    pub fn set_user_status(&mut self, status: UserStatus) {
        *self.user_status.write() = status;
        if status == UserStatus::Offline {
            *self.logged_in.write() = false;
        }
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_user_status(status).await;
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
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_personal_message(msg).await;
        });
    }

    pub fn set_user_music(&mut self, music: Option<String>) {
        *self.user_music.write() = music.clone();
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_user_music(music).await;
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
                let _ = crate::services::db::DatabaseService::save_contact_favorite(contact_id, is_fav).await;
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
            *self.selected_chat_id.write() = chats.iter().copied().find(|id| !detached.contains(&id));
        }
    }

    pub fn send_message(&mut self, contact_id: usize, text: String, font_color: String, font_family: String) {
        if text.trim().is_empty() {
            return;
        }

        let msg_id = self.message_counter();
        *self.message_counter.write() += 1;

        let now = chrono::Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        let new_msg = Message {
            id: msg_id,
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
        messages.entry(contact_id).or_insert_with(Vec::new).push(new_msg.clone());

        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_message(contact_id, new_msg).await;
        });
    }

    pub fn send_nudge(&mut self, contact_id: usize) {
        let msg_id = self.message_counter();
        *self.message_counter.write() += 1;

        let now = chrono::Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        let nudge_msg = Message {
            id: msg_id,
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
        messages.entry(contact_id).or_insert_with(Vec::new).push(nudge_msg.clone());

        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_message(contact_id, nudge_msg).await;
        });
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
            messages.entry(contact_id).or_insert_with(Vec::new).push(nudge_msg.clone());
        }

        // Se o chat não estiver desvinculado, abre e seleciona na tela principal
        self.open_chat(contact_id);

        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_message(contact_id, nudge_msg).await;
        });
    }

    pub fn receive_reply(&mut self, contact_id: usize, text: String, font_color: String, font_family: String) {
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
        messages.entry(contact_id).or_insert_with(Vec::new).push(new_msg.clone());

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
            *self.selected_chat_id.write() = active.iter().copied().find(|id| !detached.contains(&id));
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
    pub fn add_contact_dynamic(&mut self, email: String, display_name: String, status: UserStatus, personal_message: String) {
        let mut list = self.contacts;
        let mut state_clone = *self;
        
        spawn(async move {
            if let Ok(c) = crate::services::db::DatabaseService::add_contact(email, display_name, status, personal_message).await {
                list.write().push(c.clone());
                state_clone.add_toast(
                    "Contato Adicionado".to_string(),
                    format!("{} acaba de ser adicionado.", c.display_name),
                    c.avatar_id
                );
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
            let _ = crate::services::db::DatabaseService::save_settings(scale, custom_bar, theme).await;
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

    pub fn send_wink(&mut self, contact_id: usize, wink_name: String) {
        let msg_id = self.message_counter();
        *self.message_counter.write() += 1;
        
        let now = chrono::Local::now().format("%H:%M:%S").to_string();
        let new_wink_msg = Message {
            id: msg_id,
            sender_id: 0,
            sender_name: self.user_name(),
            text: format!("Você enviou uma Piscadela: {}.", match wink_name.as_str() {
                "kiss" => "Beijo de Batom",
                "hammer" => "Martelada na Tela",
                "pig" => "Porco Dançarino",
                _ => "Piscadela",
            }),
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
        self.chat_messages.write().entry(contact_id).or_default().push(new_wink_msg.clone());
        spawn(async move {
            let _ = crate::services::db::DatabaseService::save_message(contact_id, new_wink_msg).await;
        });

        // Simula recebimento de Wink do contato após 6 segundos
        let mut app_state_reply = *self;
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(6000)).await;
            let reply_wink = if wink_name == "kiss" { "hammer" } else { "kiss" };
            
            let r_msg_id = app_state_reply.message_counter();
            *app_state_reply.message_counter.write() += 1;
            let r_now = chrono::Local::now().format("%H:%M:%S").to_string();
            
            let contact_name = if let Some(c) = app_state_reply.contacts().iter().find(|c| c.id == contact_id) {
                c.display_name.clone()
            } else {
                "Contato".to_string()
            };

            let reply_msg = Message {
                id: r_msg_id,
                sender_id: contact_id,
                sender_name: contact_name.clone(),
                text: format!("{} enviou uma Piscadela: {}.", contact_name, match reply_wink {
                    "kiss" => "Beijo de Batom",
                    "hammer" => "Martelada na Tela",
                    "pig" => "Porco Dançarino",
                    _ => "Piscadela",
                }),
                timestamp: r_now,
                is_nudge: false,
                font_color: "#e6007e".to_string(),
                font_family: "Comic Sans MS".to_string(),
                is_wink: Some(reply_wink.to_string()),
                file_transfer: None,
                is_game_invite: false,
            };

            // Dispara Wink local recebido
            *app_state_reply.active_wink.write() = Some(reply_wink.to_string());
            let reply_wink_str = reply_wink.to_string();
            let mut app_state_reply_inner = app_state_reply;
            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
                let current = app_state_reply_inner.active_wink.read().clone();
                if current.as_ref() == Some(&reply_wink_str) {
                    *app_state_reply_inner.active_wink.write() = None;
                }
            });

            app_state_reply.chat_messages.write().entry(contact_id).or_default().push(reply_msg.clone());
            let _ = crate::services::db::DatabaseService::save_message(contact_id, reply_msg).await;
        });
    }

    pub fn send_file_transfer(&mut self, contact_id: usize, filename: String) {
        let msg_id = self.message_counter();
        *self.message_counter.write() += 1;
        let now = chrono::Local::now().format("%H:%M:%S").to_string();
        
        let new_msg = Message {
            id: msg_id,
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

        self.chat_messages.write().entry(contact_id).or_default().push(new_msg.clone());
        
        // Simulação do robô aceitando após 1.5 segundos
        let mut app_state = *self;
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
            app_state.accept_file_transfer(contact_id, msg_id);
        });
    }

    pub fn accept_file_transfer(&mut self, contact_id: usize, msg_id: usize) {
        let mut state_clone = *self;
        let mut messages = self.chat_messages.write();
        if let Some(list) = messages.get_mut(&contact_id) {
            if let Some(msg) = list.iter_mut().find(|m| m.id == msg_id) {
                if let Some(FileTransferState::Waiting) = msg.file_transfer {
                    msg.file_transfer = Some(FileTransferState::Downloading(0));
                    
                    // Inicia o progresso
                    let filename = msg.text.split('\'').nth(1).unwrap_or("foto.jpg").to_string();
                    spawn(async move {
                        for progress in (1..=10).map(|x| x * 10) {
                            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                            let mut msgs_write = state_clone.chat_messages.write();
                            if let Some(l) = msgs_write.get_mut(&contact_id) {
                                if let Some(m) = l.iter_mut().find(|m| m.id == msg_id) {
                                    if progress == 100 {
                                        m.file_transfer = Some(FileTransferState::Completed(filename.clone()));
                                        m.text = format!("Transferência do arquivo '{}' concluída.", filename);
                                    } else {
                                        m.file_transfer = Some(FileTransferState::Downloading(progress));
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
        self.chat_messages.write().entry(contact_id).or_default().push(new_msg);
    }

    pub fn make_game_move(&mut self, contact_id: usize, cell_idx: usize) {
        let mut state_clone = *self;
        let mut games = self.game_states.write();
        if let Some(game) = games.get_mut(&contact_id) {
            if !game.active || game.board[cell_idx] != TicTacToeCell::Empty || game.turn != TicTacToeCell::X {
                return;
            }
            
            game.board[cell_idx] = TicTacToeCell::X;
            
            if check_game_over(game) {
                return;
            }
            
            game.turn = TicTacToeCell::O;
            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(800)).await;
                let mut games_write = state_clone.game_states.write();
                if let Some(game_write) = games_write.get_mut(&contact_id) {
                    if game_write.turn == TicTacToeCell::O && game_write.active {
                        let mut play_idx = None;
                        for i in 0..9 {
                            if game_write.board[i] == TicTacToeCell::Empty {
                                play_idx = Some(i);
                                break;
                            }
                        }
                        
                        if let Some(idx) = play_idx {
                            game_write.board[idx] = TicTacToeCell::O;
                            if !check_game_over(game_write) {
                                game_write.turn = TicTacToeCell::X;
                            }
                        }
                    }
                }
            });
        }
    }
}

fn check_game_over(game: &mut TicTacToe) -> bool {
    let win_patterns = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],
        [0, 3, 6], [1, 4, 7], [2, 5, 8],
        [0, 4, 8], [2, 4, 6],
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
