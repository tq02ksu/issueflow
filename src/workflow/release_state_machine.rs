use super::types::InvalidTransition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReleaseStage {
    Idle,
    ReleaseChecking,
    ReadyForRelease,
    Releasing,
    Released,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReleaseEvent {
    PrepareRelease,
    ReleaseReady,
    Publish,
    Released,
}

pub fn next_release_stage(
    stage: ReleaseStage,
    event: ReleaseEvent,
) -> Result<ReleaseStage, InvalidTransition> {
    match (stage, event) {
        (ReleaseStage::Idle, ReleaseEvent::PrepareRelease) => Ok(ReleaseStage::ReleaseChecking),
        (ReleaseStage::ReleaseChecking, ReleaseEvent::ReleaseReady) => {
            Ok(ReleaseStage::ReadyForRelease)
        }
        (ReleaseStage::ReadyForRelease, ReleaseEvent::Publish) => Ok(ReleaseStage::Releasing),
        (ReleaseStage::Releasing, ReleaseEvent::Released) => Ok(ReleaseStage::Released),
        _ => Err(InvalidTransition::new(
            "release",
            stage_name(stage),
            event_name(event),
        )),
    }
}

fn stage_name(stage: ReleaseStage) -> &'static str {
    match stage {
        ReleaseStage::Idle => "idle",
        ReleaseStage::ReleaseChecking => "release-checking",
        ReleaseStage::ReadyForRelease => "ready-for-release",
        ReleaseStage::Releasing => "releasing",
        ReleaseStage::Released => "released",
    }
}

fn event_name(event: ReleaseEvent) -> &'static str {
    match event {
        ReleaseEvent::PrepareRelease => "prepare_release",
        ReleaseEvent::ReleaseReady => "release_ready",
        ReleaseEvent::Publish => "release",
        ReleaseEvent::Released => "released",
    }
}
