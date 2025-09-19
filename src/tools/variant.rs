use std::mem::ManuallyDrop;

use windows::Win32::{
    Foundation::VARIANT_BOOL,
    System::{
        Com::{IDispatch, SAFEARRAY},
        Variant::{
            VARENUM, VARIANT, VARIANT_0_0, VARIANT_0_0_0, VT_ARRAY, VT_BSTR, VT_BYREF, VT_DISPATCH,
            VT_I4, VT_PTR, VT_R8, VT_SAFEARRAY, VT_STORAGE, VariantToDouble,
        },
    },
};
use windows_core::BSTR;

pub trait VariantWithPtr {
    type PointerType;
    type ValueType;
    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0);
    fn get_value(variant: &VARIANT) -> Self::ValueType;
}

impl VariantWithPtr for i32 {
    type PointerType = *mut i32;
    type ValueType = i32;

    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
        (VT_BYREF | VT_I4, VARIANT_0_0_0 { pintVal: ptr })
    }
    fn get_value(variant: &VARIANT) -> Self::ValueType {
        unsafe { *variant.Anonymous.Anonymous.Anonymous.pintVal }
    }
}
impl VariantWithPtr for f64 {
    type PointerType = *mut f64;
    type ValueType = f64;

    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
        (VT_BYREF | VT_R8, VARIANT_0_0_0 { pdblVal: ptr })
    }
    fn get_value(variant: &VARIANT) -> Self::ValueType {
        unsafe {
            // println!("{:#?}", *variant.Anonymous.Anonymous.Anonymous.pdblVal);
            *variant.Anonymous.Anonymous.Anonymous.pdblVal
            // VariantToDouble(variant as *const VARIANT).unwrap()
        }
    }
}
impl VariantWithPtr for BSTR {
    type PointerType = *mut BSTR;
    type ValueType = String;

    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
        (VT_BYREF | VT_BSTR, VARIANT_0_0_0 { pbstrVal: ptr })
    }
    fn get_value(variant: &VARIANT) -> Self::ValueType {
        unsafe { (&*variant.Anonymous.Anonymous.Anonymous.pbstrVal).to_string() }
    }
}

impl VariantWithPtr for bool {
    type PointerType = *mut VARIANT_BOOL;
    type ValueType = bool;

    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
        (VT_BYREF | VT_BSTR, VARIANT_0_0_0 { pboolVal: ptr })
    }
    fn get_value(variant: &VARIANT) -> Self::ValueType {
        unsafe { (*variant.Anonymous.Anonymous.Anonymous.pboolVal).0 != 0 }
    }
}

// impl VariantWithPtr for Option<IDispatch> {
//     type PointerType = *mut Option<IDispatch>;
//     type ValueType = IDispatch;

//     fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
//         (VT_BYREF | VT_DISPATCH, VARIANT_0_0_0 { ppdispVal: ptr })
//     }
//     fn get_value(variant: &VARIANT) -> Self::ValueType {
//         unsafe {
//             (&*variant.Anonymous.Anonymous.Anonymous.ppdispVal)
//                 .clone()
//                 .unwrap()
//         }
//     }
// }

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

impl<T: SafeArrayElement> VariantWithPtr for SafeArrayP<T> {
    type PointerType = *mut *mut SAFEARRAY;
    type ValueType = *mut SAFEARRAY;

    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
        (
            VT_BYREF | VT_ARRAY | T::VT_TYPE,
            VARIANT_0_0_0 { pparray: ptr },
        )
    }
    fn get_value(variant: &VARIANT) -> Self::ValueType {
        unsafe { *variant.Anonymous.Anonymous.Anonymous.pparray }
    }
}
impl<T: SafeArrayElement> VariantWithPtr for SafeArray<T> {
    type PointerType = *mut SAFEARRAY;
    type ValueType = *mut SAFEARRAY;

    fn get_prop(ptr: Self::PointerType) -> (VARENUM, VARIANT_0_0_0) {
        (VT_ARRAY | T::VT_TYPE, VARIANT_0_0_0 { parray: ptr })
    }
    fn get_value(variant: &VARIANT) -> Self::ValueType {
        unsafe {
            let tt = *variant.Anonymous.Anonymous.Anonymous.pparray;
            return tt;
        }
    }
}

pub fn variant_with_ptr_from<T: VariantWithPtr>(ptr: T::PointerType) -> VARIANT {
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

pub fn variant_with_ptr_to<T: VariantWithPtr>(variant: &VARIANT) -> T::ValueType {
    T::get_value(variant)
}
