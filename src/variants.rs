// variants.rs
use windows::Win32::System::Com::{SAFEARRAY, SAFEARRAYBOUND};
use windows::Win32::System::Ole::{SafeArrayCreate, SafeArrayPutElement};
use windows::Win32::System::Variant::{
    VARIANT, VT_ARRAY, VT_BOOL, VT_BSTR, VT_EMPTY, VT_I2, VT_I4, VT_I8, VT_R4, VT_R8, VT_UI2,
    VT_UI4, VT_UI8,
};
use windows::core::{BSTR, HSTRING};

// VARIANT wrapper 타입 정의
#[derive(Debug)]
pub struct VariantWrapper(pub VARIANT);

impl VariantWrapper {
    pub fn into_variant(self) -> VARIANT {
        self.0
    }

    pub fn as_variant(&self) -> &VARIANT {
        &self.0
    }

    pub fn as_variant_mut(&mut self) -> &mut VARIANT {
        &mut self.0
    }
}

// 또는 ToVariant trait을 정의하여 사용
pub trait ToVariant {
    fn to_variant(self) -> VARIANT;
}

// i32 (LONG) 타입
impl From<i32> for VariantWrapper {
    fn from(value: i32) -> Self {
        unsafe {
            let mut variant = VARIANT::default();
            variant.Anonymous.Anonymous.vt = VT_I4;
            variant.Anonymous.Anonymous.Anonymous.lVal = value;
            VariantWrapper(variant)
        }
    }
}

impl ToVariant for i32 {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// i16 (SHORT) 타입
impl From<i16> for VariantWrapper {
    fn from(value: i16) -> Self {
        unsafe {
            let mut variant = VARIANT::default();
            variant.Anonymous.Anonymous.vt = VT_I2;
            variant.Anonymous.Anonymous.Anonymous.iVal = value;
            VariantWrapper(variant)
        }
    }
}

impl ToVariant for i16 {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// i64 (LONGLONG) 타입
impl From<i64> for VariantWrapper {
    fn from(value: i64) -> Self {
        unsafe {
            let mut variant = VARIANT::default();
            variant.Anonymous.Anonymous.vt = VT_I8;
            variant.Anonymous.Anonymous.Anonymous.llVal = value;
            VariantWrapper(variant)
        }
    }
}

impl ToVariant for i64 {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// u32 (ULONG) 타입
impl From<u32> for VariantWrapper {
    fn from(value: u32) -> Self {
        unsafe {
            let mut variant = VARIANT::default();
            variant.Anonymous.Anonymous.vt = VT_UI4;
            variant.Anonymous.Anonymous.Anonymous.ulVal = value;
            VariantWrapper(variant)
        }
    }
}

impl ToVariant for u32 {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// u16 (USHORT) 타입
impl From<u16> for VariantWrapper {
    fn from(value: u16) -> Self {
        unsafe {
            let mut variant = VARIANT::default();
            variant.Anonymous.Anonymous.vt = VT_UI2;
            variant.Anonymous.Anonymous.Anonymous.uiVal = value;
            VariantWrapper(variant)
        }
    }
}

impl ToVariant for u16 {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// u64 (ULONGLONG) 타입
impl From<u64> for VariantWrapper {
    fn from(value: u64) -> Self {
        unsafe {
            let mut variant = VARIANT::default();
            variant.Anonymous.Anonymous.vt = VT_UI8;
            variant.Anonymous.Anonymous.Anonymous.ullVal = value;
            VariantWrapper(variant)
        }
    }
}

impl ToVariant for u64 {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// f32 (FLOAT) 타입
impl From<f32> for VariantWrapper {
    fn from(value: f32) -> Self {
        unsafe {
            let mut variant = VARIANT::default();
            variant.Anonymous.Anonymous.vt = VT_R4;
            variant.Anonymous.Anonymous.Anonymous.fltVal = value;
            VariantWrapper(variant)
        }
    }
}

impl ToVariant for f32 {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// f64 (DOUBLE) 타입
impl From<f64> for VariantWrapper {
    fn from(value: f64) -> Self {
        unsafe {
            let mut variant = VARIANT::default();
            variant.Anonymous.Anonymous.vt = VT_R8;
            variant.Anonymous.Anonymous.Anonymous.dblVal = value;
            VariantWrapper(variant)
        }
    }
}

impl ToVariant for f64 {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// bool 타입
impl From<bool> for VariantWrapper {
    fn from(value: bool) -> Self {
        unsafe {
            let mut variant = VARIANT::default();
            variant.Anonymous.Anonymous.vt = VT_BOOL;
            variant.Anonymous.Anonymous.Anonymous.boolVal = if value {
                windows::Win32::Foundation::VARIANT_TRUE
            } else {
                windows::Win32::Foundation::VARIANT_FALSE
            };
            VariantWrapper(variant)
        }
    }
}

impl ToVariant for bool {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// String 타입 (BSTR)
impl From<String> for VariantWrapper {
    fn from(value: String) -> Self {
        unsafe {
            let mut variant = VARIANT::default();
            let hstring = HSTRING::from(&value);
            let bstr = BSTR::from(&hstring);
            variant.Anonymous.Anonymous.vt = VT_BSTR;
            variant.Anonymous.Anonymous.Anonymous.bstrVal = std::mem::transmute(bstr);
            VariantWrapper(variant)
        }
    }
}

impl ToVariant for String {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// &str 타입 (BSTR)
impl From<&str> for VariantWrapper {
    fn from(value: &str) -> Self {
        VariantWrapper::from(String::from(value))
    }
}

impl ToVariant for &str {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// Vec<i32> 타입 (SAFEARRAY)
impl From<Vec<i32>> for VariantWrapper {
    fn from(value: Vec<i32>) -> Self {
        unsafe {
            let mut variant = VARIANT::default();

            if value.is_empty() {
                variant.Anonymous.Anonymous.vt = VT_EMPTY;
                return VariantWrapper(variant);
            }

            let bounds = SAFEARRAYBOUND {
                cElements: value.len() as u32,
                lLbound: 0,
            };

            let safe_array = SafeArrayCreate(VT_I4, 1, &bounds);

            if !safe_array.is_null() {
                for (i, &val) in value.iter().enumerate() {
                    let index = i as i32;
                    let element_variant = VariantWrapper::from(val).into_variant();
                    SafeArrayPutElement(
                        safe_array,
                        &index,
                        &element_variant as *const _ as *const _,
                    );
                }

                variant.Anonymous.Anonymous.vt = VT_ARRAY | VT_I4;
                variant.Anonymous.Anonymous.Anonymous.parray = safe_array;
            }

            VariantWrapper(variant)
        }
    }
}

impl ToVariant for Vec<i32> {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// Vec<f64> 타입 (SAFEARRAY)
impl From<Vec<f64>> for VariantWrapper {
    fn from(value: Vec<f64>) -> Self {
        unsafe {
            let mut variant = VARIANT::default();

            if value.is_empty() {
                variant.Anonymous.Anonymous.vt = VT_EMPTY;
                return VariantWrapper(variant);
            }

            let bounds = SAFEARRAYBOUND {
                cElements: value.len() as u32,
                lLbound: 0,
            };

            let safe_array = SafeArrayCreate(VT_R8, 1, &bounds);

            if !safe_array.is_null() {
                for (i, &val) in value.iter().enumerate() {
                    let index = i as i32;
                    let element_variant = VariantWrapper::from(val).into_variant();
                    SafeArrayPutElement(
                        safe_array,
                        &index,
                        &element_variant as *const _ as *const _,
                    );
                }

                variant.Anonymous.Anonymous.vt = VT_ARRAY | VT_R8;
                variant.Anonymous.Anonymous.Anonymous.parray = safe_array;
            }

            VariantWrapper(variant)
        }
    }
}

impl ToVariant for Vec<f64> {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// Vec<String> 타입 (SAFEARRAY)
impl From<Vec<String>> for VariantWrapper {
    fn from(value: Vec<String>) -> Self {
        unsafe {
            let mut variant = VARIANT::default();

            if value.is_empty() {
                variant.Anonymous.Anonymous.vt = VT_EMPTY;
                return VariantWrapper(variant);
            }

            let bounds = SAFEARRAYBOUND {
                cElements: value.len() as u32,
                lLbound: 0,
            };

            let safe_array = SafeArrayCreate(VT_BSTR, 1, &bounds);

            if !safe_array.is_null() {
                for (i, val) in value.iter().enumerate() {
                    let index = i as i32;
                    let element_variant = VariantWrapper::from(val.clone()).into_variant();
                    SafeArrayPutElement(
                        safe_array,
                        &index,
                        &element_variant as *const _ as *const _,
                    );
                }

                variant.Anonymous.Anonymous.vt = VT_ARRAY | VT_BSTR;
                variant.Anonymous.Anonymous.Anonymous.parray = safe_array;
            }

            VariantWrapper(variant)
        }
    }
}

impl ToVariant for Vec<String> {
    fn to_variant(self) -> VARIANT {
        VariantWrapper::from(self).into_variant()
    }
}

// Option<T> 타입 (None의 경우 VT_EMPTY)
impl<T> From<Option<T>> for VariantWrapper
where
    T: Into<VariantWrapper>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => {
                let mut variant = VARIANT::default();
                variant.Anonymous.Anonymous.vt = VT_EMPTY;
                VariantWrapper(variant)
            }
        }
    }
}

impl<T> ToVariant for Option<T>
where
    T: ToVariant,
{
    fn to_variant(self) -> VARIANT {
        match self {
            Some(v) => v.to_variant(),
            None => {
                let mut variant = VARIANT::default();
                variant.Anonymous.Anonymous.vt = VT_EMPTY;
                variant
            }
        }
    }
}

// 빈 VARIANT 생성을 위한 Default 구현
impl Default for VariantWrapper {
    fn default() -> Self {
        unsafe { VariantWrapper(std::mem::zeroed()) }
    }
}

// 헬퍼 함수들
pub fn create_empty_variant() -> VARIANT {
    VariantWrapper::from(None::<i32>).into_variant()
}

pub fn create_variant_from_i32_vec(vec: Vec<i32>) -> VARIANT {
    VariantWrapper::from(vec).into_variant()
}

pub fn create_variant_from_string_vec(vec: Vec<String>) -> VARIANT {
    VariantWrapper::from(vec).into_variant()
}

// 편의 함수들 - ToVariant trait을 사용
pub fn to_variant<T: ToVariant>(value: T) -> VARIANT {
    value.to_variant()
}

// 매크로를 사용한 편의 함수
#[macro_export]
macro_rules! variant {
    ($value:expr) => {
        $crate::variants::to_variant($value)
    };
}
