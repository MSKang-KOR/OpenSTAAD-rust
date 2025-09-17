use serde::Serialize;
use serde_json::{Value, json};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum SpecificationType {
    Truss = 0,
    TensionOnly = 1,
    CompressionOnly = 2,
    CableOnly = 3,
    Joist = 4,
    Other = -1,
}
impl SpecificationType {
    pub fn as_value(&self) -> Value {
        json!(*self as i8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum ReleaseType {
    Release = 1,
    Spring = -1,
    MPs = -2,
    MP = -3,
}
impl ReleaseType {
    pub fn as_value(&self) -> Value {
        json!(*self as i8)
    }
}
