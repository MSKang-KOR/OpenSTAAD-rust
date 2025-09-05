use log::info;
use std::ptr::null_mut;

use anyhow::{Result, anyhow, bail};
use windows::{
    Win32::System::{
        Com::{DISPATCH_METHOD, DISPATCH_PROPERTYGET, DISPPARAMS, IDispatch},
        Variant::VARIANT,
    },
    core::{GUID, HSTRING, PCWSTR},
};

pub unsafe fn invoke_method(
    dispatch: &IDispatch,
    method: &str,
    params: &mut [VARIANT],
) -> Result<VARIANT> {
    let name = &PCWSTR::from_raw(HSTRING::from(method).as_ptr()) as *const PCWSTR;
    let mut dispid = 0;
    unsafe {
        let result =
            dispatch.GetIDsOfNames(&GUID::default() as *const GUID, name, 1, 0, &mut dispid);
        if let Err(e) = result {
            bail!("Fail to get ids of names with '{}' method: {}", &method, e);
        }
    };

    // Prepare to receive the result.
    let mut result_variant = VARIANT::default();
    let dispparams = DISPPARAMS {
        rgvarg: params.as_mut_ptr(),
        cArgs: params.len() as u32,
        cNamedArgs: 0,
        rgdispidNamedArgs: null_mut(),
    };
    unsafe {
        dispatch
            .Invoke(
                dispid,
                &GUID::default() as *const GUID,
                0,
                DISPATCH_METHOD | DISPATCH_PROPERTYGET,
                &dispparams,
                Some(&mut result_variant),
                None,
                None,
            )
            .map_err(|e| anyhow!("Invoke '{}' method failed: {}", method, e))?;
    };
    Ok(result_variant)
}

pub unsafe fn invoke_property(dispatch: &IDispatch, property: &str) -> Result<IDispatch> {
    let mut dispid = 0;
    let name = HSTRING::from(property);
    unsafe {
        dispatch
            .GetIDsOfNames(&GUID::default(), &PCWSTR(name.as_ptr()), 1, 0, &mut dispid)
            .map_err(|e| {
                anyhow!(
                    "Fail to get ids of names with '{}' property: {}",
                    property,
                    e
                )
            })?;
    };

    // Prepare to receive the result.
    let mut result_variant = VARIANT::default();

    // No arguments needed to get a property.
    let disp_params = DISPPARAMS::default();

    unsafe {
        dispatch
            .Invoke(
                dispid,
                &GUID::default(),
                0,
                DISPATCH_PROPERTYGET,
                &disp_params,
                Some(&mut result_variant),
                None,
                None,
            )
            .map_err(|e| anyhow!("Invoke '{}' property failed: {}", property, e))?;
    };

    match IDispatch::try_from(&result_variant) {
        Ok(dispatch) => Ok(dispatch),
        Err(_) => bail!(
            "Fail to convert '{}' property VARIANT to IDispatch",
            property
        ),
    }
}
