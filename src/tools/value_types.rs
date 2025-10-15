use std::ffi::c_void;

use anyhow::{Context, Result, anyhow, bail};
use serde_json::Value;
use serde_json::{self, json};
use windows::Win32::System::Variant::VARENUM;
use windows::Win32::System::{
    Com::IDispatch,
    Ole::{SafeArrayGetElement, SafeArrayGetLBound, SafeArrayGetUBound},
    Variant::{VARIANT, VariantToBoolean, VariantToDouble, VariantToInt32, VariantToStringAlloc},
};
use windows_core::BSTR;

use crate::{
    bindings::MemberSteelDgnParams,
    tools::{
        safe_array_from_vec1d, safe_array_from_vec2d, variant::SafeArray, variant_with_ptr_from,
        variant_with_ptr_to,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum InType {
    Int,
    Double,
    Str,
    Bool,
    VecInt,
    VecDouble,
    VecStr,
    Vec2dInt,
    Vec2dDouble,
    Vec2dStr,

    OptionInt,
    OptionDouble,
    OptionStr,
    OptionBool,
    OptionVecInt,
    OptionVecDouble,
    OptionVecStr,
    OptionVec2dInt,
    OptionVec2dDouble,
    OptionVec2dStr,

    MutInt,
    MutDouble,
    MutStr,
    MutBool,
    MutVecInt,
    MutVecDouble,
    MutVecStr,

    MemberSteelDgnParams,
}

impl InType {
    pub fn to_output_as(&self, variant: &VARIANT) -> Result<Value> {
        match self {
            Self::MutInt => OutType::Int.to_value(variant),
            Self::MutDouble => OutType::Double.to_value(variant),
            Self::MutStr => serde_json::to_value(&variant_with_ptr_to::<BSTR>(variant))
                .map_err(|e| anyhow!("{}", e)),
            Self::MutBool => OutType::Bool.to_value(variant),
            Self::MutVecInt => OutType::VecInt.to_value(variant),
            Self::MutVecDouble => OutType::VecDouble.to_value(variant),
            Self::MutVecStr => OutType::VecStr.to_value(variant),
            Self::MemberSteelDgnParams => OutType::MemberSteelDgnParams.to_value(variant),
            _ => bail!("`to_output_As` method can be used for only InType::Mut*."),
        }
    }
    pub fn to_input_as(&self, value: &Value) -> Result<Input> {
        match self {
            Self::Int => value
                .as_i64()
                .ok_or_else(|| anyhow!("Invalid i32 value"))?
                .try_into()
                .map(Input::Int)
                .map_err(|_| anyhow!("i32 overflow")),
            Self::Double => value
                .as_f64()
                .ok_or_else(|| anyhow!("Invalid i32 value"))
                .map(Input::Double)
                .map_err(|e| anyhow!(e)),
            Self::Str => value
                .as_str()
                .ok_or_else(|| anyhow!("Invalid str value"))?
                .try_into()
                .map(Input::Str)
                .map_err(|_| anyhow!("str overflow")),
            Self::Bool => value
                .as_bool()
                .ok_or_else(|| anyhow!("Invalid bool value"))
                .map(Input::Bool)
                .map_err(|e| anyhow!(e)),
            Self::VecInt => {
                let array = value.as_array().ok_or(anyhow!("Invalid array parameter"))?;
                let mut result = Vec::new();
                for v in array.iter() {
                    let i64_val = v.as_i64().ok_or(anyhow!("Invalid i32 in array"))?;
                    let i32_val = i64_val.try_into().map_err(|_| anyhow!("i32 overflow"))?;
                    result.push(i32_val);
                }
                Ok(Input::VecInt(result))
            }
            Self::VecDouble => {
                let array = value.as_array().ok_or(anyhow!("Invalid array parameter"))?;
                let mut result = Vec::new();
                for v in array.iter() {
                    let f64_val = v.as_f64().ok_or(anyhow!("Invalid f64 in array"))?;
                    result.push(f64_val);
                }
                Ok(Input::VecDouble(result))
            }
            Self::VecStr => {
                let array = value.as_array().ok_or(anyhow!("Invalid array parameter"))?;
                let mut result = Vec::new();
                for v in array.iter() {
                    let str_val = v.as_str().ok_or(anyhow!("Invalid str in array"))?;
                    let string_val = str_val.try_into().map_err(|_| anyhow!("str overflow"))?;
                    result.push(string_val);
                }
                Ok(Input::VecStr(result))
            }
            Self::Vec2dInt => {
                let arr2 = value
                    .as_array()
                    .ok_or(anyhow!("Invalid 2D array parameter"))?;
                let mut result = Vec::new();
                for arr in arr2.iter() {
                    let _arr = arr.as_array().ok_or(anyhow!("Invalid inner array"))?;
                    let mut row = Vec::new();
                    for v in _arr.iter() {
                        let as_v = v.as_i64().ok_or(anyhow!("Invalid i32 in 2D array"))?;
                        let _v = as_v.try_into().map_err(|_| anyhow!("i32 overflow"))?;
                        row.push(_v);
                    }
                    result.push(row);
                }
                Ok(Input::Vec2dInt(result))
            }
            Self::Vec2dDouble => {
                let arr2 = value
                    .as_array()
                    .ok_or(anyhow!("Invalid 2D array parameter"))?;
                let mut result = Vec::new();
                for arr in arr2.iter() {
                    let _arr = arr.as_array().ok_or(anyhow!("Invalid inner array"))?;
                    let mut row = Vec::new();
                    for v in _arr.iter() {
                        let _v = v.as_f64().ok_or(anyhow!("Invalid f32 in 2D array"))?;
                        row.push(_v);
                    }
                    result.push(row);
                }
                Ok(Input::Vec2dDouble(result))
            }
            Self::Vec2dStr => {
                let arr2 = value
                    .as_array()
                    .ok_or(anyhow!("Invalid 2D array parameter"))?;
                let mut result = Vec::new();
                for arr in arr2.iter() {
                    let _arr = arr.as_array().ok_or(anyhow!("Invalid inner array"))?;
                    let mut row = Vec::new();
                    for v in _arr.iter() {
                        let as_v = v.as_str().ok_or(anyhow!("Invalid str in 2D array"))?;
                        let _v = as_v.try_into().map_err(|_| anyhow!("str overflow"))?;
                        row.push(_v);
                    }
                    result.push(row);
                }
                Ok(Input::Vec2dStr(result))
            }
            _ => Err(anyhow!("`to_input_as` method can not be used {:#?}.", self)),
        }
    }

    pub fn is_general_type(&self) -> bool {
        [
            InType::Int,
            InType::Double,
            InType::Str,
            InType::Bool,
            InType::VecInt,
            InType::VecDouble,
            InType::VecStr,
            InType::Vec2dInt,
            InType::Vec2dDouble,
            InType::Vec2dStr,
        ]
        .contains(self)
    }
    pub fn count_general_type(_inputs: &Vec<InType>) -> usize {
        _inputs.iter().filter(|v| v.is_general_type()).count()
    }
    pub fn filter_general_type(_inputs: &Vec<InType>) -> Vec<Self> {
        _inputs
            .iter()
            .filter(|v| v.is_general_type())
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum OutType {
    Int,
    Double,
    Str,
    Bool,
    VecInt,
    VecDouble,
    VecStr,
    Index(i32),

    MemberSteelDgnParams,
}

impl OutType {
    pub fn to_value(&self, variant: &VARIANT) -> Result<Value> {
        match self {
            Self::Int => unsafe {
                let v = VariantToInt32(variant as *const VARIANT).unwrap();
                let o = serde_json::to_value(&v)?;
                Ok(o)
            },
            Self::Double => unsafe {
                let v = VariantToDouble(variant as *const VARIANT).unwrap();
                let o = serde_json::to_value(&v)?;
                Ok(o)
            },
            Self::Str => unsafe {
                let v = VariantToStringAlloc(variant as *const VARIANT)
                    .context("VariantToStringAlloc failed")?
                    .to_string()?;
                let o = serde_json::to_value(&v)?;
                Ok(o)
            },
            Self::Bool => unsafe {
                let v: bool = VariantToBoolean(variant as *const VARIANT).unwrap().into();
                let o = serde_json::to_value(&v)?;
                Ok(o)
            },
            Self::VecInt => unsafe {
                let sa = *variant.Anonymous.Anonymous.Anonymous.pparray;
                let ub = SafeArrayGetUBound(sa, 1)?;
                let lb = SafeArrayGetLBound(sa, 1)?;
                let count = ub - lb + 1;
                let mut _vec = Vec::with_capacity(count as usize);
                for i in 0..count {
                    let mut index = i as i32;
                    let mut value = 0;
                    let _ = SafeArrayGetElement(
                        sa,
                        &mut index as *mut i32,
                        &mut value as *mut i32 as *mut c_void,
                    );
                    _vec.push(value as i32);
                }
                let o = serde_json::to_value(&_vec)?;
                Ok(o)
            },
            Self::VecDouble => unsafe {
                let sa = *variant.Anonymous.Anonymous.Anonymous.pparray;
                let ub = SafeArrayGetUBound(sa, 1)?;
                let lb = SafeArrayGetLBound(sa, 1)?;
                let count = ub - lb + 1;
                let mut _vec = Vec::with_capacity(count as usize);
                for i in 0..count {
                    let mut index = i as i32;
                    let mut value = 0.;
                    let _ = SafeArrayGetElement(
                        sa,
                        &mut index as *mut i32,
                        &mut value as *mut f64 as *mut c_void,
                    );
                    _vec.push((value) as f64);
                }
                let o = serde_json::to_value(&_vec)?;
                Ok(o)
            },
            Self::VecStr => unsafe {
                let sa = *variant.Anonymous.Anonymous.Anonymous.pparray;
                let ub = SafeArrayGetUBound(sa, 1).unwrap();
                let lb = SafeArrayGetLBound(sa, 1).unwrap();
                let count = ub - lb + 1;
                let mut _vec = Vec::with_capacity(count as usize);
                for i in 0..count {
                    let mut index = i as i32;
                    let mut value = BSTR::default();
                    let _ = SafeArrayGetElement(
                        sa,
                        &mut index as *mut i32,
                        &mut value as *mut BSTR as *mut c_void,
                    );
                    _vec.push(value.to_string());
                }
                let o = serde_json::to_value(&_vec)?;
                Ok(o)
            },
            Self::MemberSteelDgnParams => unsafe {
                if variant.Anonymous.Anonymous.vt == VARENUM(0) {
                    return Ok(json!(Value::Null));
                }
                let idispatch = (*variant.Anonymous.Anonymous.Anonymous.ppdispVal)
                    .clone()
                    .unwrap();
                let v = MemberSteelDgnParams::invoke(&idispatch);
                Ok(json!(v))
            },
            _ => bail!("to_value method can not be used OutType::Index variant."),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Input {
    Int(i32),
    Double(f64),
    Str(String),
    Bool(bool),
    VecInt(Vec<i32>),
    VecDouble(Vec<f64>),
    VecStr(Vec<String>),
    Vec2dInt(Vec<Vec<i32>>),
    Vec2dDouble(Vec<Vec<f64>>),
    Vec2dStr(Vec<Vec<String>>),

    OptionInt(Option<i32>),
    OptionDouble(Option<f64>),
    OptionStr(Option<String>),
    OptionBool(Option<bool>),
    OptionVecInt(Option<Vec<i32>>),
    OptionVecDouble(Option<Vec<f64>>),
    OptionVecStr(Option<Vec<String>>),
    OptionVec2dInt(Option<Vec<Vec<i32>>>),
    OptionVec2dDouble(Option<Vec<Vec<f64>>>),
    OptionVec2dStr(Option<Vec<Vec<String>>>),
}

impl Input {
    pub fn to_variant(&self) -> VARIANT {
        match self {
            Self::Int(v) => VARIANT::from(*v),
            Self::Double(v) => VARIANT::from(*v),
            Self::Str(v) => VARIANT::from(v.as_str()),
            Self::Bool(v) => VARIANT::from(*v),
            Self::VecInt(v) => {
                let sa = safe_array_from_vec1d::<i32>(v.clone()).unwrap();
                variant_with_ptr_from::<SafeArray<f64>>(sa)
            }
            Self::VecDouble(v) => {
                let sa = safe_array_from_vec1d::<f64>(v.clone()).unwrap();
                variant_with_ptr_from::<SafeArray<f64>>(sa)
            }
            Self::VecStr(v) => {
                let sa = safe_array_from_vec1d::<String>(v.clone()).unwrap();
                variant_with_ptr_from::<SafeArray<BSTR>>(sa)
            }
            Self::Vec2dInt(v) => {
                let sa = safe_array_from_vec2d::<i32>(v.clone()).unwrap();
                variant_with_ptr_from::<SafeArray<i32>>(sa)
            }
            Self::Vec2dDouble(v) => {
                let sa = safe_array_from_vec2d::<f64>(v.clone()).unwrap();
                variant_with_ptr_from::<SafeArray<f64>>(sa)
            }
            Self::Vec2dStr(v) => {
                let sa = safe_array_from_vec2d::<String>(v.clone()).unwrap();
                variant_with_ptr_from::<SafeArray<BSTR>>(sa)
            }

            Self::OptionInt(v) => match v {
                Some(v) => VARIANT::from(*v),
                None => VARIANT::default(),
            },
            Self::OptionDouble(v) => match v {
                Some(v) => VARIANT::from(*v),
                None => VARIANT::default(),
            },
            Self::OptionStr(v) => match v {
                Some(v) => VARIANT::from(BSTR::from(v.clone())),
                None => VARIANT::default(),
            },
            Self::OptionBool(v) => match v {
                Some(v) => VARIANT::from(*v),
                None => VARIANT::default(),
            },
            Self::OptionVecInt(v) => match v {
                Some(v) => {
                    let sa = safe_array_from_vec1d::<i32>(v.clone()).unwrap();
                    variant_with_ptr_from::<SafeArray<i32>>(sa)
                }
                None => VARIANT::default(),
            },
            Self::OptionVecDouble(v) => match v {
                Some(v) => {
                    let sa = safe_array_from_vec1d::<f64>(v.clone()).unwrap();
                    variant_with_ptr_from::<SafeArray<f64>>(sa)
                }
                None => VARIANT::default(),
            },
            Self::OptionVecStr(v) => match v {
                Some(v) => {
                    let sa = safe_array_from_vec1d::<String>(v.clone()).unwrap();
                    variant_with_ptr_from::<SafeArray<BSTR>>(sa)
                }
                None => VARIANT::default(),
            },

            Self::OptionVec2dInt(v) => match v {
                Some(v) => {
                    let sa = safe_array_from_vec2d::<i32>(v.clone()).unwrap();
                    variant_with_ptr_from::<SafeArray<i32>>(sa)
                }
                None => VARIANT::default(),
            },
            Self::OptionVec2dDouble(v) => match v {
                Some(v) => {
                    let sa = safe_array_from_vec2d::<f64>(v.clone()).unwrap();
                    variant_with_ptr_from::<SafeArray<f64>>(sa)
                }
                None => VARIANT::default(),
            },
            Self::OptionVec2dStr(v) => match v {
                Some(v) => {
                    let sa = safe_array_from_vec2d::<String>(v.clone()).unwrap();
                    variant_with_ptr_from::<SafeArray<BSTR>>(sa)
                }
                None => VARIANT::default(),
            },
        }
    }
}

impl From<i32> for Input {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}
impl From<f64> for Input {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}
impl From<String> for Input {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}
impl From<bool> for Input {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl From<Vec<i32>> for Input {
    fn from(value: Vec<i32>) -> Self {
        Self::VecInt(value)
    }
}
impl From<Vec<f64>> for Input {
    fn from(value: Vec<f64>) -> Self {
        Self::VecDouble(value)
    }
}
impl From<Vec<String>> for Input {
    fn from(value: Vec<String>) -> Self {
        Self::VecStr(value)
    }
}
impl From<Vec<Vec<i32>>> for Input {
    fn from(value: Vec<Vec<i32>>) -> Self {
        Self::Vec2dInt(value)
    }
}
impl From<Vec<Vec<f64>>> for Input {
    fn from(value: Vec<Vec<f64>>) -> Self {
        Self::Vec2dDouble(value)
    }
}
impl From<Vec<Vec<String>>> for Input {
    fn from(value: Vec<Vec<String>>) -> Self {
        Self::Vec2dStr(value)
    }
}
impl From<Option<i32>> for Input {
    fn from(value: Option<i32>) -> Self {
        Self::OptionInt(value)
    }
}
impl From<Option<f64>> for Input {
    fn from(value: Option<f64>) -> Self {
        Self::OptionDouble(value)
    }
}
impl From<Option<String>> for Input {
    fn from(value: Option<String>) -> Self {
        Self::OptionStr(value)
    }
}
impl From<Option<bool>> for Input {
    fn from(value: Option<bool>) -> Self {
        Self::OptionBool(value)
    }
}
impl From<Option<Vec<i32>>> for Input {
    fn from(value: Option<Vec<i32>>) -> Self {
        Self::OptionVecInt(value)
    }
}
impl From<Option<Vec<f64>>> for Input {
    fn from(value: Option<Vec<f64>>) -> Self {
        Self::OptionVecDouble(value)
    }
}
impl From<Option<Vec<String>>> for Input {
    fn from(value: Option<Vec<String>>) -> Self {
        Self::OptionVecStr(value)
    }
}
impl From<Option<Vec<Vec<i32>>>> for Input {
    fn from(value: Option<Vec<Vec<i32>>>) -> Self {
        Self::OptionVec2dInt(value)
    }
}
impl From<Option<Vec<Vec<f64>>>> for Input {
    fn from(value: Option<Vec<Vec<f64>>>) -> Self {
        Self::OptionVec2dDouble(value)
    }
}
impl From<Option<Vec<Vec<String>>>> for Input {
    fn from(value: Option<Vec<Vec<String>>>) -> Self {
        Self::OptionVec2dStr(value)
    }
}

#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub inputs: Vec<InType>,
    pub outputs: Vec<OutType>,
}
