use serde::Serialize;
use serde_json::{Value, json};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum SupportType {
    Error = -1,
    NoSupport = 0,
    Pinned = 1,
    Fixed = 2,
    FixedBut = 3,
    Enforced = 4,
    EnforcedBut = 5,
    Inclined = 6,
    Footing = 7,
    ElasticMat = 8,
    PlateMat = 9,
    MultilinearPinned = 10,
    GeneratedPinned = 11,
    GeneratedFixed = 12,
    GeneratedFixedBut = 13,
}
impl SupportType {
    pub fn as_code(&self) -> Value {
        json!(*self as i8)
    }
    pub fn as_name(&self) -> Value {
        match self {
            Self::Error => json!("Error"),
            Self::NoSupport => json!("NoSupport"),
            Self::Pinned => json!("Pinned"),
            Self::Fixed => json!("Fixed"),
            Self::FixedBut => json!("FixedBut"),
            Self::Enforced => json!("Enforced"),
            Self::EnforcedBut => json!("EnforcedBut"),
            Self::Inclined => json!("Inclined"),
            Self::Footing => json!("Footing"),
            Self::ElasticMat => json!("ElasticMat"),
            Self::PlateMat => json!("PlateMat"),
            Self::MultilinearPinned => json!("MultilinearPinned"),
            Self::GeneratedPinned => json!("GeneratedPinned"),
            Self::GeneratedFixed => json!("GeneratedFixed"),
            Self::GeneratedFixedBut => json!("GeneratedFixedBut"),
        }
    }
    pub fn from(value: Value) -> Self {
        let code = value.as_i64().unwrap() as i8;
        match code {
            -1 => Self::Error,
            0 => Self::NoSupport,
            1 => Self::Pinned,
            2 => Self::Fixed,
            3 => Self::FixedBut,
            4 => Self::Enforced,
            5 => Self::EnforcedBut,
            6 => Self::Inclined,
            7 => Self::Footing,
            8 => Self::ElasticMat,
            9 => Self::PlateMat,
            10 => Self::MultilinearPinned,
            11 => Self::GeneratedPinned,
            12 => Self::GeneratedFixed,
            13 => Self::GeneratedFixedBut,
            _ => Self::Error,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SupportObj {
    pub id: Value,       // u32
    pub name: Value,     // String
    pub r#type: Value,   // i32 (SpecificationType Code)
    pub release: Value,  // Vec<i32>
    pub spring: Value,   // Vec<f64>
    pub assigned: Value, // Vec<i32>
}
