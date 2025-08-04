use crate::openstaad::{
    geometry::root::Geometry,
    tools::safe_array::safe_array_from_vec1d,
    tools::invoke::invoke_method,
    tools::variant::{SafeArray, SafeArrayP, variant_from_raw_pointer},
};

use anyhow::{Error as anyErr, Ok as anyOk, Result, bail};
use std::ffi::c_void;
use windows::Win32::System::{
    Com::SAFEARRAY,
    Ole::{SafeArrayCreateVector, SafeArrayGetElement},
    Variant::{VariantToInt32, VARIANT, VT_BSTR, VT_I4},
};
use windows_core::BSTR;

// CreateGroupEx
// DeleteGroup
// GetGroupCount
// GetGroupCountAll
// GetGroupEntities
// GetGroupEntityCount
// GetGroupNames
// UpdateGroup

#[derive(Debug)]
pub struct Group<'a> {
    pub geometry: &'a Geometry<'a>,
}

impl<'a> Group<'a> {
    pub fn new(geometry: &'a Geometry<'a>) -> Self {
        Self { geometry }
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

            let result_variant =
                invoke_method(&self.geometry.dispatch, "CreateGroupEx", &mut params);
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
            let result_variant =
                invoke_method(&self.geometry.dispatch, "DeleteGroup", &mut params);
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
            let result_variant =
                invoke_method(&self.geometry.dispatch, "GetGroupCount", &mut params);
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
        let result_variant = unsafe {
            invoke_method(&self.geometry.dispatch, "GetGroupCountAll", &mut [])
        };
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
            let result_variant =
                invoke_method(&self.geometry.dispatch, "GetGroupEntities", &mut params);
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
            let result_variant = invoke_method(
                &self.geometry.dispatch,
                "GetGroupEntityCount",
                &mut params,
            );
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
            let result_variant =
                invoke_method(&self.geometry.dispatch, "GetGroupNames", &mut params);
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

            let result_variant =
                invoke_method(&self.geometry.dispatch, "UpdateGroup", &mut params);
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
