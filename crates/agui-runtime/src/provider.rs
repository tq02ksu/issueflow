#[derive(Debug)]
pub enum ProviderDelta {
    Text(String),
    ToolStart { id: String, name: String },
    ToolArgs { id: String, delta: String },
    ToolEnd { id: String },
    Done,
}
