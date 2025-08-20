use std::{
    collections::HashMap,
    ffi::c_void,
    sync::{Arc, Mutex},
};

use crate::openstaad::{
    SafeArrayP,
    new_api::{geometry::Geometry, process::StaadProcess, root::Root},
    tools::{
        com::invoke_method,
        parameters,
        safe_array::{safe_array_from_vec1d, safe_array_from_vec2d},
        variant::{SafeArray, variant_with_ptr_from, variant_with_ptr_to},
    },
};
use anyhow::{Context, Ok as anyOk, Result, bail};
use serde_json::Value;
use windows::Win32::{
    Foundation::VARIANT_BOOL,
    System::{
        Com::{IDispatch, SAFEARRAY},
        Ole::{SafeArrayCreateVector, SafeArrayGetElement, SafeArrayGetLBound, SafeArrayGetUBound},
        Variant::{
            VARIANT, VT_BSTR, VT_I4, VT_R8, VariantToBoolean, VariantToDouble, VariantToInt32,
            VariantToStringAlloc,
        },
    },
};
use windows_core::BSTR;

#[derive(Debug, Clone)]
pub enum ParamType {
    I32,
    F64,
    String,
    Bool,
    VecI32,
    VecF64,
    VecString,
    Vec2dI32,
    Vec2dF64,
    Vec2dString,

    OptionI32,
    OptionF64,
    OptionString,
    OptionBool,
    OptionVecI32,
    OptionVecF64,
    OptionVecString,
    OptionVec2dI32,
    OptionVec2dF64,
    OptionVec2dString,

    MutI32,
    MutF64,
    MutString,
    MutBool,
    MutVecI32,
    MutVecF64,
    MutVecString,
}

#[derive(Debug, Clone)]
pub enum ResultType {
    Integer,
    Double,
    String,
    Bool,
    VecI,
    VecD,
    VecS,
    Index(i32),
}

impl ResultType {
    pub fn to_value(&self, variant: &VARIANT, factor: &f64) -> Result<Value, String> {
        match self {
            Self::Integer => unsafe {
                let v = VariantToInt32(variant as *const VARIANT).unwrap();
                serde_json::to_value(&v).map_err(|e| e.to_string())
            },
            Self::Double => unsafe {
                let v = VariantToDouble(variant as *const VARIANT).unwrap() / factor;
                serde_json::to_value(&v).map_err(|e| e.to_string())
            },
            Self::String => unsafe {
                let v = VariantToStringAlloc(variant as *const VARIANT)
                    .unwrap()
                    .to_string()
                    .unwrap();
                serde_json::to_value(&v).map_err(|e| e.to_string())
            },
            Self::Bool => unsafe {
                let v: bool = VariantToBoolean(variant as *const VARIANT).unwrap().into();
                serde_json::to_value(&v).map_err(|e| e.to_string())
            },
            Self::VecI => unsafe {
                let sa = *variant.Anonymous.Anonymous.Anonymous.pparray;
                let ub = SafeArrayGetUBound(sa, 1).unwrap();
                let lb = SafeArrayGetLBound(sa, 1).unwrap();
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
                serde_json::to_value(&_vec).map_err(|e| e.to_string())
            },
            Self::VecD => unsafe {
                let sa = *variant.Anonymous.Anonymous.Anonymous.pparray;
                let ub = SafeArrayGetUBound(sa, 1).unwrap();
                let lb = SafeArrayGetLBound(sa, 1).unwrap();
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
                    _vec.push((value / factor) as f64);
                }
                serde_json::to_value(&_vec).map_err(|e| e.to_string())
            },
            Self::VecS => unsafe {
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
                serde_json::to_value(&_vec).map_err(|e| e.to_string())
            },
            _ => serde_json::to_value(false).map_err(|e| e.to_string()),
        }
    }
    pub fn to_index(&self) -> usize {
        match self {
            Self::Index(v) => *v as usize,
            _ => 0 as usize,
        }
    }
}

pub enum TypedParam {
    I32(i32),
    F64(f64),
    String(String),
    Bool(bool),
    VecI32(Vec<i32>),
    VecF64(Vec<f64>),
    VecString(Vec<String>),
    Vec2dI32(Vec<Vec<i32>>),
    Vec2dF64(Vec<Vec<f64>>),
    Vec2dString(Vec<Vec<String>>),

    OptionI32(Option<i32>),
    OptionF64(Option<f64>),
    OptionString(Option<String>),
    OptionBool(Option<bool>),
    OptionVecI32(Option<Vec<i32>>),
    OptionVecF64(Option<Vec<f64>>),
    OptionVecString(Option<Vec<String>>),
    OptionVec2dI32(Option<Vec<Vec<i32>>>),
    OptionVec2dF64(Option<Vec<Vec<f64>>>),
    OptionVec2dString(Option<Vec<Vec<String>>>),
}

impl TypedParam {
    pub fn to_variant(&self) -> VARIANT {
        match self {
            Self::I32(v) => VARIANT::from(*v),
            Self::F64(v) => VARIANT::from(*v),
            Self::String(v) => VARIANT::from(BSTR::from(v)),
            Self::Bool(v) => VARIANT::from(*v),
            Self::VecI32(v) => {
                let sa = safe_array_from_vec1d::<i32>(v.clone()).unwrap();
                variant_with_ptr_from::<SafeArray<f64>>(sa)
            }
            Self::VecF64(v) => {
                let sa = safe_array_from_vec1d::<f64>(v.clone()).unwrap();
                variant_with_ptr_from::<SafeArray<f64>>(sa)
            }
            Self::VecString(v) => {
                let sa = safe_array_from_vec1d::<String>(v.clone()).unwrap();
                variant_with_ptr_from::<SafeArray<BSTR>>(sa)
            }
            Self::Vec2dI32(v) => {
                let sa = safe_array_from_vec2d::<i32>(v.clone()).unwrap();
                variant_with_ptr_from::<SafeArray<i32>>(sa)
            }
            Self::Vec2dF64(v) => {
                let sa = safe_array_from_vec2d::<f64>(v.clone()).unwrap();
                variant_with_ptr_from::<SafeArray<f64>>(sa)
            }
            Self::Vec2dString(v) => {
                let sa = safe_array_from_vec2d::<String>(v.clone()).unwrap();
                variant_with_ptr_from::<SafeArray<BSTR>>(sa)
            }

            Self::OptionI32(v) => match v {
                Some(v) => VARIANT::from(*v),
                None => VARIANT::default(),
            },
            Self::OptionF64(v) => match v {
                Some(v) => VARIANT::from(*v),
                None => VARIANT::default(),
            },
            Self::OptionString(v) => match v {
                Some(v) => VARIANT::from(BSTR::from(v.clone())),
                None => VARIANT::default(),
            },
            Self::OptionBool(v) => match v {
                Some(v) => VARIANT::from(*v),
                None => VARIANT::default(),
            },
            Self::OptionVecI32(v) => match v {
                Some(v) => {
                    let sa = safe_array_from_vec1d::<i32>(v.clone()).unwrap();
                    variant_with_ptr_from::<SafeArray<i32>>(sa)
                }
                None => VARIANT::default(),
            },
            Self::OptionVecF64(v) => match v {
                Some(v) => {
                    let sa = safe_array_from_vec1d::<f64>(v.clone()).unwrap();
                    variant_with_ptr_from::<SafeArray<f64>>(sa)
                }
                None => VARIANT::default(),
            },
            Self::OptionVecString(v) => match v {
                Some(v) => {
                    let sa = safe_array_from_vec1d::<String>(v.clone()).unwrap();
                    variant_with_ptr_from::<SafeArray<BSTR>>(sa)
                }
                None => VARIANT::default(),
            },

            Self::OptionVec2dI32(v) => match v {
                Some(v) => {
                    let sa = safe_array_from_vec2d::<i32>(v.clone()).unwrap();
                    variant_with_ptr_from::<SafeArray<i32>>(sa)
                }
                None => VARIANT::default(),
            },
            Self::OptionVec2dF64(v) => match v {
                Some(v) => {
                    let sa = safe_array_from_vec2d::<f64>(v.clone()).unwrap();
                    variant_with_ptr_from::<SafeArray<f64>>(sa)
                }
                None => VARIANT::default(),
            },
            Self::OptionVec2dString(v) => match v {
                Some(v) => {
                    let sa = safe_array_from_vec2d::<String>(v.clone()).unwrap();
                    variant_with_ptr_from::<SafeArray<BSTR>>(sa)
                }
                None => VARIANT::default(),
            },
        }
    }
}

impl From<i32> for TypedParam {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}
impl From<f64> for TypedParam {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}
impl From<String> for TypedParam {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<bool> for TypedParam {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl From<Vec<i32>> for TypedParam {
    fn from(value: Vec<i32>) -> Self {
        Self::VecI32(value)
    }
}
impl From<Vec<f64>> for TypedParam {
    fn from(value: Vec<f64>) -> Self {
        Self::VecF64(value)
    }
}
impl From<Vec<String>> for TypedParam {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
impl From<Vec<Vec<i32>>> for TypedParam {
    fn from(value: Vec<Vec<i32>>) -> Self {
        Self::Vec2dI32(value)
    }
}
impl From<Vec<Vec<f64>>> for TypedParam {
    fn from(value: Vec<Vec<f64>>) -> Self {
        Self::Vec2dF64(value)
    }
}
impl From<Vec<Vec<String>>> for TypedParam {
    fn from(value: Vec<Vec<String>>) -> Self {
        Self::Vec2dString(value)
    }
}

#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub arguments: Vec<ParamType>,
    pub indice: Vec<i32>,
    pub result: Vec<ResultType>,
    pub is_scaled: bool,
}

#[derive(Debug)]
pub enum StaadObject {
    Process(Arc<Mutex<StaadProcess>>),
    Root(Arc<Root>),
    Geometry(Arc<Geometry>),
    // Command(Arc<Command>),
    // Design(Arc<Design>),
    // Load(Arc<Load>),
    // Output(Arc<Output>),
    // Property(Arc<Property>),
    // Support(Arc<Support>),
}

pub fn execute_method(
    object: &StaadObject,
    name: &str,
    params: &[TypedParam],
) -> Result<Value, String> {
    let (methods, _dispatch, _staad) = match object {
        StaadObject::Root(root) => (&root.methods, &root.dispatch, &root.staad),
        StaadObject::Geometry(geometry) => (&geometry.methods, &geometry.dispatch, &geometry.staad),
        _ => return Err("Unsupported StaadObject".to_string()),
    };
    let dispatch = match _dispatch {
        Some(v) => v,
        None => return Err("Has no dispatch".to_string()),
    };
    let (_arguments, _indice, _result, _is_scaled) = match methods.get(name) {
        Some(sig) => (&sig.arguments, &sig.indice, &sig.result, &sig.is_scaled),
        None => return Err(format!("Unsupported method: {}", name)),
    };

    let params_count = params.len();
    let required_count = _indice.len();
    if params_count != required_count {
        return Err(format!(
            "'{}' method takes {} arguments but {} arguments were supplied",
            name, required_count, params_count
        ));
    }

    let mut unit_factor = 1.0;
    if *_is_scaled {
        let base_unit_result =
            unsafe { invoke_method(_staad.as_ref().unwrap(), "GetBaseUnit", &mut []) };
        if let Ok(base_unit_var) = base_unit_result {
            let base_unit = unsafe { VariantToInt32(&base_unit_var as *const VARIANT) };
            if let Ok(base_unit) = base_unit {
                if base_unit == 1i32 {
                    unit_factor = 39.37007874016;
                }
            }
        }
    }

    // Create separate storage for mutable pointers to ensure each has unique memory location
    let mut mut_storages: Vec<Box<dyn std::any::Any>> = Vec::new();
    let count = (&_arguments).len();
    let mut __variants: Vec<VARIANT> = _arguments
        .iter()
        .enumerate()
        .map(|(i, _type)| {
            if _indice.contains(&(i as i32)) {
                return params[i].to_variant();
            }
            // Create individual storage for each mutable parameter
            match _type {
                ParamType::MutI32 => {
                    let mut mut_value = Box::new(0i32);
                    let ptr = mut_value.as_mut() as *mut i32;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<i32>(ptr)
                }
                ParamType::MutF64 => {
                    let mut mut_value = Box::new(0f64);
                    let ptr = mut_value.as_mut() as *mut f64;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<f64>(ptr)
                }
                ParamType::MutString => {
                    let mut mut_value = Box::new(BSTR::default());
                    let ptr = mut_value.as_mut() as *mut BSTR;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<BSTR>(ptr)
                }
                ParamType::MutBool => {
                    let mut mut_value = Box::new(VARIANT_BOOL(-1));
                    let ptr = mut_value.as_mut() as *mut VARIANT_BOOL;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<bool>(ptr)
                }
                ParamType::MutVecI32 => unsafe {
                    let psa = SafeArrayCreateVector(VT_I4, 0, 0 as u32);
                    let mut mut_value = Box::new(psa);
                    let ptr = mut_value.as_mut() as *mut *mut SAFEARRAY;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<SafeArrayP<i32>>(ptr)
                },
                ParamType::MutVecF64 => unsafe {
                    let psa = SafeArrayCreateVector(VT_R8, 0, 0 as u32);
                    let mut mut_value = Box::new(psa);
                    let ptr = mut_value.as_mut() as *mut *mut SAFEARRAY;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<SafeArrayP<f64>>(ptr)
                },
                ParamType::MutVecString => unsafe {
                    let psa = SafeArrayCreateVector(VT_BSTR, 0, 0 as u32);
                    let mut mut_value = Box::new(psa);
                    let ptr = mut_value.as_mut() as *mut *mut SAFEARRAY;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<SafeArrayP<BSTR>>(ptr)
                },
                _ => VARIANT::default(),
            }
        })
        .collect();
    let mut __params: &mut [VARIANT] = &mut __variants[..];
    __params.reverse();
    unsafe {
        let invoke_result = invoke_method(dispatch, name, __params);
        match invoke_result {
            Ok(result_variant) => {
                let mut result_values: Vec<Value> = Vec::new();
                for (rti, rt) in _result.iter().enumerate() {
                    let serde_v = match rt {
                        ResultType::Index(i) => {
                            let par_type = _arguments.get(*i as usize).unwrap();
                            let _var = &__params[count - ((*i + 1) as usize)];
                            match par_type {
                                ParamType::MutI32 => {
                                    ResultType::Integer.to_value(_var, &unit_factor)
                                }
                                ParamType::MutF64 => {
                                    ResultType::Double.to_value(_var, &unit_factor)
                                }
                                ParamType::MutString => {
                                    ResultType::String.to_value(_var, &unit_factor)
                                }
                                ParamType::MutBool => ResultType::Bool.to_value(_var, &unit_factor),
                                ParamType::MutVecI32 => {
                                    ResultType::VecI.to_value(_var, &unit_factor)
                                }
                                ParamType::MutVecF64 => {
                                    ResultType::VecD.to_value(_var, &unit_factor)
                                }
                                ParamType::MutVecString => {
                                    ResultType::VecS.to_value(_var, &unit_factor)
                                }
                                _ => {
                                    Err(format!("Signature's Index result type is wrong: {}", name))
                                }
                            }
                        }
                        _ => rt.to_value(&result_variant, &unit_factor),
                    };
                    match serde_v {
                        Ok(_v) => {
                            result_values.push(_v);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }

                if result_values.len() > 0 {
                    if result_values.len() == 1 {
                        return serde_json::to_value(&result_values[0]).map_err(|e| e.to_string());
                    }
                    return serde_json::to_value(&result_values).map_err(|e| e.to_string());
                } else {
                    return Ok(serde_json::Value::Null);
                }
            }
            Err(e) => Err(format!("Fail to invoke method: {}", e)),
        }
        // println! {"10"};
    }
}
