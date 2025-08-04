use crate::openstaad::{
    property::root::Property,
    tools::safe_array::{safe_array_from_vec1d, safe_array_from_vec2d},
    tools::invoke::invoke_method,
    tools::variant::{SafeArray, SafeArrayP, variant_from_raw_pointer},
};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use std::ffi::c_void;
use windows::Win32::System::{
    Com::SAFEARRAY,
    Ole::{SafeArrayCreateVector, SafeArrayGetElement},
    Variant::{VARIANT, VT_I4, VT_R8, VariantToInt32, VariantToStringAlloc},
};

// :: Create Specification
// AddControlDependentRelation
// CreateElementIgnoreInplaneRotnSpec
// CreateElementNodeReleaseSpec
// CreateMemberCompressionSpec
// CreateMemberIgnoreStiffSpec
// CreateMemberInactiveSpec
// CreateMemberPartialReleaseSpec
// CreateMemberReleaseSpec
// CreateMemberTensionSpec
// CreateMemberTrussSpec
// :: Get and Remove Spectification
// DeleteAllControlDependentRelations
// DeleteMemberReleaseSpec
// DeleteMemberSpec
// DeleteProperty
// GetAlphaAngleForSection
// GetCentroidLocationForSection
// GetInactiveMemberCount
// GetInactiveMemberList
// GetMemberReleaseSpecEx
// GetMemberSpecCode
// GetPropertyUniqueID
// RemoveAllElementNodeReleaseSpec
// RemoveBeamPropertyHelper
// RemoveElementIgnoreInplaneRotnSpecFromPlate
// RemoveMemberCableSpecFromBeam
// RemoveMemberCompressionSpecFromBeam
// RemoveMemberIgnoreStiffSpecFromBeam
// RemoveMemberInactiveSpecFromBeam
// RemoveMemberReleaseSpecFromBeam
// RemoveMemberTensionSpecFromBeam
// RemoveMemberTrussSpecFromBeam
// RemovePropertyFromBeam
// SetPropertyUniqueID

#[derive(Debug)]
pub struct Specification<'a> {
    pub property: &'a Property<'a>,
}

impl<'a> Specification<'a> {
    pub fn new(property: &'a Property<'a>) -> Self {
        Self { property }
    }

    /// Add a control/dependent joint specification to specified node(s).
    /// # Parameters
    /// * `[in] varControlNode` Set node (number ID) control node.
    /// * `[in] varRegid_XY_YZ_ZX` Set plate rigid: all directions rigid (= 0), XY plate rigid (= 1), YZ plate rigid (= 2), ZX plate rigid (= 3), specific define FX, FY, FZ, MX, MY, MZ rigid (= others).
    /// * `[in] varFX` Rigid in X direction translation (Rigid = 1, Not Rigid = 0).
    /// * `[in] varFY` Rigid in Y direction translation (Rigid = 1, Not Rigid = 0).
    /// * `[in] varFZ` Rigid in Z direction translation (Rigid = 1, Not Rigid = 0).
    /// * `[in] varMX` Rigid in X direction rotation (Rigid = 1, Not Rigid = 0).
    /// * `[in] varMY` Rigid in Y direction rotation (Rigid = 1, Not Rigid = 0).
    /// * `[in] varMZ` Rigid in Z direction rotation (Rigid = 1, Not Rigid = 0).
    /// * `[in] varDependentNodeArray` Nodes number ID VARIANT array.
    /// # Return values
    /// * `0` OK.
    /// * `-106` 1 dimensional array of long expected.
    /// * `-6029` Library Error: Unable to create CONTROL/DEPENDENT specification.
    pub fn add_control_dependent_relation(
        &self,
        control_node: i32,
        regid_xy_yz_zx: i32,
        fx: i32,
        fy: i32,
        fz: i32,
        mx: i32,
        my: i32,
        mz: i32,
        dependent_nodes: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_nodes = safe_array_from_vec1d::<i32>(dependent_nodes)?;
            let variant_nodes = variant_from_raw_pointer::<SafeArray<i32>>(sa_nodes);

            let mut params = [
                variant_nodes,
                VARIANT::from(mz),
                VARIANT::from(my),
                VARIANT::from(mx),
                VARIANT::from(fz),
                VARIANT::from(fy),
                VARIANT::from(fx),
                VARIANT::from(regid_xy_yz_zx),
                VARIANT::from(control_node),
            ];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "AddControlDependentRelation",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::add_control_dependent_relation: {}", e),
            }
        }
    }

    /// Create ELEMENT INPLANE ROTATION specification.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-6019` Library Error: Unable to create ELEMENT INPLANE ROTATION specification.
    pub fn create_element_ignore_inplane_rotn_spec(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                &self.property.dispatch,
                "CreateElementIgnoreInplaneRotnSpec",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!(
                "Error::Property::create_element_ignore_inplane_rotn_spec: {}",
                e
            ),
        }
    }

    /// Creates ELEMENT NODE RELEASE specification.
    /// # Parameters
    /// * `[in] varNode` The node number ID to be released.
    /// * `[in] varDOFRelease` Degrees of freedom: No Release (= 0) or Release (= 1) for FX, FY, FZ, MX, MY and MZ.
    /// # Return values
    /// * `0` OK
    /// * `-106` 1 dimensional array of long for varDOFRelease expected.
    /// * `-108` Array size is smaller than expected (size should be 6).
    /// * `-6021` Library Error: Unable to create ELEMENT NODE RELEASE specification.
    pub fn create_element_node_release_spec(
        &self,
        node: i32,
        dof_releases: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            if dof_releases.len() != 6 {
                bail!(
                    "Error::Property::create_element_node_release_spec: DOF release array must have 6 elements"
                );
            }

            let sa_dof = safe_array_from_vec1d::<i32>(dof_releases)?;
            let variant_dof = variant_from_raw_pointer::<SafeArray<i32>>(sa_dof);

            let mut params = [variant_dof, VARIANT::from(node)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "CreateElementNodeReleaseSpec",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_element_node_release_spec: {}", e),
            }
        }
    }

    /// Create MEMBER COMPRESSION specification.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-6013` Library Error: Unable to create MEMBER COMPRESSION specification.
    pub fn create_member_compression_spec(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                &self.property.dispatch,
                "CreateMemberCompressionSpec",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!("Error::Property::create_member_compression_spec: {}", e),
        }
    }

    /// Create ELEMENT IGNORE STIFFNESS specification.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-6014` Library Error: Unable to create IGNORE STIFFNESS specification.
    pub fn create_member_ignore_stiff_spec(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                &self.property.dispatch,
                "CreateMemberIgnoreStiffSpec",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!("Error::Property::create_member_ignore_stiff_spec: {}", e),
        }
    }

    /// Create MEMBER INACTIVE specification.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-6011` Library Error: Unable to create MEMBER INACTIVE specification.
    pub fn create_member_inactive_spec(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(&self.property.dispatch, "CreateMemberInactiveSpec", &mut [])
        };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!("Error::Property::create_member_inactive_spec: {}", e),
        }
    }

    /// Creates MEMBER RELEASE specification with partial release factors.
    /// # Parameters
    /// * `[in] varLocation` The offset location at START (= 0) or END (= 1) of the member.
    /// * `[in] varDOFRelease` Degrees of freedom: No Release (= 0) or Release (= 1) for FX, FY, FZ, MX, MY and MZ.
    /// * `[in] varFactor` The partial release factor in respective DOFs.
    /// # Return values
    /// * `0` OK
    /// * `-106` 1 dimensional array of long for varDOFRelease and 1 dimensional array of double for varFactor expected.
    /// * `-108` Array size is smaller than expected (size should be 6).
    /// * `-6020` Library Error: Unable to create MEMBER RELEASE specification.
    pub fn create_member_partial_release_spec(
        &self,
        location: i32,
        dof_releases: Vec<i32>,
        factors: Vec<f64>,
    ) -> Result<i32, anyErr> {
        unsafe {
            if dof_releases.len() != 6 || factors.len() != 6 {
                bail!(
                    "Error::Property::create_member_partial_release_spec: DOF release and factor arrays must have 6 elements each"
                );
            }

            let sa_dof = safe_array_from_vec1d::<i32>(dof_releases)?;
            let sa_factors = safe_array_from_vec1d::<f64>(factors)?;
            let variant_dof = variant_from_raw_pointer::<SafeArray<i32>>(sa_dof);
            let variant_factors = variant_from_raw_pointer::<SafeArray<f64>>(sa_factors);

            let mut params = [variant_factors, variant_dof, VARIANT::from(location)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "CreateMemberPartialReleaseSpec",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_member_partial_release_spec: {}", e),
            }
        }
    }

    /// Creates MEMBER RELEASE specification with spring constants.
    /// # Parameters
    /// * `[in] varLocation` The offset location at START (= 0) or END (= 1) of the member.
    /// * `[in] varDOFRelease` Degrees of freedom: No Release (= 0) or Release (= 1) for FX, FY, FZ, MX, MY and MZ.
    /// * `[in] varSpringConst` The variable spring constants KFX, KFY, KFZ, KMX, KMY and KMZ.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-106` 1 dimensional array of long for varDOFRelease and 1 dimensional array of double for varSpringConst expected.
    /// * `-108` Array size is smaller than expected (size should be 6).
    /// * `-6020` Library Error: Unable to create MEMBER RELEASE specification.
    pub fn create_member_release_spec(
        &self,
        location: i32,
        dof_releases: Vec<i32>,
        spring_constants: Vec<f64>,
    ) -> Result<i32, anyErr> {
        unsafe {
            if dof_releases.len() != 6 || spring_constants.len() != 6 {
                bail!(
                    "Error::Property::create_member_release_spec: DOF release and spring constant arrays must have 6 elements each"
                );
            }

            let sa_dof = safe_array_from_vec1d::<i32>(dof_releases)?;
            let sa_springs = safe_array_from_vec1d::<f64>(spring_constants)?;
            let variant_dof = variant_from_raw_pointer::<SafeArray<i32>>(sa_dof);
            let variant_springs = variant_from_raw_pointer::<SafeArray<f64>>(sa_springs);

            let mut params = [variant_springs, variant_dof, VARIANT::from(location)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "CreateMemberReleaseSpec",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_member_release_spec: {}", e),
            }
        }
    }

    /// Create MEMBER TENSION specification.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-6012` Library Error: Unable to create MEMBER TENSION specification.
    pub fn create_member_tension_spec(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(&self.property.dispatch, "CreateMemberTensionSpec", &mut [])
        };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!("Error::Property::create_member_tension_spec: {}", e),
        }
    }

    /// Create MEMBER TRUSS specification.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-6010` Library Error: Unable to create MEMBER TRUSS specification.
    pub fn create_member_truss_spec(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(&self.property.dispatch, "CreateMemberTrussSpec", &mut [])
        };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!("Error::Property::create_member_truss_spec: {}", e),
        }
    }

    /// Delete all control/dependent joint specifications from model.
    /// # Return values
    /// * `0` OK Successfully deleted.
    /// * `-1` ERROR Delete unsuccessful.
    pub fn delete_all_control_dependent_relations(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                &self.property.dispatch,
                "DeleteAllControlDependentRelations",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!(
                "Error::Property::delete_all_control_dependent_relations: {}",
                e
            ),
        }
    }

    /// Delete MEMBER RELEASE specification.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Long/Integer).
    /// * `[in] varLocation` The Release location at START (= 0) or END (= 1) of the member.
    /// # Return values
    /// * `FALSE` Delete Member Release specification failed
    /// * `TRUE` Delete Member Release specification Successful.
    pub fn delete_member_release_spec(&self, beam_no: i32, location: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(location), VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "DeleteMemberReleaseSpec",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::delete_member_release_spec: {}", e),
            }
        }
    }

    /// Delete specification based on the specification number passed.
    /// # Parameters
    /// * `[in] varnSpecNo` The specification number.
    /// # Return values
    /// * `FALSE` Delete specification Failed.
    /// * `TRUE` Delete specification Successful.
    pub fn delete_member_spec(&self, spec_no: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(spec_no)];
            let result_variant =
                invoke_method(&self.property.dispatch, "DeleteMemberSpec", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::delete_member_spec: {}", e),
            }
        }
    }

    /// Delete property based the on the property ID passed.
    /// # Parameters
    /// * `[in] nProperty` Property ID (Type: Long).
    /// # Return values
    /// * `FALSE` Delete Property Generate Error.
    /// * `TRUE` Delete Property Successful.
    pub fn delete_property(&self, property_id: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(property_id)];
            let result_variant =
                invoke_method(&self.property.dispatch, "DeleteProperty", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::delete_property: {}", e),
            }
        }
    }

    /// Returns the alpha angle of the section in radian.
    /// @ALPHA = Gets the angle between the principal axis and geometric axis of the section
    /// # Parameters
    /// * `[in] nPropNo` The specified property ID.
    /// * `[out] dAlpha` alpha angle returned (in Radian).
    pub fn get_alpha_angle_for_section(&self, prop_no: i32) -> Result<f64, anyErr> {
        unsafe {
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let mut params = [
                variant_from_raw_pointer::<f64>(alpha_ptr),
                VARIANT::from(prop_no),
            ];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "GetAlphaAngleForSection",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(*alpha_ptr),
                Err(e) => bail!("Error::Property::get_alpha_angle_for_section: {}", e),
            }
        }
    }

    /// Returns the location of the Centroid of the section.
    /// Gets the location of the Centroid of the specified section.
    /// @The Cez and Cey are distances to centroid from top left outer edge to the centroid in terms of Y axis and Z axis respectively.
    /// # Parameters
    /// * `[in] nPropNo` The specified property ID.
    /// * `[out] dCey` returns offset value of Centroid along Y axis.
    /// * `[out] dCez` returns offset value of Centroid along Z axis.
    pub fn get_centroid_location_for_section(&self, prop_no: i32) -> Result<(f64, f64), anyErr> {
        unsafe {
            let cey_ptr = &mut 0.0f64 as *mut f64;
            let cez_ptr = &mut 0.0f64 as *mut f64;
            let mut params = [
                variant_from_raw_pointer::<f64>(cez_ptr),
                variant_from_raw_pointer::<f64>(cey_ptr),
                VARIANT::from(prop_no),
            ];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "GetCentroidLocationForSection",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk((*cey_ptr, *cez_ptr)),
                Err(e) => bail!("Error::Property::get_centroid_location_for_section: {}", e),
            }
        }
    }

    /// Returns the total number of inactive members in the current model.
    /// # Return values
    /// * `<Val>` The total number of inactive member(s).
    pub fn get_inactive_member_count(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(&self.property.dispatch, "GetInactiveMemberCount", &mut [])
        };
        match result_variant {
            Ok(var) => {
                let count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(count)
            }
            Err(e) => bail!("Error::Property::get_inactive_member_count: {}", e),
        }
    }

    /// Populates a list of the member ID(s) of all the inactive members in the current model.
    /// # Parameters
    /// * `[out] nInactiveMemList` VARIANT array of LONG type, for storing returned member number ID(s) of all the members that are inactive.
    pub fn get_inactive_member_list(&self) -> Result<Vec<i32>, anyErr> {
        unsafe {
            let member_count = self.get_inactive_member_count()?;
            let mut psa = SafeArrayCreateVector(VT_I4, 0, member_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "GetInactiveMemberList",
                &mut params,
            );
            match result_variant {
                Ok(_) => {
                    let member_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut member_arr = Vec::with_capacity(member_count as usize);
                    for i in 0..member_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            member_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        member_arr.push(value as i32);
                    }
                    anyOk(member_arr)
                }
                Err(e) => bail!("Error::Property::get_inactive_member_list: {}", e),
            }
        }
    }

    /// Get releases for the specified member at the specified end.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// * `[in] varnEnd` Member Start end (= 0); member End end (= 1).
    /// * `[out] varnReleaseArray` Translational release VARIANT array with 6 elements for 6 DOFs.
    /// * `[out] varfSpringConstArray` Rotational releases VARIANT array with 6 elements for 6 DOFs.
    /// * `[out] varfMPFactor` Partial moment release factor (same for MX, MY and MZ).
    /// * `[out] varfMPFactorArray` Rotational releases VARIANT array with 3 elements for 3 rotational DOFs.
    /// # Return values
    /// * `1` OK.
    /// * `0` General error.
    pub fn get_member_release_spec_ex(
        &self,
        beam_no: i32,
        end: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>, f64, Vec<f64>), anyErr> {
        unsafe {
            let mut release_array = vec![0i32; 6];
            let mut spring_const_array = vec![0.0f64; 6];
            let mp_factor_ptr = &mut 0.0f64 as *mut f64;
            let mut mp_factor_array = vec![0.0f64; 3];

            let sa_release = SafeArrayCreateVector(VT_I4, 0, 6);
            let sa_spring = SafeArrayCreateVector(VT_R8, 0, 6);
            let sa_mp_factor = SafeArrayCreateVector(VT_R8, 0, 3);

            let mut params = [
                variant_from_raw_pointer::<SafeArray<f64>>(sa_mp_factor),
                variant_from_raw_pointer::<f64>(mp_factor_ptr),
                variant_from_raw_pointer::<SafeArray<f64>>(sa_spring),
                variant_from_raw_pointer::<SafeArray<i32>>(sa_release),
                VARIANT::from(end),
                VARIANT::from(beam_no),
            ];

            let result_variant = invoke_method(
                &self.property.dispatch,
                "GetMemberReleaseSpecEx",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();

                    // Extract release array
                    for i in 0..6 {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            sa_release,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        release_array[i] = value;
                    }

                    // Extract spring constant array
                    for i in 0..6 {
                        let mut index = i as i32;
                        let mut value = 0.0f64;
                        let _ = SafeArrayGetElement(
                            sa_spring,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        spring_const_array[i] = value;
                    }

                    // Extract MP factor array
                    for i in 0..3 {
                        let mut index = i as i32;
                        let mut value = 0.0f64;
                        let _ = SafeArrayGetElement(
                            sa_mp_factor,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        mp_factor_array[i] = value;
                    }

                    anyOk((
                        result_code,
                        release_array,
                        spring_const_array,
                        *mp_factor_ptr,
                        mp_factor_array,
                    ))
                }
                Err(e) => bail!("Error::Property::get_member_release_spec_ex: {}", e),
            }
        }
    }

    /// Get the type of specification attached to specified member.
    /// # Parameters
    /// * `[in] varMembNo` The member number ID.
    /// * `[out] SpecCode` Value referring to type of member specification.
    /// # Return values
    /// * `TRUE/1` OK.
    /// * `FALSE/0` Error.
    pub fn get_member_spec_code(&self, member_no: i32) -> Result<(bool, i32), anyErr> {
        unsafe {
            let spec_code_ptr = &mut 0i32 as *mut i32;
            let mut params = [
                variant_from_raw_pointer::<i32>(spec_code_ptr),
                VARIANT::from(member_no),
            ];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "GetMemberSpecCode",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((result_code == 1, *spec_code_ptr))
                }
                Err(e) => bail!("Error::Property::get_member_spec_code: {}", e),
            }
        }
    }

    /// Get Property Unique ID.
    /// # Parameters
    /// * `[in] nPropNo` Property number (Type: Long).
    /// # Returns
    /// * Property Unique ID (Type: String).
    pub fn get_property_unique_id(&self, prop_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(prop_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "GetPropertyUniqueID",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(result)
                }
                Err(e) => bail!("Error::Property::get_property_unique_id: {}", e),
            }
        }
    }

    /// Remove all element node release specification from the model.
    /// # Return values
    /// * `1` OK.
    /// * `0` No element release specification present
    pub fn remove_all_element_node_release_spec(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                &self.property.dispatch,
                "RemoveAllElementNodeReleaseSpec",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!(
                "Error::Property::remove_all_element_node_release_spec: {}",
                e
            ),
        }
    }

    /// Remove beam property.
    /// # Parameters
    /// * `[in] beamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_beam_property_helper(&self, beam_no: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "RemoveBeamPropertyHelper",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 0)
                }
                Err(e) => bail!("Error::Property::remove_beam_property_helper: {}", e),
            }
        }
    }

    /// Remove element ignore in plane rotation specification from plate.
    /// # Parameters
    /// * `[in] varnPlateNo` The plate number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_element_ignore_inplane_rotn_spec_from_plate(
        &self,
        plate_no: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(plate_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "RemoveElementIgnoreInplaneRotnSpecFromPlate",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::remove_element_ignore_inplane_rotn_spec_from_plate: {}",
                    e
                ),
            }
        }
    }

    /// Removes the member cable specification from a particular member at the provided location type (Tension or Length).
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Long/Integer).
    /// * `[in] varLocation` The Cable location at Tension (= 0) or Length (= 1) of the member.
    /// # Return values
    /// * `FALSE` Remove Member Cable specification failed
    /// * `TRUE` Remove Member Cable specification Successful.
    pub fn remove_member_cable_spec_from_beam(
        &self,
        beam_no: i32,
        location: i32,
    ) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(location), VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "RemoveMemberCableSpecFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::remove_member_cable_spec_from_beam: {}", e),
            }
        }
    }

    /// Remove member compression specification from beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_member_compression_spec_from_beam(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "RemoveMemberCompressionSpecFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::remove_member_compression_spec_from_beam: {}",
                    e
                ),
            }
        }
    }

    /// Remove member ignore stiff specification from beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_member_ignore_stiff_spec_from_beam(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "RemoveMemberIgnoreStiffSpecFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::remove_member_ignore_stiff_spec_from_beam: {}",
                    e
                ),
            }
        }
    }

    /// Remove member inactive specification from beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_member_inactive_spec_from_beam(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "RemoveMemberInactiveSpecFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::remove_member_inactive_spec_from_beam: {}",
                    e
                ),
            }
        }
    }

    /// Removes the member specification from a particular member at the provided location (Start or End).
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Long/Integer).
    /// * `[in] varLocation` The Release location at START (= 0) or END (= 1) of the member.
    /// # Return values
    /// * `FALSE` Remove Member Release specification failed
    /// * `TRUE` Remove Member Release specification Successful.
    pub fn remove_member_release_spec_from_beam(
        &self,
        beam_no: i32,
        location: i32,
    ) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(location), VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "RemoveMemberReleaseSpecFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!(
                    "Error::Property::remove_member_release_spec_from_beam: {}",
                    e
                ),
            }
        }
    }

    /// Remove member tension specification from beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_member_tension_spec_from_beam(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "RemoveMemberTensionSpecFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::remove_member_tension_spec_from_beam: {}",
                    e
                ),
            }
        }
    }

    /// Remove member truss specification from beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_member_truss_spec_from_beam(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "RemoveMemberTrussSpecFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::remove_member_truss_spec_from_beam: {}", e),
            }
        }
    }

    /// Remove property from beam.
    /// # Parameters
    /// * `[in] nBeamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_property_from_beam(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "RemovePropertyFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::remove_property_from_beam: {}", e),
            }
        }
    }

    /// Set Property Unique ID to specification property number.
    /// # Parameters
    /// * `[in] nPropNo` Property number (Type: Long).
    /// * `[in] szID` Property Unique ID (Type: String).
    pub fn set_property_unique_id(&self, prop_no: i32, unique_id: &str) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(unique_id), VARIANT::from(prop_no)];
            let result_variant = invoke_method(
                &self.property.dispatch,
                "SetPropertyUniqueID",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Property::set_property_unique_id: {}", e),
            }
        }
    }
}
