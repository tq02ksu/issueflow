use issueflow::workflow::issue_state_machine::{next_issue_stage, IssueEvent, IssueStage};

#[test]
fn issue_state_machine_allows_start_dev_from_validated() {
    let next = next_issue_stage(IssueStage::Validated, IssueEvent::StartDev).unwrap();
    assert_eq!(next, IssueStage::MrOpened);
}
