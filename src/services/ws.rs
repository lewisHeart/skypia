use crate::models::{ClientAction, UserStatus, WsEvent};
use crate::state::AppState;
use dioxus::prelude::*;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message as TungsteniteMsg;

pub fn connect_ws(mut state: AppState, token: String) {
    let base = crate::services::api::SERVER_BASE_URL
        .replace("http://", "ws://")
        .replace("https://", "wss://");
    let ws_url = format!("{}/ws?token={}", base, token);

    // Spawna o loop de conexão em background no scheduler do Dioxus (evita erros de Send)
    dioxus::prelude::spawn(async move {
        loop {
            // Se o token foi limpo (logout), interrompe o loop
            if state.auth_token().is_none() {
                break;
            }

            println!("Conectando ao WebSocket em: {}/ws", base);
            match connect_async(&ws_url).await {
                Ok((ws_stream, _)) => {
                    println!("WebSocket conectado com sucesso!");
                    state.add_toast(
                        "Servidores conectados".to_string(),
                        "Você está conectado ao servidor do Skypia.".to_string(),
                        None,
                    );

                    // Sincroniza dados iniciais ao conectar/reconectar
                    state.load_initial_data();

                    // Cria o canal de ações específico para ESTA conexão ativa
                    let (tx, mut rx) = mpsc::unbounded_channel::<ClientAction>();
                    *state.ws_tx.write() = Some(tx);

                    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                    // Canal de cancelamento interno para as subtarefas cooperativas
                    let (close_tx, mut close_rx) = mpsc::channel::<()>(1);

                    // Task de Escrita: lê do canal rx e envia para o WebSocket (com heartbeat a cada 30 segundos)
                    let close_tx_write = close_tx.clone();
                    dioxus::prelude::spawn(async move {
                        let mut interval = tokio::time::interval(Duration::from_secs(30));
                        // Descarta o primeiro tick imediato para evitar envio instantâneo
                        interval.tick().await;

                        loop {
                            tokio::select! {
                                action_opt = rx.recv() => {
                                    match action_opt {
                                        Some(action) => {
                                            if let Ok(json_str) = serde_json::to_string(&action) {
                                                if ws_sender
                                                    .send(TungsteniteMsg::Text(json_str.into()))
                                                    .await
                                                    .is_err()
                                                {
                                                    break;
                                                }
                                            }
                                        }
                                        None => break,
                                    }
                                }
                                _ = interval.tick() => {
                                    // Envia Ping de controle do protocolo WebSocket para manter a conexão ativa
                                    if ws_sender
                                        .send(TungsteniteMsg::Ping(vec![].into()))
                                        .await
                                        .is_err()
                                    {
                                        break;
                                    }
                                }
                            }
                        }
                        let _ = close_tx_write.send(()).await;
                    });

                    // Task de Leitura: lê do WebSocket e atualiza o AppState
                    let mut state_read = state;
                    let close_tx_read = close_tx.clone();
                    dioxus::prelude::spawn(async move {
                        while let Some(msg_res) = ws_receiver.next().await {
                            if state_read.auth_token().is_none() {
                                break;
                            }
                            match msg_res {
                                Ok(TungsteniteMsg::Text(text)) => {
                                    match serde_json::from_str::<WsEvent>(&text) {
                                        Ok(event) => {
                                            process_ws_event(&mut state_read, event).await;
                                        }
                                        Err(e) => {
                                            eprintln!("❌ Erro ao desserializar WsEvent: {}. JSON recebido: {}", e, text);
                                        }
                                    }
                                }
                                Ok(TungsteniteMsg::Close(_)) | Err(_) => {
                                    break;
                                }
                                _ => {}
                            }
                        }
                        let _ = close_tx_read.send(()).await;
                    });

                    // Aguarda até que uma das tasks encerre (conexão caiu)
                    let _ = close_rx.recv().await;

                    println!("Conexão WebSocket perdida. Tentando reconectar...");
                    state.add_toast(
                        "Conexão Perdida".to_string(),
                        "Tentando reconectar ao servidor de mensagens...".to_string(),
                        None,
                    );

                    // Limpa o transmissor do WebSocket ao cair a conexão
                    *state.ws_tx.write() = None;
                }
                Err(e) => {
                    eprintln!(
                        "Erro ao conectar no WebSocket: {}. Tentando novamente...",
                        e
                    );
                }
            }

            // Espera 3 segundos antes de tentar reconectar
            tokio::time::sleep(Duration::from_secs(3)).await;
        }

        // Garante limpeza ao final do loop
        *state.ws_tx.write() = None;
    });
}

async fn process_ws_event(state: &mut AppState, event: WsEvent) {
    if state.auth_token().is_none() {
        return;
    }
    match event {
        WsEvent::ChatMessage(mut msg) => {
            let conv_id = msg.conversation_id.clone();

            // 1. Salva no SQLite local para histórico offline
            let is_group = state.group_chats().iter().any(|g| g.id == conv_id);
            let db_conv_id = if is_group {
                conv_id.clone()
            } else {
                if let Some(real_id) = crate::services::db::DatabaseService::find_11_conversation_id(&conv_id).await {
                    real_id
                } else {
                    conv_id.clone()
                }
            };
            
            let mut msg_to_save = msg.clone();
            if msg_to_save.sender_id == "0" {
                if let Some(ref s_id) = state.server_user_id() {
                    msg_to_save.sender_id = s_id.clone();
                }
            }
            msg_to_save.conversation_id = db_conv_id.clone();
            
            let db_conv_id_clone = db_conv_id.clone();
            let msg_to_save_clone = msg_to_save.clone();
            spawn(async move {
                let _ = crate::services::db::DatabaseService::save_message(db_conv_id_clone, msg_to_save_clone).await;
            });

            // Normaliza o sender_id para "0" se for o próprio usuário local
            if Some(msg.sender_id.clone()) == state.server_user_id() {
                msg.sender_id = "0".to_string();
            }

            // 2. Adiciona à lista de mensagens em memória
            {
                let mut chat_msgs = state.chat_messages.write();
                chat_msgs
                    .entry(conv_id.clone())
                    .or_default()
                    .push(msg.clone());
            }

            // 2. Efeitos visuais e sonoros para mensagens de terceiros
            if msg.sender_id != "0" {
                crate::sound::play_sound("message");

                let is_selected = state.selected_chat_id() == Some(conv_id.clone());
                let is_detached = state.detached_chats().contains(&conv_id);
                if !is_selected || is_detached {
                    state.increment_unread(&conv_id);
                }

                {
                    // Garante que o chat esteja nas abas ativas
                    let mut active = state.active_chats.write();
                    if !active.contains(&conv_id) {
                        active.push(conv_id.clone());
                    }
                }

                // Tenta achar a foto de perfil do contato remetente
                let avatar_url = state
                    .contacts()
                    .iter()
                    .find(|c| c.id == conv_id)
                    .and_then(|c| c.avatar_url.clone());

                // Dispara notificação toast
                state.add_toast(msg.sender_name.clone(), msg.text.clone(), avatar_url);
            }
        }
        WsEvent::PresenceUpdate {
            user_id,
            status,
            personal_message,
            music,
            avatar_url,
            display_name,
        } => {
            let user_status = match status.as_str() {
                "Online" => UserStatus::Online,
                "Ocupado" => UserStatus::Ocupado,
                "Ausente" => UserStatus::Ausente,
                "Invisivel" => UserStatus::Invisivel,
                _ => UserStatus::Offline,
            };

            // Self-broadcast: atualiza o perfil do próprio usuário quando recebido do servidor
            if Some(user_id.clone()) == state.server_user_id() {
                // Atualiza avatar do próprio usuário
                if let Some(ref url) = avatar_url {
                    let is_local = if let Some(local_url) = state.user_avatar_url() {
                        local_url.starts_with("/assets/")
                            || local_url.starts_with("assets/")
                            || local_url.starts_with("/_assets/")
                            || local_url.starts_with("_assets/")
                            || local_url.starts_with("dioxus-asset://")
                    } else {
                        false
                    };
                    if !is_local {
                        *state.user_avatar_url.write() = Some(url.clone());
                    }
                }
                // Atualiza display_name do próprio usuário
                if !display_name.is_empty() {
                    *state.user_name.write() = display_name.clone();
                }
                // Atualiza status, mensagem pessoal e música
                *state.user_status.write() = user_status;
                *state.user_personal_message.write() = personal_message.clone();
                *state.user_music.write() = music.clone();
                return;
            }

            let mut should_play_online = false;
            let mut name = String::new();
            let mut pm = String::new();
            let mut final_avatar_url = None;
            let mut contact_to_save = None;

            {
                // Atualiza na lista de contatos em memória
                let mut list = state.contacts.write();
                if let Some(c) = list.iter_mut().find(|c| c.id == user_id) {
                    let old_status = c.status;
                    c.status = user_status;
                    c.personal_message = personal_message;
                    c.music_listening = music;
                    if avatar_url.is_some() {
                        c.avatar_url = avatar_url.clone();
                    }
                    if !display_name.is_empty() {
                        c.display_name = display_name;
                    }
                    name = c.display_name.clone();
                    pm = c.personal_message.clone();
                    final_avatar_url = c.avatar_url.clone();
                    contact_to_save = Some(c.clone());

                    // Toca som de entrada se o usuário acabou de ficar Online
                    if old_status == UserStatus::Offline && user_status == UserStatus::Online {
                        should_play_online = true;
                    }
                }
            }

            if let Some(c) = contact_to_save {
                spawn(async move {
                    let _ = crate::services::db::DatabaseService::save_contact(&c).await;
                });
            }

            if should_play_online {
                crate::sound::play_sound("online");
                // Exibe toast de quem entrou
                state.add_toast(format!("{} está online", name), pm, final_avatar_url);
            }
        }
        WsEvent::Nudge {
            conversation_id,
            sender_id,
            sender_name,
        } => {
            let self_id = state.server_user_id();
            let is_from_self = Some(sender_id.clone()) == self_id;

            // Executa o recebimento do nudge (chacoalhar e tocar o som)
            state.receive_nudge(conversation_id.clone());

            if !is_from_self {
                let is_selected = state.selected_chat_id() == Some(conversation_id.clone());
                let is_detached = state.detached_chats().contains(&conversation_id);
                if !is_selected || is_detached {
                    state.increment_unread(&conversation_id);
                }

                let avatar_url = state
                    .contacts()
                    .iter()
                    .find(|c| c.id == conversation_id)
                    .and_then(|c| c.avatar_url.clone());
                state.add_toast(
                    sender_name,
                    "enviou um Chamar a Atenção!".to_string(),
                    avatar_url,
                );
            }
        }
        WsEvent::Typing {
            conversation_id,
            user_id,
            is_typing,
        } => {
            let conv_id = conversation_id.clone();
            let u_id = user_id.clone();

            {
                let mut typings = state.typing_contacts.write();
                let entry = typings.entry(conv_id).or_default();
                if is_typing {
                    if !entry.contains(&u_id) {
                        entry.push(u_id);
                    }
                } else {
                    entry.retain(|id| id != &u_id);
                }
            }
        }
        WsEvent::ContactRequestReceived { requester } => {
            let status_enum = match requester.status.as_str() {
                "Online" => UserStatus::Online,
                "Ocupado" => UserStatus::Ocupado,
                "Ausente" => UserStatus::Ausente,
                "Invisivel" => UserStatus::Invisivel,
                _ => UserStatus::Offline,
            };

            let contact = crate::models::Contact {
                id: requester.id.clone(),
                email: requester.email.clone(),
                display_name: requester.display_name.clone(),
                status: status_enum,
                personal_message: requester.personal_message.clone(),
                music_listening: requester.music.clone(),
                avatar_url: requester.avatar_url.clone(),
                is_favorite: false,
                relation_status: "Pendente".to_string(),
                nickname: None,
                category_name: None,
            };

            // Toca som
            crate::sound::play_sound("message");

            {
                let mut pending = state.pending_requests.write();
                if !pending.iter().any(|c| c.email == contact.email) {
                    pending.push(contact.clone());
                }
            }

            state.add_toast(
                "Solicitação de Amizade".to_string(),
                format!(
                    "{} ({}) quer adicionar você.",
                    contact.display_name, contact.email
                ),
                contact.avatar_url.clone(),
            );
        }
        WsEvent::ContactRequestAccepted { contact } => {
            // Toca som
            crate::sound::play_sound("online");

            state.add_toast(
                "Solicitação Aceita!".to_string(),
                format!(
                    "{} aceitou sua solicitação de amizade.",
                    contact.display_name
                ),
                contact.avatar_url.clone(),
            );

            state.load_initial_data();
        }
        WsEvent::ContactAdded { contact } => {
            let status_enum = match contact.status.as_str() {
                "Online" => UserStatus::Online,
                "Ocupado" => UserStatus::Ocupado,
                "Ausente" => UserStatus::Ausente,
                "Invisivel" => UserStatus::Invisivel,
                _ => UserStatus::Offline,
            };
            let mut new_contact = crate::models::Contact {
                id: contact.id.clone(),
                email: contact.email.clone(),
                display_name: contact.display_name.clone(),
                status: status_enum,
                personal_message: contact.personal_message.clone(),
                music_listening: contact.music.clone(),
                avatar_url: contact.avatar_url.clone(),
                is_favorite: false,
                relation_status: contact
                    .relation_status
                    .unwrap_or_else(|| "Pendente".to_string()),
                nickname: contact.nickname,
                category_name: None,
            };

            {
                let mut list = state.contacts.write();
                if let Some(existing) = list.iter_mut().find(|c| c.email == new_contact.email) {
                    new_contact.is_favorite = existing.is_favorite;
                    *existing = new_contact.clone();
                } else {
                    list.push(new_contact.clone());
                }
            }

            let c_save = new_contact.clone();
            spawn(async move {
                let _ = crate::services::db::DatabaseService::save_contact(&c_save).await;
            });

            state.add_toast(
                "Contato Adicionado".to_string(),
                format!("{} foi adicionado.", new_contact.display_name),
                new_contact.avatar_url,
            );
        }
        WsEvent::ContactBlocked {
            contact_id: _contact_id,
            blocked,
        } => {
            state.add_toast(
                if blocked {
                    "Contato Bloqueado".to_string()
                } else {
                    "Contato Desbloqueado".to_string()
                },
                if blocked {
                    "Você bloqueou o contato.".to_string()
                } else {
                    "Você desbloqueou o contato.".to_string()
                },
                None,
            );
            // Recarrega dados para refletir mudança
            state.load_initial_data();
        }
        WsEvent::ContactRemoved { contact_id } => {
            {
                let mut pending = state.pending_requests.write();
                pending.retain(|c| c.id != contact_id);
            }
            state.add_toast(
                "Solicitação Recusada".to_string(),
                "A solicitação de amizade foi removida.".to_string(),
                None,
            );
        }
        WsEvent::NicknameUpdated {
            contact_id,
            nickname,
        } => {
            let mut contact_to_save = None;
            {
                let mut list = state.contacts.write();
                if let Some(c) = list.iter_mut().find(|c| c.id == contact_id) {
                    c.nickname = nickname;
                    contact_to_save = Some(c.clone());
                }
            }
            if let Some(c) = contact_to_save {
                spawn(async move {
                    let _ = crate::services::db::DatabaseService::save_contact(&c).await;
                });
            }
        }
        WsEvent::Error { message } => {
            state.add_toast("Erro".to_string(), message, None);
        }
        WsEvent::FavoriteUpdated {
            contact_id,
            is_favorite,
        } => {
            let mut contact_to_save = None;
            {
                let mut list = state.contacts.write();
                if let Some(c) = list.iter_mut().find(|c| c.id == contact_id) {
                    c.is_favorite = is_favorite;
                    contact_to_save = Some(c.clone());
                }
            }
            if let Some(c) = contact_to_save {
                spawn(async move {
                    let _ = crate::services::db::DatabaseService::save_contact(&c).await;
                });
            }
        }
        WsEvent::ConversationJoined(_conversation) => todo!(),
    }
}
