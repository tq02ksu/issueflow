use std::{fs, path::Path};

use minijinja::Environment;

const SYSTEM_PROMPT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/prompts/agent_system.j2");

#[derive(Debug, Clone)]
pub struct PromptContext {
    pub workbench_name: String,
    pub project_id: i64,
    pub project_name: String,
    pub project_path: String,
}

pub fn load_system_prompt() -> Result<String, String> {
    fs::read_to_string(Path::new(SYSTEM_PROMPT_PATH))
        .map_err(|error| format!("failed to read {SYSTEM_PROMPT_PATH}: {error}"))
}

pub fn render_system_prompt(context: &PromptContext) -> Result<String, String> {
    let source = load_system_prompt()?;
    let mut env = Environment::new();
    env.add_template("agent_system", &source)
        .map_err(|error| format!("failed to parse {SYSTEM_PROMPT_PATH}: {error}"))?;

    env.get_template("agent_system")
        .map_err(|error| format!("failed to load {SYSTEM_PROMPT_PATH}: {error}"))?
        .render(minijinja::context! {
            workbench_name => &context.workbench_name,
            project_id => context.project_id,
            project_name => &context.project_name,
            project_path => &context.project_path,
        })
        .map_err(|error| format!("failed to render {SYSTEM_PROMPT_PATH}: {error}"))
}

pub fn build_model_messages(
    system_prompt: &str,
    messages: Vec<serde_json::Value>,
) -> Vec<serde_json::Value> {
    let mut prompted = Vec::with_capacity(messages.len() + 1);
    prompted.push(serde_json::json!({
        "role": "system",
        "content": system_prompt,
    }));
    prompted.extend(messages);
    prompted
}
