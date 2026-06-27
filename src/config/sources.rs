use std::{collections::HashMap, env, fs, path::Path};

use crate::config::raw::{RawConfig, RawGitConfig, RawOidcConfig, RawServerConfig};

pub fn load_raw_config() -> Result<RawConfig, String> {
    let defaults = RawConfig {
        server: Some(RawServerConfig {
            listen_addr: Some("127.0.0.1:8080".to_string()),
        }),
        git: None,
        oidc: Some(RawOidcConfig {
            enabled: Some(false),
            scopes: Some(vec![
                "openid".to_string(),
                "profile".to_string(),
                "email".to_string(),
            ]),
            ..RawOidcConfig::default()
        }),
    };

    let toml = load_toml_file(Path::new("config/issueflow.toml"))?;
    let dotenv = raw_from_env_map(load_dotenv_map(Path::new(".env"))?)?;
    let env_vars = raw_from_environment()?;

    Ok(defaults.merge(toml).merge(dotenv).merge(env_vars))
}

fn load_toml_file(path: &Path) -> Result<RawConfig, String> {
    if !path.exists() {
        return Ok(RawConfig::default());
    }

    let contents =
        fs::read_to_string(path).map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    toml::from_str(&contents).map_err(|error| format!("failed to parse {}: {error}", path.display()))
}

fn load_dotenv_map(path: &Path) -> Result<HashMap<String, String>, String> {
    if !path.exists() {
        return Ok(HashMap::new());
    }

    dotenvy::from_path_iter(path)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))?
        .collect::<Result<HashMap<_, _>, _>>()
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))
}

fn raw_from_environment() -> Result<RawConfig, String> {
    raw_from_env_map(env::vars().collect::<HashMap<_, _>>())
}

fn raw_from_env_map(values: HashMap<String, String>) -> Result<RawConfig, String> {
    let enabled = match values.get("OIDC_ENABLED") {
        Some(value) => Some(
            value
                .parse::<bool>()
                .map_err(|_| format!("invalid boolean value for OIDC_ENABLED: {value}"))?,
        ),
        None => None,
    };

    Ok(RawConfig {
        server: Some(RawServerConfig {
            listen_addr: values.get("LISTEN_ADDR").cloned(),
        }),
        git: Some(RawGitConfig {
            webhook_secret: values.get("GIT_WEBHOOK_SECRET").cloned(),
            base_url: values.get("GIT_BASE_URL").cloned(),
            token: values.get("GIT_TOKEN").cloned(),
        }),
        oidc: Some(RawOidcConfig {
            enabled,
            issuer: values.get("OIDC_ISSUER").cloned(),
            client_id: values.get("OIDC_CLIENT_ID").cloned(),
            client_secret: values.get("OIDC_CLIENT_SECRET").cloned(),
            redirect_uri: values.get("OIDC_REDIRECT_URI").cloned(),
            scopes: values
                .get("OIDC_SCOPES")
                .map(|value| value.split_whitespace().map(str::to_string).collect::<Vec<_>>()),
            state_signing_secret: values.get("OIDC_STATE_SIGNING_SECRET").cloned(),
        }),
    })
}
