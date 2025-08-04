use crate::openstaad::{
    geometry::root::Geometry,
    tools::safe_array::{safe_array_from_vec1d, safe_array_from_vec2d},
    tools::invoke::invoke_method,
    tools::variant::{SafeArray, SafeArrayP, variant_from_raw_pointer},
};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use std::ffi::c_void;
use windows::Win32::System::{
    Com::SAFEARRAY,
    Ole::{SafeArrayCreateVector, SafeArrayGetElement},
    Variant::{VARIANT, VT_I4, VariantToDouble, VariantToInt32, VariantToStringAlloc},
};
use windows_core::BSTR;

// AddBeam
// AddMultipleBeams
// BreakBeamsAtSpecificNodes
// CreateBeam
// CreateMultipleBeams
// DeleteBeam
// GetBeamLength
// GetBeamList
// GetBeamsConnectedAtNode
// GetCountOfBreakableBeamsAtSpecificNodes
// GetIntersectBeamsCount
// GetLastBeamNo
// GetMemberCount
// GetMemberIncidence
// GetMemberIncidence_CIS2
// GetMemberUniqueID
// GetNoOfBeamsConnectedAtNode
// IntersectBeams
// IsBeam
// IsColumn
// IsZUp
// MergeBeams
// RenumberBeam
// SetCheckForIdenticalEntity
// SetMemberUniqueID
// SplitBeam
// SplitBeamInEqlParts

#[derive(Debug)]
pub struct Beam<'a> {
    pub geometry: &'a Geometry<'a>,
}

impl<'a> Beam<'a> {
    pub fn new(geometry: &'a Geometry<'a>) -> Self {
        Self { geometry }
    }

    /// Adds a beam/member with specified nodes in current model, and returns the member number ID automatically assigned with.
    /// # Parameters
    /// * `[in] nNodeA` Number ID of the STARTING end node (nodeA).
    /// * `[in] nNodeB` Number ID of the ENDING end node (nodeB).
    /// # Return values
    /// * `<Val>` Member number ID assigned to this created member.
    /// * `-1` Unable to add member.
    /// * `-2001` Cannot find Node < nNodeA or nNodeB >.
    pub fn add_beam(&self, node_a: i32, node_b: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(node_b), VARIANT::from(node_a)];
            let result_variant =
                invoke_method(&self.geometry.dispatch, "AddBeam", &mut params);
            match result_variant {
                Ok(var) => {
                    let beam_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(beam_id)
                }
                Err(e) => bail!("Error::Geometry::add_beam: {}", e),
            }
        }
    }

    /// Add multiple beams with specified end node number ID(s).
    /// # Parameters
    /// * `[in] naIncidences` VARIANT array of m * 2 dimension containing member starting and ending end nodes: [NodeAi, NodeBi].
    pub fn add_multiple_beams(&self, incidences: Vec<Vec<i32>>) -> Result<(), anyErr> {
        unsafe {
            let sa_incidences = safe_array_from_vec2d::<i32>(incidences)?;
            let variant_incidences = variant_from_raw_pointer::<SafeArray<i32>>(sa_incidences);

            let mut params = [variant_incidences];
            let result_variant =
                invoke_method(&self.geometry.dispatch, "AddMultipleBeams", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::add_multiple_beams: {}", e),
            }
        }
    }

    /// Breaks beams that passes through the specified list of nodes and assigns same attributes to the newly added beams.
    /// # Parameters
    /// * `[in] nNodeIdArray` array of node numbers to be used to find the number of beams that can be split (type - Long/Integer).
    /// * `[out] nBrokenBeamIdArray` array of existing beam numbers that are broken (type - Long/Integer).
    /// * `[out] nNewBeamIdArray` array of new beam numbers that are added (type - Long/Integer).
    /// # Return values
    /// * `1` if the method is successful.
    /// * `0` if the method is unsuccessful.
    pub fn break_beams_at_specific_nodes(
        &self,
        node_ids: Vec<i32>,
    ) -> Result<(bool, Vec<i32>, Vec<i32>), anyErr> {
        unsafe {
            let sa_nodes = safe_array_from_vec1d::<i32>(node_ids.clone())?;
            let variant_nodes = variant_from_raw_pointer::<SafeArray<i32>>(sa_nodes);

            // Get count first to allocate arrays
            let count = self.get_count_of_breakable_beams_at_specific_nodes(node_ids)?;

            let mut broken_sa = SafeArrayCreateVector(VT_I4, 0, count as u32);
            let mut new_sa = SafeArrayCreateVector(VT_I4, 0, count as u32);

            let broken_sa_ptr = &mut broken_sa as *mut *mut SAFEARRAY;
            let new_sa_ptr = &mut new_sa as *mut *mut SAFEARRAY;

            let mut params = [
                variant_from_raw_pointer::<SafeArrayP<i32>>(new_sa_ptr),
                variant_from_raw_pointer::<SafeArrayP<i32>>(broken_sa_ptr),
                variant_nodes,
            ];

            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "BreakBeamsAtSpecificNodes",
                &mut params,
            );

            match result_variant {
                Ok(var) => {
                    let success = VariantToInt32(&var as *const VARIANT).unwrap() > 0;

                    let mut broken_beams = Vec::with_capacity(count as usize);
                    let mut new_beams = Vec::with_capacity(count as usize);

                    for i in 0..count {
                        let mut index = i as i32;
                        let mut broken_value = 0;
                        let mut new_value = 0;

                        let _ = SafeArrayGetElement(
                            broken_sa,
                            &mut index as *mut i32,
                            &mut broken_value as *mut i32 as *mut c_void,
                        )?;

                        let _ = SafeArrayGetElement(
                            new_sa,
                            &mut index as *mut i32,
                            &mut new_value as *mut i32 as *mut c_void,
                        )?;

                        broken_beams.push(broken_value);
                        new_beams.push(new_value);
                    }

                    anyOk((success, broken_beams, new_beams))
                }
                Err(e) => bail!("Error::Geometry::break_beams_at_specific_nodes: {}", e),
            }
        }
    }

    /// Creates a beam/member with specified nodes in current model.
    /// # Parameters
    /// * `[in] nBeamNo` Member number ID to be assigned to the newly created member.
    /// * `[in] nNodeA` Number ID of the STARTING end node (nodeA).
    /// * `[in] nNodeB` Number ID of the ENDING end node (nodeB).
    pub fn create_beam(&self, beam_no: i32, node_a: i32, node_b: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(node_b),
                VARIANT::from(node_a),
                VARIANT::from(beam_no),
            ];
            let result_variant =
                invoke_method(&self.geometry.dispatch, "CreateBeam", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::create_beam: {}", e),
            }
        }
    }

    /// Create multiple beams with specified beam number ID(s).
    /// # Parameters
    /// * `[in] nBeamIdArray` Integer array of 1 dimension containing m node IDs:[IDi]
    /// * `[in] nBeamIncidenceArray` int VARIANT array of m * 2 dimension containing member starting and ending end nodes: [NODEAi, NODEBi].
    pub fn create_multiple_beams(
        &self,
        beam_ids: Vec<i32>,
        incidences: Vec<Vec<i32>>,
    ) -> Result<(), anyErr> {
        unsafe {
            let sa_ids = safe_array_from_vec1d::<i32>(beam_ids)?;
            let sa_incidences = safe_array_from_vec2d::<i32>(incidences)?;
            let variant_ids = variant_from_raw_pointer::<SafeArray<i32>>(sa_ids);
            let variant_incidences = variant_from_raw_pointer::<SafeArray<i32>>(sa_incidences);

            let mut params = [variant_incidences, variant_ids];
            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "CreateMultipleBeams",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::create_multiple_beams: {}", e),
            }
        }
    }

    /// Delete a specified member.
    /// # Parameters
    /// * `[in] nBeamNo` Member number ID.
    pub fn delete_beam(&self, beam_no: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant =
                invoke_method(&self.geometry.dispatch, "DeleteBeam", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::delete_beam: {}", e),
            }
        }
    }

    /// Returns the length for specified member.
    /// # Parameters
    /// * `[in] nBeamNo` Member number ID for which the length is to be retrieved
    /// # Return values
    /// * `<Val>` The length of specified member in double.
    /// * `0` Cannot find member < nBeamNo >.
    pub fn get_beam_length(&self, beam_no: i32, base_unit: i32) -> Result<f64, anyErr> {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }
            let mut params = [VARIANT::from(beam_no)];
            let result_variant =
                invoke_method(&self.geometry.dispatch, "GetBeamLength", &mut params);
            match result_variant {
                Ok(var) => {
                    let length = VariantToDouble(&var as *const VARIANT).unwrap();
                    anyOk(length / unit_factor)
                }
                Err(e) => bail!("Error::Geometry::get_beam_length: {}", e),
            }
        }
    }

    /// Returns a list of all the member ID(s) the current model.
    /// # Parameters
    /// * `[out] nBeamList` VARIANT array of LONG type, for storing returned member number ID(s).
    pub fn get_beam_list(&self) -> Result<Vec<i32>, anyErr> {
        unsafe {
            let beam_count = self.get_member_count()?;
            let mut psa = SafeArrayCreateVector(VT_I4, 0, beam_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant];
            let result_variant =
                invoke_method(&self.geometry.dispatch, "GetBeamList", &mut params);
            match result_variant {
                Ok(_) => {
                    let beam_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut beam_arr = Vec::with_capacity(beam_count as usize);
                    for i in 0..beam_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            beam_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        beam_arr.push(value as i32);
                    }
                    anyOk(beam_arr)
                }
                Err(e) => bail!("Error::Geometry::get_beam_list: {}", e),
            }
        }
    }

    /// Returns a list of all the beams connected to the specified node.
    /// # Parameters
    /// * `[in] nNodeNo` Node number ID.
    /// * `[out] nBeamList` VARIANT array of LONG type, for storing returned member number ID(s).
    pub fn get_beams_connected_at_node(&self, node_no: i32) -> Result<(i32, Vec<i32>), anyErr> {
        unsafe {
            let beam_count = self.get_no_of_beams_connected_at_node(node_no)?;
            let mut psa = SafeArrayCreateVector(VT_I4, 0, beam_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant, VARIANT::from(node_no)];
            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "GetBeamsConnectedAtNode",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let count = VariantToInt32(&var as *const VARIANT).unwrap();
                    let beam_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut beam_arr = Vec::with_capacity(count as usize);
                    for i in 0..count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            beam_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        beam_arr.push(value as i32);
                    }
                    anyOk((count, beam_arr))
                }
                Err(e) => bail!("Error::Geometry::get_beams_connected_at_node: {}", e),
            }
        }
    }

    /// Returns number of beams that can be broken based on the list of node Ids.
    /// # Parameters
    /// * `[in] nNodeIdArray` array of node numbers to be used to find the number of beams that can be split (type - Long/Integer).
    /// # Return values
    /// * returns the number of beams that satifies the criteria.
    pub fn get_count_of_breakable_beams_at_specific_nodes(
        &self,
        node_ids: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_nodes = safe_array_from_vec1d::<i32>(node_ids)?;
            let variant_nodes = variant_from_raw_pointer::<SafeArray<i32>>(sa_nodes);

            let mut params = [variant_nodes];
            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "GetCountOfBreakableBeamsAtSpecificNodes",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(count)
                }
                Err(e) => bail!(
                    "Error::Geometry::get_count_of_breakable_beams_at_specific_nodes: {}",
                    e
                ),
            }
        }
    }

    /// Returns number of new beams that will be created if the specified list of beams are intersected.
    /// # Parameters
    /// * `[in] BeamNosArray` Array of Beams numbers. If the array is either null or empty then all members in current model will be considered (Long)
    /// * `[in] dTolerance` Tolerance to be used for finding beam intersection, should not be negative value, meter for Metric and inch for English in Base Unit(float/double)
    /// # Return values
    /// * returns the number of beams that satifies the criteria.
    pub fn get_intersect_beams_count(
        &self,
        beam_nos: Vec<i32>,
        tolerance: f64,
        base_unit: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }

            let sa_beams = safe_array_from_vec1d::<i32>(beam_nos)?;
            let variant_beams = variant_from_raw_pointer::<SafeArray<i32>>(sa_beams);

            let mut params = [VARIANT::from(tolerance * unit_factor), variant_beams];
            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "GetIntersectBeamsCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(count)
                }
                Err(e) => bail!("Error::Geometry::get_intersect_beams_count: {}", e),
            }
        }
    }

    /// Returns the member number ID of the last beam in the model.
    /// # Return values
    /// * `<Val>` The number of the highest beam number ID in the model (Type: Long)
    /// * `-1` General error.
    pub fn get_last_beam_no(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.geometry.dispatch, "GetLastBeamNo", &mut []) };
        match result_variant {
            Ok(var) => {
                let last_beam_no = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(last_beam_no)
            }
            Err(e) => bail!("Error::Geometry::get_last_beam_no: {}", e),
        }
    }

    /// Returns the total number of members in the current model.
    /// # Return values
    /// * `<Val>` The total number of member(s).
    pub fn get_member_count(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(&self.geometry.dispatch, "GetMemberCount", &mut [])
        };
        match result_variant {
            Ok(var) => {
                let member_count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(member_count)
            }
            Err(e) => bail!("Error::Geometry::get_member_count: {}", e),
        }
    }

    /// Returns the number ID(s) of connecting node(s) for specified member.
    /// # Parameters
    /// * `[in] nBeamNo` Member number ID.
    /// * `[out] nNodeA` Number ID of the starting end node (nodeA).
    /// * `[out] nNodeB` Number ID of the ending end node(nodeB).
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    /// * `-3001` Cannot find Node < nBeamNo >.
    pub fn get_member_incidence(&self, beam_no: i32) -> Result<(i32, i32, i32), anyErr> {
        unsafe {
            let node_a_ptr = &mut 0i32 as *mut i32;
            let node_b_ptr = &mut 0i32 as *mut i32;

            let mut params = [
                variant_from_raw_pointer::<i32>(node_b_ptr),
                variant_from_raw_pointer::<i32>(node_a_ptr),
                VARIANT::from(beam_no),
            ];

            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "GetMemberIncidence",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((result_code, *node_a_ptr, *node_b_ptr))
                }
                Err(e) => bail!("Error::Geometry::get_member_incidence: {}", e),
            }
        }
    }

    /// Returns the number ID(s) of connecting node(s) for specified member.
    /// # Parameters
    /// * `[in] nBeamNo` Member number ID.
    /// * `[out] szName` (LPCTSTR) unique string ID.
    /// * `[out] nNodeA` Number ID of the starting end node (nodeA).
    /// * `[out] nNodeB` Number ID of the ending end node(nodeB).
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    /// * `-3001` Cannot find Node < nBeamNo >.
    pub fn get_member_incidence_cis2(
        &self,
        beam_no: i32,
    ) -> Result<(i32, String, i32, i32), anyErr> {
        unsafe {
            let name_ptr = &mut BSTR::default() as *mut BSTR;
            let node_a_ptr = &mut 0i32 as *mut i32;
            let node_b_ptr = &mut 0i32 as *mut i32;

            let mut params = [
                variant_from_raw_pointer::<i32>(node_b_ptr),
                variant_from_raw_pointer::<i32>(node_a_ptr),
                variant_from_raw_pointer::<BSTR>(name_ptr),
                VARIANT::from(beam_no),
            ];

            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "GetMemberIncidence_CIS2",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    let name = (&*name_ptr).to_string();
                    anyOk((result_code, name, *node_a_ptr, *node_b_ptr))
                }
                Err(e) => bail!("Error::Geometry::get_member_incidence_cis2: {}", e),
            }
        }
    }

    /// Returns the unique string ID (GUID) for specified member.
    /// # Parameters
    /// * `[in] nMembNo` Member number ID.
    /// # Return values
    /// * `<VARIANT>` Unique string ID for specified member.
    /// * The API would return an empty string if specified member < nMembNo > is not found
    pub fn get_member_unique_id(&self, member_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(member_no)];
            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "GetMemberUniqueID",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let unique_id = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting unique id")?
                        .to_string()?;
                    anyOk(unique_id)
                }
                Err(e) => bail!("Error::Geometry::get_member_unique_id: {}", e),
            }
        }
    }

    /// Returns no of beams connected at a specified node.
    /// # Parameters
    /// * `[in] nNodeNo` Node number ID.
    pub fn get_no_of_beams_connected_at_node(&self, node_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(node_no)];
            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "GetNoOfBeamsConnectedAtNode",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(count)
                }
                Err(e) => bail!("Error::Geometry::get_no_of_beams_connected_at_node: {}", e),
            }
        }
    }

    /// A function that takes a list of beam numbers and either identify those that would be split/connected due to overlapping each other and highlighting them on the model or simply performing the intersection routine and returning a list of members resulting from running the intersection routine that have been either modified or added to the model.
    /// # Parameters
    /// * `[in] Method` Pass 1 to highlight the member(s) or 2 to intersect the member(s) (Long/Integer)
    /// * `[in] BeamNosArray` Array of Beams numbers. If the array is either null or empty then all members in current model will be considered (Long)
    /// * `[in] dTolerance` Tolerance to be used for finding beam intersection, should not be negative value, meter for Metric and inch for English in Base Unit (float/double)
    /// * `[out] NewBeamNosArray` The ids of the beams that have been changed and added, only used for intersect method.(type - array of Long)
    /// # Return values
    /// * `0` Failed
    /// * `1` Succeeded
    pub fn intersect_beams(
        &self,
        method: i32,
        beam_nos: Vec<i32>,
        tolerance: f64,
        base_unit: i32,
    ) -> Result<(bool, Vec<i32>), anyErr> {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }

            let sa_beams = safe_array_from_vec1d::<i32>(beam_nos.clone())?;
            let variant_beams = variant_from_raw_pointer::<SafeArray<i32>>(sa_beams);

            let new_beam_count = self.get_intersect_beams_count(beam_nos, tolerance, base_unit)?;
            let mut new_beams_sa = SafeArrayCreateVector(VT_I4, 0, new_beam_count as u32);
            let new_beams_sa_ptr = &mut new_beams_sa as *mut *mut SAFEARRAY;

            let mut params = [
                variant_from_raw_pointer::<SafeArrayP<i32>>(new_beams_sa_ptr),
                VARIANT::from(tolerance * unit_factor),
                variant_beams,
                VARIANT::from(method),
            ];

            let result_variant =
                invoke_method(&self.geometry.dispatch, "IntersectBeams", &mut params);

            match result_variant {
                Ok(var) => {
                    let success = VariantToInt32(&var as *const VARIANT).unwrap() > 0;

                    let mut new_beams = Vec::with_capacity(new_beam_count as usize);
                    for i in 0..new_beam_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            new_beams_sa,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        new_beams.push(value);
                    }

                    anyOk((success, new_beams))
                }
                Err(e) => bail!("Error::Geometry::intersect_beams: {}", e),
            }
        }
    }

    /// Returns if the angle of inclination for specified BEAM member is not more than given tolerance angle (for small angle only).
    /// # Parameters
    /// * `[in] nMemberNo` Beam member number ID (Type: Long).
    /// * `[in] dTolAngle` Tolerance inclination angle (Type: Double).
    /// # Return values
    /// * `1` True
    /// * `0` False
    /// * `-3001` Member number ID is not found
    pub fn is_beam(&self, member_no: i32, tolerance_angle: f64) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(tolerance_angle), VARIANT::from(member_no)];
            let result_variant =
                invoke_method(&self.geometry.dispatch, "IsBeam", &mut params);
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result)
                }
                Err(e) => bail!("Error::Geometry::is_beam: {}", e),
            }
        }
    }

    /// Returns if the angle of inclination for specified COLUMN member is not more than given tolerance angle (for small angle only).
    /// # Parameters
    /// * `[in] nMemberNo` Column member number ID (Type: Long).
    /// * `[in] dTolAngle` Tolerance inclination angle (Type: Double).
    /// # Return values
    /// * `1` True
    /// * `0` False
    /// * `-3001` Member number ID is not found
    pub fn is_column(&self, member_no: i32, tolerance_angle: f64) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(tolerance_angle), VARIANT::from(member_no)];
            let result_variant =
                invoke_method(&self.geometry.dispatch, "IsColumn", &mut params);
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result)
                }
                Err(e) => bail!("Error::Geometry::is_column: {}", e),
            }
        }
    }

    /// Returns if Z-axis is in upward direction?
    /// # Return values
    /// * `1` True;
    /// * `0` False;
    pub fn is_z_up(&self) -> Result<bool, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.geometry.dispatch, "IsZUp", &mut []) };
        match result_variant {
            Ok(var) => {
                let is_up = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() > 0 };
                anyOk(is_up)
            }
            Err(e) => bail!("Error::Geometry::is_z_up: {}", e),
        }
    }

    /// Merges multiple collinear and connected beams to a single beam with specified id, property, material and beta angle.
    /// # Parameters
    /// * `[in] nBeamIdArray` array of beam numbers to be merged (must have more than one beam) (type - Long/Integer).
    /// * `[in] varBeamNo` beam number to be assigned to the merged beam (must be present in nBeamIdArray) (type - Long/Integer).
    /// * `[in] varPropertyNo` property reference number to be assigned to the merged beam (type - Long/Integer).
    /// * `[in] varBetaAngle` beta angle (in degrees) to be assigned to the merged beam (type - float/double).
    /// * `[in] varMaterialName` material name to be assigned to the merged beam (type - String).
    /// # Return values
    /// * `1` if merging is successful.
    /// * `0` if merging is unsuccessful.
    pub fn merge_beams(
        &self,
        beam_ids: Vec<i32>,
        beam_no: i32,
        property_no: i32,
        beta_angle: f64,
        material_name: &str,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_beams = safe_array_from_vec1d::<i32>(beam_ids)?;
            let variant_beams = variant_from_raw_pointer::<SafeArray<i32>>(sa_beams);

            let mut params = [
                VARIANT::from(material_name),
                VARIANT::from(beta_angle),
                VARIANT::from(property_no),
                VARIANT::from(beam_no),
                variant_beams,
            ];

            let result_variant =
                invoke_method(&self.geometry.dispatch, "MergeBeams", &mut params);
            match result_variant {
                Ok(var) => {
                    let success = VariantToInt32(&var as *const VARIANT).unwrap() > 0;
                    anyOk(success)
                }
                Err(e) => bail!("Error::Geometry::merge_beams: {}", e),
            }
        }
    }

    /// Renumbers the existing beam id with the specified id.
    /// # Parameters
    /// * `[in] varBeamNoOld` Old beam ID.
    /// * `[in] varBeamNoNew` New beam ID.
    /// # Return values
    /// * `1` True;
    /// * `0` False;
    pub fn renumber_beam(&self, old_beam_no: i32, new_beam_no: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(new_beam_no), VARIANT::from(old_beam_no)];
            let result_variant =
                invoke_method(&self.geometry.dispatch, "RenumberBeam", &mut params);
            match result_variant {
                Ok(var) => {
                    let success = VariantToInt32(&var as *const VARIANT).unwrap() > 0;
                    anyOk(success)
                }
                Err(e) => bail!("Error::Geometry::renumber_beam: {}", e),
            }
        }
    }

    /// This API will set whether to enable checking for existing identical entities (beam, plate, node etc.) or not.
    /// # Parameters
    /// * `[in] entityType` Identification for the entity type(enum STAADEntityType).
    /// * `[in] bEnable` Whether to enable identical enitity check or not. (0 = FALSE, 1 = TRUE)
    /// # Return values
    /// * `0` (=FALSE) or 1 (=TRUE)
    pub fn set_check_for_identical_entity(
        &self,
        entity_type: i32,
        enable: bool,
    ) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(enable as i32), VARIANT::from(entity_type)];
            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "SetCheckForIdenticalEntity",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap() > 0;
                    anyOk(result)
                }
                Err(e) => bail!("Error::Geometry::set_check_for_identical_entity: {}", e),
            }
        }
    }

    /// Assigns an unique string ID (GUID) to specified member.
    /// # Parameters
    /// * `[in] nMembNo` Member number ID.
    /// * `[in] szName` (LPCTSTR) unique string ID.
    pub fn set_member_unique_id(&self, member_no: i32, unique_id: &str) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(unique_id), VARIANT::from(member_no)];
            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "SetMemberUniqueID",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::set_member_unique_id: {}", e),
            }
        }
    }

    /// Split a specified beam into several beams by specified node(s).
    /// # Parameters
    /// * `[in] nBeamNo` Number ID of the beam to split.
    /// * `[in] nNodes` The number of node(s) to be inserted in the beam.
    /// * `[in] faDistToNodes` VARIANT array of distance(s) in length from the starting end node of member.
    pub fn split_beam(
        &self,
        beam_no: i32,
        num_nodes: i32,
        distances: Vec<f64>,
        base_unit: i32,
    ) -> Result<(), anyErr> {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }

            let scaled_distances: Vec<f64> =
                distances.into_iter().map(|d| d * unit_factor).collect();

            let sa_distances = safe_array_from_vec1d::<f64>(scaled_distances)?;
            let variant_distances = variant_from_raw_pointer::<SafeArray<f64>>(sa_distances);

            let mut params = [
                variant_distances,
                VARIANT::from(num_nodes),
                VARIANT::from(beam_no),
            ];

            let result_variant =
                invoke_method(&self.geometry.dispatch, "SplitBeam", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::split_beam: {}", e),
            }
        }
    }

    /// Split a specified beam into several EQUAL beams by specified number of node(s)
    /// # Parameters
    /// * `[in] nBeamNo` Number ID of the beam to split.
    /// * `[in] nParts` The number of parts into which the beam is to be split.
    pub fn split_beam_in_equal_parts(&self, beam_no: i32, num_parts: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(num_parts), VARIANT::from(beam_no)];

            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "SplitBeamInEqlParts",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::split_beam_in_equal_parts: {}", e),
            }
        }
    }
}
