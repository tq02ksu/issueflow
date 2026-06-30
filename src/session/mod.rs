use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Clone, Debug)]
pub struct Session {
    pub user_id: i64,
    pub sub: String,
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct SessionClaims {
    pub user_id: i64,
    pub sub: String,
    pub access_token: String,
    pub exp: usize,
    pub iat: usize,
}

impl Session {
    pub fn from_bearer(header: &str, secret: &str) -> Result<Self, AppError> {
        let token = header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        let claims = verify_token(token, secret)?;

        Ok(Self {
            user_id: claims.user_id,
            sub: claims.sub,
            access_token: claims.access_token,
        })
    }
}

impl<S> FromRequestParts<S> for Session
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let header_value = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let app_state = AppState::from_ref(state);
        Self::load(&app_state.pool, header_value, &app_state.config.jwt_secret).await
    }
}

async fn session_user_exists(
    pool: &crate::db::DbPool,
    user_id: i64,
    sub: &str,
) -> Result<bool, AppError> {
    let exists: Option<i64> = sqlx::query_scalar("SELECT id FROM users WHERE id = ? AND sub = ?")
        .bind(user_id)
        .bind(sub)
        .fetch_optional(pool)
        .await?;

    Ok(exists.is_some())
}

impl Session {
    pub async fn load(
        pool: &crate::db::DbPool,
        header: &str,
        secret: &str,
    ) -> Result<Self, AppError> {
        let session = Self::from_bearer(header, secret)?;

        if session_user_exists(pool, session.user_id, &session.sub).await? {
            Ok(session)
        } else {
            Err(AppError::Unauthorized)
        }
    }
}

use crate::http::routes::AppState;

pub fn verify_token(token: &str, secret: &str) -> Result<SessionClaims, AppError> {
    let data = decode::<SessionClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized)?;
    Ok(data.claims)
}

pub fn sign_token(claims: &SessionClaims, secret: &str) -> Result<String, AppError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("jwt encoding error: {e}").into()))
}

use std::time::{SystemTime, UNIX_EPOCH};

fn now_epoch() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as usize
}

pub fn build_claims(user_id: i64, sub: &str, access_token: &str) -> SessionClaims {
    let now = now_epoch();
    SessionClaims {
        user_id,
        sub: sub.to_string(),
        access_token: access_token.to_string(),
        iat: now,
        exp: now + 86400, // 24h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_and_verify_round_trips() {
        let secret = "issueflow-default-jwt-secret";
        let claims = build_claims(1, "3", "test-access-token");
        let token = sign_token(&claims, secret)
            .unwrap_or_else(|error| panic!("sign_token should succeed: {error}"));
        let verified = verify_token(&token, secret)
            .unwrap_or_else(|error| panic!("verify_token should succeed: {error}"));
        assert_eq!(verified.user_id, 1);
        assert_eq!(verified.sub, "3");
        assert_eq!(verified.access_token, "test-access-token");
    }

    #[test]
    fn from_bearer_extracts_token() {
        let secret = "test-secret";
        let claims = build_claims(42, "user-abc", "glpat-xyz");
        let token = sign_token(&claims, secret)
            .unwrap_or_else(|error| panic!("sign_token should succeed: {error}"));
        let header = format!("Bearer {token}");
        let session = Session::from_bearer(&header, secret)
            .unwrap_or_else(|error| panic!("from_bearer should succeed: {error}"));
        assert_eq!(session.user_id, 42);
        assert_eq!(session.sub, "user-abc");
        assert_eq!(session.access_token, "glpat-xyz");
    }
}
