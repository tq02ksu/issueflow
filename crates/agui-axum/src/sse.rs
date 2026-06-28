use agui_protocol::events::AgUiEvent;
use axum::response::sse::Event;

pub fn encode_agui_event(event: &AgUiEvent) -> Event {
    Event::default()
        .json_data(event)
        .expect("agui event should serialize to JSON")
}

pub fn encode_persisted_event(event_type: &str, payload: &str) -> Event {
    match event_type {
        "CUSTOM" => match serde_json::from_str::<serde_json::Value>(payload) {
            Ok(value) => Event::default()
                .event("custom")
                .json_data(value)
                .expect("persisted custom event should serialize to JSON"),
            Err(_) => Event::default().data(payload),
        },
        _ => Event::default().event(event_type).data(payload),
    }
}
