use crate::models::render_avatar;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn GroupProfileModal(mut state: AppState) -> Element {
    let theme = state.theme();
    let group_id = state.group_profile_id();
    
    let group_opt = group_id.as_ref().and_then(|gid| {
        state.group_chats().iter().find(|g| g.id == *gid).cloned()
    });

    let group = match group_opt {
        Some(g) => g,
        None => {
            state.show_group_profile_modal.set(false);
            return rsx! {};
        }
    };

    // Verificar se o usuário local é admin
    let local_user_id = state.server_user_id().clone().unwrap_or_default();
    let is_admin = group.members.iter().any(|m| {
        m.id == local_user_id && m.role.as_deref() == Some("admin")
    });

    // Sinais locais para edição (somente admin)
    let mut editing = use_signal(|| false);
    let mut temp_name = use_signal(|| group.name.clone().unwrap_or_default());
    let mut temp_desc = use_signal(|| group.description.clone().unwrap_or_default());
    let mut temp_allow_send = use_signal(|| group.allow_member_send.unwrap_or(true));
    let mut temp_allow_invite = use_signal(|| group.allow_member_invite.unwrap_or(true));

    let member_count = group.members.len();
    let online_count = group.members.iter().filter(|m| m.status != "offline").count();
    let group_name = group.name.clone().unwrap_or("Grupo sem nome".to_string());
    let group_desc = group.description.clone().unwrap_or_default();
    let group_created = group.created_at.clone();

    rsx! {
        div {
            class: "fixed inset-0 bg-black/15 backdrop-blur-[1px] z-[200] flex items-center justify-center p-4 pointer-events-auto",
            onclick: move |_| state.show_group_profile_modal.set(false),

            div {
                class: "w-[420px] max-h-[85vh] border rounded-lg shadow-2xl flex flex-col overflow-hidden pointer-events-auto",
                style: "background: {theme.bg_chat()}; border: 1.5px solid rgba(255, 255, 255, 0.45); box-shadow: 0 10px 30px rgba(0, 0, 0, 0.25), inset 0 1px 0 rgba(255, 255, 255, 0.6);",
                onclick: move |e| e.stop_propagation(),

                // Barra de título
                div { class: "h-9 bg-gradient-to-r {theme.titlebar_gradient()} border-b {theme.titlebar_border()} flex items-center justify-between px-3 flex-shrink-0 select-none",
                    div { class: "flex items-center space-x-1.5 font-bold text-[11px] {theme.titlebar_text()} truncate",
                        span { "👥" }
                        span { "Perfil do grupo" }
                    }
                    button {
                        class: "w-[28px] h-[18px] bg-white border border-[#d1d1d1] rounded-[3px] shadow-sm flex items-center justify-center cursor-pointer transition-all hover:bg-[#e81123] hover:border-[#e81123] hover:text-white text-[#6f6f6f] focus:outline-none text-[8px] font-bold",
                        title: "Fechar",
                        onclick: move |_| state.show_group_profile_modal.set(false),
                        "✕"
                    }
                }

                // Conteúdo
                div { class: "flex-1 overflow-y-auto scrollbar-thin scrollbar-thumb-slate-300",

                    // Cabeçalho do grupo
                    div { class: "p-4 flex flex-col space-y-3 text-xs {theme.titlebar_text()}",

                        div { class: "flex items-start space-x-3 bg-white/40 p-3 rounded-xl border border-white/50 shadow-sm",
                            // Avatar do grupo
                            div {
                                class: "w-14 h-14 rounded-lg bg-gradient-to-br from-sky-400 to-blue-500 flex items-center justify-center text-white font-bold text-xl shadow-md flex-shrink-0",
                                if let Some(ref url) = group.avatar_url {
                                    img { src: "{url}", class: "w-14 h-14 rounded-lg object-cover" }
                                } else {
                                    span { "👥" }
                                }
                            }

                            // Info do grupo
                            div { class: "flex-1 min-w-0",
                                if editing() && is_admin {
                                    input {
                                        class: "w-full px-2 py-1 text-sm border border-slate-300 rounded bg-white/80 focus:outline-none focus:border-sky-400 font-semibold {theme.titlebar_text()}",
                                        value: temp_name(),
                                        oninput: move |e| temp_name.set(e.value()),
                                    }
                                } else {
                                    p { class: "text-sm font-semibold truncate", "{group_name}" }
                                }
                                p { class: "text-[10px] text-slate-500 mt-0.5",
                                    "{member_count} membros · {online_count} online"
                                }
                                if !group_desc.is_empty() && !editing() {
                                    p { class: "text-[10px] text-slate-500 mt-1 leading-relaxed",
                                        "{group_desc}"
                                    }
                                }
                            }
                        }

                        // Edição de descrição (somente admin)
                        if editing() && is_admin {
                            div { class: "space-y-2",
                                label { class: "text-[10px] font-semibold text-slate-500 uppercase tracking-wide",
                                    "Descrição do grupo"
                                }
                                textarea {
                                    class: "w-full px-2 py-1.5 text-[11px] border border-slate-300 rounded bg-white/80 focus:outline-none focus:border-sky-400 resize-none {theme.titlebar_text()}",
                                    rows: 3,
                                    value: temp_desc(),
                                    oninput: move |e| temp_desc.set(e.value()),
                                    placeholder: "Descrição do grupo..."
                                }
                            }
                        }

                        // Permissões (somente admin em modo edição)
                        if editing() && is_admin {
                            div { class: "space-y-2 bg-white/30 p-3 rounded-xl border border-white/40",
                                p { class: "text-[10px] font-semibold text-slate-500 uppercase tracking-wide mb-1",
                                    "Permissões"
                                }
                                label { class: "flex items-center space-x-2 cursor-pointer",
                                    input {
                                        r#type: "checkbox",
                                        class: "accent-sky-500",
                                        checked: temp_allow_send(),
                                        onchange: move |e: Event<FormData>| {
                                            temp_allow_send.set(e.checked());
                                        }
                                    }
                                    span { class: "text-[11px]", "Membros podem enviar mensagens" }
                                }
                                label { class: "flex items-center space-x-2 cursor-pointer",
                                    input {
                                        r#type: "checkbox",
                                        class: "accent-sky-500",
                                        checked: temp_allow_invite(),
                                        onchange: move |e: Event<FormData>| {
                                            temp_allow_invite.set(e.checked());
                                        }
                                    }
                                    span { class: "text-[11px]", "Membros podem convidar pessoas" }
                                }
                            }
                        }

                        // Botões de ação (admin)
                        if is_admin {
                            div { class: "flex items-center space-x-2",
                                if editing() {
                                    button {
                                        class: "flex-1 px-3 py-1.5 text-[10px] font-semibold bg-sky-500 text-white rounded hover:bg-sky-600 transition-colors cursor-pointer",
                                        onclick: {
                                            let gid = group.id.clone();
                                            move |_| {
                                                editing.set(false);
                                                state.update_group_info(
                                                    gid.clone(),
                                                    temp_name(),
                                                    temp_desc(),
                                                    group.avatar_url.clone(),
                                                );
                                                state.update_group_permissions(
                                                    gid.clone(),
                                                    temp_allow_send(),
                                                    temp_allow_invite(),
                                                );
                                            }
                                        },
                                        "Salvar"
                                    }
                                    button {
                                        class: "flex-1 px-3 py-1.5 text-[10px] font-semibold bg-slate-200 text-slate-700 rounded hover:bg-slate-300 transition-colors cursor-pointer",
                                        onclick: move |_| editing.set(false),
                                        "Cancelar"
                                    }
                                } else {
                                    button {
                                        class: "px-3 py-1.5 text-[10px] font-semibold bg-white/60 border border-slate-300 text-slate-700 rounded hover:bg-white/80 transition-colors cursor-pointer flex items-center space-x-1",
                                        onclick: move |_| editing.set(true),
                                        span { "✏️" }
                                        span { "Editar grupo" }
                                    }
                                }
                            }
                        }
                    }

                    // Divisor
                    div { class: "h-[1px] bg-gradient-to-r from-transparent via-slate-300/60 to-transparent mx-4" }

                    // Lista de membros
                    div { class: "p-4 space-y-2",
                        p { class: "text-[10px] font-semibold text-slate-500 uppercase tracking-wide mb-2 {theme.titlebar_text()}",
                            "Membros ({member_count})"
                        }

                        for member in &group.members {
                            {render_member_row(state, &member, is_admin, &local_user_id, group.id.clone())}
                        }
                    }

                    // Info de criação
                    div { class: "px-4 pb-4",
                        p { class: "text-[9px] text-slate-400 text-center",
                            "Grupo criado em {format_date(&group_created)}"
                        }
                    }
                }
            }
        }
    }
}

fn render_member_row(
    mut state: AppState,
    member: &crate::models::UserProfile,
    is_admin: bool,
    local_user_id: &str,
    group_id: String,
) -> Element {
    let theme = state.theme();
    let is_self = member.id == local_user_id;
    let role = member.role.as_deref().unwrap_or("member");
    let status_class = match member.status.as_str() {
        "online" => "bg-green-500",
        "busy" => "bg-red-500",
        "away" => "bg-amber-500",
        _ => "bg-slate-400",
    };

    rsx! {
        div {
            class: "flex items-center space-x-2 p-2 rounded-lg hover:bg-white/40 transition-colors group",

            // Avatar com indicador de status
            div { class: "relative flex-shrink-0",
                div {
                    class: "w-8 h-8 rounded-md overflow-hidden border border-white/40 bg-white flex items-center justify-center",
                    {render_avatar(member.avatar_url.as_deref(), 32)}
                }
                div {
                    class: "absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-[1.5px] border-white {status_class}",
                }
            }

            // Nome e papel
            div { class: "flex-1 min-w-0",
                div { class: "flex items-center space-x-1",
                    span { class: "text-[11px] font-medium truncate {theme.titlebar_text()}",
                        "{member.display_name}"
                    }
                    if is_self {
                        span { class: "text-[8px] px-1 py-0.5 bg-sky-100 text-sky-600 rounded font-semibold",
                            "Você"
                        }
                    }
                    if role == "admin" {
                        span { class: "text-[8px] px-1 py-0.5 bg-amber-100 text-amber-700 rounded font-semibold",
                            "Admin"
                        }
                    }
                }
                if !member.personal_message.is_empty() {
                    p { class: "text-[9px] text-slate-400 truncate",
                        "{member.personal_message}"
                    }
                }
            }

            // Ações (admin pode remover membros que não são ele)
            if is_admin && !is_self {
                {
                    let gid = group_id.clone();
                    let uid = member.id.clone();
                    rsx! {
                        div { class: "opacity-0 group-hover:opacity-100 transition-opacity",
                            button {
                                class: "text-[9px] px-1.5 py-0.5 text-red-500 hover:bg-red-50 rounded transition-colors cursor-pointer",
                                title: "Remover do grupo",
                                onclick: move |_| {
                                    state.remove_group_member(gid.clone(), uid.clone());
                                },
                                "✕"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_date(date_str: &str) -> String {
    // Formata ISO date para dd/mm/yyyy
    if date_str.len() >= 10 {
        let parts: Vec<&str> = date_str[..10].split('-').collect();
        if parts.len() == 3 {
            return format!("{}/{}/{}", parts[2], parts[1], parts[0]);
        }
    }
    date_str.to_string()
}
