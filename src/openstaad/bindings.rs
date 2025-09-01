use std::sync::Arc;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use windows::Win32::System::{
    Com::{CoUninitialize, IDispatch},
    Variant::{VARIANT, VariantToInt32, VariantToStringAlloc},
};

use crate::{
    openstaad::{
        app::OpenStaad, command::Command, design::Design, geometry::Geometry, load::Load,
        output::Output, property::Property, support::Support,
    },
    tools::invoke::invoke_method,
};

#[derive(Debug)]
pub enum Staad {
    OpenStaad(OpenStaad),
    Geometry(Arc<Geometry>),
    Command(Arc<Command>),
    Design(Arc<Design>),
    Load(Arc<Load>),
    Output(Arc<Output>),
    Property(Arc<Property>),
    Support(Arc<Support>),
}

unsafe impl Send for Staad {}
unsafe impl Sync for Staad {}

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
            CoUninitialize();
        }
        _vec
    }
}
