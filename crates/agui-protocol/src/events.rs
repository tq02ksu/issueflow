use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AgUiEvent {
    RunStarted {
        #[serde(rename = "threadId")]
        thread_id: String,
        #[serde(rename = "runId")]
        run_id: String,
    },
    RunFinished {
        #[serde(rename = "threadId")]
        thread_id: String,
        #[serde(rename = "runId")]
        run_id: String,
    },
    RunError {
        message: String,
        code: Option<String>,
    },
    StepStarted {
        #[serde(rename = "stepName")]
        step_name: String,
    },
    StepFinished {
        #[serde(rename = "stepName")]
        step_name: String,
    },
    TextMessageStart {
        #[serde(rename = "messageId")]
        message_id: String,
        role: String,
    },
    TextMessageContent {
        #[serde(rename = "messageId")]
        message_id: String,
        delta: String,
    },
    TextMessageEnd {
        #[serde(rename = "messageId")]
        message_id: String,
    },
    ToolCallStart {
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        #[serde(rename = "toolCallName")]
        tool_call_name: String,
        #[serde(rename = "parentMessageId")]
        parent_message_id: Option<String>,
    },
    ToolCallArgs {
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        delta: String,
    },
    ToolCallEnd {
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
    },
    ToolCallResult {
        #[serde(rename = "messageId")]
        message_id: String,
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        content: serde_json::Value,
        role: String,
    },
    StateSnapshot {
        snapshot: serde_json::Value,
    },
    MessagesSnapshot {
        messages: Vec<serde_json::Value>,
    },
    Custom {
        name: String,
        value: serde_json::Value,
    },
}
