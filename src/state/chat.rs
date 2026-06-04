use crate::state::AppState;
use crate::models::{ClientAction, Message, FileTransferState};
use dioxus::prelude::*;

impl AppState {
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
                                active.iter().cloned().find(|id| !detached.contains(id));
                        }
                    }
                }
            }
        });
    }

    pub fn open_chat(&mut self, contact_id: String) {
        let is_detached = self.detached_chats().contains(&contact_id);

        {
            let mut chats = self.active_chats.write();
            if !chats.contains(&contact_id) {
                chats.push(contact_id.clone());
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

    pub fn close_chat(&mut self, contact_id: String) {
        // Remove das janelas desvinculadas se estiver lá
        let cid_clone = contact_id.clone();
        spawn(async move {
            let _ = crate::services::db::DatabaseService::attach_chat(cid_clone).await;
        });
        self.detached_chats.write().retain(|id| id != &contact_id);

        let is_selected = (self.selected_chat_id)() == Some(contact_id.clone());

        {
            let mut chats = self.active_chats.write();
            chats.retain(|id| id != &contact_id);
        }

        if is_selected {
            // Apenas seleciona o próximo chat que NÃO esteja desvinculado
            let detached = self.detached_chats();
            let chats = self.active_chats();
            *self.selected_chat_id.write() =
                chats.iter().cloned().find(|id| !detached.contains(id));
        }
    }

    pub fn send_message(
        &mut self,
        contact_id: String,
        text: String,
        font_color: String,
        font_family: String,
    ) {
        if text.trim().is_empty() {
            return;
        }

        // Se o contato for pendente, bloqueia o envio
        if let Some(c) = self.contacts().iter().find(|c| c.id == contact_id) {
            if c.relation_status == "Pendente" {
                return;
            }
        }

        // Se o WebSocket estiver ativo, envia para o servidor
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(ClientAction::SendMessage {
                conversation_id: contact_id,
                text,
                font_color,
                font_family,
            });
            return;
        }

        let msg_id = uuid::Uuid::new_v4().to_string();

        let now = chrono::Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        let new_msg = Message {
            id: msg_id,
            conversation_id: contact_id.clone(),
            sender_id: "0".to_string(),
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
            .push(new_msg);

        // Não salva localmente, confiamos no backend
    }

    pub fn send_nudge(&mut self, contact_id: String) {
        // Se o contato for pendente, bloqueia o envio
        if let Some(c) = self.contacts().iter().find(|c| c.id == contact_id) {
            if c.relation_status == "Pendente" {
                return;
            }
        }

        // Se o WebSocket estiver ativo, envia para o servidor
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(ClientAction::SendNudge {
                conversation_id: contact_id,
            });
            return;
        }

        let msg_id = uuid::Uuid::new_v4().to_string();

        let now = chrono::Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        let nudge_msg = Message {
            id: msg_id,
            conversation_id: contact_id.clone(),
            sender_id: "0".to_string(),
            sender_name: self.user_name(),
            text: "Você enviou um Chamar a Atenção.".to_string(),
            timestamp,
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
            .push(nudge_msg);

        // Não salva localmente, confiamos no backend
    }

    pub fn set_typing(&mut self, contact_id: String, is_typing: bool) {
        // Se o contato for pendente, ignora
        if let Some(c) = self.contacts().iter().find(|c| c.id == contact_id) {
            if c.relation_status == "Pendente" {
                return;
            }
        }

        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(ClientAction::SetTyping {
                conversation_id: contact_id,
                is_typing,
            });
        }
    }

    pub fn receive_nudge(&mut self, contact_id: String) {
        let contact_name = if let Some(c) = self.contacts().iter().find(|c| c.id == contact_id) {
            c.display_name.clone()
        } else {
            "Contato".to_string()
        };

        let msg_id = uuid::Uuid::new_v4().to_string();

        let now = chrono::Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        let nudge_msg = Message {
            id: msg_id,
            conversation_id: contact_id.clone(),
            sender_id: contact_id.clone(),
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
                .entry(contact_id.clone())
                .or_insert_with(Vec::new)
                .push(nudge_msg);
        }

        // Se o chat não estiver desvinculado, abre e seleciona na tela principal
        self.open_chat(contact_id);

        // Não salva localmente, confiamos no backend
    }

    pub fn receive_reply(
        &mut self,
        contact_id: String,
        text: String,
        font_color: String,
        font_family: String,
    ) {
        let contact_name = if let Some(c) = self.contacts().iter().find(|c| c.id == contact_id) {
            c.display_name.clone()
        } else {
            "Contato".to_string()
        };

        let msg_id = uuid::Uuid::new_v4().to_string();

        let now = chrono::Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        let new_msg = Message {
            id: msg_id,
            conversation_id: contact_id.clone(),
            sender_id: contact_id.clone(),
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

        // Não salva localmente, confiamos no backend
    }

    // Gerenciamento de Janelas Desvinculadas (Janelas nativas do SO)
    pub fn detach_chat(&mut self, contact_id: String) {
        // Salva no banco de dados compartilhado
        let cid_clone = contact_id.clone();
        spawn(async move {
            let _ = crate::services::db::DatabaseService::detach_chat(cid_clone).await;
        });

        // Adiciona ao estado local
        if !self.detached_chats().contains(&contact_id) {
            self.detached_chats.write().push(contact_id.clone());
        }

        // Se este era o chat selecionado na janela principal, limpa a seleção para exibir o placeholder de boas-vindas
        if (self.selected_chat_id)() == Some(contact_id.clone()) {
            let active = self.active_chats();
            let detached = self.detached_chats();
            *self.selected_chat_id.write() =
                active.iter().cloned().find(|id| !detached.contains(id));
        }
    }

    pub fn attach_chat(&mut self, contact_id: String) {
        // Salva no banco de dados compartilhado (o loop na janela flutuante vai detectar e fechar ela)
        let cid_clone = contact_id.clone();
        spawn(async move {
            let _ = crate::services::db::DatabaseService::attach_chat(cid_clone).await;
        });

        // Remove do estado local
        self.detached_chats.write().retain(|id| id != &contact_id);
        *self.selected_chat_id.write() = Some(contact_id);
    }

    pub fn send_wink(&mut self, contact_id: String, wink_name: String) {
        // Se o contato for pendente, bloqueia
        if let Some(c) = self.contacts().iter().find(|c| c.id == contact_id) {
            if c.relation_status == "Pendente" {
                return;
            }
        }

        let msg_id = uuid::Uuid::new_v4().to_string();

        let now = chrono::Local::now().format("%H:%M:%S").to_string();
        let new_wink_msg = Message {
            id: msg_id,
            conversation_id: contact_id.clone(),
            sender_id: "0".to_string(),
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
            .push(new_wink_msg);
    }

    pub fn send_file_transfer(&mut self, contact_id: String, filename: String) {
        // Se o contato for pendente, bloqueia
        if let Some(c) = self.contacts().iter().find(|c| c.id == contact_id) {
            if c.relation_status == "Pendente" {
                return;
            }
        }

        let msg_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Local::now().format("%H:%M:%S").to_string();

        let new_msg = Message {
            id: msg_id,
            conversation_id: contact_id.clone(),
            sender_id: "0".to_string(),
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
            .push(new_msg);
    }

    pub fn accept_file_transfer(&mut self, contact_id: String, msg_id: String) {
        let mut state_clone = *self;
        let mut messages = self.chat_messages.write();
        let cid_clone = contact_id.clone();
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
                            if let Some(l) = msgs_write.get_mut(&cid_clone) {
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

    pub fn reject_file_transfer(&mut self, contact_id: String, msg_id: String) {
        let mut messages = self.chat_messages.write();
        if let Some(list) = messages.get_mut(&contact_id) {
            if let Some(msg) = list.iter_mut().find(|m| m.id == msg_id) {
                msg.file_transfer = Some(FileTransferState::Rejected);
                msg.text = "O envio do arquivo foi cancelado ou rejeitado.".to_string();
            }
        }
    }
}
