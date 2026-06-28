use serde_json::json;
use std::{env, path::PathBuf, sync::Mutex};

use tempfile::tempdir;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn original_dir() -> PathBuf {
    env::current_dir().unwrap()
}

#[test]
fn load_system_prompt_from_fixed_repository_file() {
    let prompt = issueflow::agent::prompt::load_system_prompt().unwrap();
    assert!(!prompt.trim().is_empty());
}

#[test]
fn load_system_prompt_does_not_depend_on_current_working_directory() {
    let _guard = ENV_LOCK.lock().unwrap();
    let original_dir = original_dir();
    let temp = tempdir().unwrap();

    env::set_current_dir(temp.path()).unwrap();
    let prompt = issueflow::agent::prompt::load_system_prompt().unwrap();
    env::set_current_dir(original_dir).unwrap();

    assert!(!prompt.trim().is_empty());
}

#[test]
fn render_system_prompt_includes_workbench_context() {
    let rendered =
        issueflow::agent::prompt::render_system_prompt(&issueflow::agent::prompt::PromptContext {
            workbench_name: "Issueflow".into(),
            project_id: 42,
            project_name: "issueflow".into(),
            project_path: "group/issueflow".into(),
        })
        .unwrap();

    assert!(rendered.contains("Issueflow"));
    assert!(rendered.contains("42"));
    assert!(rendered.contains("group/issueflow"));
}

#[test]
fn build_model_messages_prepends_system_prompt() {
    let prompt = "You are the issueflow agent.";
    let prompted = issueflow::agent::prompt::build_model_messages(
        prompt,
        vec![json!({
            "role": "user",
            "content": "Create an issue for the login bug"
        })],
    );

    assert_eq!(prompted.len(), 2);
    assert_eq!(prompted[0]["role"], "system");
    assert_eq!(prompted[0]["content"], prompt);
    assert_eq!(prompted[1]["role"], "user");
}
