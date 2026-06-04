use dioxus::prelude::*;
use crate::state::AppState;
use crate::components::login::Login;
use crate::components::main_window::MainWindow;
use crate::components::chat_window::ChatWindow;
use crate::components::ToastList;

mod models;
mod state;
mod sound;
mod services;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize AppState as a global context
    let app_state = use_context_provider(|| AppState::new());
    let logged_in = app_state.logged_in();
    let theme = app_state.theme();

    // Carregamento dinâmico de dados quando logado e sincronização de janelas nativas
    use_effect(move || {
        if logged_in {
            let mut state = app_state;
            state.load_initial_data();
            
            // Loop periódico para verificar se as janelas de chat destacadas foram fechadas ou reatacadas
            spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    state.sync_detached_chats();
                }
            });
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        
        // Main Application Screen (Focuses only on MSN client)
        div { 
            class: "w-screen h-screen overflow-hidden flex bg-gradient-to-br {theme.bg_gradient()} relative font-segoe select-none",
            
            // Subtle theme background bubbles
            div { class: "absolute inset-0 bg-bubbles pointer-events-none opacity-25 z-0" }

            // Active Toast Alerts floating layer
            ToastList { state: app_state }

            if !logged_in {
                // Login page occupies the whole screen (centers form card inside itself)
                Login { state: app_state }
            } else {
                // Full Screen split-pane layout
                {
                    let has_selected_chat = app_state.selected_chat_id().is_some();
                    let sidebar_class = if has_selected_chat {
                        "hidden md:block w-[280px] h-full flex-shrink-0 border-r border-[#7baad4]/30"
                    } else {
                        "w-full md:w-[280px] h-full flex-shrink-0 border-r border-[#7baad4]/30"
                    };
                    let chat_container_class = if has_selected_chat {
                        "flex-1 h-full flex flex-col min-w-0"
                    } else {
                        "hidden md:flex flex-1 h-full flex flex-col min-w-0"
                    };
                    
                    rsx! {
                        div { class: "w-full h-full flex flex-row pointer-events-auto z-10",
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
                                        h2 { class: "text-base font-bold text-[#1e395b]/80 mb-1.5", "Windows Live Messenger" }
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
