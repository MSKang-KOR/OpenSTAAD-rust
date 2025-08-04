use std::mem::ManuallyDrop;

use windows::Win32::System::{
    Com::{IDispatch, SAFEARRAY},
    Variant::{
        VARENUM, VARIANT, VARIANT_0_0, VARIANT_0_0_0, VT_ARRAY, VT_BSTR, VT_BYREF, VT_DISPATCH, VT_I4, VT_PTR, VT_R8, VT_SAFEARRAY, VT_STORAGE
    },
};
use windows_core::BSTR;

pub trait VariantCreator {
    type PointerType;
    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0);
}

impl VariantCreator for i32 {
    type PointerType = *mut i32;

    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
        (VT_BYREF | VT_I4, VARIANT_0_0_0 { pintVal: ptr })
    }
}
impl VariantCreator for f64 {
    type PointerType = *mut f64;

    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
        (VT_BYREF | VT_R8, VARIANT_0_0_0 { pdblVal: ptr })
    }
}
impl VariantCreator for BSTR {
    type PointerType = *mut BSTR;

    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
        (VT_BYREF | VT_BSTR, VARIANT_0_0_0 { pbstrVal: ptr })
    }
}

impl VariantCreator for Option<IDispatch> {
    type PointerType = *mut Option<IDispatch>;

    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
        (VT_BYREF | VT_DISPATCH, VARIANT_0_0_0 { ppdispVal: ptr })
    }
}

pub trait SafeArrayElement {
    const VT_TYPE: VARENUM;
}

impl SafeArrayElement for i32 {
    const VT_TYPE: VARENUM = VT_I4;
}

impl SafeArrayElement for f64 {
    const VT_TYPE: VARENUM = VT_R8;
}

impl SafeArrayElement for BSTR {
    const VT_TYPE: VARENUM = VT_BSTR;
}

impl SafeArrayElement for SAFEARRAY {
    const VT_TYPE: VARENUM = VT_SAFEARRAY;
}

pub struct SafeArrayP<T: SafeArrayElement>(std::marker::PhantomData<T>);
pub struct SafeArray<T: SafeArrayElement>(std::marker::PhantomData<T>);

impl<T: SafeArrayElement> VariantCreator for SafeArrayP<T> {
    type PointerType = *mut *mut SAFEARRAY;

    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
        (
            VT_BYREF | VT_ARRAY | T::VT_TYPE,
            VARIANT_0_0_0 { pparray: ptr },
        )
    }
}
impl<T: SafeArrayElement> VariantCreator for SafeArray<T> {
    type PointerType = *mut SAFEARRAY;

    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
        (VT_ARRAY | T::VT_TYPE, VARIANT_0_0_0 { parray: ptr })
    }
}

pub fn variant_from_raw_pointer<T: VariantCreator>(ptr: T::PointerType) -> VARIANT {
    let (vt, anonymous) = T::get_prop(ptr);

    let mut var = VARIANT::default();
    var.Anonymous.Anonymous = ManuallyDrop::new(VARIANT_0_0 {
        vt,
        wReserved1: 0,
        wReserved2: 0,
        wReserved3: 0,
        Anonymous: anonymous,
    });
    var
}
