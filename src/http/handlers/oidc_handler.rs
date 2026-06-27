use axum::{
    extract::{Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Redirect, Response},
};
use base64::Engine;
use serde::Deserialize;

use crate::{
    http::routes::AppState,
    oidc::{issue_state, validate_state},
    session::{SessionClaims, sign_session},
};

#[derive(Deserialize)]
pub struct OidcCallbackQuery {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    id_token: Option<String>,
}

#[derive(Deserialize)]
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

pub async fn oidc_login(State(mut state): State<AppState>) -> Result<Redirect, StatusCode> {
    let oidc = state.config.oidc.enabled().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    let state_token = issue_state(&oidc.issuer, &oidc.state_signing_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let url = state
        .config
        .oidc
        .authorize_url(&state_token)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    Ok(Redirect::temporary(&url))
}

fn redirect_with_session(location: &str, session_token: &str) -> Response {
    let cookie = format!(
        "session={}; Path=/; HttpOnly; SameSite=Lax",
        session_token
    );
    ([(header::SET_COOKIE, cookie)], Redirect::temporary(location)).into_response()
}

pub async fn oidc_callback(
    State(state): State<AppState>,
    Query(query): Query<OidcCallbackQuery>,
) -> Result<Response, StatusCode> {
    let oidc = state.config.oidc.enabled().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    if query.code.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    validate_state(&query.state, &oidc.issuer, &oidc.state_signing_secret)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let token_url = oidc.token_url().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let client = reqwest::Client::new();
    let token_result = client
        .post(token_url)
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
            let tokens: TokenResponse = resp.json().await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let id_token = tokens.id_token.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
            let claims = parse_id_token_sub(&id_token)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let user = crate::db::upsert_user(
                &state.pool,
                &claims.sub,
                claims.name.as_deref().unwrap_or(""),
                claims.email.as_deref().unwrap_or(""),
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let session_claims = SessionClaims {
                user_id: user.id,
                sub: claims.sub,
                access_token: tokens.access_token,
            };
            let session_token = sign_session(
                &session_claims,
                state.config.session_signing_secret.as_bytes(),
            );

            Ok(redirect_with_session(
                "/auth/callback/oidc?result=success",
                &session_token,
            ))
        }
        Ok(resp) => {
            let status = resp.status().as_u16();
            Ok(Redirect::temporary(&format!(
                "/auth/callback/oidc?result=token_exchange_failed&reason=http+{status}"
            ))
            .into_response())
        }
        Err(_) => Ok(Redirect::temporary(
            "/auth/callback/oidc?result=token_exchange_failed&reason=token+endpoint+unreachable"
        )
        .into_response()),
    }
}
