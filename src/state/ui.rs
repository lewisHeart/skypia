use crate::state::AppState;
use crate::state::Toast;
use dioxus::prelude::*;

impl AppState {
    pub fn add_toast(&mut self, title: String, message: String, avatar_url: Option<String>) {
        let id = self.toast_counter();
        *self.toast_counter.write() += 1;

        let toast = Toast {
            id,
            title,
            message,
            avatar_url,
        };

        self.toasts.write().push(toast);
    }

    pub fn remove_toast(&mut self, id: usize) {
        self.toasts.write().retain(|t| t.id != id);
    }

    pub fn open_my_profile(&mut self) {
        *self.profile_modal_contact_id.write() = None;
        self.show_profile_modal.set(true);
    }

    pub fn open_contact_profile(&mut self, contact_id: String) {
        *self.profile_modal_contact_id.write() = Some(contact_id);
        self.show_profile_modal.set(true);
    }

    // Getters de UI e Modais
    pub fn show_games_modal(&self) -> bool {
        (self.show_games_modal)()
    }

    pub fn show_settings_modal(&self) -> bool {
        (self.show_settings_modal)()
    }

    pub fn show_add_contact_modal(&self) -> bool {
        (self.show_add_contact_modal)()
    }

    pub fn show_music_player_modal(&self) -> bool {
        (self.show_music_player_modal)()
    }

    pub fn show_profile_modal(&self) -> bool {
        (self.show_profile_modal)()
    }

    pub fn show_about(&self) -> bool {
        (self.show_about)()
    }

    pub fn profile_modal_contact_id(&self) -> Option<String> {
        self.profile_modal_contact_id.read().clone()
    }

    pub fn show_friend_requests_modal(&self) -> bool {
        (self.show_friend_requests_modal)()
    }

    pub fn show_group_profile_modal(&self) -> bool {
        (self.show_group_profile_modal)()
    }

    pub fn group_profile_id(&self) -> Option<String> {
        self.group_profile_id.read().clone()
    }
}
