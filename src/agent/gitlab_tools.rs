use crate::{
    error::AppError,
    gitlab::{issues, repository, wiki},
    http::routes::AppState,
    session::Session,
};

pub fn tool_definitions() -> Vec<serde_json::Value> {
    vec![
        tool(
            "create_issue",
            "Create a GitLab issue in the given project",
            &[
                ("project_id", "integer", "GitLab project ID"),
                ("title", "string", "Issue title"),
                ("description", "string", "Issue description"),
            ],
            &["project_id", "title", "description"],
        ),
        tool(
            "update_issue",
            "Update a GitLab issue",
            &[
                ("project_id", "integer", ""),
                ("issue_iid", "integer", "Issue IID (not global ID)"),
                ("title", "string", ""),
                ("description", "string", ""),
                ("state_event", "string", "close to close the issue"),
            ],
            &["project_id", "issue_iid"],
        ),
        tool(
            "delete_issue",
            "Delete a GitLab issue. DANGER: this is destructive",
            &[("project_id", "integer", ""), ("issue_iid", "integer", "")],
            &["project_id", "issue_iid"],
        ),
        tool(
            "list_issues",
            "List GitLab issues in a project",
            &[
                ("project_id", "integer", ""),
                ("state", "string", "opened, closed, or all"),
            ],
            &["project_id"],
        ),
        tool(
            "get_issue",
            "Get a single GitLab issue by IID",
            &[("project_id", "integer", ""), ("issue_iid", "integer", "")],
            &["project_id", "issue_iid"],
        ),
        tool(
            "list_wiki_pages",
            "List wiki pages in a project",
            &[("project_id", "integer", "")],
            &["project_id"],
        ),
        tool(
            "get_wiki_page",
            "Get a wiki page by slug",
            &[
                ("project_id", "integer", ""),
                ("slug", "string", "Wiki page slug"),
            ],
            &["project_id", "slug"],
        ),
        tool(
            "get_repo_file",
            "Read AGENTS.md or other trusted repository documentation files",
            &[
                ("project_id", "integer", ""),
                ("file_path", "string", "File path like AGENTS.md"),
                ("ref", "string", "Git ref like main or master"),
            ],
            &["project_id", "file_path"],
        ),
    ]
}

fn tool(
    name: &str,
    description: &str,
    props: &[(&str, &str, &str)],
    required: &[&str],
) -> serde_json::Value {
    let properties: serde_json::Map<String, serde_json::Value> = props
        .iter()
        .map(|(n, t, d)| {
            (
                n.to_string(),
                serde_json::json!({"type": t, "description": d}),
            )
        })
        .collect();
    serde_json::json!({
        "type": "function",
        "function": {
            "name": name,
            "description": description,
            "parameters": {
                "type": "object",
                "properties": properties,
                "required": required
            }
        }
    })
}

pub fn is_allowed_repo_path(path: &str) -> bool {
    matches!(
        path,
        "AGENTS.md" | "README.md" | "docs/CONFIG.md" | "docs/CONFIG_zh.md"
    )
}

pub async fn execute_tool(
    name: &str,
    arguments: serde_json::Value,
    state: &AppState,
    session: &Session,
) -> Result<serde_json::Value, AppError> {
    let base_url = state
        .config
        .git
        .base_url
        .as_deref()
        .unwrap_or("https://gitlab.com");

    let user_token = &session.access_token;

    match name {
        "create_issue" => {
            let project_id = arg_u64(&arguments, "project_id")?;
            let title = arg_str(&arguments, "title")?;
            let description = arg_str(&arguments, "description").unwrap_or_default();
            let input = issues::CreateIssueInput {
                project_id,
                title: title.to_string(),
                description: description.to_string(),
            };
            let created = issues::create_issue(base_url, user_token, input)
                .await
                .map_err(|e| AppError::Internal(e.into()))?;
            Ok(serde_json::json!({
                "id": created.id,
                "iid": created.iid,
                "web_url": created.web_url,
                "title": created.title,
            }))
        }

        "list_issues" => {
            let project_id = arg_u64(&arguments, "project_id")?;
            let state_val = arg_str(&arguments, "state").unwrap_or("opened");
            let issues = issues::list_issues(base_url, user_token, project_id, state_val)
                .await
                .map_err(|e| AppError::Internal(e.into()))?;
            Ok(serde_json::json!(issues))
        }

        "get_issue" => {
            let project_id = arg_u64(&arguments, "project_id")?;
            let issue_iid = arg_u64(&arguments, "issue_iid")?;
            let issue = issues::get_issue(base_url, user_token, project_id, issue_iid)
                .await
                .map_err(|e| AppError::Internal(e.into()))?;
            Ok(serde_json::json!(issue))
        }

        "update_issue" | "delete_issue" => Err(AppError::BadRequest(format!(
            "tool {name} is defined but requires authenticated GitLab API — not yet wired for agent use"
        ))),

        "list_wiki_pages" => {
            let project_id = arg_u64(&arguments, "project_id")?;
            let pages = wiki::list_wiki_pages(base_url, user_token, project_id)
                .await
                .map_err(|e| AppError::Internal(e.into()))?;
            Ok(serde_json::json!(pages))
        }

        "get_wiki_page" => {
            let project_id = arg_u64(&arguments, "project_id")?;
            let slug = arg_str(&arguments, "slug")?;
            let page = wiki::get_wiki_page(base_url, user_token, project_id, slug)
                .await
                .map_err(|e| AppError::Internal(e.into()))?;
            Ok(serde_json::json!(page))
        }

        "get_repo_file" => {
            let project_id = arg_u64(&arguments, "project_id")?;
            let file_path = arg_str(&arguments, "file_path")?;
            if !is_allowed_repo_path(file_path) {
                return Err(AppError::BadRequest(format!(
                    "reading {file_path} is not allowed. Allowed files: AGENTS.md, README.md, docs/CONFIG.md, docs/CONFIG_zh.md"
                )));
            }
            let reference = arg_str(&arguments, "ref").unwrap_or("main");
            let content =
                repository::get_repo_file(base_url, user_token, project_id, file_path, reference)
                    .await
                    .map_err(|e| AppError::Internal(e.into()))?;
            Ok(serde_json::json!(content))
        }

        _ => Err(AppError::BadRequest(format!("unknown tool: {name}"))),
    }
}

fn arg_str<'a>(args: &'a serde_json::Value, name: &str) -> Result<&'a str, AppError> {
    args.get(name)
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest(format!("missing argument: {name}")))
}

fn arg_u64(args: &serde_json::Value, name: &str) -> Result<u64, AppError> {
    args.get(name)
        .and_then(|v| v.as_u64())
        .ok_or_else(|| AppError::BadRequest(format!("missing or invalid argument: {name}")))
}
