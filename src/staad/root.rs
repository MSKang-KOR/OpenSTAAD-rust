use anyhow::{Error as anyErr, Ok as anyOk, Result, bail};
use windows::Win32::System::{
    Com::IDispatch,
    Variant::{VARIANT, VariantToInt32},
};
use windows_core::BSTR;

use crate::staad::{utils::invoke_method_with_result, variant::variant_from_raw_pointer};

#[derive(Debug)]
pub struct Root {
    pub dispatch: Option<IDispatch>,
}

impl Root {
    pub fn new(dispatch: &IDispatch) -> Self {
        Root {
            dispatch: Some(dispatch.clone()),
        }
    }

    pub async fn analyze_ex(&self, silent: i32, hidden: i32, wait: i32) -> Result<i32, anyErr> {
        let mut params = [
            VARIANT::from(wait),
            VARIANT::from(hidden),
            VARIANT::from(silent),
        ];
        let result_variant = unsafe {
            invoke_method_with_result(self.dispatch.as_ref().unwrap(), "AnalyzeEx", &mut params)
        };
        match result_variant {
            Ok(var) => {
                let status = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(status)
            }
            Err(e) => bail!("Error::Root::analyze_ex: {}", e),
        }
    }

    pub fn get_connected_project_info(&self) -> Result<(i32, i32, String), anyErr> {
        let sz_proj_id = &mut 0 as *mut i32;
        let sz_name = &mut BSTR::default() as *mut BSTR;
        let mut params = [
            variant_from_raw_pointer::<BSTR>(sz_name),
            variant_from_raw_pointer::<i32>(sz_proj_id),
        ];
        let result_variant = unsafe {
            invoke_method_with_result(
                self.dispatch.as_ref().unwrap(),
                "GetCONNECTEDProjectInfo",
                &mut params,
            )
        };
        match result_variant {
            Ok(var) => unsafe {
                let status = VariantToInt32(&var as *const VARIANT).unwrap();
                let id = *sz_proj_id;
                let name = &*sz_name;
                anyOk((status, id, name.to_string()))
            },
            Err(e) => bail!("Error::Root::get_connected_project_info: {}", e),
        }
    }

    pub fn get_staad_file(&self, full_path: bool) -> Result<String, anyErr> {
        let path_bstr = &mut BSTR::default() as *mut BSTR;
        let mut params = [
            VARIANT::from(full_path),
            variant_from_raw_pointer::<BSTR>(path_bstr),
        ];
        let result_variant = unsafe {
            invoke_method_with_result(self.dispatch.as_ref().unwrap(), "GetSTAADFile", &mut params)
        };
        match result_variant {
            Ok(var) => unsafe {
                let path = &*path_bstr;
                anyOk(path.to_string())
            },
            Err(e) => bail!("Error::Root::get_staad_file: {}", e),
        }
    }

    pub fn get_staad_file_folder(&self) -> Result<String, anyErr> {
        let path_bstr = &mut BSTR::default() as *mut BSTR;
        let mut params = [variant_from_raw_pointer::<BSTR>(path_bstr)];
        let result_variant = unsafe {
            invoke_method_with_result(
                self.dispatch.as_ref().unwrap(),
                "GetSTAADFileFolder",
                &mut params,
            )
        };
        match result_variant {
            Ok(var) => unsafe {
                let path = &*path_bstr;
                anyOk(path.to_string())
            },
            Err(e) => bail!("Error::Root::get_staad_file_folder: {}", e),
        }
    }
    pub fn set_silent_mode(&self, flag: i32) -> Result<i32, anyErr> {
        let mut params = [VARIANT::from(flag)];
        let result_variant = unsafe {
            invoke_method_with_result(
                self.dispatch.as_ref().unwrap(),
                "SetSilentMode",
                &mut params,
            )
        };
        match result_variant {
            Ok(var) => unsafe {
                let existing_value = VariantToInt32(&var as *const VARIANT).unwrap();
                anyOk(existing_value)
            },
            Err(e) => bail!("Error::Root::set_silent_mode: {}", e),
        }
    }

    pub fn get_base_unit(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method_with_result(self.dispatch.as_ref().unwrap(), "GetBaseUnit", &mut [])
        };
        match result_variant {
            Ok(var) => {
                let base_unit = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(base_unit)
            }
            Err(e) => bail!("Error::Root::get_base_unit: {}", e),
        }
    }

    pub fn set_input_units(&self, l_unit: i32, f_unit: i32) -> Result<(), anyErr> {
        let mut params = [VARIANT::from(f_unit), VARIANT::from(l_unit)];

        let result_variant = unsafe {
            invoke_method_with_result(
                self.dispatch.as_ref().unwrap(),
                "SetInputUnits",
                &mut params,
            )
        };
        match result_variant {
            Ok(_) => anyOk(()),
            Err(e) => bail!("Error::Root::set_input_units: {}", e),
        }
    }

    pub fn update_structure(&self) -> Result<(), anyErr> {
        let mut params = [];
        let result_variant = unsafe {
            invoke_method_with_result(
                self.dispatch.as_ref().unwrap(),
                "UpdateStructure",
                &mut params,
            )
        };
        match result_variant {
            Ok(_) => anyOk(()),
            Err(e) => bail!("Error::Root::update_structure: {}", e),
        }
    }
}
