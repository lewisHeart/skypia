use crate::state::AppState;
use crate::models::AppTheme;
use dioxus::prelude::*;

impl AppState {
    pub fn save_current_settings(&self) {
        let settings = crate::models::UserSettings {
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

        // Incrementa a versão atômica do estado para notificar outras threads/janelas
        crate::state::version::increment_state_version();

        spawn(async move {
            if let Err(e) = crate::services::db::DatabaseService::save_settings(&settings).await {
                eprintln!("❌ Erro ao salvar configurações de tela e tema no SQLite: {}", e);
            }
        });
    }

    pub fn set_settings(&mut self, scale: f64, custom_bar: bool, theme: AppTheme) {
        *self.interface_scale.write() = scale;
        *self.use_custom_titlebar.write() = custom_bar;
        *self.theme.write() = theme;
        self.save_current_settings();
    }

    pub fn set_chat_mode(&mut self, mode: String) {
        *self.chat_mode.write() = mode;
        self.save_current_settings();
    }

    pub fn chat_font_color(&self) -> String {
        self.chat_font_color.read().clone()
    }

    pub fn chat_font_family(&self) -> String {
        self.chat_font_family.read().clone()
    }

    pub fn set_chat_font_color(&mut self, color: String) {
        *self.chat_font_color.write() = color;
        self.save_current_settings();
    }

    pub fn set_chat_font_family(&mut self, font_family: String) {
        *self.chat_font_family.write() = font_family;
        self.save_current_settings();
    }

    pub fn spotify_rpc_enabled(&self) -> bool {
        (self.spotify_rpc_enabled)()
    }

    pub fn set_spotify_rpc_enabled(&mut self, enabled: bool) {
        *self.spotify_rpc_enabled.write() = enabled;
        self.save_current_settings();
    }

    pub fn show_typing_notification(&self) -> bool {
        (self.show_typing_notification)()
    }

    pub fn set_show_typing_notification(&mut self, show: bool) {
        *self.show_typing_notification.write() = show;
        self.save_current_settings();
    }

    pub fn enable_sounds(&self) -> bool {
        (self.enable_sounds)()
    }

    pub fn set_enable_sounds(&mut self, enable: bool) {
        *self.enable_sounds.write() = enable;
        self.save_current_settings();
    }

    pub fn enable_toasts(&self) -> bool {
        (self.enable_toasts)()
    }

    pub fn set_enable_toasts(&mut self, enable: bool) {
        *self.enable_toasts.write() = enable;
        self.save_current_settings();
    }

    pub fn download_folder(&self) -> String {
        self.download_folder.read().clone()
    }

    pub fn set_download_folder(&mut self, folder: String) {
        *self.download_folder.write() = folder;
        self.save_current_settings();
    }

    pub fn auto_accept_files(&self) -> bool {
        (self.auto_accept_files)()
    }

    pub fn set_auto_accept_files(&mut self, auto: bool) {
        *self.auto_accept_files.write() = auto;
        self.save_current_settings();
    }

    pub fn remember_password(&self) -> bool {
        (self.remember_password)()
    }

    pub fn set_remember_password(&mut self, remember: bool) {
        *self.remember_password.write() = remember;
        self.save_current_settings();
    }

    pub fn save_chat_history(&self) -> bool {
        (self.save_chat_history)()
    }

    pub fn set_save_chat_history(&mut self, save: bool) {
        *self.save_chat_history.write() = save;
        self.save_current_settings();
    }

    pub fn saved_email(&self) -> String {
        self.saved_email.read().clone()
    }

    pub fn set_saved_email(&mut self, email: String) {
        *self.saved_email.write() = email;
        self.save_current_settings();
    }

    pub fn saved_password(&self) -> String {
        self.saved_password.read().clone()
    }

    pub fn set_saved_password(&mut self, password: String) {
        *self.saved_password.write() = password;
        self.save_current_settings();
    }

    pub fn auto_login(&self) -> bool {
        (self.auto_login)()
    }

    pub fn set_auto_login(&mut self, auto: bool) {
        *self.auto_login.write() = auto;
        self.save_current_settings();
    }

    pub fn window_x(&self) -> i32 {
        (self.window_x)()
    }

    pub fn window_y(&self) -> i32 {
        (self.window_y)()
    }

    pub fn window_width(&self) -> f64 {
        (self.window_width)()
    }

    pub fn window_height(&self) -> f64 {
        (self.window_height)()
    }

    pub fn set_window_geom(&mut self, x: i32, y: i32, w: f64, h: f64) {
        let changed = (self.window_x)() != x 
            || (self.window_y)() != y 
            || (self.window_width)() != w 
            || (self.window_height)() != h;
        if changed {
            *self.window_x.write() = x;
            *self.window_y.write() = y;
            *self.window_width.write() = w;
            *self.window_height.write() = h;
            self.save_current_settings();
        }
    }

    pub fn fav_collapsed(&self) -> bool {
        (self.fav_collapsed)()
    }

    pub fn set_fav_collapsed(&mut self, val: bool) {
        *self.fav_collapsed.write() = val;
        self.save_current_settings();
    }

    pub fn online_collapsed(&self) -> bool {
        (self.online_collapsed)()
    }

    pub fn set_online_collapsed(&mut self, val: bool) {
        *self.online_collapsed.write() = val;
        self.save_current_settings();
    }

    pub fn offline_collapsed(&self) -> bool {
        (self.offline_collapsed)()
    }

    pub fn set_offline_collapsed(&mut self, val: bool) {
        *self.offline_collapsed.write() = val;
        self.save_current_settings();
    }

    pub fn groups_collapsed(&self) -> bool {
        (self.groups_collapsed)()
    }

    pub fn set_groups_collapsed(&mut self, val: bool) {
        *self.groups_collapsed.write() = val;
        self.save_current_settings();
    }

    pub fn collapsed_categories(&self) -> String {
        self.collapsed_categories.read().clone()
    }

    pub fn is_category_collapsed(&self, name: &str) -> bool {
        let cats_str = self.collapsed_categories.read().clone();
        if let Ok(cats) = serde_json::from_str::<Vec<String>>(&cats_str) {
            cats.contains(&name.to_string())
        } else {
            false
        }
    }

    pub fn toggle_category_collapsed(&mut self, name: &str) {
        let cats_str = self.collapsed_categories.read().clone();
        let mut cats = serde_json::from_str::<Vec<String>>(&cats_str).unwrap_or_default();
        if cats.contains(&name.to_string()) {
            cats.retain(|c| c != name);
        } else {
            cats.push(name.to_string());
        }
        if let Ok(serialized) = serde_json::to_string(&cats) {
            *self.collapsed_categories.write() = serialized;
            self.save_current_settings();
        }
    }
}
