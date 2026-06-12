use crate::state::AppState;
use dioxus::prelude::*;

impl AppState {
    /// Aplica o perfil vindo do servidor ao estado local
    pub async fn apply_server_profile(&mut self, profile: crate::models::UserProfile, token: String) {
        *self.auth_token.write() = Some(token.clone());
        *self.server_user_id.write() = Some(profile.id.clone());
        *self.user_name.write() = profile.display_name.clone();
        *self.user_email.write() = profile.email.clone();
        *self.user_personal_message.write() = profile.personal_message.clone();
        *self.is_admin.write() = profile.is_admin;
        if let Some(music) = profile.music.clone() {
            *self.user_music.write() = Some(music);
        }
        // Salva token e perfil no SQLite local para auto-login e consistência
        let user_id = profile.id.clone();
        let display_name = profile.display_name.clone();
        let email = profile.email.clone();
        let pm = profile.personal_message.clone();
        let music = profile.music.clone();
        
        let _ = crate::services::db::DatabaseService::save_auth_token(token, user_id).await;
        let _ = crate::services::db::DatabaseService::save_user_profile_data(
            display_name,
            email,
            pm,
            music,
        ).await;

        // O servidor é a fonte de verdade para o avatar.
        // Se o servidor tem um avatar_url, ele sempre tem prioridade.
        // Se o servidor retorna null (ex: banco resetado), limpa o avatar local
        // para evitar exibir uma URL stale que não existe mais no servidor.
        if let Some(url) = profile.avatar_url {
            // Servidor tem avatar — usa e persiste localmente
            *self.user_avatar_url.write() = Some(url.clone());
            let _ = crate::services::db::DatabaseService::save_user_avatar_url(Some(url)).await;
        } else {
            // Servidor não tem avatar — verifica se o local é um asset predefinido embutido
            // (que funciona offline), caso contrário limpa para não exibir URL stale do servidor
            let local_url_opt = crate::services::db::DatabaseService::load_user_avatar_url()
                .await
                .ok()
                .flatten();

            let keep_local = local_url_opt.as_deref().map(|u| {
                u.starts_with("/assets/")
                    || u.starts_with("assets/")
                    || u.starts_with("/_assets/")
                    || u.starts_with("_assets/")
                    || u.starts_with("dioxus-asset://")
            }).unwrap_or(false);

            if keep_local {
                *self.user_avatar_url.write() = local_url_opt;
            } else {
                // URL era do servidor (relativa /uploads/...) — limpa pois o servidor não tem mais
                *self.user_avatar_url.write() = None;
                let _ = crate::services::db::DatabaseService::save_user_avatar_url(None).await;
            }
        }

        // Se o servidor enviou preferências de UI, aplicá-las ao estado atual.
        if let Some(prefs_val) = profile.ui_preferences {
            if let Ok(settings) = serde_json::from_value::<crate::models::UserSettings>(prefs_val) {
                *self.interface_scale.write() = settings.interface_scale;
                *self.use_custom_titlebar.write() = settings.use_custom_titlebar;
                *self.theme.write() = crate::services::db::str_to_theme(&settings.theme);
                *self.chat_mode.write() = settings.chat_mode;
                self.update_densities_from_serialized(settings.contact_density);
                *self.chat_font_color.write() = settings.font_color;
                *self.chat_font_family.write() = settings.font_family;
                *self.spotify_rpc_enabled.write() = settings.spotify_rpc_enabled;
                *self.show_typing_notification.write() = settings.show_typing_notification;
                *self.enable_sounds.write() = settings.enable_sounds;
                *self.enable_toasts.write() = settings.enable_toasts;
                *self.download_folder.write() = settings.download_folder;
                *self.auto_accept_files.write() = settings.auto_accept_files;
                // Don't overwrite local password settings from cloud
                // *self.remember_password.write() = settings.remember_password;
                // *self.save_chat_history.write() = settings.save_chat_history;
                // *self.saved_email.write() = settings.saved_email;
                // *self.saved_password.write() = settings.saved_password;
                // *self.auto_login.write() = settings.auto_login;
                *self.window_x.write() = settings.window_x;
                *self.window_y.write() = settings.window_y;
                *self.window_width.write() = settings.window_width;
                *self.window_height.write() = settings.window_height;
                *self.fav_collapsed.write() = settings.fav_collapsed;
                *self.online_collapsed.write() = settings.online_collapsed;
                *self.offline_collapsed.write() = settings.offline_collapsed;
                *self.groups_collapsed.write() = settings.groups_collapsed;
                *self.collapsed_categories.write() = settings.collapsed_categories;

                // Salva no banco local
                let current_settings = crate::models::UserSettings {
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
                    saved_email: self.saved_email(),
                    saved_password: self.saved_password(),
                    auto_login: self.auto_login(),
                    window_x: (self.window_x)(),
                    window_y: (self.window_y)(),
                    window_width: (self.window_width)(),
                    window_height: (self.window_height)(),
                    fav_collapsed: (self.fav_collapsed)(),
                    online_collapsed: (self.online_collapsed)(),
                    offline_collapsed: (self.offline_collapsed)(),
                    groups_collapsed: (self.groups_collapsed)(),
                    collapsed_categories: self.collapsed_categories.read().clone(),
                };
                let _ = crate::services::db::DatabaseService::save_settings(&current_settings).await;
            }
        }
    }


    /// Estabelece a conexão com o WebSocket do servidor
    pub fn connect_websocket(&mut self) {
        if let Some(token) = self.auth_token() {
            crate::services::ws::connect_ws(*self, token);
        }
    }

    /// Faz logout completo
    pub fn logout(&mut self) {
        let mut self_clone = *self;
        spawn(async move {
            // 1. Limpa o token no SQLite primeiro para evitar condição de corrida (race condition)
            let _ = crate::services::db::DatabaseService::clear_auth_token().await;
            
            // 2. Atualiza os estados na UI para deslogar
            *self_clone.logged_in.write() = false;
            *self_clone.auth_token.write() = None;
            *self_clone.server_user_id.write() = None;
            *self_clone.user_avatar_url.write() = None;
            *self_clone.ws_tx.write() = None;
        });
    }
}
