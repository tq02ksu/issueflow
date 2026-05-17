#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidTransition {
    pub machine: &'static str,
    pub stage: &'static str,
    pub event: &'static str,
}

impl InvalidTransition {
    pub fn new(machine: &'static str, stage: &'static str, event: &'static str) -> Self {
        Self {
            machine,
            stage,
            event,
        }
    }
}
