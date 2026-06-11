use crate::state::AppState;
use crate::models::{Contact, UserStatus};
use dioxus::prelude::*;

impl AppState {
    pub fn toggle_favorite(&mut self, contact_id: String) {
        let mut list = self.contacts.write();
        let mut contact_info = None;
        if let Some(contact) = list.iter_mut().find(|c| c.id == contact_id) {
            contact.is_favorite = !contact.is_favorite;
            contact_info = Some((
                contact.id.clone(),
                contact.email.clone(),
                contact.display_name.clone(),
                contact.is_favorite,
            ));
        }

        if let Some((cid, email, name, is_fav)) = contact_info {
            if let Some(tx) = &*self.ws_tx.read() {
                let _ = tx.send(crate::models::ClientAction::SetFavorite {
                    contact_id: cid.clone(),
                    is_favorite: is_fav,
                });
            }
            spawn(async move {
                let _ = crate::services::db::DatabaseService::save_contact_favorite(
                    cid,
                    email,
                    name,
                    is_fav,
                )
                .await;
            });
        }
    }

    // Adiciona contato dinâmico de verdade
    pub fn add_contact_dynamic(
        &mut self,
        email_or_username: String,
        display_name: String,
        status: UserStatus,
        personal_message: String,
    ) {
        let mut list = self.contacts;
        let mut state_clone = *self;
        let token_opt = self.auth_token();

        // Tenta via WebSocket primeiro
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(crate::models::ClientAction::AddContact {
                email_or_username,
            });
            return;
        }

        // Fallback HTTP
        spawn(async move {
            let mut added_contact = None;

            if let Some(token) = token_opt {
                match crate::services::api::add_contact(&token, email_or_username.clone()).await {
                    Ok(profile) => {
                        let status_enum = match profile.status.as_str() {
                            "Online" => UserStatus::Online,
                            "Ocupado" => UserStatus::Ocupado,
                            "Ausente" => UserStatus::Ausente,
                            "Invisivel" => UserStatus::Invisivel,
                            _ => UserStatus::Offline,
                        };
                        added_contact = Some(Contact {
                            id: profile.id,
                            email: profile.email,
                            display_name: profile.display_name,
                            status: status_enum,
                            personal_message: profile.personal_message,
                            music_listening: profile.music,
                            avatar_url: profile.avatar_url,
                            is_favorite: false,
                            relation_status: profile.relation_status.unwrap_or_else(|| "Aceito".to_string()),
                            nickname: profile.nickname,
                            category_name: None,
                        });
                    }
                    Err(e) => {
                        state_clone.add_toast(
                            "Erro ao Adicionar".to_string(),
                            format!("Não foi possível adicionar o contato: {}", e),
                            None,
                        );
                    }
                }
            } else {
                let temp_id = uuid::Uuid::new_v4().to_string();
                added_contact = Some(Contact {
                    id: temp_id,
                    email: email_or_username,
                    display_name,
                    status,
                    personal_message,
                    music_listening: None,
                    avatar_url: None,
                    is_favorite: false,
                    relation_status: "Aceito".to_string(),
                    nickname: None,
                    category_name: None,
                });
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
                    c.avatar_url.clone(),
                );
                state_clone.load_initial_data();
            }
        });
    }

    pub fn accept_friend_request(&mut self, contact_id: String) {
        // Tenta via WebSocket primeiro
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(crate::models::ClientAction::AcceptContact {
                contact_id: contact_id.clone(),
            });
            // Remove da lista de pendentes localmente
            self.pending_requests.write().retain(|c| c.id != contact_id);
            return;
        }

        // Fallback HTTP
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        spawn(async move {
            if let Some(token) = token_opt {
                match crate::services::api::accept_friend(&token, contact_id).await {
                    Ok(profile) => {
                        let friend_name = profile.display_name.clone();
                        state_clone.add_toast(
                            "Solicitação Aceita".to_string(),
                            format!("Você agora é amigo de {}.", friend_name),
                            profile.avatar_url,
                        );
                    }
                    Err(e) => {
                        state_clone.add_toast("Erro ao Aceitar".to_string(), e, None);
                    }
                }
            }
            state_clone.load_initial_data();
        });
    }

    pub fn reject_friend_request(&mut self, contact_id: String) {
        // Tenta via WebSocket primeiro
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(crate::models::ClientAction::RejectContact {
                contact_id: contact_id.clone(),
            });
            // Remove da lista de pendentes localmente
            self.pending_requests.write().retain(|c| c.id != contact_id);
            return;
        }

        // Fallback HTTP
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        spawn(async move {
            if let Some(token) = token_opt {
                match crate::services::api::reject_friend(&token, contact_id).await {
                    Ok(_) => {
                        state_clone.add_toast(
                            "Solicitação Recusada".to_string(),
                            "A solicitação de amizade foi removida.".to_string(),
                            None,
                        );
                    }
                    Err(e) => {
                        state_clone.add_toast("Erro ao Recusar".to_string(), e, None);
                    }
                }
            }
            state_clone.load_initial_data();
        });
    }

    pub fn block_contact(&mut self, contact_id: String, block: bool) {
        // Tenta via WebSocket primeiro
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(crate::models::ClientAction::BlockContact {
                contact_id,
                block,
            });
            return;
        }

        // Fallback HTTP
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        spawn(async move {
            if let Some(token) = token_opt {
                match crate::services::api::block_friend(&token, contact_id, block).await {
                    Ok(_) => {
                        state_clone.add_toast(
                            if block { "Contato Bloqueado".to_string() } else { "Contato Desbloqueado".to_string() },
                            if block { "Você bloqueou o contato.".to_string() } else { "Você desbloqueou o contato.".to_string() },
                            None,
                        );
                    }
                    Err(e) => {
                        state_clone.add_toast("Erro ao Atualizar".to_string(), e, None);
                    }
                }
            }
            state_clone.load_initial_data();
        });
    }

    pub fn rename_contact(&mut self, contact_id: String, nickname: Option<String>) {
        // Tenta via WebSocket primeiro
        if let Some(tx) = &*self.ws_tx.read() {
            let _ = tx.send(crate::models::ClientAction::SetNickname {
                contact_id,
                nickname,
            });
            return;
        }

        // Fallback HTTP
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        spawn(async move {
            if let Some(token) = token_opt {
                match crate::services::api::update_contact_nickname(&token, contact_id, nickname.clone()).await {
                    Ok(_) => {}
                    Err(e) => {
                        state_clone.add_toast("Erro ao Renomear".to_string(), e, None);
                    }
                }
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
                    self.user_avatar_url(),
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
                    self.user_avatar_url(),
                );
            }
        }
    }

    pub fn create_group_chat(
        &mut self,
        name: String,
        description: String,
        avatar_url: String,
        member_emails: Vec<String>,
    ) {
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        
        let desc_opt = if description.trim().is_empty() { None } else { Some(description.trim().to_string()) };
        let avatar_opt = if avatar_url.trim().is_empty() { None } else { Some(avatar_url.trim().to_string()) };
        
        spawn(async move {
            if let Some(token) = token_opt {
                match crate::services::api::create_conversation(
                    &token,
                    Some(name.clone()),
                    true,
                    member_emails,
                    avatar_opt,
                    desc_opt,
                )
                .await
                {
                    Ok(conv) => {
                        state_clone.add_toast(
                            "Grupo Criado".to_string(),
                            format!("O grupo '{}' foi criado com sucesso.", name),
                            None,
                        );
                        state_clone.load_initial_data();
                        state_clone.open_chat(conv.id);
                    }
                    Err(e) => {
                        state_clone.add_toast("Erro ao Criar Grupo".to_string(), e, None);
                    }
                }
            }
        });
    }

    pub fn leave_group_chat(&mut self, group_id: String) {
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        let gid = group_id.clone();
        spawn(async move {
            if let Some(token) = token_opt {
                let client = reqwest::Client::new();
                let resp = client
                    .post(format!("{}/conversations/{}/leave", crate::services::api::SERVER_BASE_URL, gid))
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await;
                
                match resp {
                    Ok(r) if r.status().is_success() => {
                        state_clone.add_toast(
                            "Sair do Grupo".to_string(),
                            "Você saiu do grupo com sucesso.".to_string(),
                            None,
                        );
                        if state_clone.selected_chat_id() == Some(gid.clone()) {
                            *state_clone.selected_chat_id.write() = None;
                        }
                        state_clone.active_chats.write().retain(|id| id != &gid);
                        state_clone.detached_chats.write().retain(|id| id != &gid);
                        state_clone.load_initial_data();
                    }
                    _ => {
                        state_clone.add_toast(
                            "Erro".to_string(),
                            "Não foi possível sair do grupo.".to_string(),
                            None,
                        );
                    }
                }
            }
        });
    }

    pub fn add_group_member(&mut self, group_id: String, email: String) {
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        let gid = group_id.clone();
        spawn(async move {
            if let Some(token) = token_opt {
                let client = reqwest::Client::new();
                let body = serde_json::json!({ "email": email });
                let resp = client
                    .post(format!("{}/conversations/{}/members/add", crate::services::api::SERVER_BASE_URL, gid))
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&body)
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.status().is_success() => {
                        state_clone.add_toast(
                            "Membro Adicionado".to_string(),
                            format!("{} foi adicionado ao grupo.", email),
                            None,
                        );
                        state_clone.load_initial_data();
                    }
                    Ok(r) => {
                        let text = r.text().await.unwrap_or_default();
                        let error_msg = serde_json::from_str::<serde_json::Value>(&text)
                            .ok()
                            .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
                            .unwrap_or_else(|| "Erro desconhecido.".to_string());
                        state_clone.add_toast("Erro ao Adicionar".to_string(), error_msg, None);
                    }
                    Err(_) => {
                        state_clone.add_toast(
                            "Erro de conexão".to_string(),
                            "Não foi possível conectar ao servidor.".to_string(),
                            None,
                        );
                    }
                }
            }
        });
    }

    pub fn remove_group_member(&mut self, group_id: String, user_id: String) {
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        let gid = group_id.clone();
        spawn(async move {
            if let Some(token) = token_opt {
                let client = reqwest::Client::new();
                let body = serde_json::json!({ "user_id": user_id });
                let resp = client
                    .post(format!("{}/conversations/{}/members/remove", crate::services::api::SERVER_BASE_URL, gid))
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&body)
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.status().is_success() => {
                        state_clone.add_toast(
                            "Membro Removido".to_string(),
                            "O participante foi removido do grupo.".to_string(),
                            None,
                        );
                        state_clone.load_initial_data();
                    }
                    _ => {
                        state_clone.add_toast(
                            "Erro".to_string(),
                            "Não foi possível remover o participante.".to_string(),
                            None,
                        );
                    }
                }
            }
        });
    }

    pub fn update_group_permissions(&mut self, group_id: String, allow_send: bool, allow_invite: bool) {
        if let Some(group) = self.group_chats.write().iter_mut().find(|g| g.id == group_id) {
            group.allow_member_send = Some(allow_send);
            group.allow_member_invite = Some(allow_invite);
        }
        let pool = crate::services::db::get_pool();
        let gid = group_id.clone();
        spawn(async move {
            let _ = sqlx::query("UPDATE conversations SET allow_member_send = ?, allow_member_invite = ? WHERE id = ?")
                .bind(allow_send as i32)
                .bind(allow_invite as i32)
                .bind(gid)
                .execute(pool)
                .await;
        });
    }

    pub fn update_group_member_role(&mut self, group_id: String, user_id: String, role: String) {
        if let Some(group) = self.group_chats.write().iter_mut().find(|g| g.id == group_id) {
            if let Some(member) = group.members.iter_mut().find(|m| m.id == user_id) {
                member.role = Some(role.clone());
            }
        }
        let pool = crate::services::db::get_pool();
        let gid = group_id.clone();
        let uid = user_id.clone();
        let r = role.clone();
        spawn(async move {
            let _ = sqlx::query("UPDATE conversation_members SET role = ? WHERE conversation_id = ? AND user_id = ?")
                .bind(r)
                .bind(gid)
                .bind(uid)
                .execute(pool)
                .await;
        });
    }

    pub fn delete_group_chat(&mut self, group_id: String) {
        let token_opt = self.auth_token();
        let mut state_clone = *self;
        let gid = group_id.clone();
        spawn(async move {
            if let Some(token) = token_opt {
                let client = reqwest::Client::new();
                let _ = client
                    .delete(format!("{}/conversations/{}", crate::services::api::SERVER_BASE_URL, gid))
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await;
                
                let pool = crate::services::db::get_pool();
                let _ = sqlx::query("DELETE FROM conversations WHERE id = ?").bind(&gid).execute(pool).await;
                let _ = sqlx::query("DELETE FROM conversation_members WHERE conversation_id = ?").bind(&gid).execute(pool).await;
                let _ = sqlx::query("DELETE FROM messages WHERE conversation_id = ?").bind(&gid).execute(pool).await;
                
                state_clone.add_toast(
                    "Excluir Grupo".to_string(),
                    "Grupo excluído com sucesso.".to_string(),
                    None,
                );
                
                if state_clone.selected_chat_id() == Some(gid.clone()) {
                    *state_clone.selected_chat_id.write() = None;
                }
                state_clone.active_chats.write().retain(|id| id != &gid);
                state_clone.detached_chats.write().retain(|id| id != &gid);
                state_clone.load_initial_data();
            }
        });
    }
}
