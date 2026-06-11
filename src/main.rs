// Force rebuild to trigger build.rs Android monitoring - 2026
use crate::components::auth::login::Login;
use crate::components::chat::chat_window::ChatWindow;
use crate::components::main::main_window::MainWindow;
use crate::components::ToastList;
use crate::state::AppState;
use dioxus::prelude::*;

mod components;
mod models;
mod services;
mod sound;
mod state;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn get_current_slot() -> Option<u32> {
    let my_pid = std::process::id();
    let db_dir = std::path::Path::new(".skypia_data").join("db");
    for slot in 1..=10 {
        let lock_path = db_dir.join(format!("skypia_{}.lock", slot));
        if let Ok(content) = std::fs::read_to_string(&lock_path) {
            if let Ok(pid) = content.trim().parse::<u32>() {
                if pid == my_pid {
                    return Some(slot);
                }
            }
        }
    }
    None
}

fn main() {
    // Carrega variáveis de ambiente a partir do arquivo .env
    dotenvy::dotenv().ok();

    // Cria o runtime do Tokio global persistente com suporte completo (rede, timers, etc.)
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create tokio runtime");

    // Vincula o runtime do Tokio à thread principal e todas as threads filhas criadas
    // pelo Dioxus Desktop para que o driver de rede e IO esteja disponível para o WebSocket
    let _guard = rt.enter();

    rt.block_on(async {
        if let Err(e) = crate::services::db::DatabaseService::init_pool().await {
            eprintln!("Erro ao inicializar banco de dados: {}", e);
            std::process::exit(1);
        }
    });

    #[cfg(feature = "desktop")]
    {
        let mut config = dioxus::desktop::Config::new().with_menu(None);
        if let Some(slot) = get_current_slot() {
            let data_dir = std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .join(".skypia_data")
                .join("webview")
                .join(format!("slot_{}", slot));

            // Garante a criação do diretório de dados
            let _ = std::fs::create_dir_all(&data_dir);
            config = config.with_data_directory(data_dir);
        }

        dioxus::LaunchBuilder::desktop()
            .with_cfg(config)
            .launch(App);
    }
    #[cfg(not(feature = "desktop"))]
    {
        dioxus::launch(App);
    }
}

#[component]
fn App() -> Element {
    // Initialize AppState as a global context
    let mut app_state = use_context_provider(|| AppState::new());
    let logged_in = app_state.logged_in();
    let theme = app_state.theme();

    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::use_asset_handler;

        use_asset_handler("emojis", move |request, responder| {
            let host = request.uri().host().unwrap_or("");
            let path = request.uri().path();
            let raw_path = format!("{}{}", host, path);
            let decoded_path = percent_encoding::percent_decode_str(&raw_path).decode_utf8_lossy().to_string();
            let decoded_path = decoded_path.trim_end_matches('/');
            let mut emoji_path = std::path::PathBuf::from("public/emojis").join(&decoded_path);
            if !emoji_path.exists() {
                if let Ok(res_dir) = std::env::current_exe() {
                    if let Some(parent) = res_dir.parent() {
                        let fallback = parent.join("public/emojis").join(&decoded_path);
                        if fallback.exists() {
                            emoji_path = fallback;
                        }
                    }
                }
            }

            tokio::task::spawn(async move {
                if let Ok(bytes) = std::fs::read(&emoji_path) {
                    let mime = if emoji_path.extension().map(|e| e == "svg").unwrap_or(false) {
                        "image/svg+xml"
                    } else if emoji_path.extension().map(|e| e == "png").unwrap_or(false) {
                        "image/png"
                    } else if emoji_path.extension().map(|e| e == "webp").unwrap_or(false) {
                        "image/webp"
                    } else {
                        "image/gif"
                    };
                    let response = dioxus::desktop::wry::http::Response::builder()
                        .header("Content-Type", mime)
                        .header("Access-Control-Allow-Origin", "*")
                        .body(bytes)
                        .unwrap();
                    responder.respond(response);
                } else {
                    let response = dioxus::desktop::wry::http::Response::builder()
                        .status(404)
                        .body(Vec::new())
                        .unwrap();
                    responder.respond(response);
                }
            });
        });

        use_asset_handler("emojis-anim", move |request, responder| {
            let host = request.uri().host().unwrap_or("");
            let path = request.uri().path();
            let raw_path = format!("{}{}", host, path);
            let decoded_path = percent_encoding::percent_decode_str(&raw_path).decode_utf8_lossy().to_string();
            let decoded_path = decoded_path.trim_end_matches('/');
            let mut emoji_path = std::path::PathBuf::from("public/emojis_anim").join(&decoded_path);
            if !emoji_path.exists() {
                if let Ok(res_dir) = std::env::current_exe() {
                    if let Some(parent) = res_dir.parent() {
                        let fallback = parent.join("public/emojis_anim").join(&decoded_path);
                        if fallback.exists() {
                            emoji_path = fallback;
                        }
                    }
                }
            }

            tokio::task::spawn(async move {
                if let Ok(bytes) = std::fs::read(&emoji_path) {
                    let mime = if emoji_path.extension().map(|e| e == "webp").unwrap_or(false) {
                        "image/webp"
                    } else if emoji_path.extension().map(|e| e == "gif").unwrap_or(false) {
                        "image/gif"
                    } else {
                        "image/png"
                    };
                    let response = dioxus::desktop::wry::http::Response::builder()
                        .header("Content-Type", mime)
                        .header("Access-Control-Allow-Origin", "*")
                        .body(bytes)
                        .unwrap();
                    responder.respond(response);
                } else {
                    let response = dioxus::desktop::wry::http::Response::builder()
                        .status(404)
                        .body(Vec::new())
                        .unwrap();
                    responder.respond(response);
                }
            });
        });
    }


    // Sincroniza decorações da janela nativa
    use_effect(move || {
        #[cfg(feature = "desktop")]
        {
            let desktop = dioxus::desktop::use_window();
            desktop.set_decorations(!app_state.use_custom_titlebar());
        }
    });

    let mut last_has_chat = use_signal(|| false);

    // Ajusta o tamanho da janela principal do SO baseado no estado do chat (apenas na transição para evitar flicker)
    use_effect(move || {
        #[cfg(feature = "desktop")]
        {
            let desktop = dioxus::desktop::use_window();
            let has_selected_chat =
                app_state.selected_chat_id().is_some() && app_state.chat_mode() == "integrated";
            let was_chat_open = *last_has_chat.read();
            if has_selected_chat != was_chat_open {
                last_has_chat.set(has_selected_chat);
                if has_selected_chat {
                    desktop
                        .set_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(926.0, 735.0));
                } else {
                    desktop
                        .set_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(413.0, 735.0));
                }
            }
        }
    });

    // Carregamento dinâmico de dados quando logado
    use_effect(move || {
        if app_state.logged_in() {
            let mut state = app_state;
            state.load_initial_data();
        }
    });

    // Conexão com o WebSocket em tempo real quando logado
    use_effect(move || {
        if app_state.logged_in() {
            let mut state = app_state;
            state.connect_websocket();
        }
    });

    // Loop periódico para verificar inatividade (Ausente automático)
    use_future(move || {
        let mut state = app_state;
        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                if state.logged_in() {
                    state.check_inactivity_and_update();
                }
            }
        }
    });

    // Loop periódico para verificar e aplicar mudanças de tema/escala no banco SQLite feitas pela janela de configurações nativa
    use_future(move || {
        let mut state = app_state;
        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                if state.logged_in() {
                    if let Ok(settings) = crate::services::db::DatabaseService::load_settings().await {
                        let db_theme = crate::services::db::str_to_theme(&settings.theme);
                        if db_theme != state.theme() {
                            *state.theme.write() = db_theme;
                        }
                        if settings.interface_scale != state.interface_scale() {
                            *state.interface_scale.write() = settings.interface_scale;
                        }
                        if settings.use_custom_titlebar != state.use_custom_titlebar() {
                            *state.use_custom_titlebar.write() = settings.use_custom_titlebar;
                        }
                        if settings.chat_mode != state.chat_mode() {
                            *state.chat_mode.write() = settings.chat_mode;
                        }
                    }
                    if let Ok(local_contacts) = crate::services::db::DatabaseService::load_contacts().await {
                        let current = state.contacts();
                        if current.len() != local_contacts.len() || current != local_contacts {
                            *state.contacts.write() = local_contacts;
                        }
                    }
                    if let Ok(cats) = crate::services::db::DatabaseService::get_categories().await {
                        let current = state.categories();
                        if current != cats {
                            *state.categories.write() = cats;
                        }
                    }
                    if let Ok(Some(local_banner)) = crate::services::db::DatabaseService::load_banner().await {
                        let current = state.banner_info();
                        if current.as_ref() != Some(&local_banner) {
                            *state.banner_info.write() = Some(local_banner);
                        }
                    }
                }
            }
        }
    });

    // Loop periódico para detectar música do Spotify local (Spotify RPC)
    use_future(move || {
        let mut state = app_state;
        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(*crate::services::api::SPOTIFY_CHECK_INTERVAL)).await;
                if state.logged_in() && state.spotify_rpc_enabled() {
                    let detected = crate::services::spotify::detect_current_song().await;

                    if let Some(music) = detected {
                        if state.user_music() != Some(music.clone()) {
                            state.set_user_music(Some(music));
                        }
                    } else {
                        if state.user_music().is_some() {
                            state.set_user_music(None);
                        }
                    }
                }
            }
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // Main Application Screen
        div {
            class: "w-screen h-screen overflow-hidden flex flex-col bg-gradient-to-b {theme.bg_gradient()} relative font-segoe select-none rounded-t-2xl border border-[#7baad4]/40 shadow-2xl",
            onmousemove: move |_| {
                if logged_in {
                    app_state.record_activity();
                }
            },
            onkeydown: move |_| {
                if logged_in {
                    app_state.record_activity();
                }
            },
            onclick: move |_| {
                if logged_in {
                    app_state.record_activity();
                }
            },

            // 1. Bordas e Cantos para Redimensionamento Nativo de Janela (Escala 100%)
            if app_state.use_custom_titlebar() {
                // Borda Superior
                div {
                    class: "absolute top-0 left-1.5 right-1.5 h-1.5 cursor-ns-resize z-[999] opacity-0",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        #[cfg(feature = "desktop")]
                        let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::North);
                    }
                }
                // Borda Inferior
                div {
                    class: "absolute bottom-0 left-1.5 right-1.5 h-1.5 cursor-ns-resize z-[999] opacity-0",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        #[cfg(feature = "desktop")]
                        let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::South);
                    }
                }
                // Borda Esquerda
                div {
                    class: "absolute top-1.5 bottom-1.5 left-0 w-1.5 cursor-ew-resize z-[999] opacity-0",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        #[cfg(feature = "desktop")]
                        let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::West);
                    }
                }
                // Borda Direita
                div {
                    class: "absolute top-1.5 bottom-1.5 right-0 w-1.5 cursor-ew-resize z-[999] opacity-0",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        #[cfg(feature = "desktop")]
                        let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::East);
                    }
                }
                // Canto Superior Esquerdo
                div {
                    class: "absolute top-0 left-0 w-2.5 h-2.5 cursor-nwse-resize z-[999] opacity-0",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        #[cfg(feature = "desktop")]
                        let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::NorthWest);
                    }
                }
                // Canto Superior Direito
                div {
                    class: "absolute top-0 right-0 w-2.5 h-2.5 cursor-nesw-resize z-[999] opacity-0",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        #[cfg(feature = "desktop")]
                        let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::NorthEast);
                    }
                }
                // Canto Inferior Esquerdo
                div {
                    class: "absolute bottom-0 left-0 w-2.5 h-2.5 cursor-nesw-resize z-[999] opacity-0",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        #[cfg(feature = "desktop")]
                        let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::SouthWest);
                    }
                }
                // Canto Inferior Direito
                div {
                    class: "absolute bottom-0 right-0 w-2.5 h-2.5 cursor-nwse-resize z-[999] opacity-0",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        #[cfg(feature = "desktop")]
                        let _ = dioxus::desktop::use_window().drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::SouthEast);
                    }
                }
            }

            // 2. Custom Title Bar for Windows 7 Aero-like custom styling (Escala 100%)
            if app_state.use_custom_titlebar() {
                div {
                    class: "w-full h-10 bg-transparent flex items-center justify-between z-50 flex-shrink-0 select-none px-4 relative rounded-t-2xl cursor-default",
                    style: "-webkit-app-region: drag;",
                    onmousedown: move |_| {
                        #[cfg(feature = "desktop")]
                        let _ = dioxus::desktop::use_window().drag_window();
                    },

                    // Icon and Title
                    div { class: "flex items-center space-x-1.5 font-normal text-sm pointer-events-none {theme.titlebar_text()} select-none",
                        span { class: "text-[#0d1825] font-sans text-sm", "Skypia Messenger" }
                    }

                    // Controls (X)
                    div {
                        class: "flex items-center",
                        style: "-webkit-app-region: no-drag;",
                        onmousedown: move |e| e.stop_propagation(),

                        // Close [X] Button (Matches user design exactly)
                        button {
                            class: "w-[31px] h-[20px] bg-white border border-[#d1d1d1] rounded-[4px] shadow-sm flex items-center justify-center cursor-pointer transition-all hover:bg-[#e81123] hover:border-[#e81123] hover:text-white text-[#6f6f6f] hover:text-white focus:outline-none",
                            title: "Fechar",
                            onclick: move |e| {
                                e.stop_propagation();
                                #[cfg(feature = "desktop")]
                                dioxus::desktop::use_window().close();
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

            // 3. Área de Conteúdo do Cliente (Essa sim é escalonada pelo usuário!)
            div {
                class: "flex-1 min-h-0 w-full relative",

                div {
                    class: "absolute inset-0 overflow-hidden",

                    div {
                        class: "w-full h-full relative",
                        style: "transform: scale({app_state.interface_scale()}); transform-origin: top left; width: {100.0 / app_state.interface_scale()}%; height: {100.0 / app_state.interface_scale()}%;",

                        if !logged_in {
                            // Login page occupies the whole screen (centers form card inside itself)
                            Login { state: app_state }
                        } else {
                            // Full Screen split-pane layout
                            {
                                let is_integrated = app_state.chat_mode() == "integrated";
                                let has_selected_chat = app_state.selected_chat_id().is_some() && is_integrated;
                                let sidebar_class = if has_selected_chat {
                                    "hidden md:flex w-[260px] h-full flex-col flex-shrink-0"
                                } else {
                                    "w-full h-full flex flex-col flex-shrink-0"
                                };
                                let chat_container_class = if has_selected_chat {
                                    "flex-1 h-full flex flex-col min-w-0"
                                } else {
                                    "hidden"
                                };

                                rsx! {
                                    div { class: "w-full flex-1 min-h-0 flex flex-row pointer-events-auto z-10 h-full",
                                        div { class: sidebar_class,
                                            MainWindow { state: app_state }
                                        }
                                        if has_selected_chat {
                                            div { class: chat_container_class,
                                                ChatWindow { state: app_state, contact_id_prop: None }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Active Toast Alerts floating layer
            ToastList { state: app_state }
        }

        // Modal de Solicitações de Amizade (Aero Style)
        if logged_in && app_state.show_friend_requests_modal() {
            crate::components::main::friend_requests_modal::FriendRequestsModal { state: app_state }
        }

        // About Skypia Modal Dialog
        if app_state.show_about() {
            div {
                class: "fixed inset-0 bg-black/15 backdrop-blur-[1px] z-[9999] flex items-center justify-center p-4 pointer-events-auto",
                onclick: move |_| app_state.show_about.set(false),
                div {
                    class: "w-80 border rounded-lg shadow-2xl flex flex-col overflow-hidden pointer-events-auto",
                    style: "background: {theme.bg_chat()}; border: 1.5px solid rgba(255, 255, 255, 0.45); box-shadow: 0 10px 30px rgba(0, 0, 0, 0.25), inset 0 1px 0 rgba(255, 255, 255, 0.6);",
                    onclick: move |e| e.stop_propagation(),

                    div { class: "h-9 bg-gradient-to-r {theme.titlebar_gradient()} border-b {theme.titlebar_border()} flex items-center justify-between px-3 flex-shrink-0 select-none",
                        div { class: "flex items-center space-x-1.5 font-bold text-[11px] {theme.titlebar_text()}",
                            img {
                                src: "https://cdn.jsdelivr.net/gh/microsoft/fluentui-system-icons@main/assets/Info/SVG/ic_fluent_info_24_color.svg",
                                class: "w-5 h-5 object-contain pointer-events-none"
                            }
                            span { "Sobre o Skypia" }
                        }
                        button {
                            class: "w-[28px] h-[18px] bg-white border border-[#d1d1d1] rounded-[3px] shadow-sm flex items-center justify-center cursor-pointer transition-all hover:bg-[#e81123] hover:border-[#e81123] hover:text-white text-[#6f6f6f] hover:text-white focus:outline-none text-[8px] font-bold",
                            title: "Fechar",
                            onclick: move |_| app_state.show_about.set(false),
                            "✕"
                        }
                    }

                    div { class: "p-4 flex flex-col space-y-4 text-xs {theme.titlebar_text()} bg-white/20",
                        div { class: "flex flex-col items-center text-center space-y-2 py-2",
                            img {
                                src: "{crate::models::get_emoji_url(\"butterfly.svg\")}",
                                class: "w-10 h-10 object-contain pointer-events-none"
                            }
                            span { class: "font-bold text-sm", "Skypia Messenger v14.0" }
                            span { class: "text-[10px] text-slate-500", "Copyright © 2026 Skypia Corp. Todos os direitos reservados." }
                        }

                        p { class: "text-[11px] leading-relaxed text-slate-600 bg-white/40 p-2.5 rounded-[6px] border {theme.titlebar_border()}/30 text-center",
                            "O Skypia, é uma versão nostalgica de um app de mensagem, voltando as raizes."
                        }

                        button {
                            class: "w-full py-1.5 {theme.btn_primary()} rounded-[4px] font-bold shadow-md cursor-pointer transition-all focus:outline-none text-[10px]",
                            onclick: move |_| app_state.show_about.set(false),
                            "Ok"
                        }
                    }
                }
            }
        }

        // Modal de Perfil Pessoal
        if app_state.show_profile_modal() {
            crate::components::profile::profile_modal::ProfileModal { state: app_state }
        }

        // Modal de Jogos
        if app_state.show_games_modal() {
            crate::components::chat::games_modal::GamesModal { state: app_state }
        }

        // Modal de Perfil do Grupo
        if app_state.show_group_profile_modal() {
            crate::components::profile::group_profile_modal::GroupProfileModal { state: app_state }
        }
    }
}
