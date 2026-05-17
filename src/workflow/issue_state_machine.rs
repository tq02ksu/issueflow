use super::types::InvalidTransition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IssueStage {
    New,
    Triaging,
    NeedsInfo,
    Validated,
    AwaitingStartCommand,
    MrOpened,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IssueEvent {
    Triage,
    NeedsInfo,
    Validate,
    AwaitStartCommand,
    StartDev,
}

pub fn next_issue_stage(
    stage: IssueStage,
    event: IssueEvent,
) -> Result<IssueStage, InvalidTransition> {
    match (stage, event) {
        (IssueStage::New, IssueEvent::Triage) => Ok(IssueStage::Triaging),
        (IssueStage::Triaging, IssueEvent::NeedsInfo) => Ok(IssueStage::NeedsInfo),
        (IssueStage::NeedsInfo, IssueEvent::Triage) => Ok(IssueStage::Triaging),
        (IssueStage::NeedsInfo, IssueEvent::Validate) => Ok(IssueStage::Validated),
        (IssueStage::Triaging, IssueEvent::Validate) => Ok(IssueStage::Validated),
        (IssueStage::Validated, IssueEvent::AwaitStartCommand) => {
            Ok(IssueStage::AwaitingStartCommand)
        }
        (IssueStage::AwaitingStartCommand, IssueEvent::StartDev) => Ok(IssueStage::MrOpened),
        _ => Err(InvalidTransition::new(
            "issue",
            stage_name(stage),
            event_name(event),
        )),
    }
}

fn stage_name(stage: IssueStage) -> &'static str {
    match stage {
        IssueStage::New => "new",
        IssueStage::Triaging => "triaging",
        IssueStage::NeedsInfo => "needs-info",
        IssueStage::Validated => "validated",
        IssueStage::AwaitingStartCommand => "awaiting-start-command",
        IssueStage::MrOpened => "mr-opened",
    }
}

fn event_name(event: IssueEvent) -> &'static str {
    match event {
        IssueEvent::Triage => "triage",
        IssueEvent::NeedsInfo => "needs_info",
        IssueEvent::Validate => "validate",
        IssueEvent::AwaitStartCommand => "await_start_command",
        IssueEvent::StartDev => "start_dev",
    }
}
