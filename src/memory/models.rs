use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryScopeType {
    System,
    Project,
    Workbench,
    Personal,
}

impl MemoryScopeType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            MemoryScopeType::System => "system",
            MemoryScopeType::Project => "project",
            MemoryScopeType::Workbench => "workbench",
            MemoryScopeType::Personal => "personal",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryKind {
    IssueState,
    IssueNote,
    IssueContext,
    PolicyNote,
}

impl MemoryKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            MemoryKind::IssueState => "issue_state",
            MemoryKind::IssueNote => "issue_note",
            MemoryKind::IssueContext => "issue_context",
            MemoryKind::PolicyNote => "policy_note",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct EngineeringMemoryRow {
    pub id: String,
    pub artifact_type: String,
    pub artifact_id: String,
    pub scope_type: String,
    pub scope_key: String,
    pub scope_project_id: Option<i64>,
    pub scope_workbench_id: Option<i64>,
    pub scope_user_id: Option<i64>,
    pub memory_kind: String,
    pub status: String,
    pub revision: i64,
    pub updated_by_user_id: Option<i64>,
    pub input_text: String,
    pub input_context: String,
    pub source_snapshot: Option<String>,
    pub spec: String,
    pub validation_suggestions: String,
    pub risk_notes: String,
    pub evaluation_summary: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct UpsertEngineeringMemoryInput {
    pub id: String,
    pub artifact_type: String,
    pub artifact_id: String,
    pub scope_type: MemoryScopeType,
    pub scope_project_id: Option<i64>,
    pub scope_workbench_id: Option<i64>,
    pub scope_user_id: Option<i64>,
    pub memory_kind: MemoryKind,
    pub status: String,
    pub updated_by_user_id: Option<i64>,
    pub input_text: String,
    pub input_context: String,
    pub source_snapshot: Option<String>,
    pub spec: String,
    pub validation_suggestions: String,
    pub risk_notes: String,
    pub evaluation_summary: String,
}

impl UpsertEngineeringMemoryInput {
    pub fn validate(&self) -> Result<(), AppError> {
        match self.scope_type {
            MemoryScopeType::System => {
                if self.scope_project_id.is_some()
                    || self.scope_workbench_id.is_some()
                    || self.scope_user_id.is_some()
                {
                    return Err(AppError::BadRequest(
                        "system scope cannot carry project, workbench, or user ids".into(),
                    ));
                }
            }
            MemoryScopeType::Project => {
                if self.scope_project_id.is_none()
                    || self.scope_workbench_id.is_some()
                    || self.scope_user_id.is_some()
                {
                    return Err(AppError::BadRequest(
                        "project scope requires project id only".into(),
                    ));
                }
            }
            MemoryScopeType::Workbench => {
                if self.scope_workbench_id.is_none() || self.scope_user_id.is_some() {
                    return Err(AppError::BadRequest(
                        "workbench scope requires workbench id and no user id".into(),
                    ));
                }
            }
            MemoryScopeType::Personal => {
                if self.scope_user_id.is_none() {
                    return Err(AppError::BadRequest(
                        "personal scope requires user id".into(),
                    ));
                }
            }
        }

        Ok(())
    }

    pub fn scope_key(&self) -> Result<String, AppError> {
        self.validate()?;

        match self.scope_type {
            MemoryScopeType::System => Ok("system".to_string()),
            MemoryScopeType::Project => self
                .scope_project_id
                .map(|project_id| format!("project:{project_id}"))
                .ok_or_else(|| AppError::BadRequest("project scope requires project id".into())),
            MemoryScopeType::Workbench => self
                .scope_workbench_id
                .map(|workbench_id| format!("workbench:{workbench_id}"))
                .ok_or_else(|| {
                    AppError::BadRequest("workbench scope requires workbench id".into())
                }),
            MemoryScopeType::Personal => Ok(match self.scope_project_id {
                Some(project_id) => format!(
                    "personal:{project_id}:{}",
                    self.scope_user_id.ok_or_else(|| AppError::BadRequest(
                        "personal scope requires user id".into()
                    ))?
                ),
                None => format!(
                    "personal:{}",
                    self.scope_user_id.ok_or_else(|| AppError::BadRequest(
                        "personal scope requires user id".into()
                    ))?
                ),
            }),
        }
    }
}
