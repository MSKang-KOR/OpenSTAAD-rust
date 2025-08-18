use crate::openstaad::api::{
    command::Command, design::Design, geometry::Geometry, load::Load, output::Output,
    process::StaadProcess, property::Property, root::Root, support::Support,
};
use serde_json::Value;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct MethodSignature {
    pub params: Vec<ParamType>,
    pub returns_unit_scaled: bool, // 반환값이 unit scaling을 필요로 하는지
}

#[derive(Debug)]
pub enum StaadObject {
    Process(Arc<Mutex<StaadProcess>>),
    Root(Arc<Root>),
    Geometry(Arc<Geometry>),
    Command(Arc<Command>),
    Design(Arc<Design>),
    Load(Arc<Load>),
    Output(Arc<Output>),
    Property(Arc<Property>),
    Support(Arc<Support>),
}

unsafe impl Send for StaadObject {}
unsafe impl Sync for StaadObject {}

#[derive(Debug)]
pub enum ParamType {
    I32,
    F64,
    String,
    Bool,
    VecI32,
    VecF64,
    VecVecI32,
    VecVecF64,
    VecString,
    OptionVecString,
    OptionVecF64,
    ArrayF64_6, // [f64; 6]
    ArrayF64_3, // [f64; 3]
    BaseUnit, // 특별히 처리가 필요한 base_unit 파라미터
}

#[derive(Debug)]
pub enum ConvertedParam {
    I32(i32),
    F64(f64),
    String(String),
    Bool(bool),
    VecI32(Vec<i32>),
    VecF64(Vec<f64>),
    VecVecI32(Vec<Vec<i32>>),
    VecVecF64(Vec<Vec<f64>>),
    VecString(Vec<String>),
    OptionVecString(Option<Vec<String>>),
    OptionVecF64(Option<Vec<f64>>),
    ArrayF64_6([f64; 6]),
    ArrayF64_3([f64; 3]),
}

// 파라미터 변환 함수
pub fn convert_param(value: &Value, param_type: &ParamType) -> Result<ConvertedParam, String> {
    match param_type {
        ParamType::I32 | ParamType::BaseUnit => value
            .as_i64()
            .ok_or("Invalid i32 parameter")?
            .try_into()
            .map(ConvertedParam::I32)
            .map_err(|_| "i32 overflow".to_string()),
        ParamType::F64 => value
            .as_f64()
            .ok_or("Invalid f64 parameter".to_string())
            .map(ConvertedParam::F64),
        ParamType::String => value
            .as_str()
            .ok_or("Invalid string parameter".to_string())
            .map(|s| ConvertedParam::String(s.to_string())),
        ParamType::Bool => value
            .as_bool()
            .ok_or("Invalid bool parameter".to_string())
            .map(ConvertedParam::Bool),
        ParamType::VecI32 => {
            let array = value
                .as_array()
                .ok_or("Invalid array parameter".to_string())?;
            let mut result = Vec::new();
            for v in array.iter() {
                let i64_val = v.as_i64().ok_or("Invalid i32 in array".to_string())?;
                let i32_val = i64_val.try_into().map_err(|_| "i32 overflow".to_string())?;
                result.push(i32_val);
            }
            Ok(ConvertedParam::VecI32(result))
        }
        ParamType::VecF64 => {
            let array = value
                .as_array()
                .ok_or("Invalid array parameter".to_string())?;
            let mut result = Vec::new();
            for v in array.iter() {
                let f64_val = v.as_f64().ok_or("Invalid f64 in array".to_string())?;
                result.push(f64_val);
            }
            Ok(ConvertedParam::VecF64(result))
        }
        ParamType::VecVecI32 => {
            let outer_array = value
                .as_array()
                .ok_or("Invalid 2D array parameter".to_string())?;
            let mut result = Vec::new();
            for arr in outer_array.iter() {
                let inner_array = arr.as_array().ok_or("Invalid inner array".to_string())?;
                let mut inner_result = Vec::new();
                for v in inner_array.iter() {
                    let i64_val = v.as_i64().ok_or("Invalid i32 in 2D array".to_string())?;
                    let i32_val = i64_val.try_into().map_err(|_| "i32 overflow".to_string())?;
                    inner_result.push(i32_val);
                }
                result.push(inner_result);
            }
            Ok(ConvertedParam::VecVecI32(result))
        }
        ParamType::VecVecF64 => {
            let outer_array = value
                .as_array()
                .ok_or("Invalid 2D array parameter".to_string())?;
            let mut result = Vec::new();
            for arr in outer_array.iter() {
                let inner_array = arr.as_array().ok_or("Invalid inner array".to_string())?;
                let mut inner_result = Vec::new();
                for v in inner_array.iter() {
                    let f64_val = v.as_f64().ok_or("Invalid f64 in 2D array".to_string())?;
                    inner_result.push(f64_val);
                }
                result.push(inner_result);
            }
            Ok(ConvertedParam::VecVecF64(result))
        }
        ParamType::VecString => {
            let array = value
                .as_array()
                .ok_or("Invalid array parameter".to_string())?;
            let mut result = Vec::new();
            for v in array.iter() {
                let string_val = v.as_str().ok_or("Invalid string in array".to_string())?;
                result.push(string_val.to_string());
            }
            Ok(ConvertedParam::VecString(result))
        }
        ParamType::OptionVecString => {
            if value.is_null() {
                Ok(ConvertedParam::OptionVecString(None))
            } else {
                let array = value
                    .as_array()
                    .ok_or("Invalid array parameter".to_string())?;
                let mut result = Vec::new();
                for v in array.iter() {
                    let string_val = v.as_str().ok_or("Invalid string in array".to_string())?;
                    result.push(string_val.to_string());
                }
                Ok(ConvertedParam::OptionVecString(Some(result)))
            }
        }
        ParamType::OptionVecF64 => {
            if value.is_null() {
                Ok(ConvertedParam::OptionVecF64(None))
            } else {
                let array = value
                    .as_array()
                    .ok_or("Invalid array parameter".to_string())?;
                let mut result = Vec::new();
                for v in array.iter() {
                    let f64_val = v.as_f64().ok_or("Invalid f64 in array".to_string())?;
                    result.push(f64_val);
                }
                Ok(ConvertedParam::OptionVecF64(Some(result)))
            }
        }
        ParamType::ArrayF64_6 => {
            let array = value
                .as_array()
                .ok_or("Invalid array parameter".to_string())?;
            if array.len() != 6 {
                return Err("Array must have exactly 6 elements".to_string());
            }
            let mut result = [0.0f64; 6];
            for (i, v) in array.iter().enumerate() {
                let f64_val = v.as_f64().ok_or("Invalid f64 in array".to_string())?;
                result[i] = f64_val;
            }
            Ok(ConvertedParam::ArrayF64_6(result))
        }
        ParamType::ArrayF64_3 => {
            let array = value
                .as_array()
                .ok_or("Invalid array parameter".to_string())?;
            if array.len() != 3 {
                return Err("Array must have exactly 3 elements".to_string());
            }
            let mut result = [0.0f64; 3];
            for (i, v) in array.iter().enumerate() {
                let f64_val = v.as_f64().ok_or("Invalid f64 in array".to_string())?;
                result[i] = f64_val;
            }
            Ok(ConvertedParam::ArrayF64_3(result))
        }
    }
}
