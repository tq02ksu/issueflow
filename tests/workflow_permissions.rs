use issueflow::workflow::{
    issue_state_machine::IssueStage,
    mr_state_machine::MrStage,
    permissions::{
        GitLabAction, is_issue_action_allowed, is_mr_action_allowed, is_release_action_allowed,
    },
    release_state_machine::ReleaseStage,
};

#[test]
fn issue_cannot_create_merge_request_before_start_dev() {
    assert!(!is_issue_action_allowed(
        IssueStage::New,
        GitLabAction::CreateMergeRequest,
    ));
    assert!(!is_issue_action_allowed(
        IssueStage::Validated,
        GitLabAction::CreateMergeRequest,
    ));
    assert!(!is_issue_action_allowed(
        IssueStage::AwaitingStartCommand,
        GitLabAction::CreateMergeRequest,
    ));
}

#[test]
fn issue_can_create_merge_request_after_start_dev_opens_mr_stage() {
    assert!(is_issue_action_allowed(
        IssueStage::MrOpened,
        GitLabAction::CreateMergeRequest,
    ));
}

#[test]
fn mr_development_actions_are_limited_by_stage() {
    assert!(is_mr_action_allowed(
        MrStage::ApprovedForDev,
        GitLabAction::CreateBranch,
    ));
    assert!(!is_mr_action_allowed(
        MrStage::DraftPlan,
        GitLabAction::PushBranch,
    ));
    assert!(is_mr_action_allowed(
        MrStage::InDev,
        GitLabAction::PushBranch,
    ));
    assert!(!is_mr_action_allowed(
        MrStage::AwaitingPlanConfirm,
        GitLabAction::PushBranch,
    ));
}

#[test]
fn release_publish_requires_ready_for_release_stage() {
    assert!(!is_release_action_allowed(
        ReleaseStage::Idle,
        GitLabAction::PublishRelease,
    ));
    assert!(!is_release_action_allowed(
        ReleaseStage::ReleaseChecking,
        GitLabAction::PublishRelease,
    ));
    assert!(is_release_action_allowed(
        ReleaseStage::ReadyForRelease,
        GitLabAction::PublishRelease,
    ));
}
