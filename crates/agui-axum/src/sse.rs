use agui_protocol::events::AgUiEvent;
use axum::response::sse::Event;

pub fn encode_agui_event(event: &AgUiEvent) -> Event {
    match Event::default().json_data(event) {
        Ok(encoded) => encoded,
        Err(_) => Event::default()
            .event("error")
            .data("failed to encode agui event"),
    }
}

pub fn encode_persisted_event(event_type: &str, payload: &str) -> Event {
    match event_type {
        "CUSTOM" => match serde_json::from_str::<serde_json::Value>(payload) {
            Ok(value) => match Event::default().event("custom").json_data(value) {
                Ok(encoded) => encoded,
                Err(_) => Event::default().event("custom").data(payload),
            },
            Err(_) => Event::default().data(payload),
        },
        _ => Event::default().event(event_type).data(payload),
    }
}
