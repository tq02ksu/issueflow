use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use base64::Engine;

use crate::{
    error::AppError,
    http::routes::AppState,
    oidc::{OidcMetadata, discover_metadata, issue_state, validate_state},
    session::{build_claims, sign_token},
};

#[derive(serde::Deserialize)]
pub struct OidcCallbackQuery {
    code: String,
    state: String,
}

#[derive(serde::Deserialize)]
struct TokenResponse {
    access_token: String,
    id_token: Option<String>,
}

#[derive(serde::Deserialize)]
struct IdTokenClaims {
    sub: String,
    name: Option<String>,
    email: Option<String>,
}

fn parse_id_token_sub(id_token: &str) -> Result<IdTokenClaims, String> {
    let payload = id_token
        .split('.')
        .nth(1)
        .ok_or("invalid id_token format")?;
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload)
        .map_err(|_| "invalid id_token base64")?;
    serde_json::from_slice(&decoded).map_err(|_| "invalid id_token json".to_string())
}

async fn get_or_discover_metadata(state: &AppState) -> Result<OidcMetadata, AppError> {
    let oidc = state
        .config
        .oidc
        .enabled()
        .ok_or(AppError::ServiceUnavailable("oidc not configured".into()))?;

    {
        let cached = state.oidc_metadata.read().await;
        if let Some(ref m) = *cached {
            return Ok(m.clone());
        }
    }

    if let Some(ref m) = oidc.metadata {
        return Ok(m.clone());
    }

    let metadata = discover_metadata(&oidc.issuer)
        .await
        .map_err(|e| AppError::Internal(format!("oidc discovery failed: {e}").into()))?;

    let mut cached = state.oidc_metadata.write().await;
    *cached = Some(metadata.clone());
    Ok(metadata)
}

pub async fn oidc_login(State(state): State<AppState>) -> Result<Redirect, AppError> {
    let oidc = state
        .config
        .oidc
        .enabled()
        .ok_or(AppError::ServiceUnavailable("oidc not configured".into()))?;

    let state_token = issue_state(&oidc.issuer, &oidc.state_signing_secret)
        .map_err(|e| AppError::Internal(format!("oidc state signing failed: {e}").into()))?;

    let metadata = get_or_discover_metadata(&state).await?;

    let scope = oidc.scopes.join(" ");

    let url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
        metadata.authorization_endpoint,
        encode_component(&oidc.client_id),
        encode_component(&oidc.redirect_uri),
        encode_component(&scope),
        encode_component(&state_token),
    );

    Ok(Redirect::temporary(&url))
}

pub async fn oidc_callback(
    State(state): State<AppState>,
    Query(query): Query<OidcCallbackQuery>,
) -> Result<Response, AppError> {
    let oidc = state
        .config
        .oidc
        .enabled()
        .ok_or(AppError::ServiceUnavailable("oidc not configured".into()))?;

    if query.code.trim().is_empty() {
        return Err(AppError::BadRequest("missing authorization code".into()));
    }

    validate_state(&query.state, &oidc.issuer, &oidc.state_signing_secret)
        .map_err(|_| AppError::BadRequest("invalid oidc state".into()))?;

    let metadata = get_or_discover_metadata(&state).await?;

    let client = reqwest::Client::new();
    let token_result = client
        .post(&metadata.token_endpoint)
        .form(&[
            ("client_id", oidc.client_id.as_str()),
            ("client_secret", oidc.client_secret.as_str()),
            ("code", query.code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", oidc.redirect_uri.as_str()),
        ])
        .send()
        .await;

    match token_result {
        Ok(resp) if resp.status().is_success() => {
            let body_text = resp
                .text()
                .await
                .map_err(|e| AppError::Internal(e.into()))?;
            let tokens: TokenResponse = serde_json::from_str(&body_text).map_err(|e| {
                tracing::error!(%e, %body_text, "failed to parse oidc token response");
                AppError::Internal("oidc token parse error".into())
            })?;

            let id_token = tokens.id_token.ok_or_else(|| {
                tracing::error!(%body_text, "no id_token in oidc response");
                AppError::Internal("oidc token response missing id_token".into())
            })?;

            let claims = parse_id_token_sub(&id_token).map_err(|e| {
                tracing::error!(%e, %id_token, "failed to parse id_token");
                AppError::Internal("oidc id_token parse error".into())
            })?;

            let user = crate::db::upsert_user(
                &state.pool,
                &claims.sub,
                claims.name.as_deref().unwrap_or(""),
                claims.email.as_deref().unwrap_or(""),
            )
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

            let session_claims = build_claims(user.id, &claims.sub, &tokens.access_token);
            let jwt = sign_token(&session_claims, &state.config.jwt_secret)?;

            Ok(Redirect::temporary(&format!(
                "/auth/callback/oidc?result=success&token={}",
                encode_component(&jwt)
            ))
            .into_response())
        }
        Ok(resp) => {
            let status = resp.status().as_u16();
            Ok(Redirect::temporary(&format!(
                "/auth/callback/oidc?result=token_exchange_failed&reason=http+{status}"
            ))
            .into_response())
        }
        Err(_) => Ok(Redirect::temporary(
            "/auth/callback/oidc?result=token_exchange_failed&reason=token+endpoint+unreachable",
        )
        .into_response()),
    }
}

fn encode_component(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(char::from(byte));
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}
