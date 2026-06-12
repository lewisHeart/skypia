/// Serviço HTTP para comunicação com o skypia-serve (Actix-web)
use serde::{Deserialize, Serialize};

use std::fmt;
use std::ops::Deref;
use std::sync::LazyLock;

pub struct ServerBaseUrl;

impl Deref for ServerBaseUrl {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &SERVER_BASE_URL_INNER
    }
}

impl fmt::Display for ServerBaseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *SERVER_BASE_URL_INNER)
    }
}

static SERVER_BASE_URL_INNER: LazyLock<String> = LazyLock::new(|| {
    if let Ok(url) = std::env::var("SERVER_BASE_URL") {
        return url;
    }
    if let Some(url) = option_env!("SERVER_BASE_URL") {
        return url.to_string();
    }
    "http://192.168.1.16:8082".to_string()
});

pub static SERVER_BASE_URL: ServerBaseUrl = ServerBaseUrl;

pub static SPOTIFY_CHECK_INTERVAL: LazyLock<u64> = LazyLock::new(|| {
    std::env::var("SPOTIFY_CHECK_INTERVAL")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3)
});

#[allow(dead_code)]
pub static WS_HEARTBEAT_INTERVAL: LazyLock<u64> = LazyLock::new(|| {
    std::env::var("WS_HEARTBEAT_INTERVAL")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(30)
});

// ── Structs de request/response espelhadas do servidor ────────────────────

pub use crate::models::UserProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserProfile,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub full_name: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music: Option<Option<String>>,
}

// ── API Client ────────────────────────────────────────────────────────────

/// Registra um novo usuário
pub async fn register(
    email: String,
    username: String,
    full_name: String,
    password: String,
    display_name: String,
) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();
    let req = RegisterRequest {
        email,
        username,
        full_name,
        password,
        display_name,
    };

    let resp = client
        .post(format!("{}/auth/register", SERVER_BASE_URL))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<AuthResponse>(&body)
            .map_err(|e| format!("Erro ao parsear resposta: {}", e))
    } else {
        // Tenta extrair mensagem de erro
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("Erro do servidor ({})", status));
        Err(msg)
    }
}

/// Faz login com email e senha
pub async fn login(email: String, password: String) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();
    let req = LoginRequest { email, password };

    let resp = client
        .post(format!("{}/auth/login", SERVER_BASE_URL))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<AuthResponse>(&body)
            .map_err(|e| format!("Erro ao parsear resposta: {}", e))
    } else {
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "Email ou senha inválidos.".to_string());
        Err(msg)
    }
}

/// Carrega o perfil do usuário autenticado
pub async fn get_profile(token: &str) -> Result<UserProfile, String> {
    let client = reqwest::Client::new();

    let resp = client
        .get(format!("{}/me", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<UserProfile>(&body)
            .map_err(|e| format!("Erro ao parsear perfil: {}", e))
    } else {
        Err(format!("Não autenticado ({})", status))
    }
}

/// Atualiza campos do perfil
pub async fn update_profile(token: &str, req: UpdateProfileRequest) -> Result<UserProfile, String> {
    let client = reqwest::Client::new();

    let resp = client
        .put(format!("{}/me", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<UserProfile>(&body)
            .map_err(|e| format!("Erro ao parsear perfil: {}", e))
    } else {
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "Erro ao atualizar perfil.".to_string());
        Err(msg)
    }
}

/// Faz upload do avatar (bytes crus da imagem) e retorna a URL pública
pub async fn upload_avatar(
    token: &str,
    image_bytes: Vec<u8>,
    mime_type: &str,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    let part = reqwest::multipart::Part::bytes(image_bytes)
        .file_name("avatar.jpg")
        .mime_str(mime_type)
        .map_err(|e| e.to_string())?;

    let form = reqwest::multipart::Form::new().part("avatar", part);

    let resp = client
        .post(format!("{}/me/avatar", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        let val: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;
        val["avatar_url"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Campo avatar_url ausente na resposta.".to_string())
    } else {
        Err(format!("Erro no upload ({}): {}", status, body))
    }
}

/// Faz upload genérico de um arquivo (imagem, áudio, arquivo) e retorna os metadados (url e filepath)
pub async fn upload_generic_file(
    token: &str,
    file_bytes: Vec<u8>,
    filename: &str,
    mime_type: &str,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();

    let part = reqwest::multipart::Part::bytes(file_bytes)
        .file_name(filename.to_string())
        .mime_str(mime_type)
        .map_err(|e| e.to_string())?;

    let form = reqwest::multipart::Form::new().part("file", part);

    let resp = client
        .post(format!("{}/upload", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str(&body).map_err(|e| e.to_string())
    } else {
        Err(format!("Erro no upload ({}): {}", status, body))
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct LinkPreview {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    #[allow(dead_code)]
    pub url: String,
}

/// Fetch link preview from backend
pub async fn fetch_link_preview(token: &str, url: &str) -> Result<LinkPreview, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/preview", SERVER_BASE_URL))
        .query(&[("url", url)])
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str(&body).map_err(|e| e.to_string())
    } else {
        Err(format!("Erro ao buscar preview ({}): {}", status, body))
    }
}

/// Carrega a lista de contatos do servidor
pub async fn get_contacts(token: &str) -> Result<Vec<UserProfile>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/contacts", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<Vec<UserProfile>>(&body)
            .map_err(|e| format!("Erro ao parsear contatos: {}", e))
    } else {
        Err(format!("Erro ao carregar contatos ({})", status))
    }
}

/// Carrega a lista de conversas do servidor
pub async fn get_conversations(token: &str) -> Result<Vec<crate::models::Conversation>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/conversations", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<Vec<crate::models::Conversation>>(&body)
            .map_err(|e| format!("Erro ao parsear conversas: {}", e))
    } else {
        Err(format!("Erro ao carregar conversas ({})", status))
    }
}

/// Adiciona um contato pelo e-mail ou username
pub async fn add_contact(token: &str, email_or_username: String) -> Result<UserProfile, String> {
    let client = reqwest::Client::new();
    let req = serde_json::json!({ "email_or_username": email_or_username });

    let resp = client
        .post(format!("{}/contacts/add", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<UserProfile>(&body)
            .map_err(|e| format!("Erro ao parsear contato: {}", e))
    } else {
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("Erro ao adicionar contato ({})", status));
        Err(msg)
    }
}

/// Cria uma nova conversa (ex: grupo) no servidor
pub async fn create_conversation(
    token: &str,
    name: Option<String>,
    is_group: bool,
    member_emails: Vec<String>,
    avatar_url: Option<String>,
    description: Option<String>,
) -> Result<crate::models::Conversation, String> {
    let client = reqwest::Client::new();
    let req = serde_json::json!({
        "name": name,
        "is_group": is_group,
        "member_emails": member_emails,
        "avatar_url": avatar_url,
        "description": description
    });

    let resp = client
        .post(format!("{}/conversations", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<crate::models::Conversation>(&body)
            .map_err(|e| format!("Erro ao parsear nova conversa: {}", e))
    } else {
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("Erro ao criar conversa ({})", status));
        Err(msg)
    }
}

/// Busca um perfil de usuário pelo e-mail ou nome de usuário
pub async fn search_user(token: &str, email_or_username: &str) -> Result<UserProfile, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/contacts/search", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .query(&[("email_or_username", email_or_username)])
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<UserProfile>(&body)
            .map_err(|e| format!("Erro ao parsear perfil: {}", e))
    } else {
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("Usuário não encontrado ({})", status));
        Err(msg)
    }
}

/// Carrega solicitações de amizade pendentes recebidas
pub async fn get_pending_requests(token: &str) -> Result<Vec<UserProfile>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/contacts/pending", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<Vec<UserProfile>>(&body)
            .map_err(|e| format!("Erro ao parsear solicitações pendentes: {}", e))
    } else {
        Err(format!(
            "Erro ao carregar solicitações pendentes ({})",
            status
        ))
    }
}

/// Aceita uma solicitação de contato
pub async fn accept_friend(token: &str, contact_id: String) -> Result<UserProfile, String> {
    let client = reqwest::Client::new();
    let req = serde_json::json!({ "contact_id": contact_id });

    let resp = client
        .post(format!("{}/contacts/accept", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<UserProfile>(&body)
            .map_err(|e| format!("Erro ao parsear contato aceito: {}", e))
    } else {
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("Erro ao aceitar contato ({})", status));
        Err(msg)
    }
}

/// Rejeita/recusa uma solicitação de contato
pub async fn reject_friend(token: &str, contact_id: String) -> Result<(), String> {
    let client = reqwest::Client::new();
    let req = serde_json::json!({ "contact_id": contact_id });

    let resp = client
        .post(format!("{}/contacts/reject", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        Ok(())
    } else {
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("Erro ao rejeitar contato ({})", status));
        Err(msg)
    }
}

/// Bloqueia ou desbloqueia um contato
pub async fn block_friend(token: &str, contact_id: String, block: bool) -> Result<(), String> {
    let client = reqwest::Client::new();
    let req = serde_json::json!({ "contact_id": contact_id, "block": block });

    let resp = client
        .post(format!("{}/contacts/block", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        Ok(())
    } else {
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("Erro ao atualizar bloqueio ({})", status));
        Err(msg)
    }
}

/// Atualiza o apelido local de um contato
pub async fn update_contact_nickname(
    token: &str,
    contact_id: String,
    nickname: Option<String>,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    let req = serde_json::json!({ "contact_id": contact_id, "nickname": nickname });

    let resp = client
        .post(format!("{}/contacts/nickname", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        Ok(())
    } else {
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v["error"].as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("Erro ao atualizar apelido ({})", status));
        Err(msg)
    }
}

/// Carrega o histórico de mensagens de uma conversa do servidor
pub async fn get_conversation_messages(
    token: &str,
    conversation_id: &str,
) -> Result<Vec<crate::models::Message>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!(
            "{}/conversations/{}/messages",
            SERVER_BASE_URL, conversation_id
        ))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Erro de conexão: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        serde_json::from_str::<Vec<crate::models::Message>>(&body)
            .map_err(|e| format!("Erro ao parsear mensagens: {}", e))
    } else {
        Err(format!("Erro ao carregar mensagens ({})", status))
    }
}

/// Busca o banner de anúncios ativo do servidor
pub async fn get_banner() -> Result<crate::models::BannerInfo, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/banner", SERVER_BASE_URL))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        serde_json::from_str::<crate::models::BannerInfo>(&body).map_err(|e| e.to_string())
    } else {
        Err(format!("Status de erro: {}", resp.status()))
    }
}

/// Salva a imagem do anúncio no sistema de arquivos local
#[allow(dead_code)]
pub async fn save_ad_image_local(bytes: &[u8], name: &str) -> Option<String> {
    let data_dir = crate::services::db::get_app_data_dir();
    let _ = std::fs::create_dir_all(&data_dir);
    let timestamp = chrono::Utc::now().timestamp_millis();
    let ext = if name.to_lowercase().ends_with(".png") { "png" }
              else if name.to_lowercase().ends_with(".gif") { "gif" }
              else if name.to_lowercase().ends_with(".webp") { "webp" }
              else { "jpg" };
    let file_name = format!("ad_banner_{}.{}", timestamp, ext);
    let file_path = data_dir.join(&file_name);
    
    // Limpa banners antigos para não acumular lixo
    if let Ok(entries) = std::fs::read_dir(&data_dir) {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.starts_with("ad_banner_") {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
        }
    }

    if std::fs::write(&file_path, bytes).is_ok() {
        let abs = std::fs::canonicalize(&file_path).unwrap_or(file_path);
        return Some(format!("dioxus-asset://{}", abs.to_string_lossy()));
    }
    None
}

/// Atualiza o banner de anúncios no servidor
pub async fn update_banner(token: &str, banner: &crate::models::BannerInfo) -> Result<(), String> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/banner", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(banner)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if status.is_success() {
        Ok(())
    } else {
        Err(format!("Erro {}: {}", status, body))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct BannerAdminInfo {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub action_label: String,
    pub link: String,
    pub icon: String,
    pub image_url: Option<String>,
    pub is_active: bool,
}

pub async fn list_banners_admin(token: &str) -> Result<Vec<BannerAdminInfo>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/admin/banners", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        serde_json::from_str::<Vec<BannerAdminInfo>>(&body).map_err(|e| e.to_string())
    } else {
        Err(format!("Status de erro: {}", resp.status()))
    }
}

pub async fn create_banner_admin(token: &str, title: &str, text: &str, action_label: &str, link: &str, icon: &str, image_url: Option<String>, is_active: bool) -> Result<BannerAdminInfo, String> {
    let client = reqwest::Client::new();
    let req = serde_json::json!({
        "title": title,
        "text": text,
        "action_label": action_label,
        "link": link,
        "icon": icon,
        "image_url": image_url,
        "is_active": is_active
    });
    let resp = client
        .post(format!("{}/admin/banners", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        serde_json::from_str::<BannerAdminInfo>(&body).map_err(|e| e.to_string())
    } else {
        Err(format!("Erro ao criar: {}", resp.status()))
    }
}

pub async fn update_banner_admin_req(token: &str, id: i32, title: &str, text: &str, action_label: &str, link: &str, icon: &str, image_url: Option<String>, is_active: bool) -> Result<(), String> {
    let client = reqwest::Client::new();
    let req = serde_json::json!({
        "title": title,
        "text": text,
        "action_label": action_label,
        "link": link,
        "icon": icon,
        "image_url": image_url,
        "is_active": is_active
    });
    let resp = client
        .put(format!("{}/admin/banners/{}", SERVER_BASE_URL, id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!("Erro ao atualizar: {}", resp.status()))
    }
}

pub async fn toggle_banner_admin_req(token: &str, id: i32) -> Result<(), String> {
    let client = reqwest::Client::new();
    let resp = client
        .put(format!("{}/admin/banners/{}/toggle", SERVER_BASE_URL, id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!("Erro ao alternar: {}", resp.status()))
    }
}

pub async fn delete_banner_admin_req(token: &str, id: i32) -> Result<(), String> {
    let client = reqwest::Client::new();
    let resp = client
        .delete(format!("{}/admin/banners/{}", SERVER_BASE_URL, id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!("Erro ao deletar: {}", resp.status()))
    }
}

