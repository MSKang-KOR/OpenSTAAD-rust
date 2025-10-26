use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub id: u32,
    pub source: String,
    pub r#type: String,
    pub name: String,
    pub assigned: Vec<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IsotropicMaterialType {
    Other = 0,
    Steel = 1,
    Concrete = 2,
    Aluminum = 3,
    Timber = 4,
}

impl IsotropicMaterialType {
    pub fn from(value: u8) -> IsotropicMaterialType {
        match value {
            0 => IsotropicMaterialType::Other,
            1 => IsotropicMaterialType::Steel,
            2 => IsotropicMaterialType::Concrete,
            3 => IsotropicMaterialType::Aluminum,
            4 => IsotropicMaterialType::Timber,
            _ => IsotropicMaterialType::Other,
        }
    }
    pub fn as_number(&self) -> u8 {
        *self as u8
    }
    pub fn as_string(&self) -> String {
        let mat_type = match self {
            IsotropicMaterialType::Other => "Other",
            IsotropicMaterialType::Steel => "Steel",
            IsotropicMaterialType::Concrete => "Concrete",
            IsotropicMaterialType::Aluminum => "Aluminum",
            IsotropicMaterialType::Timber => "Timber",
        };
        mat_type.to_string()
    }
}
