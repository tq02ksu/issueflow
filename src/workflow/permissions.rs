use super::{
    issue_state_machine::IssueStage, mr_state_machine::MrStage, release_state_machine::ReleaseStage,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GitLabAction {
    ReadIssue,
    WriteIssueComment,
    TriggerRobotJob,
    CreateBranch,
    PushBranch,
    CreateMergeRequest,
    UpdateMergeRequest,
    RunVerification,
    PrepareRelease,
    PublishRelease,
}

pub fn is_issue_action_allowed(stage: IssueStage, action: GitLabAction) -> bool {
    use GitLabAction::{CreateMergeRequest, TriggerRobotJob, WriteIssueComment};

    match stage {
        IssueStage::New => matches!(action, WriteIssueComment | TriggerRobotJob),
        IssueStage::Triaging => matches!(action, WriteIssueComment | TriggerRobotJob),
        IssueStage::NeedsInfo => matches!(action, WriteIssueComment),
        IssueStage::Validated => matches!(action, WriteIssueComment | TriggerRobotJob),
        IssueStage::AwaitingStartCommand => matches!(action, WriteIssueComment),
        // Issue stage only allows MR creation after explicit start-dev has moved the workflow.
        IssueStage::MrOpened => matches!(
            action,
            WriteIssueComment | TriggerRobotJob | CreateMergeRequest
        ),
    }
}

pub fn is_mr_action_allowed(stage: MrStage, action: GitLabAction) -> bool {
    use GitLabAction::{
        CreateBranch, PushBranch, RunVerification, TriggerRobotJob, UpdateMergeRequest,
        WriteIssueComment,
    };

    match stage {
        MrStage::DraftPlan => matches!(action, WriteIssueComment | TriggerRobotJob),
        MrStage::AwaitingPlanConfirm => matches!(action, WriteIssueComment),
        MrStage::ApprovedForDev => matches!(action, CreateBranch | UpdateMergeRequest),
        MrStage::InDev => matches!(action, PushBranch | UpdateMergeRequest | TriggerRobotJob),
        MrStage::Verifying => matches!(action, RunVerification | UpdateMergeRequest),
        MrStage::Done => matches!(action, WriteIssueComment | UpdateMergeRequest),
    }
}

pub fn is_release_action_allowed(stage: ReleaseStage, action: GitLabAction) -> bool {
    use GitLabAction::{PrepareRelease, PublishRelease, TriggerRobotJob, WriteIssueComment};

    match stage {
        ReleaseStage::Idle => matches!(action, PrepareRelease | TriggerRobotJob),
        ReleaseStage::ReleaseChecking => matches!(action, WriteIssueComment | TriggerRobotJob),
        ReleaseStage::ReadyForRelease => matches!(action, PublishRelease | WriteIssueComment),
        ReleaseStage::Releasing => matches!(action, WriteIssueComment),
        ReleaseStage::Released => matches!(action, WriteIssueComment),
    }
}
