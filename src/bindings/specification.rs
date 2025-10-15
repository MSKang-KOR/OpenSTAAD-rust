use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Release {
    pub id: u32,         // u32
    pub name: Value,     // String
    pub r#type: Value,   // i8(SpecificationType)
    pub assigned: Value, // Vec<u32>
    pub location: i32,   // u8
    pub is_partial_moment: bool,
    pub release_array: Value, // Vec<i8; 6>
    pub spring_array: Value,  // Vec<f64; 6>
    pub mp_array: Value,      // Vec<f64; 3>
    pub mp: Value,            // f64
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Truss {
    pub id: u32,         // u32
    pub name: Value,     // String
    pub r#type: Value,   // i8
    pub assigned: Value, // Vec<u32>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Specification {
    Release(Release),
    Truss(Truss),
}

impl Specification {
    pub fn set_id(&mut self, id: u32) -> () {
        match self {
            Specification::Release(release) => {
                release.id = id;
            }
            Specification::Truss(truss) => {
                truss.id = id;
            }
        };
    }

    pub fn as_value(self) -> Value {
        match self {
            Specification::Release(release) => {
                json!(release)
            }
            Specification::Truss(truss) => {
                json!(truss)
            }
        }
    }
}
