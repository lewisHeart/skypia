use crate::state::AppState;
use crate::models::{Contact, UserStatus};
use dioxus::prelude::*;
use std::collections::HashMap;

impl AppState {
    // Carrega dados dinâmicos do banco assincronamente em paralelo
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
        let mut saved_email_sig = self.saved_email;
        let mut saved_password_sig = self.saved_password;
        let mut auto_login_sig = self.auto_login;
        let mut categories_sig = self.categories;
        let mut window_x_sig = self.window_x;
        let mut window_y_sig = self.window_y;
        let mut window_width_sig = self.window_width;
        let mut window_height_sig = self.window_height;
        let mut fav_collapsed_sig = self.fav_collapsed;
        let mut online_collapsed_sig = self.online_collapsed;
        let mut offline_collapsed_sig = self.offline_collapsed;
        let mut groups_collapsed_sig = self.groups_collapsed;
        let mut collapsed_cats_sig = self.collapsed_categories;

        let token_opt = self.auth_token();
        let self_user_id = self.server_user_id();
        let mut self_clone = *self;

        spawn(async move {
            // 1. Carrega todas as informações básicas locais concorrentemente
            let (
                cats_res,
                contacts_res,
                conversations_res,
                settings_res,
                detached_res,
                name_res,
                music_res,
                avatar_url_res,
                songs_res,
                banner_res,
            ) = tokio::join!(
                crate::services::db::DatabaseService::get_categories(),
                crate::services::db::DatabaseService::load_contacts(),
                crate::services::db::DatabaseService::load_conversations(),
                crate::services::db::DatabaseService::load_settings(),
                crate::services::db::DatabaseService::get_detached_chats(),
                crate::services::db::DatabaseService::load_user_name(),
                crate::services::db::DatabaseService::load_user_music(),
                crate::services::db::DatabaseService::load_user_avatar_url(),
                crate::services::db::DatabaseService::get_recommended_songs(),
                crate::services::db::DatabaseService::load_banner(),
            );

            // 2. Desempacota e aplica no estado em memória
            if let Ok(cats) = cats_res {
                *categories_sig.write() = cats;
            } else if let Err(e) = cats_res {
                eprintln!("❌ Erro ao carregar categorias locais do SQLite: {}", e);
            }

            if let Ok(local_contacts) = contacts_res {
                if !local_contacts.is_empty() {
                    *contacts_sig.write() = local_contacts;
                }
            } else if let Err(e) = contacts_res {
                eprintln!("❌ Erro ao carregar contatos locais do SQLite: {}", e);
            }

            if let Ok(db_settings) = settings_res {
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
                *saved_email_sig.write() = db_settings.saved_email;
                *saved_password_sig.write() = db_settings.saved_password;
                *auto_login_sig.write() = db_settings.auto_login;
                *window_x_sig.write() = db_settings.window_x;
                *window_y_sig.write() = db_settings.window_y;
                *window_width_sig.write() = db_settings.window_width;
                *window_height_sig.write() = db_settings.window_height;
                *fav_collapsed_sig.write() = db_settings.fav_collapsed;
                *online_collapsed_sig.write() = db_settings.online_collapsed;
                *offline_collapsed_sig.write() = db_settings.offline_collapsed;
                *groups_collapsed_sig.write() = db_settings.groups_collapsed;
                *collapsed_cats_sig.write() = db_settings.collapsed_categories;
            } else if let Err(e) = settings_res {
                eprintln!("❌ Erro ao carregar configurações locais do SQLite: {}", e);
            }

            if let Ok(detached) = detached_res {
                *detached_sig.write() = detached;
            } else if let Err(e) = detached_res {
                eprintln!("❌ Erro ao carregar chats destacados do SQLite: {}", e);
            }

            if let Ok(name) = name_res {
                *name_sig.write() = name;
            } else if let Err(e) = name_res {
                eprintln!("❌ Erro ao carregar nome de usuário do SQLite: {}", e);
            }

            if let Ok(music) = music_res {
                *music_sig.write() = music;
            } else if let Err(e) = music_res {
                eprintln!("❌ Erro ao carregar recado musical do SQLite: {}", e);
            }

            if let Ok(avatar_url) = avatar_url_res {
                *avatar_url_sig.write() = avatar_url;
            } else if let Err(e) = avatar_url_res {
                eprintln!("❌ Erro ao carregar URL do avatar do SQLite: {}", e);
            }

            if let Ok(songs) = songs_res {
                *songs_sig.write() = songs;
            } else if let Err(e) = songs_res {
                eprintln!("❌ Erro ao carregar recomendações de música do SQLite: {}", e);
            }

            if let Ok(Some(local_banner)) = banner_res {
                *banner_sig.write() = Some(local_banner);
            } else {
                if let Ok(banner) = crate::services::api::get_banner().await {
                    *banner_sig.write() = Some(banner);
                } else {
                    *banner_sig.write() = None;
                }
            }

            // 3. Paraleliza o carregamento de mensagens de todas as conversas locais
            if let Ok(local_conversations) = conversations_res {
                let mut all_messages = std::collections::HashMap::new();
                let mut groups = Vec::new();
                
                let mut msg_futures = Vec::new();
                for conv in &local_conversations {
                    if conv.is_group {
                        groups.push(conv.clone());
                    }
                    let conv_id = conv.id.clone();
                    msg_futures.push(async move {
                        let msgs = crate::services::db::DatabaseService::load_messages(conv_id.clone())
                            .await
                            .unwrap_or_default();
                        (conv_id, msgs)
                    });
                }
                
                let msgs_results = futures_util::future::join_all(msg_futures).await;
                for (conv_id, mut local_messages) in msgs_results {
                    for msg in &mut local_messages {
                        if let Some(ref s_id) = self_user_id {
                            if &msg.sender_id == s_id {
                                msg.sender_id = "0".to_string();
                            }
                        }
                    }
                    all_messages.insert(conv_id, local_messages);
                }
                
                *chat_messages_sig.write() = all_messages;
                *group_chats_sig.write() = groups;
            } else if let Err(e) = conversations_res {
                eprintln!("❌ Erro ao carregar conversas locais do SQLite: {}", e);
            }

            // 4. Sincronização de rede se autenticado
            if let Some(token) = token_opt {
                if let Ok(srv_contacts) = crate::services::api::get_contacts(&token).await {
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
                        let is_fav = profile.is_favorite.unwrap_or_else(|| local_favorites.contains(&profile.id));
                        let cat_name = profile.category_name.or_else(|| local_categories_map.get(&profile.id).cloned());
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
                    let contacts_mapped_clone = contacts_mapped.clone();
                    spawn(async move {
                        if let Err(e) = crate::services::db::DatabaseService::save_contacts_bulk(contacts_mapped_clone).await {
                            eprintln!("❌ Erro ao salvar contatos sincronizados no SQLite: {}", e);
                        } else {
                            crate::state::version::increment_state_version();
                        }
                    });
                }

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

                if let Ok(srv_conversations) = crate::services::api::get_conversations(&token).await
                {
                    if let Err(e) = crate::services::db::DatabaseService::save_conversations(srv_conversations.clone()).await {
                        eprintln!("❌ Erro ao salvar conversas sincronizadas no SQLite: {}", e);
                    }
                    let mut all_messages = HashMap::new();
                    let mut groups = Vec::new();

                    for conv in srv_conversations {
                        if conv.is_group {
                            groups.push(conv.clone());
                            if let Ok(srv_messages) =
                                crate::services::api::get_conversation_messages(&token, &conv.id)
                                    .await
                            {
                                let srv_messages_clone = srv_messages.clone();
                                let conv_id = conv.id.clone();
                                spawn(async move {
                                    if let Err(e) = crate::services::db::DatabaseService::save_messages_bulk(conv_id, srv_messages_clone).await {
                                        eprintln!("❌ Erro ao salvar mensagens de grupo sincronizadas no SQLite: {}", e);
                                    } else {
                                        crate::state::version::increment_state_version();
                                    }
                                });
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
                                    let srv_messages_clone = srv_messages.clone();
                                    let conv_id = conv.id.clone();
                                    spawn(async move {
                                        if let Err(e) = crate::services::db::DatabaseService::save_messages_bulk(conv_id, srv_messages_clone).await {
                                            eprintln!("❌ Erro ao salvar mensagens 1:1 sincronizadas no SQLite: {}", e);
                                        } else {
                                            crate::state::version::increment_state_version();
                                        }
                                    });
                                    let mut normalized_messages = Vec::new();
                                    for mut msg in srv_messages {
                                        if let Some(ref s_id) = self_user_id {
                                            if &msg.sender_id == s_id {
                                                msg.sender_id = "0".to_string();
                                            }
                                        }
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
        });
    }
}
