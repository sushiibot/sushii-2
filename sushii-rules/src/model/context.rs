use dashmap::DashMap;

/// This is shared to be accessed in the rules parsing.
/// Created each time an event fires.
#[derive(Debug, Clone)]
pub struct Context {
    pub data: serde_json::Value,
}

impl Context {
    pub fn new() -> Self {
        Self {
            data: serde_json::Value::Null,
        }
    }
}
