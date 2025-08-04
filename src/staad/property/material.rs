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
    Variant::{VARIANT, VT_I4, VT_R8, VariantToDouble, VariantToInt32, VariantToStringAlloc},
};
use windows_core::BSTR;

// :: Create Material Information
// CreateIsotropicMaterialAluminum
// CreateIsotropicMaterialConcrete
// CreateIsotropicMaterialProperties
// CreateIsotropicMaterialPropertiesEx
// CreateIsotropicMaterialSteel
// CreateIsotropicMaterialTimber
// :: Get and Remove Material
// DeleteMaterial
// GetBeamMaterialName
// GetElementMaterialName
// GetIsotropicMaterialAssignedBeamCount
// GetIsotropicMaterialAssignedBeamList
// GetIsotropicMaterialAssignedPlateCount
// GetIsotropicMaterialAssignedSolidCount
// GetIsotropicMaterialAssignedSolidList
// GetIsotropicMaterialCount
// GetIsotropicMaterialProperties
// GetIsotropicMaterialPropertiesAssigned
// GetIsotropicMaterialPropertiesEx
// GetMaterialProperty
// GetMaterialPropertyEx
// GetOrthotropic2DMaterialCount
// GetOrthotropic2DMaterialProperties
// GetOrthotropic3DMaterialCount
// GetOrthotropic3DMaterialProperties
// GetPlateMaterialName
// GetSolidMaterialName
// GetTypeForIsotropicMaterial
// RemoveBeamMaterialHelper
// RemoveMaterialFromBeam
// SetTypeToIsotropicMaterial
// :: Assign Material to Section and Element
// AssignMaterialToMember
// AssignMaterialToPlate
// AssignMaterialToSolid
// SetMaterialID
// SetMaterialName

#[derive(Debug)]
pub struct Material<'a> {
    pub property: &'a Property<'a>,
}

impl<'a> Material<'a> {
    pub fn new(property: &'a Property<'a>) -> Self {
        Self { property }
    }

    /// Creates isotropic material aluminum.
    /// # Parameters
    /// * `[in] strName` Identification title of material.
    /// * `[in] dE` Modulus of elasticity (E).
    /// * `[in] dPoisson` Poisson's ratio (POI).
    /// * `[in] dG` Shear modulus (G).
    /// * `[in] dDensity` Weight density (DEN).
    /// * `[in] dAlpha` Coefficient of thermal expansion (ALP).
    /// * `[in] dCrDamp` Damping ratio (DAMP).
    /// * `[in] bPhysical` Identifies if the material is for physical member.
    /// # Return values
    /// * `1` Material is updated as a material with that name was already present.
    /// * `0` Material is created.
    /// * `-1` General Error
    pub fn create_isotropic_material_aluminum(
        &self,
        name: &str,
        e: f64,
        poisson: f64,
        g: f64,
        density: f64,
        alpha: f64,
        cr_damp: f64,
        physical: bool,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(physical as i32),
                VARIANT::from(cr_damp),
                VARIANT::from(alpha),
                VARIANT::from(density),
                VARIANT::from(g),
                VARIANT::from(poisson),
                VARIANT::from(e),
                VARIANT::from(name),
            ];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreateIsotropicMaterialAluminum",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_isotropic_material_aluminum: {}", e),
            }
        }
    }

    /// Creates isotropic material concrete.
    /// # Parameters
    /// * `[in] strName` Identification title of material.
    /// * `[in] dE` Modulus of elasticity (E).
    /// * `[in] dPoisson` Poisson's ratio (POI).
    /// * `[in] dG` Shear modulus (G).
    /// * `[in] dDensity` Weight density (DEN).
    /// * `[in] dAlpha` Coefficient of thermal expansion (ALP).
    /// * `[in] dCrDamp` Damping ratio (DAMP).
    /// * `[in] dfcu` Compressive strength (Fcu).
    /// * `[in] bPhysical` Identifies if the material is for physical member.
    /// # Return values
    /// * `1` Material is updated as a material with that name was already present.
    /// * `0` Material is created.
    /// * `-1` General Error
    pub fn create_isotropic_material_concrete(
        &self,
        name: &str,
        e: f64,
        poisson: f64,
        g: f64,
        density: f64,
        alpha: f64,
        cr_damp: f64,
        fcu: f64,
        physical: bool,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(physical as i32),
                VARIANT::from(fcu),
                VARIANT::from(cr_damp),
                VARIANT::from(alpha),
                VARIANT::from(density),
                VARIANT::from(g),
                VARIANT::from(poisson),
                VARIANT::from(e),
                VARIANT::from(name),
            ];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreateIsotropicMaterialConcrete",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_isotropic_material_concrete: {}", e),
            }
        }
    }

    /// Creates isotropic material properties.
    /// # Parameters
    /// * `[in] strName` Material Name.
    /// * `[out] varE` Modulus of elasticity (E) VARIANT array (of size 3).
    /// * `[out] varPoisson` Poisson's ratio (POI) VARIANT array (of size 3).
    /// * `[out] varG` Shear modulus (G) VARIANT array (of size 3).
    /// * `[out] varDensity` Weight density (DEN) VARIANT array (of size 3).
    /// * `[out] varAlpha` Coefficient of thermal expansion (ALP) VARIANT array (of size 3).
    /// * `[out] varCrDamp` Damping ratio (DAMP) VARIANT array (of size 3).
    /// # Return values
    /// * `1` Material is updated as a material with that name was already present.
    /// * `0` Material is created.
    /// * `-1` General Error
    pub fn create_isotropic_material_properties(
        &self,
        name: &str,
        e: f64,
        poisson: f64,
        g: f64,
        density: f64,
        alpha: f64,
        cr_damp: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(cr_damp),
                VARIANT::from(alpha),
                VARIANT::from(density),
                VARIANT::from(g),
                VARIANT::from(poisson),
                VARIANT::from(e),
                VARIANT::from(name),
            ];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreateIsotropicMaterialProperties",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::create_isotropic_material_properties: {}",
                    e
                ),
            }
        }
    }

    /// Creates isotropic material property extended.
    /// # Parameters
    /// * `[in] strName` Identification title of material.
    /// * `[in] dE` Modulus of elasticity (E).
    /// * `[in] dPoisson` Poisson's ratio (POI).
    /// * `[in] dG` Shear modulus (G).
    /// * `[in] dDensity` Weight density (DEN).
    /// * `[in] dAlpha` Coefficient of thermal expansion (ALP).
    /// * `[in] dCrDamp` Damping ratio (DAMP).
    /// * `[in] dFy` Yield stress (Fy).
    /// * `[in] dFu` Tensile strength (Fu).
    /// * `[in] dRy` Yield strength ratio (Ry).
    /// * `[in] dRt` Tensile strength ratio (Rt).
    /// * `[in] dFcu` Compressive strength (Fcu).
    /// # Return values
    /// * `1` Material is updated as a material with that name was already present.
    /// * `0` Material is created.
    /// * `-1` General Error
    pub fn create_isotropic_material_properties_ex(
        &self,
        name: &str,
        e: f64,
        poisson: f64,
        g: f64,
        density: f64,
        alpha: f64,
        cr_damp: f64,
        fy: f64,
        fu: f64,
        ry: f64,
        rt: f64,
        fcu: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(fcu),
                VARIANT::from(rt),
                VARIANT::from(ry),
                VARIANT::from(fu),
                VARIANT::from(fy),
                VARIANT::from(cr_damp),
                VARIANT::from(alpha),
                VARIANT::from(density),
                VARIANT::from(g),
                VARIANT::from(poisson),
                VARIANT::from(e),
                VARIANT::from(name),
            ];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreateIsotropicMaterialPropertiesEx",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::create_isotropic_material_properties_ex: {}",
                    e
                ),
            }
        }
    }

    /// Creates isotropic material steel.
    /// # Parameters
    /// * `[in] strName` Identification title of material.
    /// * `[in] dE` Modulus of elasticity (E).
    /// * `[in] dPoisson` Poisson's ratio (POI).
    /// * `[in] dG` Shear modulus (G).
    /// * `[in] dDensity` Weight density (DEN).
    /// * `[in] dAlpha` Coefficient of thermal expansion (ALP).
    /// * `[in] dCrDamp` Damping ratio (DAMP).
    /// * `[in] dfu` Tensile strength (Fu).
    /// * `[in] dfy` Yield stress (Fy).
    /// * `[in] drt` Tensile strength ratio (Rt).
    /// * `[in] dry` Yield strength ratio (Ry).
    /// * `[in] bPhysical` Identifies if the material is for physical member.
    /// # Return values
    /// * `1` Material is updated as a material with that name was already present.
    /// * `0` Material is created.
    /// * `-1` General Error
    pub fn create_isotropic_material_steel(
        &self,
        name: &str,
        e: f64,
        poisson: f64,
        g: f64,
        density: f64,
        alpha: f64,
        cr_damp: f64,
        fu: f64,
        fy: f64,
        rt: f64,
        ry: f64,
        physical: bool,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(physical as i32),
                VARIANT::from(ry),
                VARIANT::from(rt),
                VARIANT::from(fy),
                VARIANT::from(fu),
                VARIANT::from(cr_damp),
                VARIANT::from(alpha),
                VARIANT::from(density),
                VARIANT::from(g),
                VARIANT::from(poisson),
                VARIANT::from(e),
                VARIANT::from(name),
            ];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreateIsotropicMaterialSteel",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_isotropic_material_steel: {}", e),
            }
        }
    }

    /// Creates isotropic material timber.
    /// # Parameters
    /// * `[in] strName` Identification title of material.
    /// * `[in] dE` Modulus of elasticity (E).
    /// * `[in] dPoisson` Poisson's ratio (POI).
    /// * `[in] dG` Shear modulus (G).
    /// * `[in] dDensity` Weight density (DEN).
    /// * `[in] dAlpha` Coefficient of thermal expansion (ALP).
    /// * `[in] dCrDamp` Damping ratio (DAMP).
    /// * `[in] bPhysical` Identifies if the material is for physical member.
    /// # Return values
    /// * `1` Material is updated as a material with that name was already present.
    /// * `0` Material is created.
    /// * `-1` General Error
    pub fn create_isotropic_material_timber(
        &self,
        name: &str,
        e: f64,
        poisson: f64,
        g: f64,
        density: f64,
        alpha: f64,
        cr_damp: f64,
        physical: bool,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(physical as i32),
                VARIANT::from(cr_damp),
                VARIANT::from(alpha),
                VARIANT::from(density),
                VARIANT::from(g),
                VARIANT::from(poisson),
                VARIANT::from(e),
                VARIANT::from(name),
            ];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreateIsotropicMaterialTimber",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_isotropic_material_timber: {}", e),
            }
        }
    }

    /// Delete Material.
    /// # Parameters
    /// * `[in] varMaterialName` Material Name (Type: String).
    /// # Return values
    /// * `FALSE` Delete Material Generate Error.
    /// * `TRUE` Delete Material Successful.
    pub fn delete_material(&self, material_name: &str) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(material_name)];
            let result_variant =
                invoke_method_with_result(&self.property.dispatch, "DeleteMaterial", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::delete_material: {}", e),
            }
        }
    }

    /// Get beam material string name.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Returns
    /// * The beam material string name.
    pub fn get_beam_material_name(&self, beam_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetBeamMaterialName",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(result)
                }
                Err(e) => bail!("Error::Property::get_beam_material_name: {}", e),
            }
        }
    }

    /// Get entity material string name.
    /// # Parameters
    /// * `[in] varnPlateNo` The plate number ID.
    /// # Returns
    /// * The entity material string name.
    pub fn get_element_material_name(&self, plate_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(plate_no)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetElementMaterialName",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(result)
                }
                Err(e) => bail!("Error::Property::get_element_material_name: {}", e),
            }
        }
    }

    /// Get isotropic material assigned beam count.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of the material.
    /// # Return values
    /// * `0` OK.
    /// * `-6023` Material not found.
    pub fn get_isotropic_material_assigned_beam_count(
        &self,
        material_name: &str,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(material_name)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetIsotropicMaterialAssignedBeamCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_assigned_beam_count: {}",
                    e
                ),
            }
        }
    }

    /// Get isotropic material assigned beam list.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of the material.
    /// * `[out] nBeamList` List of beam.
    /// # Return values
    /// * `0` OK.
    /// * `-6023` Material not found.
    pub fn get_isotropic_material_assigned_beam_list(
        &self,
        material_name: &str,
    ) -> Result<Vec<i32>, anyErr> {
        unsafe {
            let beam_count = self.get_isotropic_material_assigned_beam_count(material_name)?;
            if beam_count <= 0 {
                return anyOk(Vec::new());
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, beam_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant, VARIANT::from(material_name)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetIsotropicMaterialAssignedBeamList",
                &mut params,
            );
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
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_assigned_beam_list: {}",
                    e
                ),
            }
        }
    }

    /// Get the count of plates assigned with the specific isotropic material.
    /// # Parameters
    /// * `[in] strMaterialName` Material Name (Type: String).
    /// # Returns
    /// * Count of plates assigned with the specific isotropic material (Type: Long).
    pub fn get_isotropic_material_assigned_plate_count(
        &self,
        material_name: &str,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(material_name)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetIsotropicMaterialAssignedPlateCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(count)
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_assigned_plate_count: {}",
                    e
                ),
            }
        }
    }

    /// Get the count of solids assigned with the specified isotropic material.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of the material.
    /// # Returns
    /// * Count of solids assigned with the specified isotropic material (Type: Long).
    pub fn get_isotropic_material_assigned_solid_count(
        &self,
        material_name: &str,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(material_name)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetIsotropicMaterialAssignedSolidCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(count)
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_assigned_solid_count: {}",
                    e
                ),
            }
        }
    }

    /// Get isotropic material assigned solid list.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of the material (type: string).
    /// * `[out] nSolidList` List of solid (type: long array).
    /// # Return values
    /// * `true` Get solid list successful.
    /// * `false` Generate Error.
    pub fn get_isotropic_material_assigned_solid_list(
        &self,
        material_name: &str,
    ) -> Result<Vec<i32>, anyErr> {
        unsafe {
            let solid_count = self.get_isotropic_material_assigned_solid_count(material_name)?;
            if solid_count <= 0 {
                return anyOk(Vec::new());
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, solid_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant, VARIANT::from(material_name)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetIsotropicMaterialAssignedSolidList",
                &mut params,
            );
            match result_variant {
                Ok(_) => {
                    let solid_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut solid_arr = Vec::with_capacity(solid_count as usize);
                    for i in 0..solid_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            solid_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        solid_arr.push(value as i32);
                    }
                    anyOk(solid_arr)
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_assigned_solid_list: {}",
                    e
                ),
            }
        }
    }

    /// Get the number of isotropic material present in the current structure.
    /// # Return values
    /// * `<Val>` The number of isotropic material.
    pub fn get_isotropic_material_count(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method_with_result(
                &self.property.dispatch,
                "GetIsotropicMaterialCount",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(count)
            }
            Err(e) => bail!("Error::Property::get_isotropic_material_count: {}", e),
        }
    }

    /// Get the properties for the specified isotropic material number.
    /// # Parameters
    /// * `[in] varMatNo` Zero based index of the material.
    /// * `[out] varE` Modulus of elasticity (E).
    /// * `[out] varPoisson` Poisson's ratio (POI).
    /// * `[out] varG` Shear modulus (G).
    /// * `[out] varDensity` Weight density (DEN).
    /// * `[out] varAlpha` Coefficient of thermal expansion (ALP).
    /// * `[out] varCrDamp` Damping ratio (DAMP).
    /// # Return values
    /// * `VARIANT` Material string name.
    /// * `NULL` Cannot find material varMatNo.
    pub fn get_isotropic_material_properties(
        &self,
        mat_no: i32,
    ) -> Result<(String, f64, f64, f64, f64, f64, f64), anyErr> {
        unsafe {
            let e_ptr = &mut 0.0f64 as *mut f64;
            let poisson_ptr = &mut 0.0f64 as *mut f64;
            let g_ptr = &mut 0.0f64 as *mut f64;
            let density_ptr = &mut 0.0f64 as *mut f64;
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let cr_damp_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(cr_damp_ptr),
                variant_from_raw_pointer::<f64>(alpha_ptr),
                variant_from_raw_pointer::<f64>(density_ptr),
                variant_from_raw_pointer::<f64>(g_ptr),
                variant_from_raw_pointer::<f64>(poisson_ptr),
                variant_from_raw_pointer::<f64>(e_ptr),
                VARIANT::from(mat_no),
            ];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetIsotropicMaterialProperties",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let material_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk((
                        material_name,
                        *e_ptr,
                        *poisson_ptr,
                        *g_ptr,
                        *density_ptr,
                        *alpha_ptr,
                        *cr_damp_ptr,
                    ))
                }
                Err(e) => bail!("Error::Property::get_isotropic_material_properties: {}", e),
            }
        }
    }

    /// Gets isotropic material properties and if material assigned to element(s) or not.
    /// # Parameters
    /// * `[in] varMatNo` Material number ID.
    /// * `[out] varE` Modulus of elasticity (E).
    /// * `[out] varPoisson` Poisson's ratio (POI).
    /// * `[out] varG` Shear modulus (G).
    /// * `[out] varDensity` Weight density (DEN).
    /// * `[out] varAlpha` Coefficient of thermal expansion (ALP).
    /// * `[out] varCrDamp` Damping ratio (DAMP).
    /// * `[out] varAssigned` Material assigned to elements or not: unassigned (= 1), assigned (=2).
    /// # Return values
    /// * `VARIANT` Material string name.
    /// * `NULL` Cannot find material varMatNo.
    pub fn get_isotropic_material_properties_assigned(
        &self,
        mat_no: i32,
    ) -> Result<(String, f64, f64, f64, f64, f64, f64, i32), anyErr> {
        unsafe {
            let e_ptr = &mut 0.0f64 as *mut f64;
            let poisson_ptr = &mut 0.0f64 as *mut f64;
            let g_ptr = &mut 0.0f64 as *mut f64;
            let density_ptr = &mut 0.0f64 as *mut f64;
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let cr_damp_ptr = &mut 0.0f64 as *mut f64;
            let assigned_ptr = &mut 0i32 as *mut i32;

            let mut params = [
                variant_from_raw_pointer::<i32>(assigned_ptr),
                variant_from_raw_pointer::<f64>(cr_damp_ptr),
                variant_from_raw_pointer::<f64>(alpha_ptr),
                variant_from_raw_pointer::<f64>(density_ptr),
                variant_from_raw_pointer::<f64>(g_ptr),
                variant_from_raw_pointer::<f64>(poisson_ptr),
                variant_from_raw_pointer::<f64>(e_ptr),
                VARIANT::from(mat_no),
            ];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetIsotropicMaterialPropertiesAssigned",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let material_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk((
                        material_name,
                        *e_ptr,
                        *poisson_ptr,
                        *g_ptr,
                        *density_ptr,
                        *alpha_ptr,
                        *cr_damp_ptr,
                        *assigned_ptr,
                    ))
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_properties_assigned: {}",
                    e
                ),
            }
        }
    }

    /// Get the properties for the specified isotropic material number.
    /// # Parameters
    /// * `[in] varMatNo` Zero based index of the material (Type: Long).
    /// * Extended properties including strength parameters.
    /// # Returns
    /// * The specified material Name and all properties.
    pub fn get_isotropic_material_properties_ex(
        &self,
        mat_no: i32,
    ) -> Result<
        (
            String,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
        ),
        anyErr,
    > {
        unsafe {
            let e_ptr = &mut 0.0f64 as *mut f64;
            let poisson_ptr = &mut 0.0f64 as *mut f64;
            let g_ptr = &mut 0.0f64 as *mut f64;
            let density_ptr = &mut 0.0f64 as *mut f64;
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let cr_damp_ptr = &mut 0.0f64 as *mut f64;
            let fy_ptr = &mut 0.0f64 as *mut f64;
            let fu_ptr = &mut 0.0f64 as *mut f64;
            let ry_ptr = &mut 0.0f64 as *mut f64;
            let rt_ptr = &mut 0.0f64 as *mut f64;
            let fcu_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(fcu_ptr),
                variant_from_raw_pointer::<f64>(rt_ptr),
                variant_from_raw_pointer::<f64>(ry_ptr),
                variant_from_raw_pointer::<f64>(fu_ptr),
                variant_from_raw_pointer::<f64>(fy_ptr),
                variant_from_raw_pointer::<f64>(cr_damp_ptr),
                variant_from_raw_pointer::<f64>(alpha_ptr),
                variant_from_raw_pointer::<f64>(density_ptr),
                variant_from_raw_pointer::<f64>(g_ptr),
                variant_from_raw_pointer::<f64>(poisson_ptr),
                variant_from_raw_pointer::<f64>(e_ptr),
                VARIANT::from(mat_no),
            ];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetIsotropicMaterialPropertiesEx",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let material_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk((
                        material_name,
                        *e_ptr,
                        *poisson_ptr,
                        *g_ptr,
                        *density_ptr,
                        *alpha_ptr,
                        *cr_damp_ptr,
                        *fy_ptr,
                        *fu_ptr,
                        *ry_ptr,
                        *rt_ptr,
                        *fcu_ptr,
                    ))
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_properties_ex: {}",
                    e
                ),
            }
        }
    }

    /// Get material constants based on specific material name.
    /// # Parameters
    /// * `[in] varstrMaterialName` Identification title of the material.
    /// * Material properties as output parameters.
    /// # Return values
    /// * `0` OK.
    /// * `-6023` Material not found.
    pub fn get_material_property(
        &self,
        material_name: &str,
    ) -> Result<(i32, f64, f64, f64, f64, f64), anyErr> {
        unsafe {
            let elasticity_ptr = &mut 0.0f64 as *mut f64;
            let poisson_ptr = &mut 0.0f64 as *mut f64;
            let density_ptr = &mut 0.0f64 as *mut f64;
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let damp_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(damp_ptr),
                variant_from_raw_pointer::<f64>(alpha_ptr),
                variant_from_raw_pointer::<f64>(density_ptr),
                variant_from_raw_pointer::<f64>(poisson_ptr),
                variant_from_raw_pointer::<f64>(elasticity_ptr),
                VARIANT::from(material_name),
            ];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetMaterialProperty",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((
                        result_code,
                        *elasticity_ptr,
                        *poisson_ptr,
                        *density_ptr,
                        *alpha_ptr,
                        *damp_ptr,
                    ))
                }
                Err(e) => bail!("Error::Property::get_material_property: {}", e),
            }
        }
    }

    /// Get the properties for the specified isotropic material Name.
    /// # Parameters
    /// * `[in] varstrMaterial` Name material name (type: string).
    /// * Extended material properties as output parameters.
    /// # Return values
    /// * `TRUE` Get Material Property Successful.
    /// * `FALSE` Get Material Property Failed.
    pub fn get_material_property_ex(
        &self,
        material_name: &str,
    ) -> Result<(bool, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64), anyErr> {
        unsafe {
            let elasticity_ptr = &mut 0.0f64 as *mut f64;
            let poisson_ptr = &mut 0.0f64 as *mut f64;
            let density_ptr = &mut 0.0f64 as *mut f64;
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let damp_ptr = &mut 0.0f64 as *mut f64;
            let fy_ptr = &mut 0.0f64 as *mut f64;
            let fu_ptr = &mut 0.0f64 as *mut f64;
            let ry_ptr = &mut 0.0f64 as *mut f64;
            let rt_ptr = &mut 0.0f64 as *mut f64;
            let fcu_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(fcu_ptr),
                variant_from_raw_pointer::<f64>(rt_ptr),
                variant_from_raw_pointer::<f64>(ry_ptr),
                variant_from_raw_pointer::<f64>(fu_ptr),
                variant_from_raw_pointer::<f64>(fy_ptr),
                variant_from_raw_pointer::<f64>(damp_ptr),
                variant_from_raw_pointer::<f64>(alpha_ptr),
                variant_from_raw_pointer::<f64>(density_ptr),
                variant_from_raw_pointer::<f64>(poisson_ptr),
                variant_from_raw_pointer::<f64>(elasticity_ptr),
                VARIANT::from(material_name),
            ];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetMaterialPropertyEx",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((
                        result_code == 1,
                        *elasticity_ptr,
                        *poisson_ptr,
                        *density_ptr,
                        *alpha_ptr,
                        *damp_ptr,
                        *fy_ptr,
                        *fu_ptr,
                        *ry_ptr,
                        *rt_ptr,
                        *fcu_ptr,
                    ))
                }
                Err(e) => bail!("Error::Property::get_material_property_ex: {}", e),
            }
        }
    }

    /// Return the number of 2D orthotropic material present in the current structure.
    /// # Return values
    /// * `<Val>` The number of 2D orthotropic material.
    pub fn get_orthotropic_2d_material_count(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method_with_result(
                &self.property.dispatch,
                "GetOrthotropic2DMaterialCount",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(count)
            }
            Err(e) => bail!("Error::Property::get_orthotropic_2d_material_count: {}", e),
        }
    }

    /// Get the properties for the specified 2D orthotropic material.
    /// # Parameters
    /// * `[in] varMatNo` Material number ID.
    /// * Array properties as output parameters.
    /// # Return values
    /// * `VARIANT` Material string name.
    /// * `NULL` Fail to get the properties.
    pub fn get_orthotropic_2d_material_properties(
        &self,
        mat_no: i32,
    ) -> Result<
        (
            String,
            Vec<f64>,
            Vec<f64>,
            Vec<f64>,
            Vec<f64>,
            Vec<f64>,
            Vec<f64>,
        ),
        anyErr,
    > {
        unsafe {
            let mut e_array = vec![0.0f64; 2];
            let mut poisson_array = vec![0.0f64; 2];
            let mut g_array = vec![0.0f64; 3];
            let mut density_array = vec![0.0f64; 2];
            let mut alpha_array = vec![0.0f64; 2];
            let mut cr_damp_array = vec![0.0f64; 2];

            let sa_e = SafeArrayCreateVector(VT_R8, 0, 2);
            let sa_poisson = SafeArrayCreateVector(VT_R8, 0, 2);
            let sa_g = SafeArrayCreateVector(VT_R8, 0, 3);
            let sa_density = SafeArrayCreateVector(VT_R8, 0, 2);
            let sa_alpha = SafeArrayCreateVector(VT_R8, 0, 2);
            let sa_cr_damp = SafeArrayCreateVector(VT_R8, 0, 2);

            let mut params = [
                variant_from_raw_pointer::<SafeArray<f64>>(sa_cr_damp),
                variant_from_raw_pointer::<SafeArray<f64>>(sa_alpha),
                variant_from_raw_pointer::<SafeArray<f64>>(sa_density),
                variant_from_raw_pointer::<SafeArray<f64>>(sa_g),
                variant_from_raw_pointer::<SafeArray<f64>>(sa_poisson),
                variant_from_raw_pointer::<SafeArray<f64>>(sa_e),
                VARIANT::from(mat_no),
            ];

            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetOrthotropic2DMaterialProperties",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let material_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;

                    // Extract arrays from SafeArrays
                    for i in 0..2 {
                        let mut index = i as i32;
                        let mut value = 0.0f64;
                        let _ = SafeArrayGetElement(
                            sa_e,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        e_array[i] = value;
                        let _ = SafeArrayGetElement(
                            sa_poisson,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        poisson_array[i] = value;
                        let _ = SafeArrayGetElement(
                            sa_density,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        density_array[i] = value;
                        let _ = SafeArrayGetElement(
                            sa_alpha,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        alpha_array[i] = value;
                        let _ = SafeArrayGetElement(
                            sa_cr_damp,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        cr_damp_array[i] = value;
                    }

                    for i in 0..3 {
                        let mut index = i as i32;
                        let mut value = 0.0f64;
                        let _ = SafeArrayGetElement(
                            sa_g,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        g_array[i] = value;
                    }

                    anyOk((
                        material_name,
                        e_array,
                        poisson_array,
                        g_array,
                        density_array,
                        alpha_array,
                        cr_damp_array,
                    ))
                }
                Err(e) => bail!(
                    "Error::Property::get_orthotropic_2d_material_properties: {}",
                    e
                ),
            }
        }
    }
    /// Assign material to member.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of material.
    /// * `[in] varMemberNo` Single or an array of integers containing list of member nos.
    /// # Return values
    /// * `1` TRUE.
    /// * `0` FALSE.
    pub fn assign_material_to_member(
        &self,
        material_name: &str,
        member_nos: Vec<i32>,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_members = safe_array_from_vec1d::<i32>(member_nos)?;
            let variant_members = variant_from_raw_pointer::<SafeArray<i32>>(sa_members);

            let mut params = [variant_members, VARIANT::from(material_name)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AssignMaterialToMember",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::assign_material_to_member: {}", e),
            }
        }
    }

    /// Assign material to single member.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of material.
    /// * `[in] member_no` Single member number.
    /// # Return values
    /// * `1` TRUE.
    /// * `0` FALSE.
    pub fn assign_material_to_member_single(
        &self,
        material_name: &str,
        member_no: i32,
    ) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(member_no), VARIANT::from(material_name)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AssignMaterialToMember",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::assign_material_to_member_single: {}", e),
            }
        }
    }

    /// Assign material to plate. API will skip the plate numbers which are not found.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of material. (String)
    /// * `[in] varPlateNo` Single or an array of integers containing list of plate nos. (Long / Integer)
    /// # Return values
    /// * `0` OK.
    /// * `-113` varPlateNos type error (Long or Int Expected)
    /// * `-4009` All the plate numbers are invalid.
    /// * `-4008` Some of the plate numbers are invalid.
    /// * `-6023` Material not found.
    pub fn assign_material_to_plate(
        &self,
        material_name: &str,
        plate_nos: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_plates = safe_array_from_vec1d::<i32>(plate_nos)?;
            let variant_plates = variant_from_raw_pointer::<SafeArray<i32>>(sa_plates);

            let mut params = [variant_plates, VARIANT::from(material_name)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AssignMaterialToPlate",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::assign_material_to_plate: {}", e),
            }
        }
    }

    /// Assign material to single plate.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of material.
    /// * `[in] plate_no` Single plate number.
    /// # Return values
    /// * `0` OK.
    /// * `-113` plate type error (Long or Int Expected)
    /// * `-6023` Material not found.
    pub fn assign_material_to_plate_single(
        &self,
        material_name: &str,
        plate_no: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(plate_no), VARIANT::from(material_name)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AssignMaterialToPlate",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::assign_material_to_plate_single: {}", e),
            }
        }
    }

    /// Assign material to solid.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of material.
    /// * `[in] varSolidNo` Single or an array of integers contains list of solid nos.
    /// # Return values
    /// * `1` TRUE.
    /// * `0` FALSE.
    pub fn assign_material_to_solid(
        &self,
        material_name: &str,
        solid_nos: Vec<i32>,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_solids = safe_array_from_vec1d::<i32>(solid_nos)?;
            let variant_solids = variant_from_raw_pointer::<SafeArray<i32>>(sa_solids);

            let mut params = [variant_solids, VARIANT::from(material_name)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AssignMaterialToSolid",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::assign_material_to_solid: {}", e),
            }
        }
    }

    /// Assign material to single solid.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of material.
    /// * `[in] solid_no` Single solid number.
    /// # Return values
    /// * `1` TRUE.
    /// * `0` FALSE.
    pub fn assign_material_to_solid_single(
        &self,
        material_name: &str,
        solid_no: i32,
    ) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(solid_no), VARIANT::from(material_name)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AssignMaterialToSolid",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::assign_material_to_solid_single: {}", e),
            }
        }
    }

    /// Set the string name of this material.
    /// # Parameters
    /// * `[in] strMaterialName` Name of the material.
    /// # Return values
    /// * `1` Succeed.
    pub fn set_material_name(&self, material_name: &str) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(material_name)];
            let result_variant =
                invoke_method_with_result(&self.property.dispatch, "SetMaterialName", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Property::set_material_name: {}", e),
            }
        }
    }
}
