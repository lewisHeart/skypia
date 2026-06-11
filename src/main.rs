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
            let has_selected_chat =
                app_state.selected_chat_id().is_some() && app_state.chat_mode() == "integrated";
            if has_selected_chat {
                desktop.set_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(886.0, 735.0));
            } else {
                desktop.set_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(373.0, 735.0));
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
                                    "hidden md:flex w-[220px] h-full flex-col flex-shrink-0"
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
                        class: "fixed inset-0 bg-black/10 z-[9998] flex items-center justify-center p-4 pointer-events-auto",
                        div {
                            class: "w-[360px] bg-gradient-to-b {theme.modal_gradient()} border-2 {theme.modal_border()} rounded shadow-2xl p-4 flex flex-col space-y-4 text-xs {theme.titlebar_text()} pointer-events-auto",

                            // Cabeçalho clássico
                            div { class: "flex items-center justify-between border-b {theme.titlebar_border()} pb-2",
                                span { class: "font-bold text-sm flex items-center space-x-1.5 {theme.titlebar_text()}",
                                    span { "👤" }
                                    span { "Solicitação de Amizade" }
                                }
                            }

                            // Conteúdo
                            div { class: "flex flex-col space-y-3 py-1",
                                p { class: "font-semibold text-slate-700",
                                    "{first_req.display_name} ({first_req.email}) deseja adicionar você à lista de contatos."
                                }

                                div { class: "bg-white/60 border {theme.titlebar_border()} p-3 rounded text-[11px] leading-relaxed text-slate-600 space-y-2",
                                    p { "Ao aceitar, você poderá ver o status dele, trocar mensagens em tempo real e compartilhar winks e nudges!" }
                                }
                            }

                            // Botões de Ação
                            div { class: "flex items-center justify-end space-x-2 pt-2 border-t {theme.titlebar_border()}/50",
                                button {
                                    class: "px-4 py-1.5 {theme.btn_primary()} rounded font-bold shadow-md cursor-pointer transition-all focus:outline-none",
                                    onclick: move |_| {
                                        app_state.accept_friend_request(first_req_id_accept.clone());
                                    },
                                    "Aceitar"
                                }
                                button {
                                    class: "px-4 py-1.5 bg-white hover:bg-slate-100 border border-slate-350 text-slate-700 rounded font-bold shadow-md cursor-pointer transition-all focus:outline-none",
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
                                src: "https://registry.npmmirror.com/@lobehub/assets-emoji/latest/files/assets/butterfly.webp",
                                class: "w-10 h-10 object-contain pointer-events-none"
                            }
                            span { class: "font-bold text-sm", "Skypia Messenger v14.0" }
                            span { class: "text-[10px] text-slate-500", "Copyright © 2026 Skypia Corp. Todos os direitos reservados." }
                        }

                        p { class: "text-[11px] leading-relaxed text-slate-600 bg-white/40 p-2.5 rounded-[6px] border {theme.titlebar_border()}/30 text-center",
                            "O Skypia é o clone definitivo do MSN Messenger, recriado em Rust com Dioxus 0.7 e TailwindCSS para uma experiência premium de alta fidelidade visual Aero Glass."
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
    }
}
