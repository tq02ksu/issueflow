use std::{env, fs, path::PathBuf, sync::Mutex};

use issueflow::config::Config;
use tempfile::tempdir;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn original_dir() -> PathBuf {
    env::current_dir().unwrap()
}

#[tokio::test]
async fn config_prefers_environment_over_dotenv_and_toml() {
    let _guard = ENV_LOCK.lock().unwrap();
    let original_dir = original_dir();
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(
        config_dir.join("issueflow.toml"),
        r#"
[server]
listen_addr = "127.0.0.1:9000"

[git]
webhook_secret = "toml-secret"
"#,
    )
    .unwrap();
    fs::write(
        temp.path().join(".env"),
        "LISTEN_ADDR=127.0.0.1:9100\nGIT_WEBHOOK_SECRET=dotenv-secret\n",
    )
    .unwrap();

    env::set_current_dir(temp.path()).unwrap();
    unsafe {
        env::set_var("LISTEN_ADDR", "127.0.0.1:9200");
        env::set_var("GIT_WEBHOOK_SECRET", "env-secret");
    }

    let config = Config::load().await.unwrap();

    assert_eq!(config.listen_addr, "127.0.0.1:9200");
    assert_eq!(config.git.webhook_secret, "env-secret");

    unsafe {
        env::remove_var("LISTEN_ADDR");
        env::remove_var("GIT_WEBHOOK_SECRET");
    }
    env::set_current_dir(original_dir).unwrap();
}

#[tokio::test]
async fn config_loads_gitlab_api_settings_from_dotenv() {
    let _guard = ENV_LOCK.lock().unwrap();
    let original_dir = original_dir();
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join(".env"),
        "GIT_WEBHOOK_SECRET=dotenv-secret\nGIT_BASE_URL=https://gitlab.example.com\nGIT_TOKEN=glpat-abcd1234\n",
    )
    .unwrap();

    env::set_current_dir(temp.path()).unwrap();
    unsafe {
        env::remove_var("GIT_BASE_URL");
        env::remove_var("GIT_TOKEN");
    }

    let config = Config::load().await.unwrap();

    assert_eq!(config.git.webhook_secret, "dotenv-secret");
    assert_eq!(
        config.git.base_url.as_deref(),
        Some("https://gitlab.example.com")
    );
    assert_eq!(config.git.token.as_deref(), Some("glpat-abcd1234"));

    env::set_current_dir(original_dir).unwrap();
}

#[tokio::test]
async fn config_prefers_environment_over_dotenv_for_gitlab_api_settings() {
    let _guard = ENV_LOCK.lock().unwrap();
    let original_dir = original_dir();
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join(".env"),
        "GIT_WEBHOOK_SECRET=dotenv-secret\nGIT_BASE_URL=https://dotenv.example.com\nGIT_TOKEN=dotenv-token\n",
    )
    .unwrap();

    env::set_current_dir(temp.path()).unwrap();
    unsafe {
        env::set_var("GIT_WEBHOOK_SECRET", "env-secret");
        env::set_var("GIT_BASE_URL", "https://env.example.com");
        env::set_var("GIT_TOKEN", "env-token");
    }

    let config = Config::load().await.unwrap();

    assert_eq!(config.git.webhook_secret, "env-secret");
    assert_eq!(
        config.git.base_url.as_deref(),
        Some("https://env.example.com")
    );
    assert_eq!(config.git.token.as_deref(), Some("env-token"));

    unsafe {
        env::remove_var("GIT_WEBHOOK_SECRET");
        env::remove_var("GIT_BASE_URL");
        env::remove_var("GIT_TOKEN");
    }
    env::set_current_dir(original_dir).unwrap();
}

#[tokio::test]
async fn config_uses_defaults_when_optional_sources_are_missing() {
    let _guard = ENV_LOCK.lock().unwrap();
    let original_dir = original_dir();
    let temp = tempdir().unwrap();
    env::set_current_dir(temp.path()).unwrap();
    unsafe {
        env::remove_var("LISTEN_ADDR");
        env::set_var("GIT_WEBHOOK_SECRET", "env-secret");
    }

    let config = Config::load().await.unwrap();

    assert_eq!(config.listen_addr, "127.0.0.1:8080");
    assert_eq!(config.git.webhook_secret, "env-secret");

    unsafe {
        env::remove_var("GIT_WEBHOOK_SECRET");
    }
    env::set_current_dir(original_dir).unwrap();
}

#[tokio::test]
async fn load_config_reads_agent_openai_settings_from_dotenv() {
    let _guard = ENV_LOCK.lock().unwrap();
    let original_dir = original_dir();
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join(".env"),
        "GIT_WEBHOOK_SECRET=test-secret\nAGENT_OPENAI_BASE_URL=https://api.openai.com/v1\nAGENT_OPENAI_API_KEY=sk-test\nAGENT_MODEL=gpt-4o\nAGENT_MAX_TOOL_ROUNDS=9\n",
    )
    .unwrap();

    env::set_current_dir(temp.path()).unwrap();
    let config = Config::load().await.unwrap();
    env::set_current_dir(original_dir).unwrap();

    assert_eq!(
        config.agent.openai_base_url.as_deref(),
        Some("https://api.openai.com/v1")
    );
    assert_eq!(config.agent.openai_api_key.as_deref(), Some("sk-test"));
    assert_eq!(config.agent.model.as_deref(), Some("gpt-4o"));
    assert_eq!(config.agent.max_tool_rounds, 9);
}
