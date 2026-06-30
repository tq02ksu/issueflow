use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    actions::{
        self,
        models::{CreatePendingActionInput, PendingActionRow},
        store as action_store,
    },
    agent::{models::ExecutePendingActionInput, runs},
    error::AppError,
    http::routes::AppState,
    issue_state,
    memory::{
        models::{EngineeringMemoryRow, MemoryKind, MemoryScopeType, UpsertEngineeringMemoryInput},
        store as memory_store,
    },
    session::Session,
};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingActionDetailResponse {
    pub action: PendingActionRow,
    pub preview: Option<PendingActionPreview>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingActionPreview {
    pub kind: String,
    pub title: String,
    pub body: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshEngineeringMemoryRequest {
    pub project_id: i64,
    pub artifact_type: String,
    pub artifact_id: String,
    pub input_text: String,
    pub input_context: serde_json::Value,
    pub spec: serde_json::Value,
    pub validation_suggestions: serde_json::Value,
    pub risk_notes: serde_json::Value,
    pub evaluation_summary: serde_json::Value,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePendingActionRequest {
    pub project_id: i64,
    pub artifact_type: String,
    pub artifact_id: String,
    pub action_type: String,
    pub payload: serde_json::Value,
    #[serde(default)]
    pub source_session_id: Option<String>,
    #[serde(default)]
    pub source_run_id: Option<String>,
    #[serde(default)]
    pub assigned_user_id: Option<i64>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueStateResponse {
    pub workbench_context: EngineeringMemoryRow,
    pub project_state: EngineeringMemoryRow,
    pub pending_action: PendingActionRow,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueStateDetailResponse {
    pub project_memory: Option<EngineeringMemoryRow>,
    pub personal_note: Option<EngineeringMemoryRow>,
    pub pending_action: Option<PendingActionRow>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApplyIssueStateTransitionPayload {
    current_state: String,
    proposed_next_state: String,
    target_issue: ApplyIssueTarget,
    source_memory_id: String,
    transition_summary: String,
    heavy_agent: issue_state::models::HeavyAgentDecision,
    write_back: ApplyIssueWriteBack,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApplyIssueTarget {
    project_id: i64,
    issue_iid: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApplyIssueWriteBack {
    mode: String,
    section_title: String,
    rendered_markdown: String,
    comment_body: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct WorkbenchAccessRow {
    project_id: i64,
}

pub async fn refresh_engineering_memory(
    State(state): State<AppState>,
    session: Session,
    Path(workbench_id): Path<i64>,
    Json(payload): Json<RefreshEngineeringMemoryRequest>,
) -> Result<Json<EngineeringMemoryRow>, AppError> {
    ensure_workbench_owner(&state, workbench_id, session.user_id).await?;

    let row = memory_store::upsert_engineering_memory(
        &state.pool,
        &UpsertEngineeringMemoryInput {
            id: Uuid::new_v4().to_string(),
            artifact_type: payload.artifact_type,
            artifact_id: payload.artifact_id,
            scope_type: MemoryScopeType::Project,
            scope_project_id: Some(payload.project_id),
            scope_workbench_id: None,
            scope_user_id: None,
            memory_kind: MemoryKind::IssueContext,
            status: "draft".to_string(),
            updated_by_user_id: Some(session.user_id),
            input_text: payload.input_text,
            input_context: serde_json::to_string(&payload.input_context)?,
            source_snapshot: None,
            spec: serde_json::to_string(&payload.spec)?,
            validation_suggestions: serde_json::to_string(&payload.validation_suggestions)?,
            risk_notes: serde_json::to_string(&payload.risk_notes)?,
            evaluation_summary: serde_json::to_string(&payload.evaluation_summary)?,
        },
    )
    .await?;

    Ok(Json(row))
}

pub async fn evaluate_issue_state(
    State(state): State<AppState>,
    session: Session,
    Path((workbench_id, issue_iid)): Path<(i64, u64)>,
) -> Result<Json<IssueStateResponse>, AppError> {
    let workbench = ensure_workbench_owner_and_load(&state, workbench_id, session.user_id).await?;
    let artifact_id = issue_iid.to_string();
    let existing_context = memory_store::find_engineering_memory(
        &state.pool,
        &format!("workbench:{workbench_id}"),
        "issue",
        &artifact_id,
        MemoryKind::IssueContext.as_str(),
    )
    .await?;
    let existing_project_state = memory_store::find_engineering_memory(
        &state.pool,
        &format!("project:{}", workbench.project_id),
        "issue",
        &artifact_id,
        MemoryKind::IssueState.as_str(),
    )
    .await?;

    let context_spec = existing_context
        .as_ref()
        .map(parse_issue_context_spec)
        .transpose()?
        .unwrap_or_default();
    let title = context_spec
        .summary
        .clone()
        .or_else(|| {
            existing_context
                .as_ref()
                .and_then(|context| extract_title_from_input(&context.input_text))
        })
        .unwrap_or_else(|| format!("Issue #{issue_iid}"));
    let description = existing_context.as_ref().and_then(|context| {
        let trimmed = context.input_text.trim();
        (!trimmed.is_empty()).then_some(trimmed)
    });
    let current_state = existing_project_state
        .as_ref()
        .and_then(extract_current_issue_state)
        .unwrap_or_else(|| {
            issue_state::models::IssueStateKind::New
                .as_str()
                .to_string()
        });
    let evaluation =
        issue_state::service::evaluate_issue_state(issue_state::service::IssueStateInput {
            current_state: &current_state,
            title: &title,
            description,
            acceptance_criteria: &context_spec.acceptance_criteria,
            verification_notes: &context_spec.verification_notes,
            notes: &[],
        });

    let workbench_context = if let Some(context) = existing_context {
        context
    } else {
        memory_store::upsert_engineering_memory(
            &state.pool,
            &UpsertEngineeringMemoryInput {
                id: Uuid::new_v4().to_string(),
                artifact_type: "issue".to_string(),
                artifact_id: artifact_id.clone(),
                scope_type: MemoryScopeType::Workbench,
                scope_project_id: Some(workbench.project_id),
                scope_workbench_id: Some(workbench_id),
                scope_user_id: None,
                memory_kind: MemoryKind::IssueContext,
                status: "active".to_string(),
                updated_by_user_id: Some(session.user_id),
                input_text: title.clone(),
                input_context: serde_json::to_string(&serde_json::json!({
                    "source": "issue_state_evaluate"
                }))?,
                source_snapshot: None,
                spec: serde_json::to_string(&serde_json::json!({
                    "summary": title,
                    "acceptance_criteria": [],
                    "boundary_conditions": [],
                    "open_questions": evaluation.missing_context.clone(),
                    "verification_notes": [],
                }))?,
                validation_suggestions: serde_json::to_string(&serde_json::json!({
                    "happy_path": [],
                    "failure_path": [],
                    "edge_cases": [],
                    "non_goals": [],
                }))?,
                risk_notes: "[]".to_string(),
                evaluation_summary: serde_json::to_string(&serde_json::json!({
                    "status": "captured",
                    "summary": "Workbench state context captured."
                }))?,
            },
        )
        .await?
    };

    let project_state = memory_store::upsert_engineering_memory(
        &state.pool,
        &UpsertEngineeringMemoryInput {
            id: Uuid::new_v4().to_string(),
            artifact_type: "issue".to_string(),
            artifact_id: artifact_id.clone(),
            scope_type: MemoryScopeType::Project,
            scope_project_id: Some(workbench.project_id),
            scope_workbench_id: None,
            scope_user_id: None,
            memory_kind: MemoryKind::IssueState,
            status: "active".to_string(),
            updated_by_user_id: Some(session.user_id),
            input_text: workbench_context.input_text.clone(),
            input_context: serde_json::to_string(&serde_json::json!({
                "source": "issue_state_evaluate",
                "workbenchContextId": workbench_context.id,
            }))?,
            source_snapshot: workbench_context.source_snapshot.clone(),
            spec: serde_json::to_string(&serde_json::json!({
                "summary": evaluation.summary.clone(),
                "acceptance_criteria": context_spec.acceptance_criteria.clone(),
                "boundary_conditions": [],
                "open_questions": evaluation.missing_context.clone(),
                "verification_notes": context_spec.verification_notes.clone(),
            }))?,
            validation_suggestions: serde_json::to_string(&serde_json::json!({
                "happy_path": [],
                "failure_path": [],
                "edge_cases": [],
                "non_goals": [],
            }))?,
            risk_notes: "[]".to_string(),
            evaluation_summary: serde_json::to_string(&evaluation)?,
        },
    )
    .await?;

    let pending_action = action_store::insert_pending_action(
        &state.pool,
        &CreatePendingActionInput {
            id: Uuid::new_v4().to_string(),
            workbench_id,
            project_id: workbench.project_id,
            artifact_type: "issue".to_string(),
            artifact_id: issue_iid.to_string(),
            action_type: "apply_issue_state_transition".to_string(),
            payload: serde_json::to_string(&ApplyIssueStateTransitionPayload {
                current_state: evaluation.current_state.clone(),
                proposed_next_state: evaluation.proposed_next_state.clone(),
                target_issue: ApplyIssueTarget {
                    project_id: workbench.project_id,
                    issue_iid,
                },
                source_memory_id: project_state.id.clone(),
                transition_summary: evaluation.summary.clone(),
                heavy_agent: evaluation.heavy_agent.clone(),
                write_back: ApplyIssueWriteBack {
                    mode: "description_section".to_string(),
                    section_title: "Work Item State".to_string(),
                    rendered_markdown: format!(
                        "## Work Item State\n\nCurrent: {}\n\nProposed next: {}\n\nSummary: {}\n",
                        evaluation.current_state,
                        evaluation.proposed_next_state,
                        evaluation.summary
                    ),
                    comment_body: None,
                },
            })?,
            source_session_id: None,
            source_run_id: None,
            created_by_user_id: Some(session.user_id),
            assigned_user_id: Some(session.user_id),
        },
    )
    .await?;

    Ok(Json(IssueStateResponse {
        workbench_context,
        project_state,
        pending_action,
    }))
}

pub async fn get_issue_state(
    State(state): State<AppState>,
    session: Session,
    Path((workbench_id, issue_iid)): Path<(i64, u64)>,
) -> Result<Json<IssueStateDetailResponse>, AppError> {
    let workbench = ensure_workbench_owner_and_load(&state, workbench_id, session.user_id).await?;
    let artifact_id = issue_iid.to_string();

    let project_memory = memory_store::find_engineering_memory(
        &state.pool,
        &format!("project:{}", workbench.project_id),
        "issue",
        &artifact_id,
        "issue_state",
    )
    .await?;
    let personal_note = memory_store::find_engineering_memory(
        &state.pool,
        &format!("personal:{}:{}", workbench.project_id, session.user_id),
        "issue",
        &artifact_id,
        "issue_note",
    )
    .await?;
    let pending_action = action_store::find_latest_pending_action_for_issue(
        &state.pool,
        workbench_id,
        workbench.project_id,
        &artifact_id,
        "apply_issue_state_transition",
    )
    .await
    .map_err(AppError::from)?;

    Ok(Json(IssueStateDetailResponse {
        project_memory,
        personal_note,
        pending_action,
    }))
}

pub async fn create_pending_action(
    State(state): State<AppState>,
    session: Session,
    Path(workbench_id): Path<i64>,
    Json(payload): Json<CreatePendingActionRequest>,
) -> Result<(StatusCode, Json<PendingActionRow>), AppError> {
    ensure_workbench_owner(&state, workbench_id, session.user_id).await?;

    let row = action_store::insert_pending_action(
        &state.pool,
        &CreatePendingActionInput {
            id: Uuid::new_v4().to_string(),
            workbench_id,
            project_id: payload.project_id,
            artifact_type: payload.artifact_type,
            artifact_id: payload.artifact_id,
            action_type: payload.action_type,
            payload: serde_json::to_string(&payload.payload)?,
            source_session_id: payload.source_session_id,
            source_run_id: payload.source_run_id,
            created_by_user_id: Some(session.user_id),
            assigned_user_id: payload.assigned_user_id,
        },
    )
    .await?;

    Ok((StatusCode::CREATED, Json(row)))
}

pub async fn list_pending_actions(
    State(state): State<AppState>,
    session: Session,
    Path(workbench_id): Path<i64>,
) -> Result<Json<Vec<PendingActionRow>>, AppError> {
    ensure_workbench_owner(&state, workbench_id, session.user_id).await?;
    let rows = action_store::list_pending_actions(&state.pool, workbench_id).await?;
    Ok(Json(rows))
}

pub async fn confirm_pending_action(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> Result<Json<PendingActionRow>, AppError> {
    let action = action_store::get_pending_action(&state.pool, &id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            other => AppError::from(other),
        })?;

    ensure_workbench_owner(&state, action.workbench_id, session.user_id).await?;

    let row = action_store::confirm_pending_action(&state.pool, &id, session.user_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::Conflict,
            other => AppError::from(other),
        })?;

    if let Some(source_session_id) = row.source_session_id.as_deref() {
        let input_payload = serde_json::to_string(&ExecutePendingActionInput {
            pending_action_id: row.id.clone(),
            access_token: session.access_token,
        })?;
        let run = runs::create_run(&state.pool, source_session_id, None, &input_payload).await?;
        actions::store::attach_executed_run(&state.pool, &row.id, &run.id).await?;
    }

    Ok(Json(row))
}

pub async fn get_pending_action(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> Result<Json<PendingActionDetailResponse>, AppError> {
    let action = action_store::get_pending_action(&state.pool, &id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            other => AppError::from(other),
        })?;

    ensure_workbench_owner(&state, action.workbench_id, session.user_id).await?;

    Ok(Json(PendingActionDetailResponse {
        preview: build_preview(&action),
        action,
    }))
}

async fn ensure_workbench_owner(
    state: &AppState,
    workbench_id: i64,
    user_id: i64,
) -> Result<(), AppError> {
    ensure_workbench_owner_and_load(state, workbench_id, user_id)
        .await
        .map(|_| ())
}

async fn ensure_workbench_owner_and_load(
    state: &AppState,
    workbench_id: i64,
    user_id: i64,
) -> Result<WorkbenchAccessRow, AppError> {
    sqlx::query_as("SELECT id, project_id FROM workbenches WHERE id = ? AND user_id = ?")
        .bind(workbench_id)
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::NotFound)
}

fn build_preview(action: &PendingActionRow) -> Option<PendingActionPreview> {
    match action.action_type.as_str() {
        "update_gitlab_issue" => {
            let payload: serde_json::Value = serde_json::from_str(&action.payload).ok()?;
            let issue_iid = payload
                .get("issueIid")
                .and_then(serde_json::Value::as_u64)?;
            let body = payload
                .get("renderedContent")
                .and_then(|value| value.as_str())
                .unwrap_or_default()
                .to_string();

            Some(PendingActionPreview {
                kind: "gitlab_issue_description".to_string(),
                title: format!("Replace issue #{issue_iid} description"),
                body,
            })
        }
        "publish_gitlab_comment" => {
            let payload: serde_json::Value = serde_json::from_str(&action.payload).ok()?;
            let issue_iid = payload
                .get("issueIid")
                .and_then(serde_json::Value::as_u64)?;
            let body = payload
                .get("body")
                .and_then(|value| value.as_str())
                .unwrap_or_default()
                .to_string();

            Some(PendingActionPreview {
                kind: "gitlab_issue_comment".to_string(),
                title: format!("Publish comment to issue #{issue_iid}"),
                body,
            })
        }
        "apply_issue_state_transition" => {
            let payload: ApplyIssueStateTransitionPayload =
                serde_json::from_str(&action.payload).ok()?;
            Some(PendingActionPreview {
                kind: "work_item_state_transition".to_string(),
                title: format!(
                    "Apply state transition to issue #{}",
                    payload.target_issue.issue_iid
                ),
                body: payload.write_back.rendered_markdown,
            })
        }
        _ => None,
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
struct IssueContextSpec {
    summary: Option<String>,
    acceptance_criteria: Vec<String>,
    verification_notes: Vec<String>,
}

fn parse_issue_context_spec(memory: &EngineeringMemoryRow) -> Result<IssueContextSpec, AppError> {
    serde_json::from_str(&memory.spec)
        .map_err(|e| AppError::BadRequest(format!("invalid issue context spec: {e}")))
}

fn extract_title_from_input(input_text: &str) -> Option<String> {
    input_text
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToOwned::to_owned)
}

fn extract_current_issue_state(memory: &EngineeringMemoryRow) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(&memory.evaluation_summary).ok()?;
    value
        .get("current_state")
        .and_then(|state| state.as_str())
        .or_else(|| value.get("currentState").and_then(|state| state.as_str()))
        .and_then(|state| {
            state
                .parse::<issue_state::models::IssueStateKind>()
                .ok()
                .map(|_| state.to_string())
        })
}
