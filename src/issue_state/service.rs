use crate::gitlab::issues::IssueNote;

use super::models::{
    HeavyAgentDecision, IssueStateEvaluation, IssueStateKind, IssueStateRoleNotes,
};

pub struct IssueStateInput<'a> {
    pub current_state: &'a str,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub acceptance_criteria: &'a [String],
    pub verification_notes: &'a [String],
    pub notes: &'a [IssueNote],
}

pub fn evaluate_issue_state(input: IssueStateInput<'_>) -> IssueStateEvaluation {
    let description = input.description.unwrap_or("").trim();
    let has_description = !description.is_empty();
    let has_acceptance_criteria = !input.acceptance_criteria.is_empty();
    let has_verification_notes = !input.verification_notes.is_empty();
    let note_bodies = input
        .notes
        .iter()
        .map(|note| note.body.to_lowercase())
        .collect::<Vec<_>>();

    let proposed_next_state = if contains_keywords(&note_bodies, &["done", "completed", "merged"]) {
        IssueStateKind::Done
    } else if contains_keywords(&note_bodies, &["blocked", "waiting", "on hold"]) {
        IssueStateKind::Blocked
    } else if contains_keywords(&note_bodies, &["wip", "in progress", "started", "mr !"]) {
        IssueStateKind::InExecution
    } else if !has_description {
        IssueStateKind::Clarifying
    } else if has_acceptance_criteria && has_verification_notes {
        IssueStateKind::ReadyForExecution
    } else {
        IssueStateKind::Planned
    };

    let mut missing_context = Vec::new();
    let mut role_notes = IssueStateRoleNotes::default();

    if proposed_next_state == IssueStateKind::Clarifying {
        missing_context
            .push("Issue description is missing concrete execution context.".to_string());
        role_notes
            .product
            .push("Clarify the user problem, expected outcome, and scope boundaries.".to_string());
    }
    if has_description && !has_acceptance_criteria {
        missing_context.push("Acceptance criteria are missing.".to_string());
        role_notes
            .product
            .push("Add explicit acceptance criteria before handing off execution.".to_string());
    }
    if has_description && has_acceptance_criteria && !has_verification_notes {
        missing_context.push("Verification plan is missing.".to_string());
        role_notes
            .engineering
            .push("Add the tests or verification evidence required for delivery.".to_string());
    }
    if proposed_next_state == IssueStateKind::ReadyForExecution {
        role_notes.engineering.push(
            "Implementation can be delegated once the execution owner is chosen.".to_string(),
        );
        role_notes.delivery.push(
            "Transition to execution after explicit confirmation of owner and priority."
                .to_string(),
        );
    }

    let heavy_agent = if proposed_next_state == IssueStateKind::ReadyForExecution {
        HeavyAgentDecision {
            required: false,
            reason: "Execution can be delegated to a heavy coding agent after confirmation."
                .to_string(),
            preferred_implementation: None,
        }
    } else {
        HeavyAgentDecision {
            required: false,
            reason: String::new(),
            preferred_implementation: None,
        }
    };

    IssueStateEvaluation {
        current_state: input.current_state.to_string(),
        proposed_next_state: proposed_next_state.as_str().to_string(),
        summary: if proposed_next_state == IssueStateKind::ReadyForExecution {
            format!(
                "Issue \"{}\" has enough context, acceptance criteria, and verification guidance to start execution.",
                input.title
            )
        } else if proposed_next_state == IssueStateKind::Clarifying {
            format!(
                "Issue \"{}\" needs clarification before planning.",
                input.title
            )
        } else if proposed_next_state == IssueStateKind::Blocked {
            format!("Issue \"{}\" is currently blocked.", input.title)
        } else if proposed_next_state == IssueStateKind::InExecution {
            format!("Issue \"{}\" is already in execution.", input.title)
        } else if proposed_next_state == IssueStateKind::Done {
            format!("Issue \"{}\" is already done.", input.title)
        } else {
            format!(
                "Issue \"{}\" has enough context to enter planning, but still needs clearer delivery standards.",
                input.title
            )
        },
        missing_context,
        blockers: Vec::new(),
        role_notes,
        heavy_agent,
    }
}

fn contains_keywords(texts: &[String], keywords: &[&str]) -> bool {
    texts.iter().any(|text| {
        keywords
            .iter()
            .any(|keyword| text.contains(&keyword.to_lowercase()))
    })
}
