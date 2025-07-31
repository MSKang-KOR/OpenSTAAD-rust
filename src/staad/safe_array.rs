use anyhow::{Result, bail};
use std::ffi::c_void;
use windows::Win32::System::{
    Com::{SAFEARRAY, SAFEARRAYBOUND},
    Ole::{SafeArrayCreate, SafeArrayCreateVector, SafeArrayPutElement},
    Variant::{VARENUM, VT_I4, VT_R8},
};

// SafeArray 생성을 위한 트레이트 정의
pub trait SafeArrayCreator {
    const VT_TYPE: VARENUM;
}

// i32 타입에 대한 구현
impl SafeArrayCreator for i32 {
    const VT_TYPE: VARENUM = VT_I4;
}

// f64 타입에 대한 구현
impl SafeArrayCreator for f64 {
    const VT_TYPE: VARENUM = VT_R8;
}

// 제네릭 함수들
pub fn safe_array_from_vec1d<T: SafeArrayCreator + Copy>(data: Vec<T>) -> Result<*mut SAFEARRAY> {
    unsafe {
        let sa = SafeArrayCreateVector(T::VT_TYPE, 0, data.len() as u32);
        if sa.is_null() {
            bail!("1D SAFEARRAY 생성 실패");
        }

        for (i, &value) in data.iter().enumerate() {
            let mut index = i as i32;
            SafeArrayPutElement(
                sa,
                &mut index as *mut i32,
                &value as *const T as *const c_void,
            )?;
        }
        Ok(sa)
    }
}

pub fn safe_array_from_vec2d<T: SafeArrayCreator + Copy>(
    data: Vec<Vec<T>>,
) -> Result<*mut SAFEARRAY> {
    if data.is_empty() {
        bail!("빈 2D 배열은 생성할 수 없습니다");
    }

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
        if sa.is_null() {
            bail!("2D SAFEARRAY 생성 실패");
        }

        for (i, row) in data.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                let indices = [i as i32, j as i32];
                SafeArrayPutElement(sa, indices.as_ptr(), &value as *const T as *const c_void)?;
            }
        }
        Ok(sa)
    }
}

// 사용 예시:
// 제네릭 함수 직접 사용 (원하시던 방식)
// let sa = create_1d_array::<i32>(vec![1, 2, 3, 4, 5])?;
// let sa = create_2d_array::<f64>(vec![vec![1.0, 2.0], vec![3.0, 4.0]])?;
//
// SafeArrayHelper 사용
// let sa = SafeArrayHelper::from_1d(vec![1, 2, 3])?; // 타입 추론
// let sa = SafeArrayHelper::from_2d(vec![vec![1.0, 2.0], vec![3.0, 4.0]])?; // 타입 추론
//
// 타입별 헬퍼 사용
// let sa = SafeArrayHelper::from_i32_1d(vec![1, 2, 3])?;
// let sa = SafeArrayHelper::from_f64_2d(vec![vec![1.0, 2.0], vec![3.0, 4.0]])?;
