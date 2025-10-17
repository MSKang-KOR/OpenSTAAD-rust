use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeTableRow {
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BeamTableRow {
    pub id: i32,
    pub i: i32,
    pub j: i32,
    // pub property: i32,
    // pub material: String,
    // pub beta: f64,
    // pub length: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SectionObj {
    pub id: Value,           // i32,
    pub section_type: Value, // i32,
    pub name: Value,         // String,
    pub assigned: Value,     // Vec<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BetaObj {
    pub angle: Value,    // f64
    pub assigned: Value, // Vec<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsotropicMaterialObj {
    pub name: Value,   // String
    pub r#type: Value, // Vec<i32>
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SpecificationObj {
//     pub name: Value,     // String
//     pub r#type: Value,   // i32 (SpecificationType Code)
//     pub assigned: Value, // Vec<i32>
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct SupportObj {
    pub id: Value,       // i32
    pub name: Value,     // String
    pub r#type: Value,   // i8 (SupportType Code)
    pub assigned: Value, // Vec<i32>
}


