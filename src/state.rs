use std::collections::HashMap;
use dioxus::prelude::*;
use crate::models::{Contact, Message, UserStatus, AppTheme};

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
}

impl AppState {
    pub fn new() -> Self {
        Self {
            logged_in: Signal::new(false),
            signing_in: Signal::new(false),
            user_name: Signal::new("Wellington MSN".to_string()),
            user_email: Signal::new("wk.scbd@msn.com".to_string()),
            user_status: Signal::new(UserStatus::Online),
            user_personal_message: Signal::new("Codando meu próprio clone do MSN em Dioxus! (H)".to_string()),
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
        }
    }

    // Carrega dados dinâmicos do banco assincronamente
    pub fn load_initial_data(&mut self) {
        let mut contacts_sig = self.contacts;
        let mut chat_messages_sig = self.chat_messages;
        let mut message_counter_sig = self.message_counter;
        let mut detached_sig = self.detached_chats;
        
        spawn(async move {
            if let Ok(loaded_contacts) = crate::services::db::DatabaseService::load_contacts().await {
                *contacts_sig.write() = loaded_contacts;
            }
            
            if let Ok(detached) = crate::services::db::DatabaseService::get_detached_chats().await {
                *detached_sig.write() = detached;
            }
            
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
}
