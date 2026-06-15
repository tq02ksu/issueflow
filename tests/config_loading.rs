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
