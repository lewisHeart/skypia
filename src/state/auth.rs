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
        if let Some(music) = profile.music.clone() {
            *self.user_music.write() = Some(music);
        }
        if let Some(url) = profile.avatar_url.clone() {
            *self.user_avatar_url.write() = Some(url);
        }
        // Salva token e perfil no SQLite local para auto-login e consistência
        let user_id = profile.id;
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
