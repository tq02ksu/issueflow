use std::time::{SystemTime, UNIX_EPOCH};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

const GITLAB_AUTHORIZE_URL: &str = "https://gitlab.com/oauth/authorize";
const GITLAB_TOKEN_URL: &str = "https://gitlab.com/oauth/token";
const DEFAULT_GITLAB_SCOPE: &str = "read_user";
const STATE_TTL_SECONDS: u64 = 600;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OAuthConfig {
    pub state_signing_secret: String,
    providers: Vec<OAuthProviderConfig>,
}

impl OAuthConfig {
    pub fn from_env() -> Self {
        let mut providers = Vec::new();

        if let Some(provider) = OAuthProviderConfig::gitlab_from_env() {
            providers.push(provider);
        }

        if providers.is_empty() {
            return Self::disabled();
        }

        let state_signing_secret = std::env::var("OAUTH_STATE_SIGNING_SECRET")
            .expect("OAUTH_STATE_SIGNING_SECRET must be set when OAuth providers are enabled");

        Self {
            state_signing_secret,
            providers,
        }
    }

    pub fn disabled() -> Self {
        Self {
            state_signing_secret: String::new(),
            providers: Vec::new(),
        }
    }

    pub fn for_tests(providers: Vec<OAuthProviderConfig>) -> Self {
        Self {
            state_signing_secret: "test-oauth-state-signing-secret".to_string(),
            providers,
        }
    }

    pub fn provider(&self, kind: OAuthProviderKind) -> Option<&OAuthProviderConfig> {
        self.providers.iter().find(|provider| provider.kind == kind)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OAuthProviderKind {
    GitLab,
}

impl OAuthProviderKind {
    pub fn from_slug(slug: &str) -> Option<Self> {
        match slug {
            "gitlab" => Some(Self::GitLab),
            _ => None,
        }
    }

    pub fn slug(self) -> &'static str {
        match self {
            Self::GitLab => "gitlab",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OAuthProviderConfig {
    pub kind: OAuthProviderKind,
    pub client_id: String,
    pub client_secret: String,
    pub authorize_url: String,
    pub token_url: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

impl OAuthProviderConfig {
    pub fn gitlab_from_env() -> Option<Self> {
        let client_id = std::env::var("GITLAB_OAUTH_CLIENT_ID").ok()?;
        let client_secret = std::env::var("GITLAB_OAUTH_CLIENT_SECRET")
            .expect("GITLAB_OAUTH_CLIENT_SECRET must be set when GitLab OAuth is enabled");
        let redirect_uri = std::env::var("GITLAB_OAUTH_REDIRECT_URI")
            .expect("GITLAB_OAUTH_REDIRECT_URI must be set when GitLab OAuth is enabled");
        let authorize_url = std::env::var("GITLAB_OAUTH_AUTHORIZE_URL")
            .unwrap_or_else(|_| GITLAB_AUTHORIZE_URL.to_string());
        let token_url = std::env::var("GITLAB_OAUTH_TOKEN_URL")
            .unwrap_or_else(|_| GITLAB_TOKEN_URL.to_string());
        let scopes = std::env::var("GITLAB_OAUTH_SCOPES")
            .unwrap_or_else(|_| DEFAULT_GITLAB_SCOPE.to_string())
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>();

        Some(Self {
            kind: OAuthProviderKind::GitLab,
            client_id,
            client_secret,
            authorize_url,
            token_url,
            redirect_uri,
            scopes,
        })
    }

    pub fn gitlab_for_tests() -> Self {
        Self {
            kind: OAuthProviderKind::GitLab,
            client_id: "gitlab-test-client".to_string(),
            client_secret: "gitlab-test-secret".to_string(),
            authorize_url: "https://gitlab.example.com/oauth/authorize".to_string(),
            token_url: "https://gitlab.example.com/oauth/token".to_string(),
            redirect_uri: "http://localhost:3000/auth/gitlab/callback".to_string(),
            scopes: vec!["read_user".to_string(), "api".to_string()],
        }
    }

    pub fn authorize_url(&self, state: &str) -> String {
        let scope = self.scopes.join(" ");

        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            self.authorize_url,
            encode_component(&self.client_id),
            encode_component(&self.redirect_uri),
            encode_component(&scope),
            encode_component(state),
        )
    }
}

#[derive(Debug)]
pub enum OAuthStateError {
    InvalidFormat,
    InvalidSignature,
    InvalidPayload,
    Expired,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct OAuthStateClaims {
    provider: String,
    issued_at: u64,
}

pub fn issue_state(
    provider: OAuthProviderKind,
    state_signing_secret: &str,
) -> Result<String, OAuthStateError> {
    let claims = OAuthStateClaims {
        provider: provider.slug().to_string(),
        issued_at: unix_timestamp_now(),
    };
    let claims_json = serde_json::to_vec(&claims).map_err(|_| OAuthStateError::InvalidPayload)?;
    let claims_token = URL_SAFE_NO_PAD.encode(claims_json);
    let signature = sign_state_component(&claims_token, state_signing_secret)?;

    Ok(format!("{claims_token}.{signature}"))
}

pub fn validate_state(
    encoded_state: &str,
    expected_provider: OAuthProviderKind,
    state_signing_secret: &str,
) -> Result<(), OAuthStateError> {
    let (claims_token, signature) = encoded_state
        .split_once('.')
        .ok_or(OAuthStateError::InvalidFormat)?;
    let expected_signature = sign_state_component(claims_token, state_signing_secret)?;

    if signature != expected_signature {
        return Err(OAuthStateError::InvalidSignature);
    }

    let claims_bytes = URL_SAFE_NO_PAD
        .decode(claims_token)
        .map_err(|_| OAuthStateError::InvalidPayload)?;
    let claims: OAuthStateClaims =
        serde_json::from_slice(&claims_bytes).map_err(|_| OAuthStateError::InvalidPayload)?;

    if claims.provider != expected_provider.slug() {
        return Err(OAuthStateError::InvalidPayload);
    }

    if unix_timestamp_now().saturating_sub(claims.issued_at) > STATE_TTL_SECONDS {
        return Err(OAuthStateError::Expired);
    }

    Ok(())
}

fn sign_state_component(
    claims_token: &str,
    state_signing_secret: &str,
) -> Result<String, OAuthStateError> {
    let mut mac = HmacSha256::new_from_slice(state_signing_secret.as_bytes())
        .map_err(|_| OAuthStateError::InvalidSignature)?;
    mac.update(claims_token.as_bytes());
    Ok(URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes()))
}

fn unix_timestamp_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_secs()
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
