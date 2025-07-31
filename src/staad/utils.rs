use std::ptr::null_mut;

use anyhow::{Context, Error as anyErr, Result as ResultAny, bail};
use windows::{
    Win32::System::{
        Com::{DISPATCH_METHOD, DISPATCH_PROPERTYGET, DISPPARAMS, EXCEPINFO, IDispatch},
        Variant::VARIANT,
    },
    core::{GUID, HSTRING, PCWSTR},
};

pub unsafe fn invoke_method_with_result(
    object: &IDispatch,
    method_name: &str,
    params: &mut [VARIANT],
) -> ResultAny<VARIANT> {
    let name = &PCWSTR::from_raw(HSTRING::from(method_name).as_ptr()) as *const PCWSTR;
    let mut dispid = 0;
    let iid = &GUID::default() as *const GUID;

    unsafe {
        let _ = object
            .GetIDsOfNames(iid, name, 1, 0, &mut dispid)
            .context("Method ID 가져오기 실패");
    };

    let mut result = VARIANT::default();
    let mut excepinfo = EXCEPINFO::default();

    let dispparams = DISPPARAMS {
        rgvarg: params.as_mut_ptr(),
        cArgs: params.len() as u32,
        cNamedArgs: 0,
        rgdispidNamedArgs: null_mut(),
    };

    unsafe {
        let invoked = object
            .Invoke(
                dispid,
                iid,
                0,
                DISPATCH_METHOD | DISPATCH_PROPERTYGET,
                &dispparams,
                Some(&mut result),
                Some(&mut excepinfo),
                None,
            )
            .context("Method 실행 실패");

        match invoked {
            Ok(()) => {
                return Ok(result);
            }
            Err(e) => {
                // println!("NG: {:#?}", method_name);
                bail!("{:#?}: {:#?}", e, method_name);
            }
        }
    };
}

pub unsafe fn get_dispatch(
    object: &IDispatch,
    method_name: &str,
    params: &mut [VARIANT],
) -> ResultAny<IDispatch, anyErr> {
    let result: ResultAny<VARIANT, anyErr> =
        unsafe { invoke_method_with_result(object, method_name, params) };
    match result {
        Ok(var) => {
            match IDispatch::try_from(&var) {
                Ok(dispatch) => Ok(dispatch), // 성공 시 Ok로 감싸서 반환
                Err(_) => bail!("VARIANT을 IDispatch로 변환할 수 없습니다"),
            }
        }
        Err(e) => bail!("메서드 호출 실패: {}", e),
    }
}
