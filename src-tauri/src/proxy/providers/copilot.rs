/// Copilot Provider
///
/// 将 GitHub Copilot 作为上游提供商，支持 OpenAI 和 Anthropic 协议
///
/// ⚠️ 非官方 API，使用风险自负
///
/// # 架构说明
///
/// 1. GitHub Token -> Copilot Token (通过 /copilot_internal/v2/token)
/// 2. Copilot Token -> API 调用 (通过 api.githubcopilot.com)
///
/// # 支持的模型
///
/// Copilot 支持多种模型，包括：
/// - claude-opus-4.5 (Anthropic 最强模型)
/// - claude-sonnet-4.5
/// - gpt-4.1
/// - o1-mini, o1-preview
///
/// # 参考资料
///
/// - https://github.com/ericc-ch/copilot-api
/// - https://github.com/Alorse/copilot-to-api

use axum::{
    body::Body,
    http::{header, HeaderMap, HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use futures::StreamExt;
use serde_json::Value;
use tokio::time::Duration;
use tracing::{debug, info, warn};

use crate::proxy::server::AppState;

const COPILOT_TOKEN_URL: &str = "https://api.github.com/copilot_internal/v2/token";
const COPILOT_API_BASE: &str = "https://api.githubcopilot.com";
const USER_AGENT: &str = "GithubCopilot/1.155.0";

/// GitHub Device Code Flow 响应
#[derive(Debug, serde::Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

/// GitHub OAuth Token 响应
#[derive(Debug, serde::Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

/// Copilot Token 响应
#[derive(Debug, serde::Deserialize)]
struct CopilotTokenResponse {
    token: String,
    #[serde(default)]
    expires_at: i64,
}

/// 检查 Copilot token 是否有效（提前 60 秒刷新）
fn is_token_valid(expires_at: Option<i64>) -> bool {
    if let Some(expires) = expires_at {
        let now = chrono::Utc::now().timestamp();
        // 提前 60 秒刷新
        expires > now + 60
    } else {
        false
    }
}

/// 获取/刷新 Copilot token
pub async fn ensure_copilot_token(
    github_token: &str,
    current_token: Option<String>,
    expires_at: Option<i64>,
) -> Result<(String, i64), String> {
    // 如果当前 token 有效，直接返回
    if current_token.is_some() && is_token_valid(expires_at) {
        return Ok((current_token.unwrap(), expires_at.unwrap()));
    }

    info!("🔄 Refreshing Copilot token...");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .tcp_nodelay(true)
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", github_token))
            .map_err(|e| format!("Invalid authorization header: {}", e))?,
    );
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_static(USER_AGENT),
    );

    let response = client
        .get(COPILOT_TOKEN_URL)
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch Copilot token: {}", e))?;

    if response.status().as_u16() != 200 {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Failed to get Copilot token: {}", error_text));
    }

    let token_response: CopilotTokenResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    info!("✅ Copilot token refreshed successfully");
    debug!("Token expires at: {}", token_response.expires_at);

    Ok((token_response.token, token_response.expires_at))
}

/// 模型映射：将客户端请求的模型映射到 Copilot 支持的模型
fn map_model_for_copilot(original: &str, default_model: &str) -> String {
    let m = original.to_lowercase();

    // 如果已经是 Copilot 模型格式，直接返回
    if m.starts_with("copilot:") {
        return original[8..].to_string();
    }

    // Copilot 支持的模型列表
    match m.as_str() {
        // Claude 模型
        "claude-opus-4.5" | "claude-opus-4" | "claude-opus" => "claude-opus-4.5".to_string(),
        "claude-sonnet-4.5" | "claude-sonnet-4" | "claude-sonnet" => "claude-sonnet-4.5".to_string(),

        // GPT 模型
        "gpt-4.1" | "gpt-4" | "gpt-4o" => "gpt-4.1".to_string(),
        "gpt-4o-mini" | "gpt-4-mini" => "gpt-4o-mini".to_string(),

        // O1 模型
        "o1-mini" | "o1-mini-preview" => "o1-mini".to_string(),
        "o1-preview" | "o1" => "o1-preview".to_string(),

        // 如果不认识，使用默认模型
        _ => default_model.to_string(),
    }
}

/// 构建 HTTP 客户端（支持上游代理）
fn build_client(
    upstream_proxy: Option<crate::proxy::config::UpstreamProxyConfig>,
    timeout_secs: u64,
) -> Result<reqwest::Client, String> {
    let mut builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs.max(5)));

    if let Some(config) = upstream_proxy {
        if config.enabled && !config.url.is_empty() {
            let proxy = reqwest::Proxy::all(&config.url)
                .map_err(|e| format!("Invalid upstream proxy url: {}", e))?;
            builder = builder.proxy(proxy);
        }
    }

    builder
        .tcp_nodelay(true)
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))
}

/// 复制安全的请求头
fn copy_passthrough_headers(incoming: &HeaderMap) -> HeaderMap {
    let mut out = HeaderMap::new();

    for (k, v) in incoming.iter() {
        let key = k.as_str().to_ascii_lowercase();
        match key.as_str() {
            "content-type" | "accept" | "user-agent" => {
                out.insert(k.clone(), v.clone());
            }
            "accept-encoding" | "cache-control" => {
                out.insert(k.clone(), v.clone());
            }
            _ => {}
        }
    }

    out
}

/// 设置 Copilot 认证头
fn set_copilot_auth(headers: &mut HeaderMap, copilot_token: &str) {
    // Copilot 需要 GitHub-Bearer 前缀
    // 注意：使用转义的方式绕过 HTTP header 的可见字符验证
    let token_clean = copilot_token.trim().replace('\n', "").replace('\r', "");
    let auth_value = format!("GitHub-Bearer {}", token_clean);

    // 尝试多种方式设置 header
    if let Ok(v) = HeaderValue::from_bytes(&auth_value.as_bytes().to_vec()) {
        headers.insert(header::AUTHORIZATION, v);
        debug!("Authorization header set successfully (from_bytes)");
    } else if let Ok(v) = HeaderValue::from_str(&auth_value) {
        headers.insert(header::AUTHORIZATION, v);
        debug!("Authorization header set successfully (from_str)");
    } else {
        warn!("Failed to create Authorization header");
    }

    // Copilot 要求的特定头
    if let Ok(v) = HeaderValue::from_str("vscode-chat") {
        headers.insert("Copilot-Integration-Id", v);
    }
}

/// 转发请求到 Copilot API
///
/// 支持 OpenAI 和 Anthropic 协议格式
pub async fn forward_to_copilot(
    state: &AppState,
    method: Method,
    path: &str,
    incoming_headers: &HeaderMap,
    mut body: Value,
) -> Response {
    let copilot_config = state.copilot.read().await.clone();

    if !copilot_config.enabled || copilot_config.dispatch_mode == crate::proxy::CopilotDispatchMode::Off {
        return (StatusCode::BAD_REQUEST, "Copilot is disabled").into_response();
    }

    if copilot_config.github_token.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "GitHub token is not set").into_response();
    }

    // 直接使用用户提供的 token（可能是 Copilot token 或 GitHub OAuth token）
    // Copilot token 格式：长 base64 字符串，通常包含 +/= 等字符
    let provided_token = copilot_config.github_token.trim();

    // 如果 token 看起来像 Copilot token（比较长，包含特殊字符），直接使用
    // 否则尝试通过 GitHub API 获取 Copilot token
    let copilot_token = if provided_token.len() > 100 && (provided_token.contains('+') || provided_token.contains('/') || provided_token.contains('=') || provided_token.contains('-') || provided_token.contains('_')) {
        // 看起来像 Copilot token，直接使用
        provided_token.to_string()
    } else {
        // 可能是 GitHub OAuth token，尝试获取 Copilot token
        match ensure_copilot_token(
            provided_token,
            copilot_config.copilot_token,
            copilot_config.token_expires_at,
        ).await {
            Ok((token, _expires)) => token,
            Err(e) => {
                warn!("Failed to get Copilot token: {}", e);
                return (StatusCode::UNAUTHORIZED, format!("Copilot auth failed: {}", e)).into_response();
            }
        }
    };

    // 模型映射
    if let Some(model) = body.get("model").and_then(|v| v.as_str()) {
        let mapped = map_model_for_copilot(model, &copilot_config.default_model);
        body["model"] = Value::String(mapped);
    }

    // 构建请求 URL
    let url = format!("{}{}", COPILOT_API_BASE, path);

    let timeout_secs = state.request_timeout.max(5);
    let upstream_proxy = state.upstream_proxy.read().await.clone();
    let client = match build_client(Some(upstream_proxy), timeout_secs) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };

    let mut headers = copy_passthrough_headers(incoming_headers);
    set_copilot_auth(&mut headers, &copilot_token);

    headers
        .entry(header::CONTENT_TYPE)
        .or_insert(HeaderValue::from_static("application/json"));

    let body_bytes = serde_json::to_vec(&body).unwrap_or_default();
    let body_len = body_bytes.len();

    debug!("Forwarding request to Copilot (len: {} bytes): {}", body_len, url);

    let req = client.request(method, &url)
        .headers(headers)
        .body(body_bytes);

    let resp = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                format!("Upstream request failed: {}", e),
            )
                .into_response();
        }
    };

    let status = StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);

    let mut out = Response::builder().status(status);
    if let Some(ct) = resp.headers().get(header::CONTENT_TYPE) {
        out = out.header(header::CONTENT_TYPE, ct.clone());
    }

    // Stream response body to the client
    let stream = resp.bytes_stream().map(|chunk| match chunk {
        Ok(b) => Ok::<Bytes, std::io::Error>(b),
        Err(e) => Ok(Bytes::from(format!("Upstream stream error: {}", e))),
    });

    out.body(Body::from_stream(stream)).unwrap_or_else(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to build response").into_response()
    })
}

/// 获取 Copilot 可用模型列表
pub async fn get_copilot_models(
    state: &AppState,
) -> Result<Value, String> {
    let copilot_config = state.copilot.read().await.clone();

    if !copilot_config.enabled {
        return Err("Copilot is disabled".to_string());
    }

    let (copilot_token, _) = ensure_copilot_token(
        &copilot_config.github_token,
        copilot_config.copilot_token,
        copilot_config.token_expires_at,
    ).await?;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to build client: {}", e))?;

    let response = client
        .get(&format!("{}/models", COPILOT_API_BASE))
        .header("authorization", format!("Bearer {}", copilot_token))
        .header("Copilot-Integration-Id", "vscode-chat")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch models: {}", e))?;

    if response.status().as_u16() != 200 {
        return Err(format!("Failed to get models: {}", response.status().as_u16()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse models: {}", e))
}

/// 开始 GitHub Device Code Flow - 获取设备码
pub async fn start_device_flow() -> Result<DeviceCodeResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    // Copilot 的 client_id（从官方 VS Code 扩展获取）
    let params = [
        ("client_id", "Iv1.b507a08c87ecfe98"),
        ("scope", "read:user read:org"),
    ];

    let response = client
        .post("https://github.com/login/device/code")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Failed to request device code: {}", e))?;

    if response.status().as_u16() != 200 {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("GitHub device flow failed: {}", error_text));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse device code response: {}", e))
}

/// 轮询等待用户授权并获取 OAuth token
pub async fn poll_for_token(device_code: String, interval: u64) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let params = [
        ("client_id", "Iv1.b507a08c87ecfe98"),
        ("device_code", &device_code),
        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
    ];

    // 最多轮询 5 分钟
    let max_attempts = (300 / interval) as usize;
    let mut attempts = 0;

    while attempts < max_attempts {
        tokio::time::sleep(Duration::from_secs(interval)).await;

        let response = client
            .post("https://github.com/login/oauth/access_token")
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Failed to poll for token: {}", e))?;

        let status = response.status().as_u16();

        // 仍然在等待用户授权
        if status == 400 {
            let body: Value = response.json().await.unwrap_or_default();
            if let Some(error) = body.get("error").and_then(|v| v.as_str()) {
                if error == "authorization_pending" {
                    attempts += 1;
                    continue;
                }
                if error == "slow_down" {
                    // 需要增加轮询间隔
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    attempts += 1;
                    continue;
                }
                if error == "expired" {
                    return Err("Device code expired. Please try again.".to_string());
                }
                if error == "access_denied" {
                    return Err("Authorization denied by user.".to_string());
                }
            }
            return Err(format!("Unexpected error during polling"));
        }

        // 成功获取 token
        if status == 200 {
            let token_response: OAuthTokenResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse token response: {}", e))?;

            return Ok(token_response.access_token);
        }

        attempts += 1;
    }

    Err("Authorization timed out. Please try again.".to_string())
}

/// 使用 OAuth token 获取 Copilot token
pub async fn get_copilot_token_with_oauth(oauth_token: &str) -> Result<(String, i64), String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .tcp_nodelay(true)
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", oauth_token))
            .map_err(|e| format!("Invalid authorization header: {}", e))?,
    );
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_static(USER_AGENT),
    );

    let response = client
        .get(COPILOT_TOKEN_URL)
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch Copilot token: {}", e))?;

    if response.status().as_u16() != 200 {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Failed to get Copilot token: {}", error_text));
    }

    let token_response: CopilotTokenResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    info!("✅ Copilot token obtained via OAuth");
    Ok((token_response.token, token_response.expires_at))
}
