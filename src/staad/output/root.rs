use crate::staad::{
    root::Root,
    safe_array::safe_array_from_vec1d,
    utils::{get_dispatch, invoke_method_with_result},
    variant::{SafeArray, SafeArrayP, variant_from_raw_pointer},
};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use std::ffi::c_void;
use windows::Win32::System::{
    Com::{IDispatch, SAFEARRAY},
    Ole::{SafeArrayCreateVector, SafeArrayGetElement},
    Variant::{VariantToInt32, VariantToStringAlloc, VARIANT, VT_R8},
};
use windows_core::BSTR;

#[derive(Debug)]
pub struct Output<'a> {
    pub root: &'a Root,
    pub dispatch: IDispatch,
}

impl<'a> Output<'a> {
    pub fn new(root: &'a Root) -> Self {
        let dispatch =
            unsafe { get_dispatch(root.dispatch.as_ref().unwrap(), "Output", &mut []).unwrap() };
        Self { root, dispatch }
    }

    /// Returns the design section name for specified member.
    /// # Parameters
    /// * `[in] varnBeamNo` Member number ID.
    /// # Return values
    /// * Returns design section name for the specified member. Returns empty string if not found.
    pub fn get_member_design_section_name(&self, member_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(member_no)];
            let result_variant = invoke_method_with_result(
                &self.dispatch,
                "GetMemberDesignSectionName",
                &mut params,
            );
            match result_variant {
                Ok(v) => {
                    let section_name = VariantToStringAlloc(&v as *const VARIANT)
                        .context("converting section name")?
                        .to_string()?;
                    anyOk(section_name)
                }
                Err(e) => bail!("Error::Output::get_member_design_section_name: {}", e),
            }
        }
    }

    /// Fetches the maximum failure ratio across all beams in the model.
    /// # Parameters
    /// * `[out] pdRatio` The maximum failure ratio.
    /// # Returns
    /// * true if maximum ratio can be obtained successfully otherwise false.
    pub fn get_member_steel_design_max_failure_ratio(&self) -> Result<(bool, f64), anyErr> {
        unsafe {
            let ratio_ptr = &mut 0. as *mut f64;
            let mut params = [variant_from_raw_pointer::<f64>(ratio_ptr)];
            let result_variant = invoke_method_with_result(
                &self.dispatch,
                "GetMemberSteelDesignMaxFailureRatio",
                &mut params,
            );
            match result_variant {
                Ok(v) => {
                    let success = VariantToInt32(&v as *const VARIANT).unwrap() > 0;
                    anyOk((success, *ratio_ptr))
                }
                Err(e) => bail!(
                    "Error::Output::get_member_steel_design_max_failure_ratio: {}",
                    e
                ),
            }
        }
    }

    /// Fetches the minimum failure ratio across all beams in the model.
    /// # Parameters
    /// * `[out] pdRatio` The minimum failure ratio.
    /// # Returns
    /// * true if minimum ratio can be obtained successfully otherwise false.
    pub fn get_member_steel_design_min_failure_ratio(&self) -> Result<(bool, f64), anyErr> {
        unsafe {
            let ratio_ptr = &mut 0. as *mut f64;
            let mut params = [variant_from_raw_pointer::<f64>(ratio_ptr)];
            let result_variant = invoke_method_with_result(
                &self.dispatch,
                "GetMemberSteelDesignMinFailureRatio",
                &mut params,
            );
            match result_variant {
                Ok(v) => {
                    let success = VariantToInt32(&v as *const VARIANT).unwrap() > 0;
                    anyOk((success, *ratio_ptr))
                }
                Err(e) => bail!(
                    "Error::Output::get_member_steel_design_min_failure_ratio: {}",
                    e
                ),
            }
        }
    }

    /// Returns the critical steel design ratio for a steel member. This method will return the results from the last parameter block for which the beam has been designed.
    /// # Parameters
    /// * `[in] vnBeamNo` Beam number ID.
    /// * `[out] vfCriticalRatio` Returns the critical steel design ratio. Returns -999 if analysis is performed but the member is not designed. Returns -1 if analysis is not performed.
    /// # Returns
    /// * Returns true if ratio for the specified member is obtained successfully otherwise false.
    pub fn get_member_steel_design_ratio(&self, member_no: i32) -> Result<(bool, f64), anyErr> {
        unsafe {
            let ratio_ptr = &mut 0. as *mut f64;
            let mut params = [
                variant_from_raw_pointer::<f64>(ratio_ptr),
                VARIANT::from(member_no),
            ];
            let result_variant =
                invoke_method_with_result(&self.dispatch, "GetMemberSteelDesignRatio", &mut params);
            match result_variant {
                Ok(v) => {
                    let success = VariantToInt32(&v as *const VARIANT).unwrap() > 0;
                    anyOk((success, *ratio_ptr))
                }
                Err(e) => bail!("Error::Output::get_member_steel_design_ratio: {}", e),
            }
        }
    }

    /// Fetches steel design results for the specified member. This method will return the results from the last parameter block for which the beam has been designed.
    /// # Parameters
    /// * `[in] vnBeamNo` Id of the member for which design results should be retrieved.
    /// # Returns
    /// * true if design results can be obtained successfully otherwise false.
    /// * Returns tuple containing (success, design_code, design_status, critical_ratio, allowable_ratio, critical_load_case, critical_section, critical_clause, design_section, design_forces, kl_by_r)
    pub fn get_member_steel_design_results(
        &self,
        member_no: i32,
        base_unit: i32,
    ) -> Result<
        (
            bool,
            String,
            String,
            f64,
            f64,
            i32,
            f64,
            String,
            String,
            Vec<f64>,
            f64,
        ),
        anyErr,
    > {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }

            let design_code_ptr = &mut BSTR::default() as *mut BSTR;
            let design_status_ptr = &mut BSTR::default() as *mut BSTR;
            let critical_ratio_ptr = &mut 0. as *mut f64;
            let allowable_ratio_ptr = &mut 0. as *mut f64;
            let critical_load_case_ptr = &mut 0i32 as *mut i32;
            let critical_section_ptr = &mut 0. as *mut f64;
            let critical_clause_ptr = &mut BSTR::default() as *mut BSTR;
            let design_section_ptr = &mut BSTR::default() as *mut BSTR;
            let kl_by_r_ptr = &mut 0. as *mut f64;

            // Create SafeArray for design forces (3 elements)
            let mut forces_sa = SafeArrayCreateVector(VT_R8, 0, 3);
            let forces_sa_ptr = &mut forces_sa as *mut *mut SAFEARRAY;

            let mut params = [
                variant_from_raw_pointer::<f64>(kl_by_r_ptr),
                variant_from_raw_pointer::<SafeArrayP<f64>>(forces_sa_ptr),
                variant_from_raw_pointer::<BSTR>(design_section_ptr),
                variant_from_raw_pointer::<BSTR>(critical_clause_ptr),
                variant_from_raw_pointer::<f64>(critical_section_ptr),
                variant_from_raw_pointer::<i32>(critical_load_case_ptr),
                variant_from_raw_pointer::<f64>(allowable_ratio_ptr),
                variant_from_raw_pointer::<f64>(critical_ratio_ptr),
                variant_from_raw_pointer::<BSTR>(design_status_ptr),
                variant_from_raw_pointer::<BSTR>(design_code_ptr),
                VARIANT::from(member_no),
            ];

            let result_variant = invoke_method_with_result(
                &self.dispatch,
                "GetMemberSteelDesignResults",
                &mut params,
            );

            match result_variant {
                Ok(v) => {
                    let success = VariantToInt32(&v as *const VARIANT).unwrap() > 0;

                    let design_code = (&*design_code_ptr).to_string();
                    let design_status = (&*design_status_ptr).to_string();
                    let critical_clause = (&*critical_clause_ptr).to_string();
                    let design_section = (&*design_section_ptr).to_string();

                    // Extract design forces from SafeArray
                    let mut design_forces = Vec::with_capacity(3);
                    for i in 0..3 {
                        let mut index = i as i32;
                        let mut value = 0.0;
                        let _ = SafeArrayGetElement(
                            forces_sa,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        design_forces.push(value / unit_factor);
                    }

                    anyOk((
                        success,
                        design_code,
                        design_status,
                        *critical_ratio_ptr,
                        *allowable_ratio_ptr,
                        *critical_load_case_ptr,
                        *critical_section_ptr / unit_factor,
                        critical_clause,
                        design_section,
                        design_forces,
                        *kl_by_r_ptr,
                    ))
                }
                Err(e) => bail!("Error::Output::get_member_steel_design_results: {}", e),
            }
        }
    }

    /// Returns the maximum critical steel design ratio across all parameter blocks for a steel member.
    /// # Parameters
    /// * `[in] vnBeamNo` Beam number ID.
    /// * `[out] vfCriticalRatio` Returns the maximum critical steel design ratio.
    /// # Returns
    /// * Returns true if ratio for the specified member is obtained successfully otherwise false.
    pub fn get_multiple_member_steel_design_max_ratio(
        &self,
        member_no: i32,
    ) -> Result<(bool, f64), anyErr> {
        unsafe {
            let ratio_ptr = &mut 0. as *mut f64;
            let mut params = [
                variant_from_raw_pointer::<f64>(ratio_ptr),
                VARIANT::from(member_no),
            ];
            let result_variant = invoke_method_with_result(
                &self.dispatch,
                "GetMultipleMemberSteelDesignMaxRatio",
                &mut params,
            );
            match result_variant {
                Ok(v) => {
                    let success = VariantToInt32(&v as *const VARIANT).unwrap() > 0;
                    anyOk((success, *ratio_ptr))
                }
                Err(e) => bail!(
                    "Error::Output::get_multiple_member_steel_design_max_ratio: {}",
                    e
                ),
            }
        }
    }

    /// Returns the critical steel design ratio for a steel member. This function is for AISC 360-16 code only.
    /// # Parameters
    /// * `[in] vszParamBlkName` Steel design parameter block name.
    /// * `[in] vnBeamNo` Beam number ID.
    /// * `[out] vfCriticalRatio` Returns the critical steel design ratio.
    /// # Returns
    /// * Returns true if ratio for the specified member is obtained successfully otherwise false.
    pub fn get_multiple_member_steel_design_ratio(
        &self,
        param_block_name: &str,
        member_no: i32,
    ) -> Result<(bool, f64), anyErr> {
        unsafe {
            let ratio_ptr = &mut 0. as *mut f64;
            let mut params = [
                variant_from_raw_pointer::<f64>(ratio_ptr),
                VARIANT::from(member_no),
                VARIANT::from(param_block_name),
            ];
            let result_variant = invoke_method_with_result(
                &self.dispatch,
                "GetMultipleMemberSteelDesignRatio",
                &mut params,
            );
            match result_variant {
                Ok(v) => {
                    let success = VariantToInt32(&v as *const VARIANT).unwrap() > 0;
                    anyOk((success, *ratio_ptr))
                }
                Err(e) => bail!(
                    "Error::Output::get_multiple_member_steel_design_ratio: {}",
                    e
                ),
            }
        }
    }

    /// Returns the critical steel design result information for a steel member.
    /// # Parameters
    /// * `[in] vszParamBlkName` Steel design parameter block name.
    /// * `[in] vnBeamNo` Member number ID.
    /// # Returns
    /// * Returns true if results for the specified member is obtained successfully otherwise false.
    /// * Returns tuple containing (success, design_code, design_status, critical_ratio, allowable_ratio, critical_load_case, critical_clause, design_section)
    pub fn get_multiple_member_steel_design_results(
        &self,
        param_block_name: &str,
        member_no: i32,
    ) -> Result<(bool, String, String, f64, f64, i32, String, String), anyErr> {
        unsafe {
            let design_code_ptr = &mut BSTR::default() as *mut BSTR;
            let design_status_ptr = &mut BSTR::default() as *mut BSTR;
            let critical_ratio_ptr = &mut 0. as *mut f64;
            let allowable_ratio_ptr = &mut 0. as *mut f64;
            let critical_load_case_ptr = &mut 0i32 as *mut i32;
            let critical_clause_ptr = &mut BSTR::default() as *mut BSTR;
            let design_section_ptr = &mut BSTR::default() as *mut BSTR;

            let mut params = [
                variant_from_raw_pointer::<BSTR>(design_section_ptr),
                variant_from_raw_pointer::<BSTR>(critical_clause_ptr),
                variant_from_raw_pointer::<i32>(critical_load_case_ptr),
                variant_from_raw_pointer::<f64>(allowable_ratio_ptr),
                variant_from_raw_pointer::<f64>(critical_ratio_ptr),
                variant_from_raw_pointer::<BSTR>(design_status_ptr),
                variant_from_raw_pointer::<BSTR>(design_code_ptr),
                VARIANT::from(member_no),
                VARIANT::from(param_block_name),
            ];

            let result_variant = invoke_method_with_result(
                &self.dispatch,
                "GetMultipleMemberSteelDesignResults",
                &mut params,
            );

            match result_variant {
                Ok(v) => {
                    let success = VariantToInt32(&v as *const VARIANT).unwrap() > 0;

                    let design_code = (&*design_code_ptr).to_string();
                    let design_status = (&*design_status_ptr).to_string();
                    let critical_clause = (&*critical_clause_ptr).to_string();
                    let design_section = (&*design_section_ptr).to_string();

                    anyOk((
                        success,
                        design_code,
                        design_status,
                        *critical_ratio_ptr,
                        *allowable_ratio_ptr,
                        *critical_load_case_ptr,
                        critical_clause,
                        design_section,
                    ))
                }
                Err(e) => bail!(
                    "Error::Output::get_multiple_member_steel_design_results: {}",
                    e
                ),
            }
        }
    }

    /// Returns the count of steel design parameter blocks in the model. This function is for AISC 360-16 code only.
    /// # Returns
    /// * Returns the count of steel design parameter blocks.
    pub fn get_steel_design_parameter_block_count(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method_with_result(&self.dispatch, "GetSteelDesignParameterBlockCount", &mut [])
        };
        match result_variant {
            Ok(v) => {
                let count = unsafe { VariantToInt32(&v as *const VARIANT).unwrap() };
                anyOk(count)
            }
            Err(e) => bail!(
                "Error::Output::get_steel_design_parameter_block_count: {}",
                e
            ),
        }
    }

    /// Returns steel design parameter name at the specified index. This function is for AISC 360-16 code only.
    /// # Parameters
    /// * `[in] vnIdx` The index value of steel design parameter block list. Note, the index is zero based.
    /// * `[out] vszParamBlkName` Steel design parameter block name
    /// # Returns
    /// * Returns true if successful, false otherwise.
    pub fn get_steel_design_parameter_block_name_by_index(
        &self,
        index: i32,
    ) -> Result<(bool, String), anyErr> {
        unsafe {
            let name_ptr = &mut BSTR::default() as *mut BSTR;
            let mut params = [
                variant_from_raw_pointer::<BSTR>(name_ptr),
                VARIANT::from(index),
            ];
            let result_variant = invoke_method_with_result(
                &self.dispatch,
                "GetSteelDesignParameterBlockNameByIndex",
                &mut params,
            );
            match result_variant {
                Ok(v) => {
                    let success = VariantToInt32(&v as *const VARIANT).unwrap() > 0;
                    let name = (&*name_ptr).to_string();
                    anyOk((success, name))
                }
                Err(e) => bail!(
                    "Error::Output::get_steel_design_parameter_block_name_by_index: {}",
                    e
                ),
            }
        }
    }

    /// This function returns whether steel design results from multiple design block can be extracted or not. If true, then relevant multiple steel design parameters like GetMultipleMemberSteelDesignRatio or GetMultipleMemberSteelDesignResults can be used. Currently, this facility is limited to AISC 360-16 code only.
    /// # Returns
    /// * Returns TRUE (for Boolean variable, for Long variable, return value is 1) if result extraction from multiple steel design block is possible (i.e. AISC 360-16 code is used). Else the return value will be FALSE (0 for Long variable).
    pub fn is_multiple_member_steel_design_results_available(&self) -> Result<bool, anyErr> {
        let result_variant = unsafe {
            invoke_method_with_result(
                &self.dispatch,
                "IsMultipleMemberSteelDesignResultsAvailable",
                &mut [],
            )
        };
        match result_variant {
            Ok(v) => {
                let available = unsafe { VariantToInt32(&v as *const VARIANT).unwrap() > 0 };
                anyOk(available)
            }
            Err(e) => bail!(
                "Error::Output::is_multiple_member_steel_design_results_available: {}",
                e
            ),
        }
    }
}
