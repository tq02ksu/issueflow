use std::time::{SystemTime, UNIX_EPOCH};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::config::raw::RawOidcConfig;

type HmacSha256 = Hmac<Sha256>;
const STATE_TTL_SECONDS: u64 = 600;

#[derive(Clone, Debug)]
pub enum OidcConfig {
    Disabled,
    Enabled(OidcEnabledConfig),
}

#[derive(Clone, Debug)]
pub struct OidcEnabledConfig {
    pub issuer: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub state_signing_secret: String,
    pub metadata: Option<OidcMetadata>,
}

#[derive(Clone, Debug)]
pub struct OidcMetadata {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
}

#[derive(Debug, Deserialize)]
struct OidcDiscoveryDocument {
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct OidcStateClaims {
    issuer: String,
    issued_at: u64,
}

#[derive(Debug)]
pub enum OidcStateError {
    InvalidFormat,
    InvalidSignature,
    InvalidPayload,
    Expired,
}

impl OidcConfig {
    pub fn disabled() -> Self {
        Self::Disabled
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled(_))
    }

    pub fn enabled(&self) -> Option<&OidcEnabledConfig> {
        match self {
            Self::Enabled(config) => Some(config),
            Self::Disabled => None,
        }
    }

    pub async fn authorize_url(&mut self, state: &str) -> Result<String, String> {
        match self {
            Self::Enabled(config) => config.authorize_url(state).await,
            Self::Disabled => Err("oidc is disabled".to_string()),
        }
    }

    pub async fn from_raw(raw: RawOidcConfig) -> Result<Self, String> {
        if !raw.enabled.unwrap_or(false) {
            return Ok(Self::Disabled);
        }

        let issuer = raw
            .issuer
            .ok_or("missing required configuration: oidc.issuer")?;
        let client_id = raw
            .client_id
            .ok_or("missing required configuration: oidc.client_id")?;
        let client_secret = raw
            .client_secret
            .ok_or("missing required configuration: oidc.client_secret")?;
        let redirect_uri = raw
            .redirect_uri
            .ok_or("missing required configuration: oidc.redirect_uri")?;
        let state_signing_secret = raw
            .state_signing_secret
            .ok_or("missing required configuration: oidc.state_signing_secret")?;
        let scopes = raw.scopes.unwrap_or_else(|| {
            vec![
                "openid".to_string(),
                "profile".to_string(),
                "email".to_string(),
            ]
        });

        Ok(Self::Enabled(OidcEnabledConfig {
            issuer,
            client_id,
            client_secret,
            redirect_uri,
            scopes,
            state_signing_secret,
            metadata: None,
        }))
    }
}

impl OidcEnabledConfig {
    pub fn token_url(&self) -> Option<&str> {
        self.metadata.as_ref().map(|m| m.token_endpoint.as_str())
    }

    pub async fn authorize_url(&mut self, state: &str) -> Result<String, String> {
        let metadata = match &self.metadata {
            Some(m) => m.clone(),
            None => {
                let m = discover_metadata(&self.issuer).await?;
                self.metadata = Some(m.clone());
                m
            }
        };

        let scope = self.scopes.join(" ");

        Ok(format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            metadata.authorization_endpoint,
            encode_component(&self.client_id),
            encode_component(&self.redirect_uri),
            encode_component(&scope),
            encode_component(state),
        ))
    }
}

pub async fn discover_metadata(issuer: &str) -> Result<OidcMetadata, String> {
    let issuer = issuer.trim_end_matches('/');
    let discovery_url = format!("{issuer}/.well-known/openid-configuration");
    let document = Client::new()
        .get(&discovery_url)
        .send()
        .await
        .map_err(|error| format!("failed oidc discovery request: {error}"))?
        .error_for_status()
        .map_err(|error| format!("failed oidc discovery response: {error}"))?
        .json::<OidcDiscoveryDocument>()
        .await
        .map_err(|error| format!("failed oidc discovery decode: {error}"))?;

    Ok(OidcMetadata {
        issuer: document.issuer,
        authorization_endpoint: document.authorization_endpoint,
        token_endpoint: document.token_endpoint,
    })
}

pub fn issue_state(issuer: &str, signing_secret: &str) -> Result<String, OidcStateError> {
    let claims = OidcStateClaims {
        issuer: issuer.to_string(),
        issued_at: unix_timestamp_now(),
    };
    let claims_json = serde_json::to_vec(&claims).map_err(|_| OidcStateError::InvalidPayload)?;
    let claims_token = URL_SAFE_NO_PAD.encode(claims_json);
    let signature = sign_state_component(&claims_token, signing_secret)?;

    Ok(format!("{claims_token}.{signature}"))
}

pub fn validate_state(
    encoded_state: &str,
    issuer: &str,
    signing_secret: &str,
) -> Result<(), OidcStateError> {
    let (claims_token, signature) = encoded_state
        .split_once('.')
        .ok_or(OidcStateError::InvalidFormat)?;
    let expected_signature = sign_state_component(claims_token, signing_secret)?;

    if signature != expected_signature {
        return Err(OidcStateError::InvalidSignature);
    }

    let claims_bytes = URL_SAFE_NO_PAD
        .decode(claims_token)
        .map_err(|_| OidcStateError::InvalidPayload)?;
    let claims: OidcStateClaims =
        serde_json::from_slice(&claims_bytes).map_err(|_| OidcStateError::InvalidPayload)?;

    if claims.issuer != issuer {
        return Err(OidcStateError::InvalidPayload);
    }

    if unix_timestamp_now().saturating_sub(claims.issued_at) > STATE_TTL_SECONDS {
        return Err(OidcStateError::Expired);
    }

    Ok(())
}

fn sign_state_component(claims_token: &str, signing_secret: &str) -> Result<String, OidcStateError> {
    let mut mac = HmacSha256::new_from_slice(signing_secret.as_bytes())
        .map_err(|_| OidcStateError::InvalidSignature)?;
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
