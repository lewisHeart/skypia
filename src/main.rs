use dioxus::prelude::*;
use crate::state::AppState;
use crate::components::login::Login;
use crate::components::main_window::MainWindow;
use crate::components::chat_window::ChatWindow;
use crate::components::ToastList;
use crate::models::{UserStatus, AppTheme};
use crate::sound::play_sound;

mod models;
mod state;
mod sound;
mod services;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(feature = "desktop")]
    {
        dioxus::LaunchBuilder::desktop()
            .with_cfg(dioxus::desktop::Config::new().with_menu(None))
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
    let use_custom_bar = app_state.use_custom_titlebar();
    
    // Sinais locais para controle da barra de título e opções
    let mut show_about = use_signal(|| false);
    let mut show_options_menu = use_signal(|| false);
    let mut show_theme_menu = use_signal(|| false);

    // Sincroniza decorações da janela nativa
    use_effect(move || {
        #[cfg(feature = "desktop")]
        {
            let desktop = dioxus::desktop::use_window();
            desktop.set_decorations(!use_custom_bar);
        }
    });

    // Carregamento dinâmico de dados quando logado e sincronização periódica do banco compartilhado (acelerado para 200ms)
    use_effect(move || {
        if logged_in {
            let mut state = app_state;
            state.load_initial_data();
            
            // Loop periódico ultra rápido de 200ms para verificar atualizações no banco compartilhado
            spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                    
                    // 1. Sincroniza chats destacados
                    state.sync_detached_chats();
                    
                    // 2. Sincroniza contatos se mudou
                    if let Ok(contacts) = crate::services::db::DatabaseService::load_contacts().await {
                        let current = state.contacts.read().clone();
                        if current != contacts {
                            *state.contacts.write() = contacts;
                        }
                    }
                    
                    // 3. Sincroniza mensagens do chat ativo atual se houver e se mudou
                    if let Some(selected_id) = state.selected_chat_id() {
                        if let Ok(msgs) = crate::services::db::DatabaseService::load_messages(selected_id).await {
                            let current = state.chat_messages.read().get(&selected_id).cloned().unwrap_or_default();
                            if current != msgs {
                                let mut chat_msgs = state.chat_messages.write();
                                chat_msgs.insert(selected_id, msgs);
                            }
                        }
                    }
                }
            });
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        
        // Main Application Screen
        div { 
            class: "w-screen h-screen overflow-hidden flex flex-col bg-gradient-to-br {theme.bg_gradient()} relative font-segoe select-none rounded-t-2xl border border-[#7baad4]/40 shadow-2xl",
            onclick: move |_| {
                show_options_menu.set(false);
                show_theme_menu.set(false);
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
                            for status in &[UserStatus::Online, UserStatus::Ocupado, UserStatus::Ausente, UserStatus::Offline] {
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
                                    *app_state.logged_in.write() = false;
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
                            for status in &[UserStatus::Online, UserStatus::Ocupado, UserStatus::Ausente, UserStatus::Offline] {
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
                                    *app_state.logged_in.write() = false;
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
                                let has_selected_chat = app_state.selected_chat_id().is_some();
                                let sidebar_class = if has_selected_chat {
                                    "hidden md:flex w-[350px] h-full flex-col flex-shrink-0 border-r border-[#7baad4]/30"
                                } else {
                                    "w-full md:w-[350px] h-full flex flex-col flex-shrink-0 border-r border-[#7baad4]/30"
                                };
                                let chat_container_class = if has_selected_chat {
                                    "flex-1 h-full flex flex-col min-w-0"
                                } else {
                                    "hidden md:flex flex-1 h-full flex flex-col min-w-0"
                                };
                                
                                rsx! {
                                    div { class: "w-full flex-1 min-h-0 flex flex-row pointer-events-auto z-10 h-full",
                                        div { class: sidebar_class,
                                            MainWindow { state: app_state }
                                        }
                                        div { class: chat_container_class,
                                            if has_selected_chat {
                                                ChatWindow { state: app_state, contact_id_prop: None }
                                            } else {
                                                // Welcome/Placeholder Panel
                                                div { 
                                                    class: "flex-1 h-full flex flex-col items-center justify-center text-center p-8 bg-white/20",
                                                    svg { view_box: "0 0 100 100", class: "w-28 h-28 mb-6 filter drop-shadow-md opacity-75 animate-pulse",
                                                        g { fill: "#00adef",
                                                            circle { cx: "38", cy: "35", r: "15" }
                                                            path { d: "M18 75 C18 55, 58 55, 58 75 Z" }
                                                        }
                                                        g { fill: "#7cc576",
                                                            circle { cx: "62", cy: "45", r: "13" }
                                                            path { d: "M45 80 C45 62, 79 62, 79 80 Z" }
                                                        }
                                                    }
                                                    h2 { class: "text-base font-bold text-[#1e395b]/80 mb-1.5", "Skypia Messenger" }
                                                    p { class: "text-xs text-slate-500 max-w-[280px]", "Dê um clique duplo em qualquer contato da lista para iniciar uma conversa." }
                                                }
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
    }
}
