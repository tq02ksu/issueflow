use issueflow::workflow::issue_state_machine::{next_issue_stage, IssueEvent, IssueStage};
use issueflow::workflow::types::InvalidTransition;

#[test]
fn issue_state_machine_allows_all_task_2_transitions() {
    let cases = [
        (IssueStage::New, IssueEvent::Triage, IssueStage::Triaging),
        (
            IssueStage::Triaging,
            IssueEvent::NeedsInfo,
            IssueStage::NeedsInfo,
        ),
        (IssueStage::NeedsInfo, IssueEvent::Triage, IssueStage::Triaging),
        (
            IssueStage::Triaging,
            IssueEvent::Validate,
            IssueStage::Validated,
        ),
        (
            IssueStage::Validated,
            IssueEvent::StartDev,
            IssueStage::MrOpened,
        ),
    ];

    for (stage, event, expected) in cases {
        let next = next_issue_stage(stage, event).unwrap();
        assert_eq!(next, expected);
    }
}

#[test]
fn issue_state_machine_returns_consistent_invalid_transition_payload() {
    let error = next_issue_stage(IssueStage::New, IssueEvent::Validate).unwrap_err();

    assert_eq!(
        error,
        InvalidTransition {
            machine: "issue",
            stage: "new",
            event: "validate",
        }
    );
}
