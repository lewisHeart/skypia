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
        *self.is_admin.write() = profile.is_admin.unwrap_or(false);
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
