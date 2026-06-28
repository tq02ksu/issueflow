use std::convert::Infallible;

use agui_protocol::events::AgUiEvent;
use axum::response::{IntoResponse, Sse};
use futures::stream;

async fn render_event(event: axum::response::sse::Event) -> String {
    let response =
        Sse::new(stream::once(async move { Ok::<_, Infallible>(event) })).into_response();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    String::from_utf8(body.to_vec()).unwrap()
}

#[tokio::test]
async fn agui_event_is_encoded_as_json_sse() {
    let encoded = agui_axum::sse::encode_agui_event(&AgUiEvent::RunStarted {
        thread_id: "thread-1".into(),
        run_id: "run-1".into(),
    });

    let rendered = render_event(encoded).await;

    assert_eq!(
        rendered,
        "data: {\"type\":\"RUN_STARTED\",\"threadId\":\"thread-1\",\"runId\":\"run-1\"}\n\n"
    );
}

#[tokio::test]
async fn persisted_custom_event_uses_custom_name_and_json_payload() {
    let encoded = agui_axum::sse::encode_persisted_event(
        "CUSTOM",
        r#"{"name":"a2ui_render","value":{"component":"IssueCard"}}"#,
    );

    let rendered = render_event(encoded).await;

    assert_eq!(
        rendered,
        "event: custom\ndata: {\"name\":\"a2ui_render\",\"value\":{\"component\":\"IssueCard\"}}\n\n"
    );
}
