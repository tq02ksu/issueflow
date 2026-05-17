#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidTransition {
    pub machine: &'static str,
    pub from: &'static str,
    pub event: &'static str,
}

impl InvalidTransition {
    pub fn new(machine: &'static str, from: &'static str, event: &'static str) -> Self {
        Self {
            machine,
            from,
            event,
        }
    }
}
