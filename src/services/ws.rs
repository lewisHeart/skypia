use crate::models::{ClientAction, UserStatus, WsEvent};
use crate::state::AppState;
use dioxus::prelude::*;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message as TungsteniteMsg;

pub fn connect_ws(mut state: AppState, token: String) {
    let ws_url = format!("ws://127.0.0.1:8082/ws?token={}", token);

    // Spawna o loop de conexão em background no scheduler do Dioxus (evita erros de Send)
    dioxus::prelude::spawn(async move {
        loop {
            // Se o token foi limpo (logout), interrompe o loop
            if state.auth_token().is_none() {
                break;
            }

            println!("Conectando ao WebSocket em: ws://127.0.0.1:8082/ws");
            match connect_async(&ws_url).await {
                Ok((ws_stream, _)) => {
                    println!("WebSocket conectado com sucesso!");
                    state.add_toast(
                        "Tempo Real Conectado".to_string(),
                        "Você está conectado ao servidor do Skypia.".to_string(),
                        None,
                    );

                    // Cria o canal de ações específico para ESTA conexão ativa
                    let (tx, mut rx) = mpsc::unbounded_channel::<ClientAction>();
                    *state.ws_tx.write() = Some(tx);

                    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                    // Canal de cancelamento interno para as subtarefas cooperativas
                    let (close_tx, mut close_rx) = mpsc::channel::<()>(1);

                    // Task de Escrita: lê do canal rx e envia para o WebSocket
                    let close_tx_write = close_tx.clone();
                    dioxus::prelude::spawn(async move {
                        while let Some(action) = rx.recv().await {
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
                        let _ = close_tx_write.send(()).await;
                    });

                    // Task de Leitura: lê do WebSocket e atualiza o AppState
                    let mut state_read = state;
                    let close_tx_read = close_tx.clone();
                    dioxus::prelude::spawn(async move {
                        while let Some(msg_res) = ws_receiver.next().await {
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
    match event {
        WsEvent::ChatMessage(mut msg) => {
            let conv_id = msg.conversation_id.clone();

            // Normaliza o sender_id para "0" se for o próprio usuário local
            if Some(msg.sender_id.clone()) == state.server_user_id() {
                msg.sender_id = "0".to_string();
            }

            // 1. Adiciona à lista de mensagens em memória
            {
                let mut chat_msgs = state.chat_messages.write();
                chat_msgs.entry(conv_id.clone()).or_default().push(msg.clone());
            }

            // 2. Efeitos visuais e sonoros para mensagens de terceiros
            if msg.sender_id != "0" {
                crate::sound::play_sound("message");

                {
                    // Garante que o chat esteja nas abas ativas
                    let mut active = state.active_chats.write();
                    if !active.contains(&conv_id) {
                        active.push(conv_id.clone());
                    }
                }

                // Tenta achar a foto de perfil do contato remetente
                let avatar_url = state.contacts().iter().find(|c| c.id == conv_id).and_then(|c| c.avatar_url.clone());

                // Dispara notificação toast
                state.add_toast(
                    msg.sender_name.clone(),
                    msg.text.clone(),
                    avatar_url,
                );
            }
        }
        WsEvent::PresenceUpdate {
            user_id,
            status,
            personal_message,
            music,
            avatar_url,
        } => {
            let user_status = match status.as_str() {
                "Online" => UserStatus::Online,
                "Ocupado" => UserStatus::Ocupado,
                "Ausente" => UserStatus::Ausente,
                "Invisivel" => UserStatus::Invisivel,
                _ => UserStatus::Offline,
            };

            let mut should_play_online = false;
            let mut name = String::new();
            let mut pm = String::new();
            let mut final_avatar_url = None;

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
                    name = c.display_name.clone();
                    pm = c.personal_message.clone();
                    final_avatar_url = c.avatar_url.clone();

                    // Toca som de entrada se o usuário acabou de ficar Online
                    if old_status == UserStatus::Offline && user_status == UserStatus::Online {
                        should_play_online = true;
                    }
                }
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
            if sender_id != "0" {
                // Toca som do nudge
                crate::sound::play_sound("nudge");

                // Ativa animação de tremor no AppState para essa conversa
                // Se a conversa não estiver aberta, seleciona e abre
                state.open_chat(conversation_id.clone());

                let avatar_url = state.contacts().iter().find(|c| c.id == conversation_id).and_then(|c| c.avatar_url.clone());

                // Exibe notificação toast
                state.add_toast(sender_name, "enviou um Chamar a Atenção!".to_string(), avatar_url);
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
    }
}
