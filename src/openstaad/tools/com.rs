use std::ptr::null_mut;

use anyhow::{Error as anyErr, Result as ResultAny, bail};
use windows::{
    Win32::System::{
        Com::{DISPATCH_METHOD, DISPATCH_PROPERTYGET, DISPPARAMS, EXCEPINFO, IDispatch},
        Variant::VARIANT,
    },
    core::{GUID, HSTRING, PCWSTR},
};

pub unsafe fn invoke_method(
    object: &IDispatch,
    method_name: &str,
    params: &mut [VARIANT],
) -> ResultAny<VARIANT> {
    let name = &PCWSTR::from_raw(HSTRING::from(method_name).as_ptr()) as *const PCWSTR;
    let mut dispid = 0;
    let iid = &GUID::default() as *const GUID;

    unsafe {
        let result = object.GetIDsOfNames(iid, name, 1, 0, &mut dispid);
        if let Err(e) = result {
            bail!(
                "Fail to get ids of names with '{}' method: {}",
                &method_name,
                e
            );
        }
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
        let invoked = object.Invoke(
            dispid,
            iid,
            0,
            DISPATCH_METHOD | DISPATCH_PROPERTYGET,
            &dispparams,
            Some(&mut result),
            Some(&mut excepinfo),
            None,
        );
        match invoked {
            Ok(()) => return Ok(result),
            Err(e) => {
                println!("invoke_method: {} failed with error: {:?}", method_name, e);
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
    let result: ResultAny<VARIANT, anyErr> = unsafe { invoke_method(object, method_name, params) };
    match result {
        Ok(v) => {
            match IDispatch::try_from(&v) {
                Ok(dispatch) => Ok(dispatch), // 성공 시 Ok로 감싸서 반환
                Err(_) => bail!("VARIANT을 IDispatch로 변환할 수 없습니다"),
            }
        }
        Err(e) => bail!("메서드 호출 실패: {}", e),
    }
}
