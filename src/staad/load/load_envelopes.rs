use crate::staad::{
    geometry::root::Geometry,
    load::root::Load,
    property::root::Property,
    safe_array::{safe_array_from_vec1d, safe_array_from_vec2d},
    utils::invoke_method_with_result,
    variant::{SafeArray, SafeArrayP, variant_from_raw_pointer},
};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use std::ffi::c_void;
use windows::Win32::System::{
    Com::SAFEARRAY,
    Ole::{SafeArrayCreateVector, SafeArrayGetElement},
    Variant::{VARIANT, VT_I4, VT_R8, VariantToDouble, VariantToInt32, VariantToStringAlloc},
};
use windows_core::BSTR;

#[derive(Debug)]
pub struct LoadEnvelopes<'a> {
    pub load: &'a Load<'a>,
}

impl<'a> LoadEnvelopes<'a> {
    pub fn new(load: &'a Load<'a>) -> Self {
        Self { load }
    }
}
