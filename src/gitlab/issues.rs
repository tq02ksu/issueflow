use gitlab::api::AsyncQuery;

use crate::config::GitConfig;

#[derive(Debug)]
pub struct CreateIssueInput {
    pub project_id: u64,
    pub title: String,
    pub description: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreatedIssue {
    pub id: u64,
    pub iid: u64,
    pub project_id: u64,
    pub title: String,
    pub web_url: String,
}

pub async fn create_issue(
    git_config: &GitConfig,
    input: CreateIssueInput,
) -> Result<CreatedIssue, String> {
    let base_url = git_config
        .base_url
        .as_deref()
        .ok_or("missing git.base_url configuration")?;
    let token = git_config
        .token
        .as_deref()
        .ok_or("missing git.token configuration")?;

    let client = gitlab::GitlabBuilder::new(base_url, token)
        .build_async()
        .await
        .map_err(|e| format!("failed to create GitLab client: {e}"))?;

    let endpoint = gitlab::api::projects::issues::CreateIssue::builder()
        .project(input.project_id)
        .title(&input.title)
        .description(&input.description)
        .build()
        .map_err(|e| format!("failed to build issue create endpoint: {e}"))?;

    let issue: gitlab::types::Issue = endpoint
        .query_async(&client)
        .await
        .map_err(|e| format!("failed to create gitlab issue: {e}"))?;

    Ok(CreatedIssue {
        id: issue.id.value(),
        iid: issue.iid.value(),
        project_id: issue.project_id.value(),
        title: issue.title,
        web_url: issue.web_url,
    })
}
