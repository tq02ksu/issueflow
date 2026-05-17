use issueflow::workflow::release_state_machine::{
    next_release_stage, ReleaseEvent, ReleaseStage,
};

#[test]
fn release_state_machine_allows_prepare_from_idle() {
    let next = next_release_stage(ReleaseStage::Idle, ReleaseEvent::PrepareRelease).unwrap();
    assert_eq!(next, ReleaseStage::ReleaseChecking);
}
