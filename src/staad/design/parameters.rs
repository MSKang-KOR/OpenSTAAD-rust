use anyhow::Context;
use windows::Win32::System::{
    Com::IDispatch,
    Variant::{VARIANT, VariantToInt32, VariantToStringAlloc},
};

use crate::staad::utils::invoke_method_with_result;

#[derive(Debug)]
pub struct DesignParameters {
    pub name: Vec<String>,
    pub value: Vec<String>,
    pub unit: Vec<String>,
    pub description: Vec<String>,
    pub default: Vec<String>,
}

impl DesignParameters {
    pub fn new(dispatch: &IDispatch) -> Self {
        let mut name = vec![];
        let mut value = vec![];
        let mut unit = vec![];
        let mut description = vec![];
        let mut default = vec![];
        unsafe {
            let count_var = invoke_method_with_result(dispatch, "Count", &mut []).unwrap();
            let count = VariantToInt32(&count_var as *const VARIANT).unwrap();
            println!("count {:#?}", count);
            for i in 0..count {
                let index_variant = VARIANT::from(i);
                let mut index_params = [index_variant.clone()];
                ["Name", "Value", "Unit", "Description", "Default"]
                    .iter()
                    .for_each(|k| {
                        let _var =
                            invoke_method_with_result(dispatch, k, &mut index_params).unwrap();
                        let _str = VariantToStringAlloc(&_var as *const VARIANT)
                            .context("converting parameter name")
                            .unwrap()
                            .to_string();
                        match *k {
                            "Name" => name.push(_str.unwrap()),
                            "Value" => value.push(_str.unwrap()),
                            "Unit" => unit.push(_str.unwrap()),
                            "Description" => description.push(_str.unwrap()),
                            "Default" => default.push(_str.unwrap()),
                            _ => {}
                        }
                    });
            }
            Self {
                name: name.to_vec(),
                value: value.to_vec(),
                unit: unit.to_vec(),
                description: description.to_vec(),
                default: default.to_vec(),
            }
        }
    }
}
