use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionClaims {
    pub user_id: i64,
    pub sub: String,
    pub access_token: String,
}

#[derive(Clone, Debug)]
pub struct Session {
    pub user_id: i64,
    pub sub: String,
    pub access_token: String,
}

#[derive(Clone)]
pub struct SessionConfig {
    pub signing_secret: String,
}

impl Session {
    pub fn from_cookie(cookie_header: &str, secret: &[u8]) -> Result<Self, StatusCode> {
        let session_cookie = cookie_header
            .split(';')
            .map(|c| c.trim())
            .find(|c| c.starts_with("session="))
            .and_then(|c| c.strip_prefix("session="))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let claims = verify_session(session_cookie, secret)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(Session {
            user_id: claims.user_id,
            sub: claims.sub,
            access_token: claims.access_token,
        })
    }
}

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret: &[u8] = parts
            .extensions
            .get::<SessionConfig>()
            .map(|c| c.signing_secret.as_bytes())
            .unwrap_or(b"");

        let cookie_header = parts
            .headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        Session::from_cookie(cookie_header, secret)
    }
}

pub fn sign_session(claims: &SessionClaims, secret: &[u8]) -> String {
    let payload = serde_json::to_vec(claims).expect("session claims should serialize");
    let payload_b64 = URL_SAFE_NO_PAD.encode(&payload);

    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC key should be valid");
    mac.update(payload_b64.as_bytes());
    let signature = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());

    format!("{}.{}", payload_b64, signature)
}

pub fn verify_session(token: &str, secret: &[u8]) -> Result<SessionClaims, String> {
    let (payload_b64, signature) = token
        .split_once('.')
        .ok_or("invalid session token format")?;

    let mut mac = HmacSha256::new_from_slice(secret).map_err(|_| "invalid secret")?;
    mac.update(payload_b64.as_bytes());
    let expected_sig = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());

    if signature != expected_sig {
        return Err("invalid session signature".to_string());
    }

    let payload = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|_| "invalid session payload")?;

    let claims: SessionClaims =
        serde_json::from_slice(&payload).map_err(|_| "invalid session claims")?;

    Ok(claims)
}
