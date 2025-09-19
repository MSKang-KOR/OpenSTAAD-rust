use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use windows::Win32::System::{
    Com::{CoUninitialize, IDispatch},
    Variant::{VARIANT, VariantToInt32, VariantToStringAlloc},
};

use crate::tools::invoke_method;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberSteelDgnParams {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub description: String,
    pub default: String,
}

impl MemberSteelDgnParams {
    pub fn invoke(dispatch: &IDispatch) -> Vec<Self> {
        let mut _vec = Vec::new();
        unsafe {
            let count_var = invoke_method(dispatch, "Count", &mut []).unwrap();
            let count = VariantToInt32(&count_var as *const VARIANT).unwrap();
            for i in 0..count {
                let index_variant = VARIANT::from(i);
                let params = &mut [index_variant];
                let mut name = "".to_string();
                let mut value = "".to_string();
                let mut unit = "".to_string();
                let mut description = "".to_string();
                let mut default = "".to_string();
                ["Name", "Value", "Unit", "Description", "Default"]
                    .iter()
                    .for_each(|k| {
                        let _var = invoke_method(dispatch, k, params).unwrap();
                        let _str = VariantToStringAlloc(&_var as *const VARIANT)
                            .unwrap()
                            .to_string();
                        match *k {
                            "Name" => name = _str.unwrap(),
                            "Value" => value = _str.unwrap(),
                            "Unit" => unit = _str.unwrap(),
                            "Description" => description = _str.unwrap(),
                            "Default" => default = _str.unwrap(),
                            _ => {}
                        }
                    });
                _vec.push(MemberSteelDgnParams {
                    name,
                    value,
                    unit,
                    description,
                    default,
                })
            }
        }
        _vec
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MemberSteelDesignResult {
    pub member: Value,
    pub code: Value,               // i32
    pub status: Value,             // String
    pub critical_ratio: Value,     // f64
    pub allowabale_ratio: Value,   // f64
    pub critical_load_case: Value, // i32
    pub critical_section: Value,   // f64
    pub critical_clause: Value,    // String
    pub design_section: Value,     // String
    pub design_force: Value,       // Vec<f64; 3>
    pub kl_by_r: Value,            // f64
}

impl MemberSteelDesignResult {
    pub fn from(member: Value, results: Value) -> Self {
        let code = results[1].clone();
        if code == json!("360-16 L") {
            let status = results[2].clone();
            let critical_ratio = results[3].clone();
            let allowabale_ratio = results[4].clone();
            let critical_load_case = results[5].clone();
            let critical_section = Value::Null;
            let critical_clause = results[6].clone();
            let design_section = results[7].clone();
            let design_force = Value::Null;
            let kl_by_r = Value::Null;
            Self {
                member,
                code,
                status,
                critical_ratio,
                allowabale_ratio,
                critical_load_case,
                critical_section,
                critical_clause,
                design_section,
                design_force,
                kl_by_r,
            }
        } else {
            let code = results[1].clone();
            let status = results[2].clone();
            let critical_ratio = results[3].clone();
            let allowabale_ratio = results[4].clone();
            let critical_load_case = results[5].clone();
            let critical_section = results[6].clone();
            let critical_clause = results[7].clone();
            let design_section = results[8].clone();
            let design_force = results[9].clone();
            let kl_by_r = results[10].clone();
            Self {
                member,
                code,
                status,
                critical_ratio,
                allowabale_ratio,
                critical_load_case,
                critical_section,
                critical_clause,
                design_section,
                design_force,
                kl_by_r,
            }
        }
    }
}
