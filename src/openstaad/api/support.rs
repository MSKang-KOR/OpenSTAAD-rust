use crate::openstaad::tools::{
    com::{get_dispatch, invoke_method},
    safe_array::safe_array_from_vec1d,
    variant::{SafeArray, SafeArrayP, variant_with_ptr_from},
};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use serde::{Deserialize, Serialize};
use std::ffi::c_void;
use windows::Win32::System::{
    Com::{IDispatch, SAFEARRAY},
    Ole::{SafeArrayCreateVector, SafeArrayGetElement},
    Variant::{VARIANT, VT_I4, VT_R8, VariantToInt32, VariantToStringAlloc},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Support {
    #[serde(skip)]
    pub dispatch: Option<IDispatch>,
    pub id: String,
}

impl Support {
    pub fn new(staad: Option<IDispatch>) -> Self {
        let _support = unsafe { get_dispatch(staad.as_ref().unwrap(), "Support", &mut []).unwrap() };
        Self {
            dispatch: Some(_support),
            id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// Assigns the specified support to node(s).
    /// # Parameters
    /// * `[in] varnNodeNo` The node number ID(s) VARIANT array.
    /// * `[in] varnSupportNo` Support Reference number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn assign_support_to_node(&self, node_no: i32, support_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(support_no), VARIANT::from(node_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AssignSupportToNode",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Support::assign_support_to_node: {}", e),
            }
        }
    }

    /// Creates inclined fixed support with releases in specified directions or a spring support with spring constants in specified directions.
    /// # Parameters
    /// * `[in] varInclinedType` Type of the Inclined support (Fixed, Pinned, FixedBut):
    ///   - 1: Pinned
    ///   - 2: Fixed
    ///   - 3: FixedBut
    ///   - 4: Enforced
    ///   - 5: EnforcedBut
    /// * `[in] varType` Type of the reference point define:
    ///   - 0: fRefX, fRefY, fRefY global distances from the joint to the reference point.
    ///   - 1: fRefX, fRefY, fRefY global coordinates of the reference point.
    ///   - 2: a joint number (vaRefNode) whose x, y, z global coordinates is the reference point.
    /// * `[in] vaRefNode` The reference node number ID.
    /// * `[in] varCoord` Distance or coordinate in (X,Y,Z) direction.
    /// * `[in] varReleaseSpec` Degrees of freedom: Fixed (= 0) or Release (= 1) for FX, FY, FZ, MX, MY and MZ.(Type: Double)
    /// * `[in] varSpringSpec` The variable spring constants: KFX, KFY, KFZ, KMX, KMY and KMZ.(Type: Double)
    /// # Return values
    /// * `<Val>` Support Reference number ID.
    /// * `-1` General error.
    /// * `-2001` Cannot find Node vaRefNode.
    pub fn create_inclined_support(
        &self,
        inclined_type: i32,
        ref_type: i32,
        ref_node: i32,
        coord: Vec<f64>,
        release_spec: Vec<f64>,
        spring_spec: Vec<f64>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_coord = safe_array_from_vec1d::<f64>(coord)?;
            let sa_release = safe_array_from_vec1d::<f64>(release_spec)?;
            let sa_spring = safe_array_from_vec1d::<f64>(spring_spec)?;

            let variant_coord = variant_with_ptr_from::<SafeArray<f64>>(sa_coord);
            let variant_release = variant_with_ptr_from::<SafeArray<f64>>(sa_release);
            let variant_spring = variant_with_ptr_from::<SafeArray<f64>>(sa_spring);

            let mut params = [
                variant_spring,
                variant_release,
                variant_coord,
                VARIANT::from(ref_node),
                VARIANT::from(ref_type),
                VARIANT::from(inclined_type),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateInclinedSupport",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let support_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(support_id)
                }
                Err(e) => bail!("Error::Support::create_inclined_support: {}", e),
            }
        }
    }

    /// Creates a fully fixed support.
    /// # Return values
    /// * `<Val>` Support Reference number ID.
    /// * `-1` General error.
    pub fn create_support_fixed(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateSupportFixed",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let support_id = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(support_id)
            }
            Err(e) => bail!("Error::Support::create_support_fixed: {}", e),
        }
    }

    /// Creates fixed support with releases in specified directions or a spring support with spring constants in specified directions.
    /// # Parameters
    /// * `[in] varReleaseSpec` Degrees of freedom: Fixed (= 0) or Release (= 1) for FX, FY, FZ, MX, MY and MZ.(Type : Double)
    /// * `[in] varSpringSpec` The variable spring constants: KFX, KFY, KFZ, KMX, KMY and KMZ.(Type : Double)
    /// # Return values
    /// * `<Val>` Support Reference number ID.
    /// * `-1` General error.
    pub fn create_support_fixed_but(
        &self,
        release_spec: Vec<f64>,
        spring_spec: Vec<f64>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_release = safe_array_from_vec1d::<f64>(release_spec)?;
            let sa_spring = safe_array_from_vec1d::<f64>(spring_spec)?;

            let variant_release = variant_with_ptr_from::<SafeArray<f64>>(sa_release);
            let variant_spring = variant_with_ptr_from::<SafeArray<f64>>(sa_spring);

            let mut params = [variant_spring, variant_release];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateSupportFixedBut",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let support_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(support_id)
                }
                Err(e) => bail!("Error::Support::create_support_fixed_but: {}", e),
            }
        }
    }

    /// Creates a pinned support (i.e., free to rotate about local y and z axis, fixed in all other degrees of freedom).
    /// # Return values
    /// * `<Val>` Support Reference number ID.
    /// * `-1` General error.
    pub fn create_support_pinned(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateSupportPinned",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let support_id = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(support_id)
            }
            Err(e) => bail!("Error::Support::create_support_pinned: {}", e),
        }
    }

    /// Removes support items used in the model.
    /// # Parameters
    /// * `[in] nSupportNo` The support item number. Removing the support will also remove the assignment of the support
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn delete_support(&self, support_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(support_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "DeleteSupport",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Support::delete_support: {}", e),
            }
        }
    }

    /// Get support string name.
    /// # Parameters
    /// * `[in] nSupportNo` The support node number.
    /// # Returns
    /// * The support name string.
    pub fn get_support_name(&self, support_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(support_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetSupportName",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let support_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting support name")?
                        .to_string()?;
                    anyOk(support_name)
                }
                Err(e) => bail!("Error::Support::get_support_name: {}", e),
            }
        }
    }

    /// Get unique ID GUID string for a support item.
    /// # Parameters
    /// * `[in] nSupportNo` The support item number.
    /// # Returns
    /// * The support GUID string.
    pub fn get_support_unique_id(&self, support_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(support_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetSupportUniqueID",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let unique_id = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting unique id")?
                        .to_string()?;
                    anyOk(unique_id)
                }
                Err(e) => bail!("Error::Support::get_support_unique_id: {}", e),
            }
        }
    }

    /// Removes support assignment from a node.
    /// # Parameters
    /// * `[in] nNodeNo` The node number.
    /// # Return values
    /// * `1/TRUE` OK.
    /// * `0/FALSE` General error.
    pub fn remove_support_from_node(&self, node_no: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(node_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "RemoveSupportFromNode",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let success = VariantToInt32(&var as *const VARIANT).unwrap() > 0;
                    anyOk(success)
                }
                Err(e) => bail!("Error::Support::remove_support_from_node: {}", e),
            }
        }
    }

    /// Set unique ID for a support item.
    /// # Parameters
    /// * `[in] nSupportNo` The support item number.
    /// * `[in] szName` A GUID string
    pub fn set_support_unique_id(&self, support_no: i32, unique_id: &str) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(unique_id), VARIANT::from(support_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SetSupportUniqueID",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Support::set_support_unique_id: {}", e),
            }
        }
    }

    /// Gets the total number of supported nodes exist in the current structure.
    /// # Returns
    /// * The number of support.
    pub fn get_support_count(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(self.dispatch.as_ref().unwrap(), "GetSupportCount", &mut []) };
        match result_variant {
            Ok(var) => {
                let count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(count)
            }
            Err(e) => bail!("Error::Support::get_support_count: {}", e),
        }
    }

    /// Gets support information for the specified node, including release specification and spring specification.
    /// # Parameters
    /// * `[in] vaSupportNode` The supported node number ID.
    /// * `[out] varReleaseSpec` Degrees of freedom: Release (= 1) or Fixed (= 0) or Spring (=-1) for FX, FY, FZ, MX, MY and MZ.(Type:int)
    /// * `[out] varSpringSpec` The variable spring constants: KFX, KFY, KFZ, KMX, KMY and KMZ.(Type:double)
    /// # Return values
    /// * `0` No support.
    /// * `1` Pinned support.
    /// * `2` Fixed support.
    /// * `3` Fixed support with releases.
    /// * `4` Enforced support.
    /// * `5` Enforced support with releases.
    /// * `6` Inclined support.
    /// * `7` Footing foundation.
    /// * `8` Elastic mat foundation.
    /// * `9` Plate mat foundation.
    /// * `10` MultiLinear spring support.
    /// * `11` Generated pinned support.
    /// * `12` Generated fixed support.
    /// * `13` Generated fixed support with releases.
    /// * `-1` General error.
    pub fn get_support_information(
        &self,
        support_node: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>), anyErr> {
        unsafe {
            // Create arrays for release and spring specifications (6 elements each: FX, FY, FZ, MX, MY, MZ)
            let mut release_sa = SafeArrayCreateVector(VT_I4, 0, 6);
            let mut spring_sa = SafeArrayCreateVector(VT_R8, 0, 6);

            let release_sa_ptr = &mut release_sa as *mut *mut SAFEARRAY;
            let spring_sa_ptr = &mut spring_sa as *mut *mut SAFEARRAY;

            let mut params = [
                variant_with_ptr_from::<SafeArrayP<f64>>(spring_sa_ptr),
                variant_with_ptr_from::<SafeArrayP<i32>>(release_sa_ptr),
                VARIANT::from(support_node),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetSupportInformation",
                &mut params,
            );

            match result_variant {
                Ok(var) => {
                    let support_type = VariantToInt32(&var as *const VARIANT).unwrap();

                    let mut release_spec = Vec::with_capacity(6);
                    let mut spring_spec = Vec::with_capacity(6);

                    // Extract release specification
                    for i in 0..6 {
                        let mut index = i as i32;
                        let mut release_value = 0;
                        let _ = SafeArrayGetElement(
                            release_sa,
                            &mut index as *mut i32,
                            &mut release_value as *mut i32 as *mut c_void,
                        )?;
                        release_spec.push(release_value);
                    }

                    // Extract spring specification
                    for i in 0..6 {
                        let mut index = i as i32;
                        let mut spring_value = 0.0;
                        let _ = SafeArrayGetElement(
                            spring_sa,
                            &mut index as *mut i32,
                            &mut spring_value as *mut f64 as *mut c_void,
                        )?;
                        spring_spec.push(spring_value);
                    }

                    anyOk((support_type, release_spec, spring_spec))
                }
                Err(e) => bail!("Error::Support::get_support_information: {}", e),
            }
        }
    }

    /// Gets support information for the specified node, including release specification and spring specification.
    /// # Parameters
    /// * `[in] vaSupportNode` The supported node number ID.
    /// * `[out] varSupportNo` The support reference number
    /// * `[out] varSupportType` The support type code
    /// * `[out] varReleaseSpec` Degrees of freedom: Release ( = 1) or Fixed ( = 0) or Spring (=-1) for FX, FY, FZ, MX, MY and MZ.(Type:int)
    /// * `[out] varSpringSpec` The variable spring constants: KFX, KFY, KFZ, KMX, KMY and KMZ.(Type:double)
    /// # Return values
    /// * `0` No support.
    /// * `1` Pinned support.
    /// * `2` Fixed support.
    /// * `3` Fixed support with releases.
    /// * `4` Enforced support.
    /// * `5` Enforced support with releases.
    /// * `6` Inclined support.
    /// * `7` Footing foundation.
    /// * `8` Elastic mat foundation.
    /// * `9` Plate mat foundation.
    /// * `10` MultiLinear spring support.
    /// * `11` Generated pinned support.
    /// * `12` Generated fixed support.
    /// * `13` Generated fixed support with releases.
    /// * `-1` General error.
    pub fn get_support_information_ex(
        &self,
        support_node: i32,
    ) -> Result<(i32, i32, i32, Vec<i32>, Vec<f64>), anyErr> {
        unsafe {
            let support_no_ptr = &mut 0i32 as *mut i32;
            let support_type_ptr = &mut 0i32 as *mut i32;

            // Create arrays for release and spring specifications (6 elements each: FX, FY, FZ, MX, MY, MZ)
            let mut release_sa = SafeArrayCreateVector(VT_I4, 0, 6);
            let mut spring_sa = SafeArrayCreateVector(VT_R8, 0, 6);

            let release_sa_ptr = &mut release_sa as *mut *mut SAFEARRAY;
            let spring_sa_ptr = &mut spring_sa as *mut *mut SAFEARRAY;

            let mut params = [
                variant_with_ptr_from::<SafeArrayP<f64>>(spring_sa_ptr),
                variant_with_ptr_from::<SafeArrayP<i32>>(release_sa_ptr),
                variant_with_ptr_from::<i32>(support_type_ptr),
                variant_with_ptr_from::<i32>(support_no_ptr),
                VARIANT::from(support_node),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetSupportInformationEx",
                &mut params,
            );

            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();

                    let mut release_spec = Vec::with_capacity(6);
                    let mut spring_spec = Vec::with_capacity(6);

                    // Extract release specification
                    for i in 0..6 {
                        let mut index = i as i32;
                        let mut release_value = 0;
                        let _ = SafeArrayGetElement(
                            release_sa,
                            &mut index as *mut i32,
                            &mut release_value as *mut i32 as *mut c_void,
                        )?;
                        release_spec.push(release_value);
                    }

                    // Extract spring specification
                    for i in 0..6 {
                        let mut index = i as i32;
                        let mut spring_value = 0.0;
                        let _ = SafeArrayGetElement(
                            spring_sa,
                            &mut index as *mut i32,
                            &mut spring_value as *mut f64 as *mut c_void,
                        )?;
                        spring_spec.push(spring_value);
                    }

                    anyOk((
                        result_code,
                        *support_no_ptr,
                        *support_type_ptr,
                        release_spec,
                        spring_spec,
                    ))
                }
                Err(e) => bail!("Error::Support::get_support_information_ex: {}", e),
            }
        }
    }

    /// Gets all supported nodes in an array.
    /// # Parameters
    /// * `[out] varNodeNos` The supported node number ID(s) VARIANT array.
    /// # Return values
    /// * `<val>` The number of supported node(s).
    /// * `-1` General error.
    pub fn get_support_nodes(&self) -> Result<(i32, Vec<i32>), anyErr> {
        unsafe {
            let support_count = self.get_support_count()?;
            if support_count <= 0 {
                return anyOk((support_count, Vec::new()));
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, support_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_with_ptr_from::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetSupportNodes",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let node_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    let node_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut node_arr = Vec::with_capacity(node_count as usize);

                    for i in 0..node_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            node_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        node_arr.push(value as i32);
                    }
                    anyOk((node_count, node_arr))
                }
                Err(e) => bail!("Error::Support::get_support_nodes: {}", e),
            }
        }
    }

    /// Gets the support type for the specified node.
    /// # Parameters
    /// * `[in] vaSupportNode` The supported node number ID.
    /// # Return values
    /// * `0` No support.
    /// * `1` Pinned support.
    /// * `2` Fixed support.
    /// * `3` Fixed support with releases.
    /// * `4` Enforced support.
    /// * `5` Enforced support with releases.
    /// * `6` Inclined support.
    /// * `7` Footing foundation.
    /// * `8` Elastic mat foundation.
    /// * `9` Plate mat foundation.
    /// * `10` MultiLinear spring support.
    /// * `11` Generated pinned support.
    /// * `12` Generated fixed support.
    /// * `13` Generated fixed support with releases.
    /// * `-1` General error.
    pub fn get_support_type(&self, support_node: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(support_node)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetSupportType",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let support_type = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(support_type)
                }
                Err(e) => bail!("Error::Support::get_support_type: {}", e),
            }
        }
    }
}

unsafe impl Send for Support{}
unsafe impl Sync for Support{}

// AssignSupportToNode
// CreateInclinedSupport
// CreateSupportFixed
// CreateSupportFixedBut
// CreateSupportPinned
// DeleteSupport
// GetSupportName
// GetSupportUniqueID
// RemoveSupportFromNode
// SetSupportUniqueID
// GetSupportCount
// GetSupportInformation
// GetSupportInformationEx
// GetSupportNodes
// GetSupportType
