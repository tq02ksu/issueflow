use std::path::Path;

use agui_protocol::events::AgUiEvent;

#[test]
fn workspace_contains_agui_crates() {
    assert!(Path::new("crates/agui-protocol/Cargo.toml").exists());
    assert!(Path::new("crates/agui-runtime/Cargo.toml").exists());
    assert!(Path::new("crates/agui-axum/Cargo.toml").exists());
}

#[test]
fn protocol_event_serializes_with_expected_tag_names() {
    let event = AgUiEvent::RunStarted {
        thread_id: "thread-1".into(),
        run_id: "run-1".into(),
    };

    let json = serde_json::to_value(&event).unwrap();

    assert_eq!(json["type"], "RUN_STARTED");
    assert_eq!(json["threadId"], "thread-1");
    assert_eq!(json["runId"], "run-1");
}
