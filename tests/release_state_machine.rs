use issueflow::workflow::release_state_machine::{
    next_release_stage, ReleaseEvent, ReleaseStage,
};

#[test]
fn release_state_machine_allows_all_task_2_transitions() {
    let cases = [
        (
            ReleaseStage::Idle,
            ReleaseEvent::PrepareRelease,
            ReleaseStage::ReleaseChecking,
        ),
        (
            ReleaseStage::ReleaseChecking,
            ReleaseEvent::ReleaseReady,
            ReleaseStage::ReadyForRelease,
        ),
        (
            ReleaseStage::ReadyForRelease,
            ReleaseEvent::Publish,
            ReleaseStage::Releasing,
        ),
        (
            ReleaseStage::Releasing,
            ReleaseEvent::Released,
            ReleaseStage::Released,
        ),
    ];

    for (stage, event, expected) in cases {
        let next = next_release_stage(stage, event).unwrap();
        assert_eq!(next, expected);
    }
}
