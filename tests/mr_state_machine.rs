use issueflow::workflow::mr_state_machine::{next_mr_stage, MrEvent, MrStage};

#[test]
fn mr_state_machine_allows_confirm_from_awaiting_plan() {
    let next = next_mr_stage(MrStage::AwaitingPlanConfirm, MrEvent::ConfirmPlan).unwrap();
    assert_eq!(next, MrStage::ApprovedForDev);
}
