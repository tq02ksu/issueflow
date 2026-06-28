use agui_protocol::events::AgUiEvent;
use agui_runtime::provider::ProviderDelta;
use futures::stream;
use std::sync::Arc;
use tokio::sync::Mutex;

#[test]
fn provider_delta_supports_tool_roundtrip_markers() {
    let deltas = vec![
        ProviderDelta::ToolStart {
            id: "call_1".into(),
            name: "list_issues".into(),
        },
        ProviderDelta::ToolArgs {
            id: "call_1".into(),
            delta: "{\"project_id\":1}".into(),
        },
        ProviderDelta::ToolEnd {
            id: "call_1".into(),
        },
    ];

    assert_eq!(deltas.len(), 3);
}

#[tokio::test]
async fn runtime_engine_streams_text_events_and_collects_assistant_output() {
    let emitted = Arc::new(Mutex::new(Vec::new()));
    let emitted_clone = emitted.clone();

    let result = agui_runtime::engine::run_chat_rounds(
        vec![serde_json::json!({
            "role": "user",
            "content": "hello"
        })],
        vec![],
        |_messages, _tools| async move {
            Ok(stream::iter(vec![
                Ok(ProviderDelta::Text("hello back".into())),
                Ok(ProviderDelta::Done),
            ]))
        },
        |_tool_name, _args| async move { Ok(serde_json::Value::Null) },
        move |event| {
            let emitted = emitted_clone.clone();
            let event = event.clone();
            async move {
                emitted.lock().await.push(event);
                Ok(())
            }
        },
    )
    .await
    .unwrap();

    let emitted = emitted.lock().await;
    assert_eq!(result.assistant_messages, vec!["hello back"]);
    assert!(emitted.iter().any(|event| matches!(
        event,
        AgUiEvent::TextMessageContent { delta, .. } if delta == "hello back"
    )));
    assert!(result.events.iter().any(|event| matches!(
        event,
        AgUiEvent::TextMessageContent { delta, .. } if delta == "hello back"
    )));
}
