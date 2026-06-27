use super::types::InvalidTransition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MrStage {
    DraftPlan,
    AwaitingPlanConfirm,
    ApprovedForDev,
    InDev,
    Verifying,
    Done,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MrEvent {
    PlanReady,
    ConfirmPlan,
    StartImplement,
    StartVerify,
    VerifyPassed,
}

pub fn next_mr_stage(stage: MrStage, event: MrEvent) -> Result<MrStage, InvalidTransition> {
    match (stage, event) {
        (MrStage::DraftPlan, MrEvent::PlanReady) => Ok(MrStage::AwaitingPlanConfirm),
        (MrStage::AwaitingPlanConfirm, MrEvent::ConfirmPlan) => Ok(MrStage::ApprovedForDev),
        (MrStage::ApprovedForDev, MrEvent::StartImplement) => Ok(MrStage::InDev),
        (MrStage::InDev, MrEvent::StartVerify) => Ok(MrStage::Verifying),
        (MrStage::Verifying, MrEvent::VerifyPassed) => Ok(MrStage::Done),
        _ => Err(InvalidTransition::new(
            "mr",
            stage_name(stage),
            event_name(event),
        )),
    }
}

fn stage_name(stage: MrStage) -> &'static str {
    match stage {
        MrStage::DraftPlan => "draft-plan",
        MrStage::AwaitingPlanConfirm => "awaiting-plan-confirm",
        MrStage::ApprovedForDev => "approved-for-dev",
        MrStage::InDev => "in-dev",
        MrStage::Verifying => "verifying",
        MrStage::Done => "done",
    }
}

fn event_name(event: MrEvent) -> &'static str {
    match event {
        MrEvent::PlanReady => "plan_ready",
        MrEvent::ConfirmPlan => "confirm_plan",
        MrEvent::StartImplement => "implement_started",
        MrEvent::StartVerify => "verify_started",
        MrEvent::VerifyPassed => "verify_passed",
    }
}
