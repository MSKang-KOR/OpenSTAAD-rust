use crate::staad::{
    property::root::Property,
    utils::invoke_method_with_result,
    variant::{SafeArray, variant_from_raw_pointer},
};

use anyhow::{Error as anyErr, Ok as anyOk, Result, bail};
use std::ffi::c_void;
use windows::Win32::System::{
    Ole::{SafeArrayCreateVector, SafeArrayGetElement},
    Variant::{VARIANT, VT_I4, VT_R8, VariantToInt32},
};
//GetBeamConstants
// GetBeamProperty
// GetBeamPropertyAll
// GetMemberReleaseSpec
// GetPlateSectionPropertyRefNo

#[derive(Debug)]
pub struct ElementProperty<'a> {
    pub property: &'a Property<'a>,
}

impl<'a> ElementProperty<'a> {
    pub fn new(property: &'a Property<'a>) -> Self {
        Self { property }
    }

    /// Get material constants by specified beam number ID.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Long)
    /// * `[out] vardElasticity` Modulus of elasticity (E) (Type: Double)
    /// * `[out] vardPoisson` Poisson's ratio (POI) (Type: Double)
    /// * `[out] vardDensity` Weight density (DEN) (Type: Double)
    /// * `[out] vardAlpha` Coefficient of thermal expansion (ALP) (Type: Double)
    /// * `[out] vardDamp` Damping ratio (DAMP) (Type: Double)
    /// # Return values
    /// * `TRUE` Beam Constants found.
    /// * `FALSE` Beam not found/beam constants not found.
    pub fn get_beam_constants(
        &self,
        beam_no: i32,
    ) -> Result<(bool, f64, f64, f64, f64, f64), anyErr> {
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
                VARIANT::from(beam_no),
            ];
            let result_variant =
                invoke_method_with_result(&self.property.dispatch, "GetBeamConstants", &mut params);
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
                    ))
                }
                Err(e) => bail!("Error::Property::get_beam_constants: {}", e),
            }
        }
    }

    /// Retrieve short member properties of the specified beam member.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Int)
    /// * `[out] varfWidth` Width of the section (WID) (Type: Double)
    /// * `[out] varfDepth` Depth of the section (DEP) (Type: Double)
    /// * `[out] varfAx` Cross section area (Ax) (Type: Double)
    /// * `[out] varfAy` Shear area in local y-axis (Ay) (Type: Double)
    /// * `[out] varfAz` Shear area in local z-axis (Az) (Type: Double)
    /// * `[out] varfIx` Moment of inertia about local z-axis (Ix) (Type: Double)
    /// * `[out] varfIy` Moment of inertia about local y-axis (Iy) (Type: Double)
    /// * `[out] varfIz` Torsional constant (Iz) (Type: Double)
    /// # Return values
    /// * `1` OK.
    /// * `0` General Error / Cannot find member / No property attached.
    pub fn get_beam_property(
        &self,
        beam_no: i32,
    ) -> Result<(i32, f64, f64, f64, f64, f64, f64, f64, f64), anyErr> {
        unsafe {
            let width_ptr = &mut 0.0f64 as *mut f64;
            let depth_ptr = &mut 0.0f64 as *mut f64;
            let ax_ptr = &mut 0.0f64 as *mut f64;
            let ay_ptr = &mut 0.0f64 as *mut f64;
            let az_ptr = &mut 0.0f64 as *mut f64;
            let ix_ptr = &mut 0.0f64 as *mut f64;
            let iy_ptr = &mut 0.0f64 as *mut f64;
            let iz_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(iz_ptr),
                variant_from_raw_pointer::<f64>(iy_ptr),
                variant_from_raw_pointer::<f64>(ix_ptr),
                variant_from_raw_pointer::<f64>(az_ptr),
                variant_from_raw_pointer::<f64>(ay_ptr),
                variant_from_raw_pointer::<f64>(ax_ptr),
                variant_from_raw_pointer::<f64>(depth_ptr),
                variant_from_raw_pointer::<f64>(width_ptr),
                VARIANT::from(beam_no),
            ];
            let result_variant =
                invoke_method_with_result(&self.property.dispatch, "GetBeamProperty", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((
                        result_code,
                        *width_ptr,
                        *depth_ptr,
                        *ax_ptr,
                        *ay_ptr,
                        *az_ptr,
                        *ix_ptr,
                        *iy_ptr,
                        *iz_ptr,
                    ))
                }
                Err(e) => bail!("Error::Property::get_beam_property: {}", e),
            }
        }
    }

    /// Retrieve long member properties of the specified beam member.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Int)
    /// * `[out] varfWidth` Width of the section (WID) (Type: Double)
    /// * `[out] varfDepth` Depth of the section (DEP) (Type: Double)
    /// * `[out] varfAx` Cross section area (Ax) (Type: Double)
    /// * `[out] varfAy` Shear area in local y-axis (Ay) (Type: Double)
    /// * `[out] varfAz` Shear area in local z-axis (Az) (Type: Double)
    /// * `[out] varfIx` Moment of inertia about local z-axis (Ix) (Type: Double)
    /// * `[out] varfIy` Moment of inertia about local y-axis (Iy) (Type: Double)
    /// * `[out] varfIz` Torsional constant (Iz) (Type: Double)
    /// * `[out] varfTf` Thickness of top flange (Tf) (Type: Double)
    /// * `[out] varfTw` Thickness of web (Tw) (Type: Double)
    /// # Return values
    /// * `1` OK.
    /// * `0` Cannot find member / No property attached.
    pub fn get_beam_property_all(
        &self,
        beam_no: i32,
    ) -> Result<(i32, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64), anyErr> {
        unsafe {
            let width_ptr = &mut 0.0f64 as *mut f64;
            let depth_ptr = &mut 0.0f64 as *mut f64;
            let ax_ptr = &mut 0.0f64 as *mut f64;
            let ay_ptr = &mut 0.0f64 as *mut f64;
            let az_ptr = &mut 0.0f64 as *mut f64;
            let ix_ptr = &mut 0.0f64 as *mut f64;
            let iy_ptr = &mut 0.0f64 as *mut f64;
            let iz_ptr = &mut 0.0f64 as *mut f64;
            let tf_ptr = &mut 0.0f64 as *mut f64;
            let tw_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(tw_ptr),
                variant_from_raw_pointer::<f64>(tf_ptr),
                variant_from_raw_pointer::<f64>(iz_ptr),
                variant_from_raw_pointer::<f64>(iy_ptr),
                variant_from_raw_pointer::<f64>(ix_ptr),
                variant_from_raw_pointer::<f64>(az_ptr),
                variant_from_raw_pointer::<f64>(ay_ptr),
                variant_from_raw_pointer::<f64>(ax_ptr),
                variant_from_raw_pointer::<f64>(depth_ptr),
                variant_from_raw_pointer::<f64>(width_ptr),
                VARIANT::from(beam_no),
            ];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetBeamPropertyAll",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((
                        result_code,
                        *width_ptr,
                        *depth_ptr,
                        *ax_ptr,
                        *ay_ptr,
                        *az_ptr,
                        *ix_ptr,
                        *iy_ptr,
                        *iz_ptr,
                        *tf_ptr,
                        *tw_ptr,
                    ))
                }
                Err(e) => bail!("Error::Property::get_beam_property_all: {}", e),
            }
        }
    }

    /// Get releases for the specified member at the specified end.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// * `[in] varnEnd` Member Start end (= 0); member End end (= 1).
    /// * `[out] varnReleaseArray` Translational release VARIANT array with 6 elements for 6 DOFs.
    /// * `[out] varfSpringConstArray` Rotational releases VARIANT array with 6 elements for 6 DOFs.
    /// # Return values
    /// * `1` OK.
    /// * `0` General error.
    pub fn get_member_release_spec(
        &self,
        beam_no: i32,
        end: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>), anyErr> {
        unsafe {
            let mut release_array = vec![0i32; 6];
            let mut spring_const_array = vec![0.0f64; 6];

            let sa_release = SafeArrayCreateVector(VT_I4, 0, 6);
            let sa_spring = SafeArrayCreateVector(VT_R8, 0, 6);

            let mut params = [
                variant_from_raw_pointer::<SafeArray<f64>>(sa_spring),
                variant_from_raw_pointer::<SafeArray<i32>>(sa_release),
                VARIANT::from(end),
                VARIANT::from(beam_no),
            ];

            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetMemberReleaseSpec",
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

                    anyOk((result_code, release_array, spring_const_array))
                }
                Err(e) => bail!("Error::Property::get_member_release_spec: {}", e),
            }
        }
    }

    /// Get the assigned section property ID of specified plate.
    /// # Parameters
    /// * `[in] nPlateNo` The plate number ID.
    /// # Return values
    /// * `<Val>` the assigned section property ID.
    /// * `-4001` Cannot find plate nPlateNo.
    /// * `-6022` No property is attached to the plate.
    pub fn get_plate_section_property_ref_no(&self, plate_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(plate_no)];
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetPlateSectionPropertyRefNo",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::get_plate_section_property_ref_no: {}", e),
            }
        }
    }
}
