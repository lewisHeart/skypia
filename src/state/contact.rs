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
}
