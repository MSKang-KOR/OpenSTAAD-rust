use crate::openstaad::tools::{
    invoke::{get_dispatch, invoke_method},
    safe_array::{safe_array_from_vec1d, safe_array_from_vec2d},
    variant::{SafeArray, SafeArrayP, variant_from_raw_pointer},
};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use std::ffi::c_void;
use windows::Win32::System::{
    Com::{IDispatch, SAFEARRAY},
    Ole::{SafeArrayCreateVector, SafeArrayGetElement},
    Variant::{VARIANT, VT_BSTR, VT_I4, VariantToDouble, VariantToInt32, VariantToStringAlloc},
};
use windows_core::BSTR;

#[derive(Debug)]
pub struct Geometry<'a> {
    pub staad: &'a IDispatch,
    pub dispatch: IDispatch,
}

impl<'a> Geometry<'a> {
    pub fn new(staad: &'a IDispatch) -> Self {
        let _geometry = unsafe { get_dispatch(staad, "Geometry", &mut []).unwrap() };
        Self {
            staad,
            dispatch: _geometry,
        }
    }
    pub fn add_multiple_nodes(
        &self,
        coordinates: Vec<Vec<f64>>,
        base_unit: i32,
    ) -> Result<(), anyErr> {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }
            let _coords: Vec<Vec<f64>> = coordinates
                .into_iter()
                .map(|coord| coord.into_iter().map(|value| value * unit_factor).collect())
                .collect();
            let sa_coords = safe_array_from_vec2d::<f64>(_coords)?;
            let variant_coords = variant_from_raw_pointer::<SafeArray<f64>>(sa_coords);

            let mut params = [variant_coords];
            let result_variant = invoke_method(&self.dispatch, "AddMultipleNodes", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::add_multiple_nodes: {}", e),
            }
        }
    }

    /// Adds a node with specified coordinates in current model and returns the node number ID automatically assigned with.
    /// # Parameters
    /// * `[in] fCoordX` Nodal coordinate X in GLOBAL.
    /// * `[in] fCoordY` Nodal coordinate Y in GLOBAL.
    /// * `[in] fCoordZ` Nodal coordinate Z in GLOBAL.
    /// # Return values
    /// * `<Val>` Node number ID assigned to this created node.
    /// * `0` OK.
    /// * `-2004` Unable to add Node.
    pub fn add_node(
        &self,
        coord_x: f64,
        coord_y: f64,
        coord_z: f64,
        base_unit: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }
            let mut params = [
                VARIANT::from(coord_z * unit_factor),
                VARIANT::from(coord_y * unit_factor),
                VARIANT::from(coord_x * unit_factor),
            ];
            let result_variant = invoke_method(&self.dispatch, "AddNode", &mut params);
            match result_variant {
                Ok(code_variant) => {
                    let result_code = VariantToInt32(&code_variant as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Geometry::create_node: {}", e),
            }
        }
    }

    /// Create multiple nodes with specified node number ID(s).
    /// # Parameters
    /// * `[in] nNodeIdArray` Integer array of 1 dimension containing m node IDs:[IDi]
    /// * `[in] dCoordArray` Double array of 2 dimensions containing M*3 elements i.e node coordinate: [Xi, Yi, Zi].
    pub fn create_multiple_nodes(
        &self,
        node_ids: Vec<i32>,
        coordinates: Vec<Vec<f64>>,
    ) -> Result<(), anyErr> {
        unsafe {
            let sa_ids = safe_array_from_vec1d::<i32>(node_ids)?;
            let sa_coords = safe_array_from_vec2d::<f64>(coordinates)?;
            let variant_ids = variant_from_raw_pointer::<SafeArray<i32>>(sa_ids);
            let variant_coords = variant_from_raw_pointer::<SafeArray<f64>>(sa_coords);

            let mut params = [variant_coords, variant_ids];
            let result_variant = invoke_method(&self.dispatch, "CreateMultipleNodes", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::create_multiple_nodes: {}", e),
            }
        }
    }

    /// Creates a node with specified coordinates in the structure with the number specified in nNodeNo.
    /// # Parameters
    /// * `[in]	nNodeNo` Node number ID to be assigned to the newly created node.
    /// * `[in]	fCoordX` Nodal coordinate X in GLOBAL.
    /// * `[in]	fCoordY` Nodal coordinate Y in GLOBAL.
    /// * `[in]	fCoordZ` Nodal coordinate Z in GLOBAL.
    pub fn create_node(
        &self,
        node_no: i32,
        coord_x: f64,
        coord_y: f64,
        coord_z: f64,
    ) -> Result<(), anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(coord_z),
                VARIANT::from(coord_y),
                VARIANT::from(coord_x),
                VARIANT::from(node_no),
            ];
            let result_variant = invoke_method(&self.dispatch, "CreateNode", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::create_node: {}", e),
            }
        }
    }

    /// Delete a specified node.
    /// # Parameters
    /// * `[in]	nNodeNo` Node number ID.
    pub fn delete_node(&self, node_no: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(node_no)];
            let result_variant = invoke_method(&self.dispatch, "DeleteNode", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::delete_node: {}", e),
            }
        }
    }

    /// Returns the node number ID of the last node in the model.
    /// # Return values
    /// * `<Val>` The number of the highest node number ID in the model .
    /// * `-1` General error.
    pub fn get_last_node_no(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe { invoke_method(&self.dispatch, "GetLastNodeNo", &mut []) };
        match result_variant {
            Ok(v) => {
                let result_node_no = unsafe { VariantToInt32(&v as *const VARIANT).unwrap() };
                anyOk(result_node_no)
            }
            Err(e) => bail!("Error::Geometry::get_last_node_no: {}", e),
        }
    }

    /// Returns the coordinates of the specified node.
    /// # Parameters
    /// * `[in]	nNodeNo` Node number ID.
    /// * `[out] fCoordX` Nodal coordinate X in GLOBAL.
    /// * `[out] fCoordY` Nodal coordinate Y in GLOBAL.
    /// * `[out] fCoordZ` Nodal coordinate Z in GLOBAL.
    pub fn get_node_coordinates(&self, node_no: i32, base_unit: i32) -> Result<Vec<f64>, anyErr> {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }
            let x_ptr: *mut f64 = &mut 0. as *mut f64;
            let y_ptr: *mut f64 = &mut 0. as *mut f64;
            let z_ptr: *mut f64 = &mut 0. as *mut f64;
            let mut params: [VARIANT; 4] = [
                variant_from_raw_pointer::<f64>(z_ptr),
                variant_from_raw_pointer::<f64>(y_ptr),
                variant_from_raw_pointer::<f64>(x_ptr),
                VARIANT::from(node_no),
            ];
            let result_variant = invoke_method(&self.dispatch, "GetNodeCoordinates", &mut params);
            match result_variant {
                Ok(_) => anyOk(vec![
                    *x_ptr / unit_factor,
                    *y_ptr / unit_factor,
                    *z_ptr / unit_factor,
                ]),
                Err(e) => bail!("Error::Geometry::get_node_coordinates: {}", e),
            }
        }
    }

    /// Returns the total number of nodes in the current model.
    ///
    /// Note: Count of nodes can vary depending upon the flag set for consideration of hidden entities (see OSGeometryUI::SetFlagForHiddenEntities).
    /// # Return values
    /// * `<Val>` The total number of node(s).
    pub fn get_node_count(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe { invoke_method(&self.dispatch, "GetNodeCount", &mut []) };
        match result_variant {
            Ok(v) => {
                let result_node_no = unsafe { VariantToInt32(&v as *const VARIANT).unwrap() };
                anyOk(result_node_no)
            }
            Err(e) => bail!("Error::Geometry::get_node_count: {}", e),
        }
    }

    /// Returns the distance between two specified nodes.
    /// # Parameters
    /// * `[in]	nNodeNoA` Number ID of one node (Type: Long).
    /// * `[in]	nNodeNoB` Number ID of one of the other node (Type: Long).
    /// # Return values
    /// * `<Val>`	The distances in double.
    /// * `-1` General error.
    /// * `-2001` Cannot find Node < nNodeNoA > or < nNodeNoB >.
    pub fn get_node_distance(
        &self,
        node_a: i32,
        node_b: i32,
        base_unit: i32,
    ) -> Result<f64, anyErr> {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }
            let mut params = [VARIANT::from(node_a), VARIANT::from(node_b)];
            let result_variant = invoke_method(&self.dispatch, "GetNodeDistance", &mut params);
            match result_variant {
                Ok(v) => {
                    let distance = VariantToDouble(&v as *const VARIANT).unwrap();
                    anyOk(distance / unit_factor)
                }
                Err(e) => bail!("Error::Geometry::get_node_distance: {}", e),
            }
        }
    }

    /// Return the coordinates of the specified node.
    /// # Parameters
    /// * `[in]	nNodeNo` Node number ID (Type: Long).
    /// * `[out] fCoordX` Nodal coordinate X in GLOBAL (Type: Double).
    /// * `[out] fCoordY` Nodal coordinate Y in GLOBAL (Type: Double).
    /// * `[out] fCoordZ` Nodal coordinate Z in GLOBAL (Type: Double).
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    /// * `-2001` Cannot find Node < nNodeNo >.
    pub fn get_node_incidence(
        &self,
        node_no: i32,
        base_unit: i32,
    ) -> Result<(i32, Vec<f64>), anyErr> {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }
            let x_ptr = &mut 0. as *mut f64;
            let y_ptr = &mut 0. as *mut f64;
            let z_ptr = &mut 0. as *mut f64;
            let mut params: [VARIANT; 4] = [
                variant_from_raw_pointer::<f64>(z_ptr),
                variant_from_raw_pointer::<f64>(y_ptr),
                variant_from_raw_pointer::<f64>(x_ptr),
                VARIANT::from(node_no),
            ];
            let result_variant = invoke_method(&self.dispatch, "GetNodeIncidence", &mut params);
            match result_variant {
                Ok(v) => {
                    let result_code = VariantToInt32(&v as *const VARIANT).unwrap();
                    anyOk((
                        result_code,
                        vec![
                            *x_ptr / unit_factor,
                            *y_ptr / unit_factor,
                            *z_ptr / unit_factor,
                        ],
                    ))
                }
                Err(e) => bail!("Error::Geometry::get_node_incidence: {}", e),
            }
        }
    }

    /// Return the coordinates of the specified node.
    /// # Parameters
    /// * `[in] nNodeNo` Node number ID.
    /// * `[out] szName` (LPCTSTR) unique string ID.
    /// * `[out] fCoordX` Nodal coordinate X in GLOBAL.
    /// * `[out] fCoordY` Nodal coordinate Y in GLOBAL.
    /// * `[out] fCoordZ` Nodal coordinate Z in GLOBAL.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    /// * `-2001` Cannot find Node < nNodeNo >.
    pub fn get_node_incidence_cis2(
        &self,
        node_no: i32,
        base_unit: i32,
    ) -> Result<(i32, String, Vec<f64>), anyErr> {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }

            let name_ptr = &mut BSTR::default() as *mut BSTR;
            let x_ptr = &mut 0. as *mut f64;
            let y_ptr = &mut 0. as *mut f64;
            let z_ptr = &mut 0. as *mut f64;
            let mut params: [VARIANT; 5] = [
                variant_from_raw_pointer::<f64>(z_ptr),
                variant_from_raw_pointer::<f64>(y_ptr),
                variant_from_raw_pointer::<f64>(x_ptr),
                variant_from_raw_pointer::<BSTR>(name_ptr),
                VARIANT::from(node_no),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "GetNodeIncidence_CIS2", &mut params);
            match result_variant {
                Ok(v) => {
                    let result_code = VariantToInt32(&v as *const VARIANT).unwrap();
                    let name_bstr = &*name_ptr;
                    anyOk((
                        result_code,
                        name_bstr.to_string(),
                        vec![
                            *x_ptr / unit_factor,
                            *y_ptr / unit_factor,
                            *z_ptr / unit_factor,
                        ],
                    ))
                }
                Err(e) => bail!("Error::Geometry::get_node_incidence_cis2: {:#?}", e),
            }
        }
    }

    /// Returns the list of all the node number ID(s) in the current model.
    ///
    /// Note: List of nodes can be different depending upon the flag set for consideration of hidden entities (see OSGeometryUI::SetFlagForHiddenEntities).
    /// # Parameters
    /// * `[out] nNodeList`	VARIANT array of LONG type, for storing returned node number ID(s).
    pub fn get_node_list(&self) -> Result<Vec<i32>, anyErr> {
        unsafe {
            let node_count = self.get_node_count()?;
            let mut psa = SafeArrayCreateVector(VT_I4, 0, node_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant];
            let result_variant = invoke_method(&self.dispatch, "GetNodeList", &mut params);
            match result_variant {
                Ok(_) => {
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
                    anyOk(node_arr)
                }
                Err(e) => bail!("Error::Geometry::get_node_list: {}", e),
            }
        }
    }

    /// Returns the number ID of the node at specified coordinates.
    /// # Parameters
    /// * `[in] fCoordX` New coordinate X in GLOBAL. (Type: Double)
    /// * `[in] fCoordY` New coordinate Y in GLOBAL. (Type: Double)
    /// * `[in] fCoordZ` New coordinate Z in GLOBAL. (Type: Double)
    /// # Return values
    /// * `<Val>` Node number ID. (Type: Long)
    /// * `-2001` Cannot find Node with coordinates < fCoordX >, < fCoordY > and < fCoordZ >.
    /// * `-1` General error.
    pub fn get_node_number(
        &self,
        coord_x: f64,
        coord_y: f64,
        coord_z: f64,
        base_unit: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }
            let mut params = [
                VARIANT::from(coord_z * unit_factor),
                VARIANT::from(coord_y * unit_factor),
                VARIANT::from(coord_x * unit_factor),
            ];
            let result_variant = invoke_method(&self.dispatch, "GetNodeNumber", &mut params);
            match result_variant {
                Ok(v) => {
                    let node_num = VariantToInt32(&v as *const VARIANT).unwrap();
                    anyOk(node_num)
                }
                Err(e) => bail!("Error::Geometry::get_node_number: {}", e),
            }
        }
    }

    /// Returns the unique string ID (GUID) for specified node.
    /// # Parameters
    /// * `[in] nNodeNo` Node number ID.
    /// # Return values
    /// * `<VARIANT>` Unique string ID for specified node.
    /// * The API would return an empty string if specified node < nNodeNo > is not found
    pub fn get_node_unique_id(&self, node_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(node_no)];
            let result_variant = invoke_method(&self.dispatch, "GetNodeUniqueID", &mut params);
            match result_variant {
                Ok(v) => {
                    let result = VariantToStringAlloc(&v as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(result)
                }
                Err(e) => bail!("Error::Geometry::get_node_unique_id: {}", e),
            }
        }
    }

    /// Returns whether the specified node is orphan node or not?
    /// # Parameters
    /// * `[in]	varNodeNo` Number ID of the node.
    /// # Return values
    /// * `1` True;
    /// * `0` False;
    pub fn is_orphan_node(&self, node_no: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(node_no)];
            let result_variant = invoke_method(&self.dispatch, "IsOrphanNode", &mut params);
            match result_variant {
                Ok(v) => {
                    let result = VariantToInt32(&v as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Geometry::is_orphan_node: {}", e),
            }
        }
    }

    /// Sets or replaces the coordinate of the nNodeNo node.
    /// # Parameters
    /// * `[in]	nNodeNo` Node number ID.
    /// * `[in]	fCoordX` New coordinate X in GLOBAL.
    /// * `[in]	fCoordY` New coordinate Y in GLOBAL.
    /// * `[in]	fCoordZ` New coordinate Z in GLOBAL.
    pub fn set_node_coordinate(
        &self,
        node_no: i32,
        coord_x: f64,
        coord_y: f64,
        coord_z: f64,
        base_unit: i32,
    ) -> Result<(), anyErr> {
        unsafe {
            let mut unit_factor = 1.0;
            if base_unit == 1i32 {
                unit_factor = 39.37007874016;
            }
            let mut params: [VARIANT; 4] = [
                VARIANT::from(coord_z * unit_factor),
                VARIANT::from(coord_y * unit_factor),
                VARIANT::from(coord_x * unit_factor),
                VARIANT::from(node_no),
            ];
            let result_variant = invoke_method(&self.dispatch, "SetNodeCoordinate", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::set_node_coordinate: {}", e),
            }
        }
    }

    /// Assigns an unique string ID (GUID) to specified node.
    /// # Parameters
    /// * `[in]	nNodeNo` Number ID of the node to be assigned.
    /// * `[in]	szName` (LPCTSTR) unique string ID.
    pub fn set_node_unique_id(&self, node_no: i32, unique_id: &str) -> Result<(), anyErr> {
        unsafe {
            let no_var = VARIANT::from(node_no);
            let id_var = VARIANT::from(unique_id);

            let mut params = [id_var, no_var];
            let result_variant = invoke_method(&self.dispatch, "SetNodeUniqueID", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::set_node_unique_id: {}", e),
            }
        }
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
            let result_variant = invoke_method(&self.dispatch, "AddBeam", &mut params);
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
            let result_variant = invoke_method(&self.dispatch, "AddMultipleBeams", &mut params);
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

            let result_variant =
                invoke_method(&self.dispatch, "BreakBeamsAtSpecificNodes", &mut params);

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
            let result_variant = invoke_method(&self.dispatch, "CreateBeam", &mut params);
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
            let result_variant = invoke_method(&self.dispatch, "CreateMultipleBeams", &mut params);
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
            let result_variant = invoke_method(&self.dispatch, "DeleteBeam", &mut params);
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
            let result_variant = invoke_method(&self.dispatch, "GetBeamLength", &mut params);
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
            let result_variant = invoke_method(&self.dispatch, "GetBeamList", &mut params);
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
            let result_variant =
                invoke_method(&self.dispatch, "GetBeamsConnectedAtNode", &mut params);
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
                &self.dispatch,
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
            let result_variant =
                invoke_method(&self.dispatch, "GetIntersectBeamsCount", &mut params);
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
        let result_variant = unsafe { invoke_method(&self.dispatch, "GetLastBeamNo", &mut []) };
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
        let result_variant = unsafe { invoke_method(&self.dispatch, "GetMemberCount", &mut []) };
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

            let result_variant = invoke_method(&self.dispatch, "GetMemberIncidence", &mut params);
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

            let result_variant =
                invoke_method(&self.dispatch, "GetMemberIncidence_CIS2", &mut params);
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
            let result_variant = invoke_method(&self.dispatch, "GetMemberUniqueID", &mut params);
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
            let result_variant =
                invoke_method(&self.dispatch, "GetNoOfBeamsConnectedAtNode", &mut params);
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

            let result_variant = invoke_method(&self.dispatch, "IntersectBeams", &mut params);

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
            let result_variant = invoke_method(&self.dispatch, "IsBeam", &mut params);
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
            let result_variant = invoke_method(&self.dispatch, "IsColumn", &mut params);
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
        let result_variant = unsafe { invoke_method(&self.dispatch, "IsZUp", &mut []) };
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

            let result_variant = invoke_method(&self.dispatch, "MergeBeams", &mut params);
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
            let result_variant = invoke_method(&self.dispatch, "RenumberBeam", &mut params);
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
            let result_variant =
                invoke_method(&self.dispatch, "SetCheckForIdenticalEntity", &mut params);
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
            let result_variant = invoke_method(&self.dispatch, "SetMemberUniqueID", &mut params);
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

            let result_variant = invoke_method(&self.dispatch, "SplitBeam", &mut params);
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

            let result_variant = invoke_method(&self.dispatch, "SplitBeamInEqlParts", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::split_beam_in_equal_parts: {}", e),
            }
        }
    }
    /// Creates a group with specified name for the specified type for selected entities.
    /// # Parameters
    /// * `[in] varGroupType` Type of entities in group:
    ///   - 1: Nodes
    ///   - 2: Members
    ///   - 3: Plates
    ///   - 4: Solids
    ///   - 5: Geometry (Members, Plates and Solids)
    ///   - 6: Floor (Floor beam)
    /// * `[in] szGroupName` (LPCTSTR) String name of the group.
    /// * `[in] varEntityCount` Entity count VARIANT array.
    /// * `[in] varEntityList` Entity number ID(s) VARIANT array.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    /// * `-100` Invalid Argument.
    /// * `-110` No beam / plate / solid has been selected.
    /// * `-2005` No node has been selected.
    /// * `-3005` No member has been selected.
    /// * `-4005` No plate has been selected.
    /// * `-5005` No solid has been selected.
    /// * `-7001` Group already exists.
    pub fn create_group_ex(
        &self,
        group_type: i32,
        group_name: &str,
        entity_count: i32,
        entity_list: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_entities = safe_array_from_vec1d::<i32>(entity_list)?;
            let variant_entities = variant_from_raw_pointer::<SafeArray<i32>>(sa_entities);

            let mut params = [
                variant_entities,
                VARIANT::from(entity_count),
                VARIANT::from(group_name),
                VARIANT::from(group_type),
            ];

            let result_variant = invoke_method(&self.dispatch, "CreateGroupEx", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Geometry::create_group_ex: {}", e),
            }
        }
    }

    /// Deletes a group specified by group string name.
    /// # Parameters
    /// * `[in] szGroupName` (LPCTSTR) Group string name.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn delete_group(&self, group_name: &str) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(group_name)];
            let result_variant = invoke_method(&self.dispatch, "DeleteGroup", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Geometry::delete_group: {}", e),
            }
        }
    }

    /// Returns the number of group with specified type in the current model.
    /// # Parameters
    /// * `[in] varGroupType` Type of entities in group:
    ///   - 1: Nodes
    ///   - 2: Members
    ///   - 3: Plates
    ///   - 4: Solids
    ///   - 5: Geometry (Members, Plates and Solids)
    ///   - 6: Floor (Floor beam)
    /// # Returns
    /// * The total number of group(s).
    pub fn get_group_count(&self, group_type: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(group_type)];
            let result_variant = invoke_method(&self.dispatch, "GetGroupCount", &mut params);
            match result_variant {
                Ok(var) => {
                    let count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(count)
                }
                Err(e) => bail!("Error::Geometry::get_group_count: {}", e),
            }
        }
    }

    /// Returns the number of all group types in the current model.
    /// # Returns
    /// * The total number of group(s).
    pub fn get_group_count_all(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe { invoke_method(&self.dispatch, "GetGroupCountAll", &mut []) };
        match result_variant {
            Ok(var) => {
                let count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(count)
            }
            Err(e) => bail!("Error::Geometry::get_group_count_all: {}", e),
        }
    }

    /// A function to obtain the all entities in a certain group.
    /// # Parameters
    /// * `[in] szGroupName` (LPCTSTR) Group string name.
    /// * `[out] varEntityList` Entity number ID(s) VARIANT array.
    /// # Returns
    /// * The total number of entities in specified group.
    /// # Return values
    /// * `-1` General error.
    /// * `-107` Array of integer expected.
    pub fn get_group_entities(&self, group_name: &str) -> Result<(i32, Vec<i32>), anyErr> {
        unsafe {
            let entity_count = self.get_group_entity_count(group_name)?;
            if entity_count <= 0 {
                return anyOk((entity_count, Vec::new()));
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, entity_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant, VARIANT::from(group_name)];
            let result_variant = invoke_method(&self.dispatch, "GetGroupEntities", &mut params);
            match result_variant {
                Ok(var) => {
                    let total_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    let entity_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut entity_arr = Vec::with_capacity(total_count as usize);

                    for i in 0..total_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            entity_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        entity_arr.push(value as i32);
                    }
                    anyOk((total_count, entity_arr))
                }
                Err(e) => bail!("Error::Geometry::get_group_entities: {}", e),
            }
        }
    }

    /// Returns the total number of entities in certain group.
    /// # Parameters
    /// * `[in] szGroupName` (LPCTSTR) Group string name.
    /// # Returns
    /// * The total number of entities in specified group.
    pub fn get_group_entity_count(&self, group_name: &str) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(group_name)];
            let result_variant = invoke_method(&self.dispatch, "GetGroupEntityCount", &mut params);
            match result_variant {
                Ok(var) => {
                    let count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(count)
                }
                Err(e) => bail!("Error::Geometry::get_group_entity_count: {}", e),
            }
        }
    }

    /// Returns the list of string name of group(s) with specified group type in current model.
    /// # Parameters
    /// * `[in] varGroupType` Type of entities in group:
    ///   - 1: Nodes
    ///   - 2: Members
    ///   - 3: Plates
    ///   - 4: Solids
    ///   - 5: Geometry (Members, Plates and Solids)
    ///   - 6: Floor (Floor beam)
    /// * `[out] szGroupNameList` Group string name VARIANT array.
    /// # Return values
    /// * `0` OK.
    /// * `-107` Array of string expected.
    pub fn get_group_names(&self, group_type: i32) -> Result<(i32, Vec<String>), anyErr> {
        unsafe {
            let group_count = self.get_group_count(group_type)?;
            if group_count <= 0 {
                return anyOk((0, Vec::new()));
            }

            let mut psa = SafeArrayCreateVector(VT_BSTR, 0, group_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<BSTR>>(psa_ptr);

            let mut params = [variant, VARIANT::from(group_type)];
            let result_variant = invoke_method(&self.dispatch, "GetGroupNames", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    let name_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut name_arr = Vec::with_capacity(group_count as usize);

                    for i in 0..group_count {
                        let mut index = i as i32;
                        let mut bstr_value = BSTR::default();
                        let _ = SafeArrayGetElement(
                            name_safe_arr,
                            &mut index as *mut i32,
                            &mut bstr_value as *mut BSTR as *mut c_void,
                        )?;
                        name_arr.push(bstr_value.to_string());
                    }
                    anyOk((result_code, name_arr))
                }
                Err(e) => bail!("Error::Geometry::get_group_names: {}", e),
            }
        }
    }

    /// Updates (replaces, removes, adds) entities to a specified group.
    /// # Parameters
    /// * `[in] szGroupName` (LPCTSTR) Group string name.
    /// * `[in] varFlag` Option for operation:
    ///   - 0: replace the group entities with a array of entities
    ///   - 1: remove entities from this group
    ///   - 2: add entities to this group
    /// * `[in] varEntityCount` Entity count VARIANT array.
    /// * `[in] varEntityList` Entity number ID(s) VARIANT array.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    /// * `-107` Array of integer expected.
    pub fn update_group(
        &self,
        group_name: &str,
        flag: i32,
        entity_count: i32,
        entity_list: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_entities = safe_array_from_vec1d::<i32>(entity_list)?;
            let variant_entities = variant_from_raw_pointer::<SafeArray<i32>>(sa_entities);

            let mut params = [
                variant_entities,
                VARIANT::from(entity_count),
                VARIANT::from(flag),
                VARIANT::from(group_name),
            ];

            let result_variant = invoke_method(&self.dispatch, "UpdateGroup", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Geometry::update_group: {}", e),
            }
        }
    }
}

// :: Node
// AddMultipleNodes
// AddNode
// CreateMultipleNodes
// CreateNode
// DeleteNode
// GetLastNodeNo
// GetNodeCoordinates
// GetNodeCount
// GetNodeDistance
// GetNodeIncidence
// GetNodeIncidence_CIS2
// GetNodeList
// GetNodeNumber
// GetNodeUniqueId
// IsOrphanNode
// SetNodeCoordinate
// SetNodeUniqueId
// :: Beam
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
// :: Group
// CreateGroupEx
// DeleteGroup
// GetGroupCount
// GetGroupCountAll
// GetGroupEntities
// GetGroupEntityCount
// GetGroupNames
// UpdateGroup
