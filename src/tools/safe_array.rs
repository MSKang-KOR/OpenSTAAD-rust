use anyhow::{Result, bail};
use std::ffi::c_void;
use std::mem;
use windows::Win32::System::{
    Com::{SAFEARRAY, SAFEARRAYBOUND},
    Ole::{
        SafeArrayCreate, SafeArrayCreateVector, SafeArrayGetElement, SafeArrayGetLBound,
        SafeArrayGetUBound, SafeArrayPutElement,
    },
    Variant::{VARENUM, VT_BSTR, VT_I4, VT_R8},
};
use windows_core::BSTR;

// SafeArray 생성을 위한 트레이트 정의
pub trait SafeArrayCreator {
    const VT_TYPE: VARENUM;

    // SAFEARRAY에 값을 넣는 메서드
    unsafe fn put(&self) -> *const c_void;
}

// i32 타입에 대한 구현
impl SafeArrayCreator for i32 {
    const VT_TYPE: VARENUM = VT_I4;

    unsafe fn put(&self) -> *const c_void {
        self as *const i32 as *const c_void
    }
}

// f64 타입에 대한 구현
impl SafeArrayCreator for f64 {
    const VT_TYPE: VARENUM = VT_R8;

    unsafe fn put(&self) -> *const c_void {
        self as *const f64 as *const c_void
    }
}

impl SafeArrayCreator for String {
    const VT_TYPE: VARENUM = VT_BSTR;

    unsafe fn put(&self) -> *const c_void {
        // String을 BSTR로 변환
        let mut bstr = BSTR::from(self.as_str());
        let bstr_ptr = &mut bstr as *mut BSTR;
        // BSTR을 heap에 저장하여 메모리가 유지되도록 함
        // let bstr_ptr = Box::into_raw(Box::new(bstr));
        bstr_ptr as *const c_void
    }
}

pub fn safe_array_from_vec1d<T: SafeArrayCreator + Clone>(data: Vec<T>) -> Result<*mut SAFEARRAY> {
    unsafe {
        let sa = SafeArrayCreateVector(T::VT_TYPE, 0, data.len() as u32);

        for (i, value) in data.iter().enumerate() {
            let mut index = i as i32;
            SafeArrayPutElement(sa, &mut index as *mut i32, value.put() as *mut c_void)?;
        }
        Ok(sa)
    }
}

pub fn safe_array_from_vec2d<T: SafeArrayCreator + Clone>(
    data: Vec<Vec<T>>,
) -> Result<*mut SAFEARRAY> {
    unsafe {
        let rows = data.len() as u32;
        let cols = data[0].len() as u32;

        let mut bounds = [
            SAFEARRAYBOUND {
                cElements: rows,
                lLbound: 0,
            },
            SAFEARRAYBOUND {
                cElements: cols,
                lLbound: 0,
            },
        ];

        let sa = SafeArrayCreate(T::VT_TYPE, 2, bounds.as_mut_ptr());
        for (i, row) in data.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                let indices = [i as i32, j as i32];
                SafeArrayPutElement(sa, indices.as_ptr(), value.put() as *mut c_void)?;
            }
        }
        Ok(sa)
    }
}

pub trait SafeArrayExtractor {
    type Output;
    const VT_TYPE: VARENUM;

    // SAFEARRAY에서 값을 추출하는 메서드
    unsafe fn extract_from_safearray(ptr: *mut c_void) -> Self::Output;
}

// i32 타입에 대한 구현
impl SafeArrayExtractor for i32 {
    type Output = i32;
    const VT_TYPE: VARENUM = VT_I4;

    unsafe fn extract_from_safearray(ptr: *mut c_void) -> Self::Output {
        *(ptr as *mut i32)
    }
}

// f64 타입에 대한 구현
impl SafeArrayExtractor for f64 {
    type Output = f64;
    const VT_TYPE: VARENUM = VT_R8;

    unsafe fn extract_from_safearray(ptr: *mut c_void) -> Self::Output {
        *(ptr as *mut f64)
    }
}

// BSTR 타입에 대한 구현 (String 반환)
impl SafeArrayExtractor for BSTR {
    type Output = String;
    const VT_TYPE: VARENUM = VT_BSTR;

    unsafe fn extract_from_safearray(ptr: *mut c_void) -> Self::Output {
        // let bstr_ptr = *(ptr as *mut BSTR);
        let bstr_ptr = ptr as *mut BSTR;
        let str = unsafe { (*bstr_ptr).to_string() };
        str
    }
}

pub fn safe_array_to_vec1d<T: SafeArrayExtractor>(sa: *mut SAFEARRAY) -> Result<Vec<T::Output>> {
    unsafe {
        let ub = SafeArrayGetUBound(sa, 1)?;
        let lb = SafeArrayGetLBound(sa, 1)?;
        let count = ub - lb + 1;
        let mut vec = Vec::with_capacity(count as usize);

        for i in 0..count {
            let mut index = lb + i;
            // 타입에 맞는 크기의 버퍼 생성
            let mut buffer = vec![0u8; mem::size_of::<T>()];

            SafeArrayGetElement(
                sa,
                &mut index as *mut i32,
                buffer.as_mut_ptr() as *mut c_void,
            )?;

            let value = T::extract_from_safearray(buffer.as_mut_ptr() as *mut c_void);
            vec.push(value);
        }

        Ok(vec)
    }
}

pub fn safe_array_to_vec2d<T: SafeArrayExtractor>(
    sa: *mut SAFEARRAY,
) -> Result<Vec<Vec<T::Output>>> {
    unsafe {
        let row_ub = SafeArrayGetUBound(sa, 1)?;
        let row_lb = SafeArrayGetLBound(sa, 1)?;
        let col_ub = SafeArrayGetUBound(sa, 2)?;
        let col_lb = SafeArrayGetLBound(sa, 2)?;

        let row_count = (row_ub - row_lb + 1) as usize;
        let col_count = (col_ub - col_lb + 1) as usize;

        let mut vec = Vec::with_capacity(row_count);

        for i in 0..row_count {
            let mut row = Vec::with_capacity(col_count);

            for j in 0..col_count {
                let indices = [row_lb + i as i32, col_lb + j as i32];
                let mut buffer = vec![0u8; mem::size_of::<T>()];

                SafeArrayGetElement(sa, indices.as_ptr(), buffer.as_mut_ptr() as *mut c_void)?;

                let value = T::extract_from_safearray(buffer.as_mut_ptr() as *mut c_void);
                row.push(value);
            }

            vec.push(row);
        }

        Ok(vec)
    }
}
