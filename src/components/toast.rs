use crate::models::render_avatar;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn ToastList(mut state: AppState) -> Element {
    let toasts = state.toasts();

    rsx! {
        div {
            class: "fixed bottom-4 right-4 z-[9999] flex flex-col space-y-2.5 max-w-[280px] w-full pointer-events-none select-none",
            for toast in toasts {
                ToastCard { toast, state }
            }
        }
    }
}

#[component]
fn ToastCard(toast: crate::state::Toast, mut state: AppState) -> Element {
    let toast_id = toast.id;

    // Self-destruct effect: removes the toast after 4 seconds
    use_effect(move || {
        let mut app_state = state;
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(4)).await;
            app_state.remove_toast(toast_id);
        });
    });

    rsx! {
        div {
            class: "w-full p-3 rounded-lg border border-[#7baad4] shadow-xl flex items-center space-x-3 toast-in pointer-events-auto cursor-pointer select-none bg-sky-50/95 hover:bg-sky-100/95 transition-colors",
            style: "background: linear-gradient(135deg, rgba(240, 248, 255, 0.95) 0%, rgba(215, 235, 252, 0.95) 100%);",
            onclick: move |_| {
                state.remove_toast(toast_id);
            },

            // Avatar
            div { class: "w-9 h-9 rounded border border-slate-300 overflow-hidden flex-shrink-0 bg-white shadow-sm",
                {render_avatar(toast.avatar_url.as_deref(), 36)}
            }

            // Content
            div { class: "flex-1 min-w-0 flex flex-col space-y-0.5",
                span { class: "font-bold text-xs text-[#1e395b] truncate", "{toast.title}" }
                span { class: "text-[10px] text-slate-600 truncate", "{toast.message}" }
            }

            // Close button
            button {
                class: "text-slate-400 hover:text-slate-600 font-bold text-xs pr-1 focus:outline-none flex-shrink-0",
                onclick: move |e| {
                    e.stop_propagation();
                    state.remove_toast(toast_id);
                },
                "×"
            }
        }
    }
}
