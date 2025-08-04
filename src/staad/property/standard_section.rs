use crate::staad::{
    geometry::root::Geometry,
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
    Variant::{VARIANT, VT_I4, VariantToDouble, VariantToInt32, VariantToStringAlloc},
};
use windows_core::BSTR;

// GetDefaultStandardProfileDBFolder
// GetStandardProfileDBFolder
// GetStandardSectionDatabaseName
// GetStandardSectionName
// GetStandardSectionTableName
// IsStandardDatabaseSection
// SetStandardProfileDBFolder

#[derive(Debug)]
pub struct StandardSection<'a> {
    pub property: &'a Property<'a>,
}

impl<'a> StandardSection<'a> {
    pub fn new(property: &'a Property<'a>) -> Self {
        Self { property }
    }

    /// Gets standard profile default database folder path.
    /// # Returns
    /// * The standard profile default database folder path.
    pub fn get_default_standard_profile_db_folder(&self) -> Result<String, anyErr> {
        let result_variant = unsafe {
            invoke_method_with_result(
                &self.property.dispatch,
                "GetDefaultStandardProfileDBFolder",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let folder_path = unsafe {
                    VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?
                };
                anyOk(folder_path)
            }
            Err(e) => bail!(
                "Error::Property::get_default_standard_profile_db_folder: {}",
                e
            ),
        }
    }

    /// Gets standard profile database folder path.
    /// # Returns
    /// * The standard profile database folder path.
    pub fn get_standard_profile_db_folder(&self) -> Result<String, anyErr> {
        let result_variant = unsafe {
            invoke_method_with_result(
                &self.property.dispatch,
                "GetStandardProfileDBFolder",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let folder_path = unsafe {
                    VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?
                };
                anyOk(folder_path)
            }
            Err(e) => bail!("Error::Property::get_standard_profile_db_folder: {}", e),
        }
    }

    /// Gets standard section database name for the specified section property reference number.
    /// # Parameters
    /// * `[in] varSecRefNo` The section property reference ID (Type: Long/Integer).
    /// # Return values
    /// * `<Non-Empty-String>` The standard section database name.
    /// * `<Empty-String>` Specified section property reference does not belong to Standard section database.
    pub fn get_standard_section_database_name(&self, sec_ref_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(sec_ref_no)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetStandardSectionDatabaseName",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let database_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(database_name)
                }
                Err(e) => bail!("Error::Property::get_standard_section_database_name: {}", e),
            }
        }
    }

    /// Get the section name from the standard section database and table for the specified standard section property reference number.
    /// # Parameters
    /// * `[in] varSecRefNo` The section property reference ID (Type: Long/Integer).
    /// # Return values
    /// * `<Non-Empty-String>` The standard section name.
    /// * `<Empty-String>` Specified section property does not belong to any table of Standard section database.
    pub fn get_standard_section_name(&self, sec_ref_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(sec_ref_no)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetStandardSectionName",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let section_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(section_name)
                }
                Err(e) => bail!("Error::Property::get_standard_section_name: {}", e),
            }
        }
    }

    /// Get the table name from the standard section database for the specified standard section property reference number.
    /// # Parameters
    /// * `[in] varSecRefNo` The section property reference ID (Type: Long/Integer).
    /// # Return values
    /// * `<Non-Empty-String>` The standard section table name.
    /// * `<Empty-String>` Specified section property does not belong to any table of Standard section database.
    pub fn get_standard_section_table_name(&self, sec_ref_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(sec_ref_no)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetStandardSectionTableName",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let table_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(table_name)
                }
                Err(e) => bail!("Error::Property::get_standard_section_table_name: {}", e),
            }
        }
    }

    /// Checks if the specified section property reference number is from standard section database source or not.
    /// # Parameters
    /// * `[in] nSecRefNo` The section property reference ID (Type: Long/Integer).
    /// # Return values
    /// * `TRUE` Section source is standard database.
    /// * `FALSE` Section source is other than standard database.
    pub fn is_standard_database_section(&self, sec_ref_no: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(sec_ref_no)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "IsStandardDatabaseSection",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::is_standard_database_section: {}", e),
            }
        }
    }

    /// Sets standard profile database path.
    /// # Parameters
    /// * `[in] strFolderName` Path of the folder.
    /// # Return values
    /// * `0` Succeed.
    /// * `-1` Error, If path is empty or does not exist.
    pub fn set_standard_profile_db_folder(&self, folder_name: &str) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(folder_name)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "SetStandardProfileDBFolder",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::set_standard_profile_db_folder: {}", e),
            }
        }
    }
}
