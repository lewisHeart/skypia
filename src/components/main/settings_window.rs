use crate::state::AppState;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn SettingsWindow() -> Element {
    #[cfg(feature = "desktop")]
    {
        let mut app_state = use_context_provider(|| AppState::new());
        let desktop = dioxus::desktop::use_window();

        // Garante login local na nova thread de execução do Dioxus
        *app_state.logged_in.write() = true;

        use_effect(move || {
            let mut state = app_state;
            spawn(async move {
                // Carrega o token do banco SQLite para iniciar sessão
                if let Ok(Some((token, user_id))) =
                    crate::services::db::DatabaseService::load_auth_token().await
                {
                    *state.auth_token.write() = Some(token.clone());
                    *state.server_user_id.write() = Some(user_id.clone());
                    *state.logged_in.write() = true;
                    state.load_initial_data();
                }
            });
        });

        // Loop periódico para verificar e aplicar mudanças de tema no banco SQLite feitas por outras janelas
        use_effect(move || {
            let mut state = app_state;
            spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    if let Ok(settings) = crate::services::db::DatabaseService::load_settings().await {
                        let db_theme = crate::services::db::str_to_theme(&settings.theme);
                        if db_theme != state.theme() {
                            *state.theme.write() = db_theme;
                        }
                        if settings.interface_scale != state.interface_scale() {
                            *state.interface_scale.write() = settings.interface_scale;
                        }
                    }
                }
            });
        });

        // Sincroniza as decorações da janela
        let desktop_dec = desktop.clone();
        use_effect(move || {
            let use_custom = app_state.use_custom_titlebar();
            desktop_dec.set_decorations(!use_custom);
        });

        let theme = app_state.theme();

        rsx! {
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }

            div {
                class: "w-screen h-screen overflow-hidden flex flex-col bg-gradient-to-br {theme.bg_gradient()} font-segoe select-none border border-[#7baad4]/40 shadow-2xl relative",
                
                if app_state.use_custom_titlebar() {
                    div {
                        class: "w-full h-10 bg-transparent flex items-center justify-between z-50 flex-shrink-0 select-none px-4 relative cursor-default",
                        style: "-webkit-app-region: drag;",
                        onmousedown: move |_| {
                            desktop.drag_window();
                        },
                        div { class: "flex items-center space-x-1.5 font-normal text-sm pointer-events-none {theme.titlebar_text()} select-none",
                            span { class: "text-[#0d1825] font-sans text-sm", "Opções" }
                        }
                        div {
                            class: "flex items-center",
                            style: "-webkit-app-region: no-drag;",
                            onmousedown: move |e| e.stop_propagation(),
                            button {
                                class: "w-[31px] h-[20px] bg-white border border-[#d1d1d1] rounded-[4px] shadow-sm flex items-center justify-center cursor-pointer transition-all hover:bg-[#e81123] hover:border-[#e81123] hover:text-white text-[#6f6f6f] focus:outline-none",
                                title: "Fechar",
                                onclick: move |_| {
                                    desktop.close();
                                },
                                svg {
                                    view_box: "0 0 12 12",
                                    class: "w-2.5 h-2.5 stroke-current",
                                    path { d: "M2 2 L10 10 M10 2 L2 10", stroke_width: "1.5", stroke_linecap: "round" }
                                }
                            }
                        }
                    }
                }

                div {
                    class: "flex-1 min-h-0 w-full relative",
                    crate::components::main::settings_content::SettingsContent {
                        state: app_state,
                        is_native_window: true
                    }
                }
            }
        }
    }
    #[cfg(not(feature = "desktop"))]
    {
        rsx! {
            div { "Não suportado em web/mobile." }
        }
    }
}

/// Abre a janela nativa de configurações no desktop
#[cfg(feature = "desktop")]
pub fn open_settings_window() {
    let dom = VirtualDom::new(SettingsWindow);
    spawn(async move {
        let mut config = dioxus::desktop::Config::new().with_menu(None);
        let my_pid = std::process::id();
        let db_dir = std::path::Path::new(".skypia_data").join("db");
        let mut active_slot = None;
        for slot in 1..=10 {
            let lock_path = db_dir.join(format!("skypia_{}.lock", slot));
            if let Ok(content) = std::fs::read_to_string(&lock_path) {
                if let Ok(pid) = content.trim().parse::<u32>() {
                    if pid == my_pid {
                        active_slot = Some(slot);
                        break;
                    }
                }
            }
        }
        if let Some(slot) = active_slot {
            let data_dir = std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .join(".skypia_data")
                .join("webview")
                .join(format!("slot_{}", slot));
            let _ = std::fs::create_dir_all(&data_dir);
            config = config.with_data_directory(data_dir);
        }
        let _ = dioxus::desktop::window().new_window(
            dom,
            config.with_window(
                dioxus::desktop::WindowBuilder::new()
                    .with_title("Opções - Skypia Messenger")
                    .with_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(620.0, 480.0))
                    .with_resizable(false)
                    .with_decorations(false)
            )
        ).await;
    });
}
