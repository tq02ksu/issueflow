use crate::{
    actions::{
        models::{CreatePendingActionInput, PendingActionRow},
        store as action_store,
    },
    db::DbPool,
    error::AppError,
    gitlab::issues::GitlabIssue,
    memory::{
        models::{EngineeringMemoryRow, UpsertEngineeringMemoryInput},
        store as memory_store,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrepareIssueCommand {
    pub issue_iid: u64,
}

#[derive(Debug)]
pub struct PreparedIssueOutcome {
    pub memory: EngineeringMemoryRow,
    pub pending_action: PendingActionRow,
    pub assistant_message: String,
}

#[derive(Debug)]
struct PreparedIssueDraft {
    input_text: String,
    input_context: serde_json::Value,
    spec: serde_json::Value,
    validation_suggestions: serde_json::Value,
    risk_notes: serde_json::Value,
    evaluation_summary: serde_json::Value,
    rendered_content: String,
    fallback_comment_body: String,
}

pub fn parse_prepare_issue_command(message: &str) -> Option<PrepareIssueCommand> {
    let trimmed = message.trim();
    let rest = trimmed
        .strip_prefix("/prepare-issue")
        .or_else(|| trimmed.strip_prefix("/prepare_issue"))?
        .trim();

    let issue_iid = rest.strip_prefix('#').unwrap_or(rest).parse::<u64>().ok()?;
    Some(PrepareIssueCommand { issue_iid })
}

pub async fn persist_issue_preparation(
    pool: &DbPool,
    workbench_id: i64,
    session_id: &str,
    user_id: i64,
    issue: &GitlabIssue,
) -> Result<PreparedIssueOutcome, AppError> {
    let draft = build_prepared_issue_draft(issue);

    let memory = memory_store::upsert_engineering_memory(
        pool,
        &UpsertEngineeringMemoryInput {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: issue.project_id as i64,
            artifact_type: "issue".to_string(),
            artifact_id: issue.iid.to_string(),
            status: "draft".to_string(),
            updated_by_user_id: Some(user_id),
            input_text: draft.input_text,
            input_context: serde_json::to_string(&draft.input_context)?,
            spec: serde_json::to_string(&draft.spec)?,
            validation_suggestions: serde_json::to_string(&draft.validation_suggestions)?,
            risk_notes: serde_json::to_string(&draft.risk_notes)?,
            evaluation_summary: serde_json::to_string(&draft.evaluation_summary)?,
        },
    )
    .await?;

    let pending_action = action_store::insert_pending_action(
        pool,
        &CreatePendingActionInput {
            id: uuid::Uuid::new_v4().to_string(),
            workbench_id,
            project_id: issue.project_id as i64,
            artifact_type: "issue".to_string(),
            artifact_id: issue.iid.to_string(),
            action_type: "update_gitlab_issue".to_string(),
            payload: serde_json::json!({
                "issueIid": issue.iid,
                "targetField": "description",
                "updateMode": "replace_full",
                "renderedContent": draft.rendered_content,
                "fallbackCommentBody": draft.fallback_comment_body,
            })
            .to_string(),
            source_session_id: Some(session_id.to_string()),
            source_run_id: None,
            created_by_user_id: Some(user_id),
            assigned_user_id: Some(user_id),
        },
    )
    .await?;

    Ok(PreparedIssueOutcome {
        memory,
        pending_action,
        assistant_message: format!(
            "Prepared a draft update for issue #{}. Review it in Pending Actions before writing to GitLab.",
            issue.iid
        ),
    })
}

fn build_prepared_issue_draft(issue: &GitlabIssue) -> PreparedIssueDraft {
    let original_description = issue
        .description
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("No issue description provided.");

    let acceptance_criteria = vec![
        format!(
            "Given the scope described in issue #{} ({}), when the implementation is complete, then the requested user-visible outcome is available in the intended workflow.",
            issue.iid, issue.title
        ),
        "Given the change is deployed, when the primary path is exercised, then the behavior completes without manual workarounds or hidden prerequisite steps."
            .to_string(),
        "Given the issue is reviewed for acceptance, when a teammate reads the updated issue, then they can understand success conditions and key limits without extra clarification."
            .to_string(),
    ];
    let success_conditions = vec![
        "Primary workflow completes successfully.".to_string(),
        "Expected output is visible to the user or caller.".to_string(),
        "No regression is introduced in adjacent behavior.".to_string(),
    ];
    let boundary_conditions = vec![
        "Missing or incomplete input is handled gracefully.".to_string(),
        "Empty-state and permission-related behavior is explicit.".to_string(),
        "Existing data remains safe during updates or retries.".to_string(),
    ];
    let happy_path = vec![
        "Run the main user flow end-to-end with representative data.".to_string(),
        "Verify the expected result appears in the UI or API response.".to_string(),
    ];
    let failure_path = vec![
        "Try invalid or incomplete input and verify the failure mode is clear.".to_string(),
        "Check behavior when the upstream dependency or permission is missing.".to_string(),
    ];
    let edge_cases = vec![
        "Exercise empty-state behavior.".to_string(),
        "Exercise repeated execution or retry behavior.".to_string(),
    ];
    let risk_notes = vec![
        "The original issue may not fully describe non-happy-path expectations.".to_string(),
        "Permission, state-transition, or data-shape edge cases may still need human review."
            .to_string(),
    ];

    let rendered_content = format!(
        "## Original Request\n\n{}\n\n## AI Generated Acceptance Criteria\n\n- {}\n- {}\n- {}\n\n## Success Conditions\n\n- {}\n- {}\n- {}\n\n## Boundary Conditions\n\n- {}\n- {}\n- {}\n\n## AI Test Suggestions\n\n### Happy Path\n- {}\n- {}\n\n### Failure Path\n- {}\n- {}\n\n### Edge Cases\n- {}\n- {}\n\n## Risks\n\n- {}\n- {}\n",
        original_description,
        acceptance_criteria[0],
        acceptance_criteria[1],
        acceptance_criteria[2],
        success_conditions[0],
        success_conditions[1],
        success_conditions[2],
        boundary_conditions[0],
        boundary_conditions[1],
        boundary_conditions[2],
        happy_path[0],
        happy_path[1],
        failure_path[0],
        failure_path[1],
        edge_cases[0],
        edge_cases[1],
        risk_notes[0],
        risk_notes[1],
    );

    PreparedIssueDraft {
        input_text: format!("{}\n\n{}", issue.title, original_description),
        input_context: serde_json::json!({
            "source": "agent_session_prepare_issue",
            "issue": issue,
        }),
        spec: serde_json::json!({
            "summary": format!("Prepared acceptance draft for issue #{}: {}", issue.iid, issue.title),
            "acceptance_criteria": acceptance_criteria,
            "success_conditions": success_conditions,
            "boundary_conditions": boundary_conditions,
            "open_questions": [],
        }),
        validation_suggestions: serde_json::json!({
            "happy_path": happy_path,
            "failure_path": failure_path,
            "edge_cases": edge_cases,
            "non_goals": [],
        }),
        risk_notes: serde_json::json!(risk_notes),
        evaluation_summary: serde_json::json!({
            "status": "draft",
            "summary": "Prepared from current issue context. Pending user confirmation before write-back.",
            "coverage_notes": [],
            "missing_cases": [],
        }),
        fallback_comment_body: rendered_content.clone(),
        rendered_content,
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_prepare_issue_command, persist_issue_preparation};
    use crate::{db, gitlab::issues::GitlabIssue};

    async fn isolated_memory_pool() -> crate::db::DbPool {
        sqlx::any::install_default_drivers();
        let db_url = format!(
            "sqlite:file:{}?mode=memory&cache=shared",
            uuid::Uuid::new_v4()
        );
        let pool = sqlx::AnyPool::connect(&db_url).await.unwrap();
        db::run_migrations(&pool, &db_url).await.unwrap();
        pool
    }

    #[test]
    fn parse_prepare_issue_command_accepts_supported_forms() {
        assert_eq!(
            parse_prepare_issue_command("/prepare-issue 77"),
            Some(super::PrepareIssueCommand { issue_iid: 77 })
        );
        assert_eq!(
            parse_prepare_issue_command("/prepare_issue #88"),
            Some(super::PrepareIssueCommand { issue_iid: 88 })
        );
        assert_eq!(parse_prepare_issue_command("prepare issue 77"), None);
    }

    #[tokio::test]
    async fn persist_issue_preparation_creates_memory_and_pending_action() {
        let pool = isolated_memory_pool().await;
        let user = db::upsert_user(&pool, "user-sub", "Test User", "test@example.com")
            .await
            .unwrap();
        let workbench_id: i64 = sqlx::query_scalar(
            "INSERT INTO workbenches (user_id, project_id, project_name, project_path, name)
             VALUES (?, ?, ?, ?, ?)
             RETURNING id",
        )
        .bind(user.id)
        .bind(123i64)
        .bind("group/project")
        .bind("group/project")
        .bind("Workbench")
        .fetch_one(&pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO agent_sessions (
                id, user_id, workbench_id, title, last_message_at, created_at, updated_at
             ) VALUES (?, ?, ?, 'Test Session', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        )
        .bind("session-1")
        .bind(user.id)
        .bind(workbench_id)
        .execute(&pool)
        .await
        .unwrap();

        let issue = GitlabIssue {
            id: 1001,
            iid: 77,
            project_id: 123,
            title: "Add export button".to_string(),
            description: Some("Users need a CSV export.".to_string()),
            state: "opened".to_string(),
            web_url: "https://gitlab.example.com/group/project/-/issues/77".to_string(),
            milestone: None,
            labels: vec!["feature".to_string()],
            created_at: None,
            updated_at: None,
        };

        let outcome = persist_issue_preparation(&pool, workbench_id, "session-1", user.id, &issue)
            .await
            .unwrap();

        assert_eq!(outcome.memory.project_id, 123);
        assert_eq!(outcome.memory.artifact_id, "77");
        assert!(outcome.memory.spec.contains("Add export button"));
        assert_eq!(outcome.pending_action.action_type, "update_gitlab_issue");
        assert_eq!(
            outcome.pending_action.source_session_id.as_deref(),
            Some("session-1")
        );
        assert!(
            outcome
                .pending_action
                .payload
                .contains("AI Generated Acceptance Criteria")
        );
        assert!(outcome.assistant_message.contains("Pending Actions"));
    }
}
