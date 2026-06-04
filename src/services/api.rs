/// Serviço HTTP para comunicação com o skypia-serve (Actix-web)
use serde::{Deserialize, Serialize};

pub const SERVER_BASE_URL: &str = "http://127.0.0.1:8080";

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
    password: String,
    display_name: String,
) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();
    let req = RegisterRequest { email, password, display_name };

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
pub async fn update_profile(
    token: &str,
    req: UpdateProfileRequest,
) -> Result<UserProfile, String> {
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
pub async fn upload_avatar(token: &str, image_bytes: Vec<u8>, mime_type: &str) -> Result<String, String> {
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
        let val: serde_json::Value =
            serde_json::from_str(&body).map_err(|e| e.to_string())?;
        val["avatar_url"]
            .as_str()
            .map(|s| format!("{}{}", SERVER_BASE_URL, s))
            .ok_or_else(|| "Campo avatar_url ausente na resposta.".to_string())
    } else {
        Err(format!("Erro no upload ({}): {}", status, body))
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

/// Adiciona um contato pelo e-mail
pub async fn add_contact(token: &str, email: String) -> Result<UserProfile, String> {
    let client = reqwest::Client::new();
    let req = serde_json::json!({ "email": email });

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
) -> Result<crate::models::Conversation, String> {
    let client = reqwest::Client::new();
    let req = serde_json::json!({
        "name": name,
        "is_group": is_group,
        "member_emails": member_emails
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

/// Busca um perfil de usuário pelo email
pub async fn search_user(token: &str, email: &str) -> Result<UserProfile, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/contacts/search", SERVER_BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .query(&[("email", email)])
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
        Err(format!("Erro ao carregar solicitações pendentes ({})", status))
    }
}

/// Aceita uma solicitação de contato
pub async fn accept_friend(token: &str, contact_id: i64) -> Result<UserProfile, String> {
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
pub async fn reject_friend(token: &str, contact_id: i64) -> Result<(), String> {
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
pub async fn block_friend(token: &str, contact_id: i64, block: bool) -> Result<(), String> {
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
pub async fn update_contact_nickname(token: &str, contact_id: i64, nickname: Option<String>) -> Result<(), String> {
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
