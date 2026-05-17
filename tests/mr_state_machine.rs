use issueflow::workflow::mr_state_machine::{next_mr_stage, MrEvent, MrStage};

#[test]
fn mr_state_machine_allows_all_task_2_transitions() {
    let cases = [
        (
            MrStage::DraftPlan,
            MrEvent::PlanReady,
            MrStage::AwaitingPlanConfirm,
        ),
        (
            MrStage::AwaitingPlanConfirm,
            MrEvent::ConfirmPlan,
            MrStage::ApprovedForDev,
        ),
        (
            MrStage::ApprovedForDev,
            MrEvent::StartImplement,
            MrStage::InDev,
        ),
        (
            MrStage::InDev,
            MrEvent::StartVerify,
            MrStage::Verifying,
        ),
        (
            MrStage::Verifying,
            MrEvent::VerifyPassed,
            MrStage::Done,
        ),
    ];

    for (stage, event, expected) in cases {
        let next = next_mr_stage(stage, event).unwrap();
        assert_eq!(next, expected);
    }
}
