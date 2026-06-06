use dioxus::prelude::*;
use crate::state::AppState;
use crate::components::auth::login::Login;
use crate::components::main::main_window::MainWindow;
use crate::components::chat::chat_window::ChatWindow;
use crate::components::ToastList;
use crate::models::{UserStatus, AppTheme};


mod models;
mod state;
mod sound;
mod services;
mod components;

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
    
    
    // Sinais locais para controle da barra de título e opções
    let mut show_about = use_signal(|| false);
    let mut show_options_menu = use_signal(|| false);
    let mut show_theme_menu = use_signal(|| false);

    // Sincroniza decorações da janela nativa
    use_effect(move || {
        #[cfg(feature = "desktop")]
        {
            let desktop = dioxus::desktop::use_window();
            desktop.set_decorations(!app_state.use_custom_titlebar());
        }
    });

    // Ajusta o tamanho da janela principal do SO baseado no estado do chat
    use_effect(move || {
        #[cfg(feature = "desktop")]
        {
            let desktop = dioxus::desktop::use_window();
            let has_selected_chat = app_state.selected_chat_id().is_some() && app_state.chat_mode() == "integrated";
            if has_selected_chat {
                desktop.set_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(850.0, 620.0));
            } else {
                desktop.set_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(350.0, 620.0));
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


    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        
        // Main Application Screen
        div { 
            class: "w-screen h-screen overflow-hidden flex flex-col bg-gradient-to-br {theme.bg_gradient()} relative font-segoe select-none rounded-t-2xl border border-[#7baad4]/40 shadow-2xl",
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
                show_options_menu.set(false);
                show_theme_menu.set(false);
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
                    class: "w-full h-8 bg-gradient-to-b {theme.titlebar_gradient()} flex items-center justify-between z-50 flex-shrink-0 select-none border-b {theme.titlebar_border()} px-3 relative rounded-t-2xl shadow-sm cursor-default",
                    style: "-webkit-app-region: drag;",
                    onmousedown: move |_| {
                        #[cfg(feature = "desktop")]
                        let _ = dioxus::desktop::use_window().drag_window();
                    },
                    
                    // Icon and Title
                    div { class: "flex items-center space-x-1.5 font-bold text-xs pointer-events-none {theme.titlebar_text()} select-none",
                        span { class: "text-base", "👥" }
                        span { "Skypia Messenger" }
                    }
                    
                    // Controls (🎨, ☰, X)
                    div { 
                        class: "flex items-center space-x-2.5",
                        style: "-webkit-app-region: no-drag;",
                        onmousedown: move |e| e.stop_propagation(),
                        
                        // Theme Select Trigger Button
                        button { 
                            class: "w-6 h-[22px] flex items-center justify-center rounded hover:bg-black/5 text-slate-600 cursor-pointer transition-colors text-sm focus:outline-none",
                            title: "Mudar cor da skin",
                            onclick: move |e| {
                                e.stop_propagation();
                                show_theme_menu.set(!show_theme_menu());
                                show_options_menu.set(false);
                            },
                            "🎨"
                        }

                        // Options Menu Toggle Button
                        button { 
                            class: "w-6 h-[22px] flex items-center justify-center rounded hover:bg-black/5 {theme.titlebar_text()} cursor-pointer transition-colors font-bold text-sm focus:outline-none",
                            title: "Opções",
                            onclick: move |e| {
                                e.stop_propagation();
                                show_options_menu.set(!show_options_menu());
                                show_theme_menu.set(false);
                            },
                            "☰"
                        }
                        
                        // Close [X] Button (Matches user screenshot exactly)
                        button { 
                            class: "h-[22px] px-3.5 bg-white border border-[#f2d3ce] rounded font-bold text-xs text-[#8c2222] shadow-sm cursor-pointer transition-all hover:bg-[#e81123] hover:border-[#e81123] hover:text-white flex items-center justify-center focus:outline-none",
                            title: "Fechar",
                            onclick: move |e| {
                                e.stop_propagation();
                                #[cfg(feature = "desktop")]
                                dioxus::desktop::use_window().close();
                            },
                            "X"
                        }
                    }

                    // Dropdown: Theme selector
                    if show_theme_menu() {
                        div { 
                            class: "absolute right-20 top-[33px] w-40 bg-white border border-slate-300 rounded shadow-lg z-[999] p-1 flex flex-col text-xs text-slate-700 font-normal",
                            style: "-webkit-app-region: no-drag;",
                            onmousedown: move |e| e.stop_propagation(),
                            for (theme_opt, label, color_class) in &[
                                (AppTheme::AeroBlue, "Azul Aero", "bg-sky-400 border-sky-500"),
                                (AppTheme::RubyPink, "Rosa Choque", "bg-pink-400 border-pink-500"),
                                (AppTheme::ForestGreen, "Verde Natureza", "bg-emerald-400 border-emerald-500"),
                                (AppTheme::SilverMetallic, "Prata Metálico", "bg-slate-400 border-slate-500")
                            ] {
                                button { 
                                    class: "px-2 py-1.5 text-left hover:bg-sky-100 rounded transition-colors flex items-center space-x-2 cursor-pointer",
                                    onclick: move |_| {
                                        app_state.set_settings(app_state.interface_scale(), app_state.use_custom_titlebar(), *theme_opt);
                                        show_theme_menu.set(false);
                                    },
                                    div { class: "w-2.5 h-2.5 rounded {color_class} border" }
                                    span { "{label}" }
                                }
                            }
                        }
                    }

                    // Dropdown: Options Menu
                    if show_options_menu() {
                        div { 
                            class: "absolute right-12 top-[33px] w-44 bg-white border border-slate-300 rounded shadow-xl z-[999] p-1 flex flex-col text-xs text-slate-700 font-normal",
                            style: "-webkit-app-region: no-drag;",
                            onmousedown: move |e| e.stop_propagation(),
                            
                            div { class: "px-2 py-1 text-slate-400 font-bold text-[9px] uppercase tracking-wider", "Status" }
                            for status in &[UserStatus::Online, UserStatus::Ocupado, UserStatus::Ausente, UserStatus::Invisivel, UserStatus::Offline] {
                                button { 
                                    class: "px-2 py-1 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                                    onclick: move |_| {
                                        app_state.set_user_status(*status);
                                        show_options_menu.set(false);
                                    },
                                    div { class: "w-2 h-2 rounded-full {status.color_class()}" }
                                    span { "{status.as_str()}" }
                                }
                            }
                            div { class: "h-[1px] bg-slate-200 my-1" }
                            button { 
                                class: "px-2 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                                onclick: move |_| {
                                    app_state.show_music_player_modal.set(true);
                                    show_options_menu.set(false);
                                },
                                span { "🎵" }
                                span { "Definir Música..." }
                            }
                            button { 
                                class: "px-2 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                                onclick: move |_| {
                                    app_state.open_my_profile();
                                    show_options_menu.set(false);
                                },
                                span { "👤" }
                                span { "Meu Perfil..." }
                            }
                            button { 
                                class: "px-2 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                                onclick: move |_| {
                                    app_state.show_settings_modal.set(true);
                                    show_options_menu.set(false);
                                },
                                span { "⚙️" }
                                span { "Configurações..." }
                            }
                            button { 
                                class: "px-2 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                                onclick: move |_| {
                                    app_state.show_add_contact_modal.set(true);
                                    show_options_menu.set(false);
                                },
                                span { "👥" }
                                span { "Adicionar contato..." }
                            }
                            button { 
                                class: "px-2 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                                onclick: move |_| {
                                    show_about.set(true);
                                    show_options_menu.set(false);
                                },
                                span { "ℹ️" }
                                span { "Sobre o Skypia..." }
                            }
                            div { class: "h-[1px] bg-slate-200 my-1" }
                            button { 
                                class: "px-2 py-1.5 hover:bg-red-50 text-red-600 rounded text-left flex items-center space-x-2 cursor-pointer",
                                onclick: move |_| {
                                    app_state.logout();
                                    show_options_menu.set(false);
                                },
                                span { "🚪" }
                                span { "Desconectar" }
                            }
                        }
                    }
                }
            } else if logged_in {
                // Se decorações do sistema estão habilitadas, renderiza uma barra superior discreta para Acessar Menu de Opções
                div { 
                    class: "w-full h-8 bg-gradient-to-b {theme.titlebar_gradient()} flex items-center justify-between z-50 flex-shrink-0 select-none border-b {theme.titlebar_border()} px-3 relative",
                    div { class: "flex items-center space-x-1.5 font-bold text-xs {theme.titlebar_text()} pointer-events-none",
                        span { "👥" }
                        span { "Skypia Messenger" }
                    }
                    div { class: "flex items-center space-x-2.5",
                        button { 
                            class: "w-6 h-6 flex items-center justify-center rounded hover:bg-black/5 text-slate-600 cursor-pointer transition-colors text-sm focus:outline-none",
                            title: "Mudar cor da skin",
                            onclick: move |e| {
                                e.stop_propagation();
                                show_theme_menu.set(!show_theme_menu());
                                show_options_menu.set(false);
                            },
                            "🎨"
                        }
                        button { 
                            class: "w-6 h-6 flex items-center justify-center rounded hover:bg-black/5 {theme.titlebar_text()} cursor-pointer transition-colors font-bold text-sm focus:outline-none",
                            title: "Opções",
                            onclick: move |e| {
                                e.stop_propagation();
                                show_options_menu.set(!show_options_menu());
                                show_theme_menu.set(false);
                            },
                            "☰"
                        }
                    }

                    // Dropdowns em decorações nativas
                    if show_theme_menu() {
                        div { 
                            class: "absolute right-12 top-8 w-40 bg-white border border-slate-300 rounded shadow-lg z-[999] p-1 flex flex-col text-xs text-slate-700 font-normal",
                            onmousedown: move |e| e.stop_propagation(),
                            for (theme_opt, label, color_class) in &[
                                (AppTheme::AeroBlue, "Azul Aero", "bg-sky-400 border-sky-500"),
                                (AppTheme::RubyPink, "Rosa Choque", "bg-pink-400 border-pink-500"),
                                (AppTheme::ForestGreen, "Verde Natureza", "bg-emerald-400 border-emerald-500"),
                                (AppTheme::SilverMetallic, "Prata Metálico", "bg-slate-400 border-slate-500")
                            ] {
                                button { 
                                    class: "px-2 py-1.5 text-left hover:bg-sky-100 rounded transition-colors flex items-center space-x-2 cursor-pointer",
                                    onclick: move |_| {
                                        app_state.set_settings(app_state.interface_scale(), app_state.use_custom_titlebar(), *theme_opt);
                                        show_theme_menu.set(false);
                                    },
                                    div { class: "w-2.5 h-2.5 rounded {color_class} border" }
                                    span { "{label}" }
                                }
                            }
                        }
                    }

                    if show_options_menu() {
                        div { 
                            class: "absolute right-4 top-8 w-44 bg-white border border-slate-300 rounded shadow-xl z-[999] p-1 flex flex-col text-xs text-slate-700 font-normal",
                            onmousedown: move |e| e.stop_propagation(),
                            
                            div { class: "px-2 py-1 text-slate-400 font-bold text-[9px] uppercase tracking-wider", "Status" }
                            for status in &[UserStatus::Online, UserStatus::Ocupado, UserStatus::Ausente, UserStatus::Invisivel, UserStatus::Offline] {
                                button { 
                                    class: "px-2 py-1 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                                    onclick: move |_| {
                                        app_state.set_user_status(*status);
                                        show_options_menu.set(false);
                                    },
                                    div { class: "w-2 h-2 rounded-full {status.color_class()}" }
                                    span { "{status.as_str()}" }
                                }
                            }
                            div { class: "h-[1px] bg-slate-200 my-1" }
                            button { 
                                class: "px-2 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                                onclick: move |_| {
                                    app_state.show_music_player_modal.set(true);
                                    show_options_menu.set(false);
                                },
                                span { "🎵" }
                                span { "Definir Música..." }
                            }
                            button { 
                                class: "px-2 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                                onclick: move |_| {
                                    app_state.open_my_profile();
                                    show_options_menu.set(false);
                                },
                                span { "👤" }
                                span { "Meu Perfil..." }
                            }
                            button { 
                                class: "px-2 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                                onclick: move |_| {
                                    app_state.show_settings_modal.set(true);
                                    show_options_menu.set(false);
                                },
                                span { "⚙️" }
                                span { "Configurações..." }
                            }
                            button { 
                                class: "px-2 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                                onclick: move |_| {
                                    app_state.show_add_contact_modal.set(true);
                                    show_options_menu.set(false);
                                },
                                span { "👥" }
                                span { "Adicionar contato..." }
                            }
                            button { 
                                class: "px-2 py-1.5 hover:bg-sky-100 rounded text-left flex items-center space-x-2 cursor-pointer",
                                onclick: move |_| {
                                    show_about.set(true);
                                    show_options_menu.set(false);
                                },
                                span { "ℹ️" }
                                span { "Sobre o Skypia..." }
                            }
                            div { class: "h-[1px] bg-slate-200 my-1" }
                            button { 
                                class: "px-2 py-1.5 hover:bg-red-50 text-red-600 rounded text-left flex items-center space-x-2 cursor-pointer",
                                onclick: move |_| {
                                    app_state.logout();
                                    show_options_menu.set(false);
                                },
                                span { "🚪" }
                                span { "Desconectar" }
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
                                    "hidden md:flex w-[350px] h-full flex-col flex-shrink-0 border-r border-[#7baad4]/30"
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

        // Modal de Solicitação de Amizade Pendente (MSN Style)
        {
            if logged_in && !app_state.pending_requests().is_empty() {
                let pending_list = app_state.pending_requests();
                let first_req = pending_list[0].clone();
                let first_req_id_accept = first_req.id.clone();
                let first_req_id_reject = first_req.id.clone();
                
                rsx! {
                    div { 
                        class: "fixed inset-0 bg-black/45 backdrop-blur-sm z-[9998] flex items-center justify-center p-4 pointer-events-auto",
                        div { 
                            class: "w-[360px] bg-gradient-to-b from-[#f2f7fc] to-[#d8e8f7] border-2 border-[#5c98d6] rounded shadow-2xl p-4 flex flex-col space-y-4 text-xs text-[#1e395b] pointer-events-auto",
                            
                            // Cabeçalho clássico
                            div { class: "flex items-center justify-between border-b border-[#a8c9eb] pb-2",
                                span { class: "font-bold text-sm flex items-center space-x-1.5",
                                    span { "👤" }
                                    span { "Solicitação de Amizade" }
                                }
                            }
                            
                            // Conteúdo
                            div { class: "flex flex-col space-y-3 py-1",
                                p { class: "font-semibold text-slate-700",
                                    "{first_req.display_name} ({first_req.email}) deseja adicionar você à lista de contatos."
                                }
                                
                                div { class: "bg-white/60 border border-[#a8c9eb] p-3 rounded text-[11px] leading-relaxed text-slate-600 space-y-2",
                                    p { "Ao aceitar, você poderá ver o status dele, trocar mensagens em tempo real e compartilhar winks e nudges!" }
                                }
                            }
                            
                            // Botões de Ação
                            div { class: "flex items-center justify-end space-x-2 pt-2 border-t border-[#a8c9eb]/50",
                                button { 
                                    class: "px-4 py-1.5 bg-gradient-to-b from-emerald-400 to-emerald-500 hover:from-emerald-500 hover:to-emerald-600 text-white rounded font-bold shadow-md cursor-pointer transition-all focus:outline-none",
                                    onclick: move |_| {
                                        app_state.accept_friend_request(first_req_id_accept.clone());
                                    },
                                    "Aceitar"
                                }
                                button { 
                                    class: "px-4 py-1.5 bg-gradient-to-b from-rose-400 to-rose-500 hover:from-rose-500 hover:to-rose-600 text-white rounded font-bold shadow-md cursor-pointer transition-all focus:outline-none",
                                    onclick: move |_| {
                                        app_state.reject_friend_request(first_req_id_reject.clone());
                                    },
                                    "Recusar"
                                }
                            }
                        }
                    }
                }
            } else {
                rsx! {}
            }
        }
        
        // About Skypia Modal Dialog
        if show_about() {
            div { 
                class: "fixed inset-0 bg-black/45 backdrop-blur-sm z-[9999] flex items-center justify-center p-4 pointer-events-auto",
                onclick: move |_| show_about.set(false),
                div { 
                    class: "w-80 bg-gradient-to-b from-[#e6f1fc] to-[#c8def5] border border-[#7ba9d4] rounded-lg shadow-2xl p-4 flex flex-col space-y-4 text-xs text-[#1e395b] pointer-events-auto",
                    onclick: move |e| e.stop_propagation(),
                    
                    div { class: "flex items-center justify-between border-b border-white/40 pb-2",
                        span { class: "font-bold text-sm", "ℹ️ Sobre o Skypia" }
                        button { 
                            class: "w-5 h-5 flex items-center justify-center rounded hover:bg-red-500 hover:text-white border border-transparent font-bold cursor-pointer transition-colors",
                            onclick: move |_| show_about.set(false),
                            "✕"
                        }
                    }
                    
                    div { class: "flex flex-col items-center text-center space-y-2 py-2",
                        span { class: "text-3xl", "🦋" }
                        span { class: "font-bold text-sm", "Skypia Messenger v14.0" }
                        span { class: "text-[10px] text-slate-500", "Copyright © 2026 Skypia Corp. Todos os direitos reservados." }
                    }
                    
                    p { class: "text-[11px] leading-relaxed text-slate-600 bg-white/40 p-2.5 rounded border border-white/30 text-center",
                        "O Skypia é o clone definitivo do MSN Messenger, recriado em Rust com Dioxus 0.7 e TailwindCSS para uma experiência premium de alta fidelidade visual Aero Glass."
                    }
                    
                    button { 
                        class: "w-full py-1.5 bg-gradient-to-b from-sky-400 to-sky-500 hover:from-sky-500 hover:to-sky-600 text-white rounded font-bold shadow-md cursor-pointer transition-all",
                        onclick: move |_| show_about.set(false),
                        "Ok"
                    }
                }
            }
        }

        // Modal de Perfil Pessoal
        if app_state.show_profile_modal() {
            crate::components::profile::profile_modal::ProfileModal { state: app_state }
        }
    }
}
