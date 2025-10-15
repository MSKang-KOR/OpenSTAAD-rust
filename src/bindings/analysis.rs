use serde::Serialize;
use serde_json::{Value, json};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum AnalysisStatus {
    InvalidModelPath = -2,
    Terminated = -1,
    Error = 0,
    Progress = 1,
    Complete = 2,
    CompleteWithWarnings = 3,
    ComperteWithErrors = 4,
    NotPerformed = 5,
}
impl AnalysisStatus {
    pub fn as_code(&self) -> Value {
        json!(*self as i8)
    }
    pub fn as_str(&self) -> String {
        let msg = match self {
            Self::InvalidModelPath => "InvalidModelPath",
            Self::Terminated => "Terminated",
            Self::Error => "Error",
            Self::Progress => "Progress",
            Self::Complete => "Complete",
            Self::CompleteWithWarnings => "CompleteWithWarnings",
            Self::ComperteWithErrors => "ComperteWithErrors",
            Self::NotPerformed => "NotPerformed",
        };
        msg.to_string()
    }
    pub fn from(value: Value) -> Self {
        let code = value.as_i64().unwrap() as i8;
        match code {
            -2 => Self::InvalidModelPath,
            -1 => Self::Terminated,
            0 => Self::Error,
            1 => Self::Progress,
            2 => Self::Complete,
            3 => Self::CompleteWithWarnings,
            4 => Self::ComperteWithErrors,
            5 => Self::NotPerformed,
            _ => Self::Error,
        }
    }
}
