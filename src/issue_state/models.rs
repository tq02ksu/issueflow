use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueStateKind {
    New,
    Clarifying,
    Planned,
    ReadyForExecution,
    InExecution,
    Blocked,
    Done,
}

impl IssueStateKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            IssueStateKind::New => "new",
            IssueStateKind::Clarifying => "clarifying",
            IssueStateKind::Planned => "planned",
            IssueStateKind::ReadyForExecution => "ready_for_execution",
            IssueStateKind::InExecution => "in_execution",
            IssueStateKind::Blocked => "blocked",
            IssueStateKind::Done => "done",
        }
    }
}

impl FromStr for IssueStateKind {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "new" => Ok(Self::New),
            "clarifying" => Ok(Self::Clarifying),
            "planned" => Ok(Self::Planned),
            "ready_for_execution" => Ok(Self::ReadyForExecution),
            "in_execution" => Ok(Self::InExecution),
            "blocked" => Ok(Self::Blocked),
            "done" => Ok(Self::Done),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueStateRoleNotes {
    pub product: Vec<String>,
    pub engineering: Vec<String>,
    pub delivery: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeavyAgentDecision {
    pub required: bool,
    pub reason: String,
    pub preferred_implementation: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueStateEvaluation {
    pub current_state: String,
    pub proposed_next_state: String,
    pub summary: String,
    pub missing_context: Vec<String>,
    pub blockers: Vec<String>,
    pub role_notes: IssueStateRoleNotes,
    pub heavy_agent: HeavyAgentDecision,
}
