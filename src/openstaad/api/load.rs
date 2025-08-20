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
use windows_core::BSTR;

#[derive(Debug, Serialize, Deserialize)]
pub struct Load {
    #[serde(skip)]
    pub dispatch: Option<IDispatch>,
    pub id: String,
}

impl Load {
    pub fn new(staad: Option<IDispatch>) -> Self {
        let _load = unsafe { get_dispatch(staad.as_ref().unwrap(), "Load", &mut []).unwrap() };
        Self {
            dispatch: Some(_load),
            id: uuid::Uuid::new_v4().to_string(),
        }
    }
    /// Adds a Wind Definition named "varTypeName" with number ID varTypeNo.
    /// # Parameters
    /// * `[in] varTypeNo` Wind Definition Type number ID.
    /// * `[in] varTypeName` String name of this new type ("Comment" in STAAD).
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_wind_definition(&self, type_no: i32, type_name: &str) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(type_name), VARIANT::from(type_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddWindDefinition",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_wind_definition: {}", e),
            }
        }
    }

    /// Generates the Wind Definition Parameters using ASCE CODE.
    /// # Parameters
    /// * `[in] type_no` Wind Definition Type number ID (Integer).
    /// * `[in] code` ASCE CODE (0=ASCE7_95, 1=ASCE7_02, 2=ASCE7_05_10, 3=ASCE7_16).
    /// * `[in] wind_speed` Wind speed (Double).
    /// * `[in] height_above_sea_lvl` Ground Height above sea level (Double) [Required only for ASCE7-2016].
    /// * `[in] bldg_class` Building Classification Category (0=TypeI, 1=TypeII, 2=TypeIII, 3=TypeIV).
    /// * `[in] bldg_type` Structure Type (0=Building, 1=Chimney, 2=Solidsign, 3=Opensign, 4=Latticeframe, 5=Trusstower).
    /// * `[in] exp_cat` Exposure Category (0=ExpA, 1=ExpB, 2=ExpC, 3=ExpD).
    /// * `[in] escarpment` Consider Wind Speed-up over Hills (false) or Escarpment (true).
    /// * `[in] wall_type` Building wall type (0=WindWard, 1=LeeWard, 2=SideWall).
    /// * `[in] is_flexible` Consider structure is Flexible (true) or RIGID (false).
    /// * `[in] escarpment_data` Array of 4 doubles for escarpment data.
    /// * `[in] bldg_data` Array of 7 doubles for building data.
    /// * `[in] units_data` Array of 8 integers for units data.
    /// * `[in] factors_user_input` Array of 8 integers for factor user input flags.
    /// * `[in] factors` Array of 8 doubles for factor values.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_wind_definition_asce7_parameters(
        &self,
        type_no: i32,
        code: i32,
        wind_speed: f64,
        height_above_sea_lvl: f64,
        bldg_class: i32,
        bldg_type: i32,
        exp_cat: i32,
        escarpment: bool,
        wall_type: i32,
        is_flexible: bool,
        escarpment_data: Vec<f64>,
        bldg_data: Vec<f64>,
        units_data: Vec<i32>,
        factors_user_input: Vec<i32>,
        factors: Vec<f64>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_escarpment = safe_array_from_vec1d::<f64>(escarpment_data)?;
            let sa_bldg = safe_array_from_vec1d::<f64>(bldg_data)?;
            let sa_units = safe_array_from_vec1d::<i32>(units_data)?;
            let sa_factors_user = safe_array_from_vec1d::<i32>(factors_user_input)?;
            let sa_factors = safe_array_from_vec1d::<f64>(factors)?;

            let variant_escarpment = variant_with_ptr_from::<SafeArray<f64>>(sa_escarpment);
            let variant_bldg = variant_with_ptr_from::<SafeArray<f64>>(sa_bldg);
            let variant_units = variant_with_ptr_from::<SafeArray<i32>>(sa_units);
            let variant_factors_user = variant_with_ptr_from::<SafeArray<i32>>(sa_factors_user);
            let variant_factors = variant_with_ptr_from::<SafeArray<f64>>(sa_factors);

            let mut params = [
                variant_factors,
                variant_factors_user,
                variant_units,
                variant_bldg,
                variant_escarpment,
                VARIANT::from(is_flexible as i32),
                VARIANT::from(wall_type),
                VARIANT::from(escarpment as i32),
                VARIANT::from(exp_cat),
                VARIANT::from(bldg_type),
                VARIANT::from(bldg_class),
                VARIANT::from(height_above_sea_lvl),
                VARIANT::from(wind_speed),
                VARIANT::from(code),
                VARIANT::from(type_no),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddWindDefinitionASCE7Parameters",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_wind_definition_asce7_parameters: {}", e),
            }
        }
    }

    /// Adds Wind Exposures factor to Wind Definitions and assign to node(s).
    /// # Parameters
    /// * `[in] varTypeNo` Wind Definition Type number ID.
    /// * `[in] varExposureFactor` Exposure factor.
    /// * `[in] varNodeArray` Node number ID(s) VARIANT array.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_wind_exposure(
        &self,
        type_no: i32,
        exposure_factor: f64,
        node_array: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_nodes = safe_array_from_vec1d::<i32>(node_array)?;
            let variant_nodes = variant_with_ptr_from::<SafeArray<i32>>(sa_nodes);

            let mut params = [
                variant_nodes,
                VARIANT::from(exposure_factor),
                VARIANT::from(type_no),
            ];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddWindExposure",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_wind_exposure: {}", e),
            }
        }
    }

    /// Adds to Wind Definitions Wind Intensity by giving Intensity vs. Height.
    /// # Parameters
    /// * `[in] varTypeNo` Wind Definition Type number ID.
    /// * `[in] varIntensity` Intensity value VARIANT (Double) array (Unit: Force/Length^2).
    /// * `[in] varHeight` Height value VARIANT (Double) array (Unit: Length).
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_wind_intensity(
        &self,
        type_no: i32,
        intensity: Vec<f64>,
        height: Vec<f64>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_intensity = safe_array_from_vec1d::<f64>(intensity)?;
            let sa_height = safe_array_from_vec1d::<f64>(height)?;
            let variant_intensity = variant_with_ptr_from::<SafeArray<f64>>(sa_intensity);
            let variant_height = variant_with_ptr_from::<SafeArray<f64>>(sa_height);

            let mut params = [variant_height, variant_intensity, VARIANT::from(type_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddWindIntensity",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_wind_intensity: {}", e),
            }
        }
    }

    /// Adds single intensity-height pair to Wind Definitions.
    /// # Parameters
    /// * `[in] type_no` Wind Definition Type number ID.
    /// * `[in] intensity` Single intensity value (Unit: Force/Length^2).
    /// * `[in] height` Single height value (Unit: Length).
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_wind_intensity_single(
        &self,
        type_no: i32,
        intensity: f64,
        height: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(height),
                VARIANT::from(intensity),
                VARIANT::from(type_no),
            ];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddWindIntensity",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_wind_intensity_single: {}", e),
            }
        }
    }

    /// Generates the wall wind pressure profile using ASCE CODE.
    /// # Parameters
    /// * `[in] code` ASCE CODE (0=ASCE7Y95, 1=ACSE702, 2=ACSE705_10).
    /// * `[in] wind_speed` Wind speed.
    /// * `[in] bldg_class` Building Classification Category (0=TypeI, 1=TypeII, 2=TypeIII, 3=TypeIV).
    /// * `[in] bldg_type` Structure Type (0=Building, 1=Chimney, 2=Solidsign, 3=Opensign, 4=Latticeframe, 5=Trusstower).
    /// * `[in] exp_cat` Exposure Category (0=ExpA, 1=ExpB, 2=ExpC, 3=ExpD).
    /// * `[in] escarpment` Consider Wind Speed-up over Hills (false) or Escarpment (true).
    /// * `[in] unit_data` Array for unit data.
    /// * `[in] escarpment_data` Array of 4 doubles for escarpment data.
    /// * `[in] bldg_data` Array of 7 doubles for building data.
    /// * `[in] wall_type` Building wall type (0=WindWard, 1=LeeWard, 2=SideWall).
    /// # Return values
    /// * Number of Height or Intensity data.
    /// * `-1` General error.
    pub fn compute_wall_wind_pressure_profile(
        &self,
        code: i32,
        wind_speed: f64,
        bldg_class: i32,
        bldg_type: i32,
        exp_cat: i32,
        escarpment: bool,
        unit_data: Vec<i32>,
        escarpment_data: Vec<f64>,
        bldg_data: Vec<f64>,
        wall_type: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_unit = safe_array_from_vec1d::<i32>(unit_data)?;
            let sa_escarpment = safe_array_from_vec1d::<f64>(escarpment_data)?;
            let sa_bldg = safe_array_from_vec1d::<f64>(bldg_data)?;

            let variant_unit = variant_with_ptr_from::<SafeArray<i32>>(sa_unit);
            let variant_escarpment = variant_with_ptr_from::<SafeArray<f64>>(sa_escarpment);
            let variant_bldg = variant_with_ptr_from::<SafeArray<f64>>(sa_bldg);

            let mut params = [
                VARIANT::from(wall_type),
                variant_bldg,
                variant_escarpment,
                variant_unit,
                VARIANT::from(escarpment as i32),
                VARIANT::from(exp_cat),
                VARIANT::from(bldg_type),
                VARIANT::from(bldg_class),
                VARIANT::from(wind_speed),
                VARIANT::from(code),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "ComputeWallWindPressureProfile",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::compute_wall_wind_pressure_profile: {}", e),
            }
        }
    }

    /// Generates the wall wind pressure profile using ASCE7-2016 version only.
    /// # Parameters
    /// * `[in] wind_speed` Wind speed. Default value 85 mph.
    /// * `[in] height_above_sea_lvl` Ground height above sea level. Used only for ASCE7-2016 Wind. Default value 0.0 ft.
    /// * `[in] bldg_class` Building Classification Category (0=TypeI, 1=TypeII, 2=TypeIII, 3=TypeIV).
    /// * `[in] bldg_type` Structure Type (0=Building, 1=Chimney, 2=Solidsign, 3=Opensign, 4=Latticeframe, 5=Trusstower).
    /// * `[in] exp_cat` Exposure Category (0=ExpA, 1=ExpB, 2=ExpC, 3=ExpD).
    /// * `[in] escarpment` Consider Wind Speed-up over Hills (false) or Escarpment (true).
    /// * `[in] unit_data` Array of 8 integers for units data.
    /// * `[in] escarpment_data` Array of 4 doubles for escarpment data.
    /// * `[in] bldg_data` Array of 7 doubles for building data.
    /// * `[in] wall_type` Building wall type (0=WindWard, 1=LeeWard, 2=SideWall).
    /// # Return values
    /// * Number of Height or Intensity data.
    /// * `-1` General error.
    pub fn compute_wall_wind_pressure_profile_asce7_2016(
        &self,
        wind_speed: f64,
        height_above_sea_lvl: f64,
        bldg_class: i32,
        bldg_type: i32,
        exp_cat: i32,
        escarpment: bool,
        unit_data: Vec<i32>,
        escarpment_data: Vec<f64>,
        bldg_data: Vec<f64>,
        wall_type: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_unit = safe_array_from_vec1d::<i32>(unit_data)?;
            let sa_escarpment = safe_array_from_vec1d::<f64>(escarpment_data)?;
            let sa_bldg = safe_array_from_vec1d::<f64>(bldg_data)?;

            let variant_unit = variant_with_ptr_from::<SafeArray<i32>>(sa_unit);
            let variant_escarpment = variant_with_ptr_from::<SafeArray<f64>>(sa_escarpment);
            let variant_bldg = variant_with_ptr_from::<SafeArray<f64>>(sa_bldg);

            let mut params = [
                VARIANT::from(wall_type),
                variant_bldg,
                variant_escarpment,
                variant_unit,
                VARIANT::from(escarpment as i32),
                VARIANT::from(exp_cat),
                VARIANT::from(bldg_type),
                VARIANT::from(bldg_class),
                VARIANT::from(height_above_sea_lvl),
                VARIANT::from(wind_speed),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "ComputeWallWindPressureProfileASCE72016",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Load::compute_wall_wind_pressure_profile_asce7_2016: {}",
                    e
                ),
            }
        }
    }

    /// Deletes Wind definition. All definitions will be deleted if this input is set as 0.
    /// # Parameters
    /// * `[in] nTypeNo` Type of Wind.
    /// # Return values
    /// * `0` OK.
    /// * `-8039` Invalid load definition.
    pub fn delete_wind_definition(&self, type_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(type_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "DeleteWindDefinition",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::delete_wind_definition: {}", e),
            }
        }
    }

    /// Adds Response Spectrum load item to the currently active load case.
    /// # Parameters
    /// * `[in] rsa_code` Response Spectrum Loading Code.
    /// * `[in] rsa_combination` Modal combination rule (SRSS=0, ABS=1, CQC=2, ASCE=3, TEN=4, CSM=5, GRP=6).
    /// * `[in] set1_names` VARIANT BSTR array containing parameter keywords.
    /// * `[in] set1_vals` Parameters values corresponding to the keywords supplied in set1_names array.
    /// * `[in] set2_names` Optional VARIANT BSTR array containing parameter keywords for spectrum generation.
    /// * `[in] set2_vals` Parameters values corresponding to the keywords supplied in set2_names array.
    /// * `[in] spectrum_data_pairs` VARIANT double array containing pairs of time period and acceleration data.
    /// # Return values
    /// * `0` FALSE.
    /// * `1` TRUE.
    pub fn add_response_spectrum_load_ex(
        &self,
        rsa_code: i32,
        rsa_combination: i32,
        set1_names: Vec<String>,
        set1_vals: Vec<f64>,
        set2_names: Option<Vec<String>>,
        set2_vals: Option<Vec<f64>>,
        spectrum_data_pairs: Option<Vec<f64>>,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_set1_names = safe_array_from_vec1d::<String>(set1_names)?;
            let sa_set1_vals = safe_array_from_vec1d::<f64>(set1_vals)?;
            let variant_set1_names = variant_with_ptr_from::<SafeArray<BSTR>>(sa_set1_names);
            let variant_set1_vals = variant_with_ptr_from::<SafeArray<f64>>(sa_set1_vals);

            let (variant_set2_names, variant_set2_vals, variant_spectrum_data) =
                match (set2_names, set2_vals, spectrum_data_pairs) {
                    (Some(names), Some(vals), None) => {
                        let sa_set2_names = safe_array_from_vec1d::<String>(names)?;
                        let sa_set2_vals = safe_array_from_vec1d::<f64>(vals)?;
                        (
                            variant_with_ptr_from::<SafeArray<BSTR>>(sa_set2_names),
                            variant_with_ptr_from::<SafeArray<f64>>(sa_set2_vals),
                            VARIANT::default(), // NULL
                        )
                    }
                    (None, None, Some(data)) => {
                        let sa_spectrum = safe_array_from_vec1d::<f64>(data)?;
                        (
                            VARIANT::default(), // NULL
                            VARIANT::default(), // NULL
                            variant_with_ptr_from::<SafeArray<f64>>(sa_spectrum),
                        )
                    }
                    _ => (VARIANT::default(), VARIANT::default(), VARIANT::default()),
                };

            let mut params = [
                variant_spectrum_data,
                variant_set2_vals,
                variant_set2_names,
                variant_set1_vals,
                variant_set1_names,
                VARIANT::from(rsa_combination),
                VARIANT::from(rsa_code),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddResponseSpectrumLoadEx",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Load::add_response_spectrum_load_ex: {}", e),
            }
        }
    }

    /// Adds a Seismic Definition with default parameters.
    /// # Parameters
    /// * `[in] var_type` Type of seismic code (0-24, see documentation for codes).
    /// * `[in] var_accidental` Consider accidental torsion (=1); ignore (=0).
    /// # Return values
    /// * `TRUE` Successful.
    /// * `FALSE` Unsuccessful.
    pub fn add_seismic_definition(
        &self,
        seismic_type: i32,
        accidental: bool,
    ) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(accidental as i32),
                VARIANT::from(seismic_type),
            ];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddSeismicDefinition",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Load::add_seismic_definition: {}", e),
            }
        }
    }

    /// Adds joint self weight to Seismic Definition.
    /// # Parameters
    /// * `[in] var_weight` Weight value (Type: Double).
    /// * `[in] var_node_array` Node number ID(s) VARIANT array (Type: Long Array/Long).
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    /// * `-100` Invalid argument.
    /// * `-106` 1 dimensional array of long expected.
    /// * `-113` Integer array/Integer expected.
    /// * `-2006` Invalid Node Number.
    /// * `-8034` Seismic Code not found.
    pub fn add_seismic_def_joint_weight(
        &self,
        weight: f64,
        node_array: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_nodes = safe_array_from_vec1d::<i32>(node_array)?;
            let variant_nodes = variant_with_ptr_from::<SafeArray<i32>>(sa_nodes);

            let mut params = [variant_nodes, VARIANT::from(weight)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddSeismicDefJointWeight",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_seismic_def_joint_weight: {}", e),
            }
        }
    }

    /// Adds member concentrated/uniform weight to Seismic Definition.
    /// # Parameters
    /// * `[in] var_seismic_type` Type of seismic code (0-25, see documentation).
    /// * `[in] var_load_type` 1=UNI, 2=CON (Type: long).
    /// * `[in] var_weight` Uniform weight (Type: double).
    /// * `[in] var_start_dist` Starting distance (Type: double).
    /// * `[in] var_end_dist` Ending distance (Type: double).
    /// * `[in] var_member_array` Member number ID(s) VARIANT array (Type: long).
    /// # Return values
    /// * `TRUE` If it successfully adds member weight to Seismic Definition.
    /// * `FALSE` Unsuccessful.
    pub fn add_seismic_def_member_weight(
        &self,
        seismic_type: i32,
        load_type: i32,
        weight: f64,
        start_dist: f64,
        end_dist: f64,
        member_array: Vec<i32>,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_members = safe_array_from_vec1d::<i32>(member_array)?;
            let variant_members = variant_with_ptr_from::<SafeArray<i32>>(sa_members);

            let mut params = [
                variant_members,
                VARIANT::from(end_dist),
                VARIANT::from(start_dist),
                VARIANT::from(weight),
                VARIANT::from(load_type),
                VARIANT::from(seismic_type),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddSeismicDefMemberWeight",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Load::add_seismic_def_member_weight: {}", e),
            }
        }
    }

    /// Adds self weight to Seismic Definition.
    /// # Parameters
    /// * `[in] var_weight_factor` Weight factor.
    /// # Return values
    /// * `true` Successful.
    /// * `false` General error.
    pub fn add_seismic_def_self_weight(&self, weight_factor: f64) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(weight_factor)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddSeismicDefSelfWeight",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Load::add_seismic_def_self_weight: {}", e),
            }
        }
    }

    /// Wall Area is only available in IS1893-2016 seismic code.
    /// Adds wall area to Seismic Definition.
    /// # Parameters
    /// * `[in] n_type_no` Type of seismic code (15=Indian: IS 1893-2016).
    /// * `[in] sz_direction` Direction value ["X" or "Z"].
    /// * `[in] var_memb_array` Length and Width VARIANT array.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    /// * `-107` 1 dimensional array of long expected.
    /// * `-8034` Invalid seismic code.
    /// * `-8038` Invalid Direction.
    pub fn add_seismic_def_wall_area(
        &self,
        type_no: i32,
        direction: &str,
        member_array: Vec<f64>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_members = safe_array_from_vec1d::<f64>(member_array)?;
            let variant_members = variant_with_ptr_from::<SafeArray<f64>>(sa_members);

            let mut params = [
                variant_members,
                VARIANT::from(direction),
                VARIANT::from(type_no),
            ];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddSeismicDefWallArea",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_seismic_def_wall_area: {}", e),
            }
        }
    }

    /// Modifies or adds a seismic parameter in the existing seismic definition.
    /// # Parameters
    /// * `[in] var_param_name` Parameter name for the corresponding code in the seismic definition.
    /// * `[in] var_value` Value corresponding to the above parameter (Type: Double).
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn modify_seismic_definition_params(
        &self,
        param_name: &str,
        value: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(value), VARIANT::from(param_name)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "ModifySeismicDefinitionParams",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::modify_seismic_definition_params: {}", e),
            }
        }
    }
    /// Creates a new Reference Load Definition and set as active.
    /// # Parameters
    /// * `[in] nLoadNo` (Reference) load case reference ID to be assigned to new created reference load case.
    /// * `[in] varLoadCaseTitle` Load case string title.
    /// * `[in] nLoadType` Type of the load.
    ///
    /// ## Load Types:
    /// * `0` Dead
    /// * `1` Live
    /// * `2` Roof Live
    /// * `3` Wind
    /// * `4` Seismic-H
    /// * `5` Seismic-V
    /// * `6` Snow
    /// * `7` Fluids
    /// * `8` Soil
    /// * `9` Rain
    /// * `10` Ponding
    /// * `11` Dust
    /// * `12` Traffic
    /// * `13` Temp
    /// * `14` Imperfection
    /// * `15` Accidental
    /// * `16` Flood
    /// * `17` Ice
    /// * `18` Wind Ice
    /// * `19` Crane Hook
    /// * `20` Mass
    /// * `21` Gravity
    /// * `22` Push
    /// * `23` None
    ///
    /// # Return values
    /// * `<Val>` Reference load case number ID.
    /// * `-1` General error.
    /// * `-8004` Fail to create load.
    pub fn create_new_reference_load(
        &self,
        load_no: i32,
        load_case_title: &str,
        load_type: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(load_type),
                VARIANT::from(load_case_title),
                VARIANT::from(load_no),
            ];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateNewReferenceLoad",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::create_new_reference_load: {}", e),
            }
        }
    }

    /// Returns the number of reference load case defined in Reference Load Definitions.
    /// # Return values
    /// * `<Val>` The number of reference load case.
    /// * `-1` General error.
    pub fn get_reference_load_case_count(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetReferenceLoadCaseCount",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_count)
            }
            Err(e) => bail!("Error::Load::get_reference_load_case_count: {}", e),
        }
    }

    /// Gets reference load case number ID(s) in Reference Load Definitions.
    /// # Parameters
    /// * `[out] LoadNoArray` Reference load case number ID(s) VARIANT array
    /// # Return values
    /// * `<Val>` The number of reference load case(s).
    /// * `-1` General error.
    /// * `-106` 1 dimensional array of long expected.
    /// * `-114` OLE Exception Occurred.
    pub fn get_reference_load_case_numbers(&self) -> Result<(i32, Vec<i32>), anyErr> {
        unsafe {
            let load_count = self.get_reference_load_case_count()?;
            if load_count <= 0 {
                return anyOk((load_count, Vec::new()));
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, load_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_with_ptr_from::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetReferenceLoadCaseNumbers",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    let load_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut load_arr = Vec::with_capacity(load_count as usize);

                    for i in 0..load_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            load_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        load_arr.push(value as i32);
                    }
                    anyOk((result_count, load_arr))
                }
                Err(e) => bail!("Error::Load::get_reference_load_case_numbers: {}", e),
            }
        }
    }

    /// Identify a Load Case in Load Case Details to add, count and get reference load item(s).
    /// # Parameters
    /// * `[in] nLoadCaseNo` Load case reference ID in Load Cases Details.
    /// # Return values
    /// * `<Val>` Reference load case number ID.
    /// * `-1` General error.
    /// * `-8002` Load Case nLoadCaseNo not found.
    pub fn set_reference_load_active(&self, load_case_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_case_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SetReferenceLoadActive",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::set_reference_load_active: {}", e),
            }
        }
    }

    /// Adds Direct Analysis Definition (FLEX,AXIAL parameters).
    /// # Parameters
    /// * `[in] pParamType` Type of direct analysis parameter to be added.
    ///   - `DirectAnalysisParameterTypes::Flex` (0) - FLEX parameter
    ///   - `DirectAnalysisParameterTypes::Axial` (2) - AXIAL parameter
    /// * `[in] members` One dimensional array of members of int/long type.
    /// * `[in] param` Parameter value of double type.
    ///   - For FLEX: factor value (e.g., 0.6)
    ///   - For AXIAL: Not Applicable, should pass 0.0
    ///
    /// # Return values
    /// * `true` OK.
    /// * `false` ERROR
    pub fn add_direct_analysis_definition_parameter(
        &self,
        param_type: i32,
        members: Vec<i32>,
        param: f64,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_members = safe_array_from_vec1d::<i32>(members)?;
            let variant_members = variant_with_ptr_from::<SafeArray<i32>>(sa_members);

            let mut params = [
                VARIANT::from(param),
                variant_members,
                VARIANT::from(param_type),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddDirectAnalysisDefinitionParameter",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!(
                    "Error::Load::add_direct_analysis_definition_parameter: {}",
                    e
                ),
            }
        }
    }

    /// Deletes whole Direct Analysis Definition.
    /// # Return values
    /// * `true` OK.
    /// * `false` ERROR
    pub fn delete_direct_analysis_definition(&self) -> Result<bool, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                self.dispatch.as_ref().unwrap(),
                "DeleteDirectAnalysisDefinition",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result == 1)
            }
            Err(e) => bail!("Error::Load::delete_direct_analysis_definition: {}", e),
        }
    }

    /// Deletes respective parameters from Direct Analysis Definition based on the Parameter Type passed as argument (FLEX/AXIAL).
    /// # Parameters
    /// * `[in] pParamType` Type of direct analysis parameter to be deleted.
    ///   - `DirectAnalysisParameterTypes::Flex` (0) - Delete all FLEX parameters
    ///   - `DirectAnalysisParameterTypes::Axial` (2) - Delete all AXIAL parameters
    ///
    /// # Return values
    /// * `true` OK.
    /// * `false` ERROR
    pub fn delete_direct_analysis_definition_parameter(
        &self,
        param_type: i32,
    ) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(param_type)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "DeleteDirectAnalysisDefinitionParameter",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!(
                    "Error::Load::delete_direct_analysis_definition_parameter: {}",
                    e
                ),
            }
        }
    }
    /// Adds a self weight to the active load case and assign it to all entities (beams, plates and solids).
    /// # Parameters
    /// * `[in] varInDirection` Self weight direction index:
    ///   - `1` for X direction
    ///   - `2` for Y direction  
    ///   - `3` for Z direction
    /// * `[in] varLoadFactor` Multiplying factor for self-weight.
    ///
    /// # Return values
    /// * `true` Add self weight successful.
    /// * `false` General error.
    pub fn add_self_weight_in_xyz(&self, direction: i32, load_factor: f64) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_factor), VARIANT::from(direction)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddSelfWeightInXYZ",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Load::add_self_weight_in_xyz: {}", e),
            }
        }
    }

    /// Adds a self weight to plate(s), Beam(s) and Solid(s).
    /// # Parameters
    /// * `[in] varGeomNo` Beam, Plate or Solid number ID(s) array.
    /// * `[in] varInDirection` Self weight direction index:
    ///   - `1` for X direction
    ///   - `2` for Y direction
    ///   - `3` for Z direction
    /// * `[in] varLoadFactor` Multiplying factor for self-weight.
    ///
    /// # Return values
    /// * `true` OK.
    /// * `false` General error.
    pub fn add_self_weight_in_xyz_to_geometry(
        &self,
        geom_nos: Vec<i32>,
        direction: i32,
        load_factor: f64,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_geom_nos = safe_array_from_vec1d::<i32>(geom_nos)?;
            let variant_geom_nos = variant_with_ptr_from::<SafeArray<i32>>(sa_geom_nos);

            let mut params = [
                VARIANT::from(load_factor),
                VARIANT::from(direction),
                variant_geom_nos,
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddSelfWeightInXYZToGeometry",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Load::add_self_weight_in_xyz_to_geometry: {}", e),
            }
        }
    }
    /// Adds JOINT LOAD to the specified node number or numbers.
    /// # Parameters
    /// * `[in] varNodeNo` Node number ID(s) array.
    /// * `[in] varFX` Force in X direction.
    /// * `[in] varFY` Force in Y direction.
    /// * `[in] varFZ` Force in Z direction.
    /// * `[in] varMX` Moment in X direction.
    /// * `[in] varMY` Moment in Y direction.
    /// * `[in] varMZ` Moment in Z direction.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    /// * `-106` One dimensional array of long expected.
    /// * `-8004` Library Error: Unable to add self weight.
    /// * `-8005` Library Error: Unable to assign load.
    pub fn add_nodal_load(
        &self,
        node_nos: Vec<i32>,
        fx: f64,
        fy: f64,
        fz: f64,
        mx: f64,
        my: f64,
        mz: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_node_nos = safe_array_from_vec1d::<i32>(node_nos)?;
            let variant_node_nos = variant_with_ptr_from::<SafeArray<i32>>(sa_node_nos);

            let mut params = [
                VARIANT::from(mz),
                VARIANT::from(my),
                VARIANT::from(mx),
                VARIANT::from(fz),
                VARIANT::from(fy),
                VARIANT::from(fx),
                variant_node_nos,
            ];

            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "AddNodalLoad", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_nodal_load: {}", e),
            }
        }
    }

    /// Adds SUPPORT DISPLACEMENT to node or nodes.
    /// # Parameters
    /// * `[in] varNodeNo` Node number ID(s) array.
    /// * `[in] varDirection` A number indicating direction:
    ///   - `1` for X direction
    ///   - `2` for Y direction
    ///   - `3` for Z direction
    /// * `[in] varDispValue` Support displacement in given direction.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_support_displacement(
        &self,
        node_nos: Vec<i32>,
        direction: i32,
        disp_value: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_node_nos = safe_array_from_vec1d::<i32>(node_nos)?;
            let variant_node_nos = variant_with_ptr_from::<SafeArray<i32>>(sa_node_nos);

            let mut params = [
                VARIANT::from(disp_value),
                VARIANT::from(direction),
                variant_node_nos,
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddSupportDisplacement",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_support_displacement: {}", e),
            }
        }
    }

    /// Get number of nodal loads present for the specified node.
    /// # Parameters
    /// * `[in] nNodeNo` The number of node.
    ///
    /// # Return values
    /// * `<Val>` The number of nodal loads.
    /// * `-1` General error (perhaps load case not found).
    pub fn get_nodal_load_count(&self, node_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(node_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetNodalLoadCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_count)
                }
                Err(e) => bail!("Error::Load::get_nodal_load_count: {}", e),
            }
        }
    }

    /// Gets nodal load(s) generated by specified load item in specified load case.
    /// # Parameters
    /// * `[in] loadIndex` Load item index.
    /// * `[out] varForce` Nodal force array: [FX, FY, FZ, MX, MY, MZ].
    ///
    /// # Return values
    /// * `false` Failed
    /// * `true` Success
    pub fn get_nodal_load_info(&self, load_index: i32) -> Result<(bool, Vec<f64>), anyErr> {
        unsafe {
            let mut psa = SafeArrayCreateVector(VT_R8, 0, 6);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant_force = variant_with_ptr_from::<SafeArrayP<f64>>(psa_ptr);

            let mut params = [variant_force, VARIANT::from(load_index)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetNodalLoadInfo",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    let force_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut force_arr = Vec::with_capacity(6);

                    for i in 0..6 {
                        let mut index = i as i32;
                        let mut value = 0.0;
                        let _ = SafeArrayGetElement(
                            force_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        force_arr.push(value);
                    }
                    anyOk((result == 1, force_arr))
                }
                Err(e) => bail!("Error::Load::get_nodal_load_info: {}", e),
            }
        }
    }

    /// Gets the nodal load(s) values for the specified node.
    /// # Parameters
    /// * `[in] nNodeNo` The node number ID.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    ///
    /// Returns tuple of (result_code, forces, moments) where:
    /// - forces: [FX, FY, FZ] arrays
    /// - moments: [MX, MY, MZ] arrays
    pub fn get_nodal_loads(
        &self,
        node_no: i32,
    ) -> Result<(i32, [Vec<f64>; 3], [Vec<f64>; 3]), anyErr> {
        unsafe {
            let load_count = self.get_nodal_load_count(node_no)?;
            if load_count <= 0 {
                return anyOk((
                    load_count,
                    [Vec::new(), Vec::new(), Vec::new()],
                    [Vec::new(), Vec::new(), Vec::new()],
                ));
            }

            let mut psa_fx = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_fy = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_fz = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_mx = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_my = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_mz = SafeArrayCreateVector(VT_R8, 0, load_count as u32);

            let psa_fx_ptr = &mut psa_fx as *mut *mut SAFEARRAY;
            let psa_fy_ptr = &mut psa_fy as *mut *mut SAFEARRAY;
            let psa_fz_ptr = &mut psa_fz as *mut *mut SAFEARRAY;
            let psa_mx_ptr = &mut psa_mx as *mut *mut SAFEARRAY;
            let psa_my_ptr = &mut psa_my as *mut *mut SAFEARRAY;
            let psa_mz_ptr = &mut psa_mz as *mut *mut SAFEARRAY;

            let variant_fx = variant_with_ptr_from::<SafeArrayP<f64>>(psa_fx_ptr);
            let variant_fy = variant_with_ptr_from::<SafeArrayP<f64>>(psa_fy_ptr);
            let variant_fz = variant_with_ptr_from::<SafeArrayP<f64>>(psa_fz_ptr);
            let variant_mx = variant_with_ptr_from::<SafeArrayP<f64>>(psa_mx_ptr);
            let variant_my = variant_with_ptr_from::<SafeArrayP<f64>>(psa_my_ptr);
            let variant_mz = variant_with_ptr_from::<SafeArrayP<f64>>(psa_mz_ptr);

            let mut params = [
                variant_mz,
                variant_my,
                variant_mx,
                variant_fz,
                variant_fy,
                variant_fx,
                VARIANT::from(node_no),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetNodalLoads",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();

                    let fx_safe_arr = *params[5].Anonymous.Anonymous.Anonymous.pparray;
                    let fy_safe_arr = *params[4].Anonymous.Anonymous.Anonymous.pparray;
                    let fz_safe_arr = *params[3].Anonymous.Anonymous.Anonymous.pparray;
                    let mx_safe_arr = *params[2].Anonymous.Anonymous.Anonymous.pparray;
                    let my_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let mz_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut fx_arr = Vec::with_capacity(load_count as usize);
                    let mut fy_arr = Vec::with_capacity(load_count as usize);
                    let mut fz_arr = Vec::with_capacity(load_count as usize);
                    let mut mx_arr = Vec::with_capacity(load_count as usize);
                    let mut my_arr = Vec::with_capacity(load_count as usize);
                    let mut mz_arr = Vec::with_capacity(load_count as usize);

                    for i in 0..load_count {
                        let mut index = i as i32;
                        let mut fx_val = 0.0;
                        let mut fy_val = 0.0;
                        let mut fz_val = 0.0;
                        let mut mx_val = 0.0;
                        let mut my_val = 0.0;
                        let mut mz_val = 0.0;

                        let _ = SafeArrayGetElement(
                            fx_safe_arr,
                            &mut index,
                            &mut fx_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            fy_safe_arr,
                            &mut index,
                            &mut fy_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            fz_safe_arr,
                            &mut index,
                            &mut fz_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            mx_safe_arr,
                            &mut index,
                            &mut mx_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            my_safe_arr,
                            &mut index,
                            &mut my_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            mz_safe_arr,
                            &mut index,
                            &mut mz_val as *mut f64 as *mut c_void,
                        )?;

                        fx_arr.push(fx_val);
                        fy_arr.push(fy_val);
                        fz_arr.push(fz_val);
                        mx_arr.push(mx_val);
                        my_arr.push(my_val);
                        mz_arr.push(mz_val);
                    }

                    anyOk((
                        result_code,
                        [fx_arr, fy_arr, fz_arr],
                        [mx_arr, my_arr, mz_arr],
                    ))
                }
                Err(e) => bail!("Error::Load::get_nodal_loads: {}", e),
            }
        }
    }
    /// Adds AREA LOAD to beam(s).
    /// # Parameters
    /// * `[in] varBeamNo` Member number ID(s) array.
    /// * `[in] varLoad` Magnitude of the load value.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_member_area_load(&self, beam_nos: Vec<i32>, load: f64) -> Result<i32, anyErr> {
        unsafe {
            let sa_beam_nos = safe_array_from_vec1d::<i32>(beam_nos)?;
            let variant_beam_nos = variant_with_ptr_from::<SafeArray<i32>>(sa_beam_nos);

            let mut params = [VARIANT::from(load), variant_beam_nos];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddMemberAreaLoad",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_member_area_load: {}", e),
            }
        }
    }

    /// Adds CONCENTRATED FORCE to beam(s).
    /// # Parameters
    /// * `[in] varBeamNo` Member number ID(s) array.
    /// * `[in] varDirection` Load direction:
    ///   - `1-3` for LocalX, LocalY, LocalZ
    ///   - `4-6` for GlobalX, GlobalY, GlobalZ
    /// * `[in] varForce` Magnitude of the concentrate force in current units.
    /// * `[in] varD1` Distance from the start of the member to concentrated force.
    /// * `[in] varD2` Perpendicular distance from the member shear center to the local plane of loading.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_member_conc_force(
        &self,
        beam_nos: Vec<i32>,
        direction: i32,
        force: f64,
        d1: f64,
        d2: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_beam_nos = safe_array_from_vec1d::<i32>(beam_nos)?;
            let variant_beam_nos = variant_with_ptr_from::<SafeArray<i32>>(sa_beam_nos);

            let mut params = [
                VARIANT::from(d2),
                VARIANT::from(d1),
                VARIANT::from(force),
                VARIANT::from(direction),
                variant_beam_nos,
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddMemberConcForce",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_member_conc_force: {}", e),
            }
        }
    }

    /// Adds CONCENTRATED MOMENT to beam(s).
    /// # Parameters
    /// * `[in] varBeamNo` Member number ID(s) array.
    /// * `[in] varDirection` Load direction:
    ///   - `1-3` for LocalX, LocalY, LocalZ
    ///   - `4-6` for GlobalX, GlobalY, GlobalZ
    /// * `[in] varMoment` Magnitude of the concentrate moment in current units.
    /// * `[in] varD1` Distance from the start of the member to concentrated moment.
    /// * `[in] varD2` Perpendicular distance from the member shear center to the local plane of loading.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_member_conc_moment(
        &self,
        beam_nos: Vec<i32>,
        direction: i32,
        moment: f64,
        d1: f64,
        d2: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_beam_nos = safe_array_from_vec1d::<i32>(beam_nos)?;
            let variant_beam_nos = variant_with_ptr_from::<SafeArray<i32>>(sa_beam_nos);

            let mut params = [
                VARIANT::from(d2),
                VARIANT::from(d1),
                VARIANT::from(moment),
                VARIANT::from(direction),
                variant_beam_nos,
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddMemberConcMoment",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_member_conc_moment: {}", e),
            }
        }
    }

    /// Adds FIXED END LOAD to beam(s).
    /// # Parameters
    /// * `[in] varBeamNo` Member number ID(s) array.
    /// * `[in] varLoadStart` Load at starting point, array of 6 elements [FX, FY, FZ, MX, MY, MZ].
    /// * `[in] varLoadEnd` Load at stopping point, array of 6 elements [FX, FY, FZ, MX, MY, MZ].
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_member_fixed_end(
        &self,
        beam_nos: Vec<i32>,
        load_start: [f64; 6],
        load_end: [f64; 6],
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_beam_nos = safe_array_from_vec1d::<i32>(beam_nos)?;
            let variant_beam_nos = variant_with_ptr_from::<SafeArray<i32>>(sa_beam_nos);

            let sa_load_start = safe_array_from_vec1d::<f64>(load_start.to_vec())?;
            let variant_load_start = variant_with_ptr_from::<SafeArray<f64>>(sa_load_start);

            let sa_load_end = safe_array_from_vec1d::<f64>(load_end.to_vec())?;
            let variant_load_end = variant_with_ptr_from::<SafeArray<f64>>(sa_load_end);

            let mut params = [variant_load_end, variant_load_start, variant_beam_nos];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddMemberFixedEnd",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_member_fixed_end: {}", e),
            }
        }
    }

    /// Adds LINEARLY VARYING load to beam(s).
    /// # Parameters
    /// * `[in] varBeamNo` Member number ID(s) array.
    /// * `[in] varDirection` Load direction (1-3 for local X, Y, Z).
    /// * `[in] varW1` Load at the start of the member.
    /// * `[in] varW2` Load at the end of the member.
    /// * `[in] varW3` Load in the middle of the member (for triangular load).
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    /// * `-8001` Load direction is invalid.
    pub fn add_member_linear_vari(
        &self,
        beam_nos: Vec<i32>,
        direction: i32,
        w1: f64,
        w2: f64,
        w3: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_beam_nos = safe_array_from_vec1d::<i32>(beam_nos)?;
            let variant_beam_nos = variant_with_ptr_from::<SafeArray<i32>>(sa_beam_nos);

            let mut params = [
                VARIANT::from(w3),
                VARIANT::from(w2),
                VARIANT::from(w1),
                VARIANT::from(direction),
                variant_beam_nos,
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddMemberLinearVari",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_member_linear_vari: {}", e),
            }
        }
    }

    /// Adds trapezoidal linearly varying load to beam(s).
    /// # Parameters
    /// * `[in] varBeamNo` Member number ID(s) array.
    /// * `[in] varDirection` Load direction:
    ///   - `1-3` for LocalX, LocalY, LocalZ
    ///   - `4-6` for GlobalX, GlobalY, GlobalZ
    ///   - `7-9` for ProjectedX, ProjectedY, ProjectedZ
    /// * `[in] varW1` Load at the start of the member.
    /// * `[in] varW2` Load at the end of the member.
    /// * `[in] varD1` Distance from the start of the member to loading starting point.
    /// * `[in] varD2` Distance from the start of the member to loading stopping point.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_member_trapezoidal(
        &self,
        beam_nos: Vec<i32>,
        direction: i32,
        w1: f64,
        w2: f64,
        d1: f64,
        d2: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_beam_nos = safe_array_from_vec1d::<i32>(beam_nos)?;
            let variant_beam_nos = variant_with_ptr_from::<SafeArray<i32>>(sa_beam_nos);

            let mut params = [
                VARIANT::from(d2),
                VARIANT::from(d1),
                VARIANT::from(w2),
                VARIANT::from(w1),
                VARIANT::from(direction),
                variant_beam_nos,
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddMemberTrapezoidal",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_member_trapezoidal: {}", e),
            }
        }
    }

    /// Adds UNIFORM FORCE to beam(s).
    /// # Parameters
    /// * `[in] varBeamNo` Member number ID(s) array.
    /// * `[in] varDirection` Load direction:
    ///   - `1-3` for LocalX, LocalY, LocalZ
    ///   - `4-6` for GlobalX, GlobalY, GlobalZ
    ///   - `7-9` for ProjectedX, ProjectedY, ProjectedZ
    /// * `[in] varForce` Magnitude of the uniform force in current units.
    /// * `[in] varD1` Distance from the start of the member to the start of the load.
    /// * `[in] varD2` Distance from the start of the member to the end of the load.
    /// * `[in] varD3` Perpendicular distance from the member shear center to the local plane of loading.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_member_uniform_force(
        &self,
        beam_nos: Vec<i32>,
        direction: i32,
        force: f64,
        d1: f64,
        d2: f64,
        d3: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_beam_nos = safe_array_from_vec1d::<i32>(beam_nos)?;
            let variant_beam_nos = variant_with_ptr_from::<SafeArray<i32>>(sa_beam_nos);

            let mut params = [
                VARIANT::from(d3),
                VARIANT::from(d2),
                VARIANT::from(d1),
                VARIANT::from(force),
                VARIANT::from(direction),
                variant_beam_nos,
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddMemberUniformForce",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_member_uniform_force: {}", e),
            }
        }
    }

    /// Adds UNIFORM MOMENT to beam(s).
    /// # Parameters
    /// * `[in] varBeamNo` Member number ID(s) array.
    /// * `[in] varDirection` Load direction:
    ///   - `1-3` for LocalX, LocalY, LocalZ
    ///   - `4-6` for GlobalX, GlobalY, GlobalZ
    ///   - `7-9` for ProjectedX, ProjectedY, ProjectedZ
    /// * `[in] varMoment` Magnitude of the uniform moment in current units.
    /// * `[in] varD1` Distance from the start of the member to the start of the load.
    /// * `[in] varD2` Distance from the start of the member to the end of the load.
    /// * `[in] varD3` Perpendicular distance from the member shear center to the local plane of loading.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_member_uniform_moment(
        &self,
        beam_nos: Vec<i32>,
        direction: i32,
        moment: f64,
        d1: f64,
        d2: f64,
        d3: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_beam_nos = safe_array_from_vec1d::<i32>(beam_nos)?;
            let variant_beam_nos = variant_with_ptr_from::<SafeArray<i32>>(sa_beam_nos);

            let mut params = [
                VARIANT::from(d3),
                VARIANT::from(d2),
                VARIANT::from(d1),
                VARIANT::from(moment),
                VARIANT::from(direction),
                variant_beam_nos,
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddMemberUniformMoment",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_member_uniform_moment: {}", e),
            }
        }
    }

    /// Get number of concentrated force(s) present for the specified beam.
    /// # Parameters
    /// * `[in] nBeamNo` Beam number ID.
    ///
    /// # Return values
    /// * `<Val>` The number of concentrated force item(s) applied.
    /// * `-1` General error.
    pub fn get_conc_force_count(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetConcForceCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_count)
                }
                Err(e) => bail!("Error::Load::get_conc_force_count: {}", e),
            }
        }
    }

    /// Returns the concentrated force(s) with all the parameters for the specified member.
    /// # Parameters
    /// * `[in] nBeamNo` Beam number ID.
    ///
    /// # Return values
    /// * `0` OK
    /// * `-1` General error.
    ///
    /// Returns tuple of (result_code, directions, forces, d1_values, d2_values)
    pub fn get_conc_forces(
        &self,
        beam_no: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>, Vec<f64>, Vec<f64>), anyErr> {
        unsafe {
            let force_count = self.get_conc_force_count(beam_no)?;
            if force_count <= 0 {
                return anyOk((force_count, Vec::new(), Vec::new(), Vec::new(), Vec::new()));
            }

            let mut psa_dir = SafeArrayCreateVector(VT_I4, 0, force_count as u32);
            let mut psa_force = SafeArrayCreateVector(VT_R8, 0, force_count as u32);
            let mut psa_d1 = SafeArrayCreateVector(VT_R8, 0, force_count as u32);
            let mut psa_d2 = SafeArrayCreateVector(VT_R8, 0, force_count as u32);

            let psa_dir_ptr = &mut psa_dir as *mut *mut SAFEARRAY;
            let psa_force_ptr = &mut psa_force as *mut *mut SAFEARRAY;
            let psa_d1_ptr = &mut psa_d1 as *mut *mut SAFEARRAY;
            let psa_d2_ptr = &mut psa_d2 as *mut *mut SAFEARRAY;

            let variant_dir = variant_with_ptr_from::<SafeArrayP<i32>>(psa_dir_ptr);
            let variant_force = variant_with_ptr_from::<SafeArrayP<f64>>(psa_force_ptr);
            let variant_d1 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_d1_ptr);
            let variant_d2 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_d2_ptr);

            let mut params = [
                variant_d2,
                variant_d1,
                variant_force,
                variant_dir,
                VARIANT::from(beam_no),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetConcForces",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();

                    let dir_safe_arr = *params[3].Anonymous.Anonymous.Anonymous.pparray;
                    let force_safe_arr = *params[2].Anonymous.Anonymous.Anonymous.pparray;
                    let d1_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let d2_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut dir_arr = Vec::with_capacity(force_count as usize);
                    let mut force_arr = Vec::with_capacity(force_count as usize);
                    let mut d1_arr = Vec::with_capacity(force_count as usize);
                    let mut d2_arr = Vec::with_capacity(force_count as usize);

                    for i in 0..force_count {
                        let mut index = i as i32;
                        let mut dir_val = 0;
                        let mut force_val = 0.0;
                        let mut d1_val = 0.0;
                        let mut d2_val = 0.0;

                        let _ = SafeArrayGetElement(
                            dir_safe_arr,
                            &mut index,
                            &mut dir_val as *mut i32 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            force_safe_arr,
                            &mut index,
                            &mut force_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            d1_safe_arr,
                            &mut index,
                            &mut d1_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            d2_safe_arr,
                            &mut index,
                            &mut d2_val as *mut f64 as *mut c_void,
                        )?;

                        dir_arr.push(dir_val);
                        force_arr.push(force_val);
                        d1_arr.push(d1_val);
                        d2_arr.push(d2_val);
                    }

                    anyOk((result_code, dir_arr, force_arr, d1_arr, d2_arr))
                }
                Err(e) => bail!("Error::Load::get_conc_forces: {}", e),
            }
        }
    }

    /// Gets number of concentrated moment(s) present for the specified beam.
    /// # Parameters
    /// * `[in] nBeamNo` Beam number ID.
    ///
    /// # Return values
    /// * `<Val>` The number of concentrated moment item(s) applied.
    /// * `-1` General error.
    pub fn get_conc_moment_count(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetConcMomentCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_count)
                }
                Err(e) => bail!("Error::Load::get_conc_moment_count: {}", e),
            }
        }
    }

    /// Returns the concentrated moment(s) with all the parameters for the specified member.
    /// # Parameters
    /// * `[in] nBeamNo` Beam number ID.
    ///
    /// # Return values
    /// * `0` OK
    /// * `-1` General error.
    ///
    /// Returns tuple of (result_code, directions, moments, d1_values, d2_values)
    pub fn get_conc_moments(
        &self,
        beam_no: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>, Vec<f64>, Vec<f64>), anyErr> {
        unsafe {
            let moment_count = self.get_conc_moment_count(beam_no)?;
            if moment_count <= 0 {
                return anyOk((moment_count, Vec::new(), Vec::new(), Vec::new(), Vec::new()));
            }

            let mut psa_dir = SafeArrayCreateVector(VT_I4, 0, moment_count as u32);
            let mut psa_moment = SafeArrayCreateVector(VT_R8, 0, moment_count as u32);
            let mut psa_d1 = SafeArrayCreateVector(VT_R8, 0, moment_count as u32);
            let mut psa_d2 = SafeArrayCreateVector(VT_R8, 0, moment_count as u32);

            let psa_dir_ptr = &mut psa_dir as *mut *mut SAFEARRAY;
            let psa_moment_ptr = &mut psa_moment as *mut *mut SAFEARRAY;
            let psa_d1_ptr = &mut psa_d1 as *mut *mut SAFEARRAY;
            let psa_d2_ptr = &mut psa_d2 as *mut *mut SAFEARRAY;

            let variant_dir = variant_with_ptr_from::<SafeArrayP<i32>>(psa_dir_ptr);
            let variant_moment = variant_with_ptr_from::<SafeArrayP<f64>>(psa_moment_ptr);
            let variant_d1 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_d1_ptr);
            let variant_d2 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_d2_ptr);

            let mut params = [
                variant_d2,
                variant_d1,
                variant_moment,
                variant_dir,
                VARIANT::from(beam_no),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetConcMoments",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();

                    let dir_safe_arr = *params[3].Anonymous.Anonymous.Anonymous.pparray;
                    let moment_safe_arr = *params[2].Anonymous.Anonymous.Anonymous.pparray;
                    let d1_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let d2_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut dir_arr = Vec::with_capacity(moment_count as usize);
                    let mut moment_arr = Vec::with_capacity(moment_count as usize);
                    let mut d1_arr = Vec::with_capacity(moment_count as usize);
                    let mut d2_arr = Vec::with_capacity(moment_count as usize);

                    for i in 0..moment_count {
                        let mut index = i as i32;
                        let mut dir_val = 0;
                        let mut moment_val = 0.0;
                        let mut d1_val = 0.0;
                        let mut d2_val = 0.0;

                        let _ = SafeArrayGetElement(
                            dir_safe_arr,
                            &mut index,
                            &mut dir_val as *mut i32 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            moment_safe_arr,
                            &mut index,
                            &mut moment_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            d1_safe_arr,
                            &mut index,
                            &mut d1_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            d2_safe_arr,
                            &mut index,
                            &mut d2_val as *mut f64 as *mut c_void,
                        )?;

                        dir_arr.push(dir_val);
                        moment_arr.push(moment_val);
                        d1_arr.push(d1_val);
                        d2_arr.push(d2_val);
                    }

                    anyOk((result_code, dir_arr, moment_arr, d1_arr, d2_arr))
                }
                Err(e) => bail!("Error::Load::get_conc_moments: {}", e),
            }
        }
    }

    /// Returns number of linear varying load(s) present for the specified beam.
    /// # Parameters
    /// * `[in] nBeamNo` Beam number ID.
    ///
    /// # Return values
    /// * `<Val>` The number of linear varying load item(s) applied.
    /// * `-1` General error.
    pub fn get_linear_varying_load_count(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLinearVaryingLoadCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_count)
                }
                Err(e) => bail!("Error::Load::get_linear_varying_load_count: {}", e),
            }
        }
    }

    /// Returns parameters for defining linear varying loads for specified beam.
    /// # Parameters
    /// * `[in] nBeamNo` Beam number ID.
    ///
    /// # Return values
    /// * `0` OK
    /// * `-1` General error.
    ///
    /// Returns tuple of (result_code, directions, w1_values, w2_values, w3_values)
    pub fn get_linear_varying_loads(
        &self,
        beam_no: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>, Vec<f64>, Vec<f64>), anyErr> {
        unsafe {
            let load_count = self.get_linear_varying_load_count(beam_no)?;
            if load_count <= 0 {
                return anyOk((load_count, Vec::new(), Vec::new(), Vec::new(), Vec::new()));
            }

            let mut psa_dir = SafeArrayCreateVector(VT_I4, 0, load_count as u32);
            let mut psa_w1 = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_w2 = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_w3 = SafeArrayCreateVector(VT_R8, 0, load_count as u32);

            let psa_dir_ptr = &mut psa_dir as *mut *mut SAFEARRAY;
            let psa_w1_ptr = &mut psa_w1 as *mut *mut SAFEARRAY;
            let psa_w2_ptr = &mut psa_w2 as *mut *mut SAFEARRAY;
            let psa_w3_ptr = &mut psa_w3 as *mut *mut SAFEARRAY;

            let variant_dir = variant_with_ptr_from::<SafeArrayP<i32>>(psa_dir_ptr);
            let variant_w1 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_w1_ptr);
            let variant_w2 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_w2_ptr);
            let variant_w3 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_w3_ptr);

            let mut params = [
                variant_w3,
                variant_w2,
                variant_w1,
                variant_dir,
                VARIANT::from(beam_no),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLinearVaryingLoads",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();

                    let dir_safe_arr = *params[3].Anonymous.Anonymous.Anonymous.pparray;
                    let w1_safe_arr = *params[2].Anonymous.Anonymous.Anonymous.pparray;
                    let w2_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let w3_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut dir_arr = Vec::with_capacity(load_count as usize);
                    let mut w1_arr = Vec::with_capacity(load_count as usize);
                    let mut w2_arr = Vec::with_capacity(load_count as usize);
                    let mut w3_arr = Vec::with_capacity(load_count as usize);

                    for i in 0..load_count {
                        let mut index = i as i32;
                        let mut dir_val = 0;
                        let mut w1_val = 0.0;
                        let mut w2_val = 0.0;
                        let mut w3_val = 0.0;

                        let _ = SafeArrayGetElement(
                            dir_safe_arr,
                            &mut index,
                            &mut dir_val as *mut i32 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            w1_safe_arr,
                            &mut index,
                            &mut w1_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            w2_safe_arr,
                            &mut index,
                            &mut w2_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            w3_safe_arr,
                            &mut index,
                            &mut w3_val as *mut f64 as *mut c_void,
                        )?;

                        dir_arr.push(dir_val);
                        w1_arr.push(w1_val);
                        w2_arr.push(w2_val);
                        w3_arr.push(w3_val);
                    }

                    anyOk((result_code, dir_arr, w1_arr, w2_arr, w3_arr))
                }
                Err(e) => bail!("Error::Load::get_linear_varying_loads: {}", e),
            }
        }
    }

    /// Gets member load(s) information generated by specified load item in specified load case.
    /// # Parameters
    /// * `[in] loadIndex` Load item index (Zero based).
    ///
    /// # Return values
    /// * `false` Failed
    /// * `true` Success
    ///
    /// Returns tuple of (success, direction, force_params, distance_params)
    pub fn get_member_load_info(
        &self,
        load_index: i32,
    ) -> Result<(bool, i32, Vec<f64>, Vec<f64>), anyErr> {
        unsafe {
            let dir_ptr = &mut 0i32 as *mut i32;
            let variant_dir = variant_with_ptr_from::<i32>(dir_ptr);

            let mut psa_force = SafeArrayCreateVector(VT_R8, 0, 3);
            let psa_force_ptr = &mut psa_force as *mut *mut SAFEARRAY;
            let variant_force = variant_with_ptr_from::<SafeArrayP<f64>>(psa_force_ptr);

            let mut psa_dist = SafeArrayCreateVector(VT_R8, 0, 3);
            let psa_dist_ptr = &mut psa_dist as *mut *mut SAFEARRAY;
            let variant_dist = variant_with_ptr_from::<SafeArrayP<f64>>(psa_dist_ptr);

            let mut params = [
                variant_dist,
                variant_force,
                variant_dir,
                VARIANT::from(load_index),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetMemberLoadInfo",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    let direction = *dir_ptr;

                    let force_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let dist_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut force_arr = Vec::with_capacity(3);
                    let mut dist_arr = Vec::with_capacity(3);

                    for i in 0..3 {
                        let mut index = i as i32;
                        let mut force_val = 0.0;
                        let mut dist_val = 0.0;

                        let _ = SafeArrayGetElement(
                            force_safe_arr,
                            &mut index,
                            &mut force_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            dist_safe_arr,
                            &mut index,
                            &mut dist_val as *mut f64 as *mut c_void,
                        )?;

                        force_arr.push(force_val);
                        dist_arr.push(dist_val);
                    }

                    anyOk((result == 1, direction, force_arr, dist_arr))
                }
                Err(e) => bail!("Error::Load::get_member_load_info: {}", e),
            }
        }
    }

    /// Get number of trapezoidal load(s) present for the specified beam.
    /// # Parameters
    /// * `[in] nBeamNo` Beam number ID.
    ///
    /// # Return values
    /// * `<Val>` The number of trapezoidal load item(s) applied.
    /// * `-1` General error.
    pub fn get_trap_load_count(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetTrapLoadCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_count)
                }
                Err(e) => bail!("Error::Load::get_trap_load_count: {}", e),
            }
        }
    }

    /// Returns the trapezoidal load(s) with all the parameters for the specified member.
    /// # Parameters
    /// * `[in] nBeamNo` Beam number ID.
    ///
    /// # Return values
    /// * `0` OK
    /// * `-1` General error.
    ///
    /// Returns tuple of (result_code, directions, w1_values, w2_values, d1_values, d2_values)
    pub fn get_trap_loads(
        &self,
        beam_no: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), anyErr> {
        unsafe {
            let load_count = self.get_trap_load_count(beam_no)?;
            if load_count <= 0 {
                return anyOk((
                    load_count,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ));
            }

            let mut psa_dir = SafeArrayCreateVector(VT_I4, 0, load_count as u32);
            let mut psa_w1 = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_w2 = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_d1 = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_d2 = SafeArrayCreateVector(VT_R8, 0, load_count as u32);

            let psa_dir_ptr = &mut psa_dir as *mut *mut SAFEARRAY;
            let psa_w1_ptr = &mut psa_w1 as *mut *mut SAFEARRAY;
            let psa_w2_ptr = &mut psa_w2 as *mut *mut SAFEARRAY;
            let psa_d1_ptr = &mut psa_d1 as *mut *mut SAFEARRAY;
            let psa_d2_ptr = &mut psa_d2 as *mut *mut SAFEARRAY;

            let variant_dir = variant_with_ptr_from::<SafeArrayP<i32>>(psa_dir_ptr);
            let variant_w1 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_w1_ptr);
            let variant_w2 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_w2_ptr);
            let variant_d1 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_d1_ptr);
            let variant_d2 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_d2_ptr);

            let mut params = [
                variant_d2,
                variant_d1,
                variant_w2,
                variant_w1,
                variant_dir,
                VARIANT::from(beam_no),
            ];

            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "GetTrapLoads", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();

                    let dir_safe_arr = *params[4].Anonymous.Anonymous.Anonymous.pparray;
                    let w1_safe_arr = *params[3].Anonymous.Anonymous.Anonymous.pparray;
                    let w2_safe_arr = *params[2].Anonymous.Anonymous.Anonymous.pparray;
                    let d1_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let d2_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut dir_arr = Vec::with_capacity(load_count as usize);
                    let mut w1_arr = Vec::with_capacity(load_count as usize);
                    let mut w2_arr = Vec::with_capacity(load_count as usize);
                    let mut d1_arr = Vec::with_capacity(load_count as usize);
                    let mut d2_arr = Vec::with_capacity(load_count as usize);

                    for i in 0..load_count {
                        let mut index = i as i32;
                        let mut dir_val = 0;
                        let mut w1_val = 0.0;
                        let mut w2_val = 0.0;
                        let mut d1_val = 0.0;
                        let mut d2_val = 0.0;

                        let _ = SafeArrayGetElement(
                            dir_safe_arr,
                            &mut index,
                            &mut dir_val as *mut i32 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            w1_safe_arr,
                            &mut index,
                            &mut w1_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            w2_safe_arr,
                            &mut index,
                            &mut w2_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            d1_safe_arr,
                            &mut index,
                            &mut d1_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            d2_safe_arr,
                            &mut index,
                            &mut d2_val as *mut f64 as *mut c_void,
                        )?;

                        dir_arr.push(dir_val);
                        w1_arr.push(w1_val);
                        w2_arr.push(w2_val);
                        d1_arr.push(d1_val);
                        d2_arr.push(d2_val);
                    }

                    anyOk((result_code, dir_arr, w1_arr, w2_arr, d1_arr, d2_arr))
                }
                Err(e) => bail!("Error::Load::get_trap_loads: {}", e),
            }
        }
    }

    /// Gets the number of uniformly distributed load(s) present for the specified beam.
    /// # Parameters
    /// * `[in] nBeamNo` The beam number ID.
    ///
    /// # Return values
    /// * `<Val>` The number of uniformly distributed load item(s) applied.
    /// * `-1` General error.
    pub fn get_udl_load_count(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetUDLLoadCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_count)
                }
                Err(e) => bail!("Error::Load::get_udl_load_count: {}", e),
            }
        }
    }

    /// Returns the uniformly distributed load(s) with all the parameters for the specified member.
    /// # Parameters
    /// * `[in] nBeamNo` Beam number ID.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    ///
    /// Returns tuple of (result_code, directions, forces, d1_values, d2_values, d3_values)
    pub fn get_udl_loads(
        &self,
        beam_no: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), anyErr> {
        unsafe {
            let load_count = self.get_udl_load_count(beam_no)?;
            if load_count <= 0 {
                return anyOk((
                    load_count,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ));
            }

            let mut psa_dir = SafeArrayCreateVector(VT_I4, 0, load_count as u32);
            let mut psa_force = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_d1 = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_d2 = SafeArrayCreateVector(VT_R8, 0, load_count as u32);
            let mut psa_d3 = SafeArrayCreateVector(VT_R8, 0, load_count as u32);

            let psa_dir_ptr = &mut psa_dir as *mut *mut SAFEARRAY;
            let psa_force_ptr = &mut psa_force as *mut *mut SAFEARRAY;
            let psa_d1_ptr = &mut psa_d1 as *mut *mut SAFEARRAY;
            let psa_d2_ptr = &mut psa_d2 as *mut *mut SAFEARRAY;
            let psa_d3_ptr = &mut psa_d3 as *mut *mut SAFEARRAY;

            let variant_dir = variant_with_ptr_from::<SafeArrayP<i32>>(psa_dir_ptr);
            let variant_force = variant_with_ptr_from::<SafeArrayP<f64>>(psa_force_ptr);
            let variant_d1 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_d1_ptr);
            let variant_d2 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_d2_ptr);
            let variant_d3 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_d3_ptr);

            let mut params = [
                variant_d3,
                variant_d2,
                variant_d1,
                variant_force,
                variant_dir,
                VARIANT::from(beam_no),
            ];

            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "GetUDLLoads", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();

                    let dir_safe_arr = *params[4].Anonymous.Anonymous.Anonymous.pparray;
                    let force_safe_arr = *params[3].Anonymous.Anonymous.Anonymous.pparray;
                    let d1_safe_arr = *params[2].Anonymous.Anonymous.Anonymous.pparray;
                    let d2_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let d3_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut dir_arr = Vec::with_capacity(load_count as usize);
                    let mut force_arr = Vec::with_capacity(load_count as usize);
                    let mut d1_arr = Vec::with_capacity(load_count as usize);
                    let mut d2_arr = Vec::with_capacity(load_count as usize);
                    let mut d3_arr = Vec::with_capacity(load_count as usize);

                    for i in 0..load_count {
                        let mut index = i as i32;
                        let mut dir_val = 0;
                        let mut force_val = 0.0;
                        let mut d1_val = 0.0;
                        let mut d2_val = 0.0;
                        let mut d3_val = 0.0;

                        let _ = SafeArrayGetElement(
                            dir_safe_arr,
                            &mut index,
                            &mut dir_val as *mut i32 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            force_safe_arr,
                            &mut index,
                            &mut force_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            d1_safe_arr,
                            &mut index,
                            &mut d1_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            d2_safe_arr,
                            &mut index,
                            &mut d2_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            d3_safe_arr,
                            &mut index,
                            &mut d3_val as *mut f64 as *mut c_void,
                        )?;

                        dir_arr.push(dir_val);
                        force_arr.push(force_val);
                        d1_arr.push(d1_val);
                        d2_arr.push(d2_val);
                        d3_arr.push(d3_val);
                    }

                    anyOk((result_code, dir_arr, force_arr, d1_arr, d2_arr, d3_arr))
                }
                Err(e) => bail!("Error::Load::get_udl_loads: {}", e),
            }
        }
    }

    /// Gets the count of uniformly distributed (UNI) moment applied to the specified member.
    /// # Parameters
    /// * `[in] nBeamNo` Beam number ID.
    ///
    /// # Return values
    /// * `<Val>` The number of uniformly distributed (UNI) moment item(s) applied.
    /// * `-1` General error.
    pub fn get_uni_moment_count(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetUNIMomentCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_count)
                }
                Err(e) => bail!("Error::Load::get_uni_moment_count: {}", e),
            }
        }
    }

    /// Returns the uniformly distributed (UNI) moments with all the parameters for the specified member.
    /// # Parameters
    /// * `[in] nBeamNo` The beam number ID.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    ///
    /// Returns tuple of (result_code, directions, moments, d1_values, d2_values, d3_values)
    pub fn get_uni_moments(
        &self,
        beam_no: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), anyErr> {
        unsafe {
            let moment_count = self.get_uni_moment_count(beam_no)?;
            if moment_count <= 0 {
                return anyOk((
                    moment_count,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ));
            }

            let mut psa_dir = SafeArrayCreateVector(VT_I4, 0, moment_count as u32);
            let mut psa_moment = SafeArrayCreateVector(VT_R8, 0, moment_count as u32);
            let mut psa_d1 = SafeArrayCreateVector(VT_R8, 0, moment_count as u32);
            let mut psa_d2 = SafeArrayCreateVector(VT_R8, 0, moment_count as u32);
            let mut psa_d3 = SafeArrayCreateVector(VT_R8, 0, moment_count as u32);

            let psa_dir_ptr = &mut psa_dir as *mut *mut SAFEARRAY;
            let psa_moment_ptr = &mut psa_moment as *mut *mut SAFEARRAY;
            let psa_d1_ptr = &mut psa_d1 as *mut *mut SAFEARRAY;
            let psa_d2_ptr = &mut psa_d2 as *mut *mut SAFEARRAY;
            let psa_d3_ptr = &mut psa_d3 as *mut *mut SAFEARRAY;

            let variant_dir = variant_with_ptr_from::<SafeArrayP<i32>>(psa_dir_ptr);
            let variant_moment = variant_with_ptr_from::<SafeArrayP<f64>>(psa_moment_ptr);
            let variant_d1 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_d1_ptr);
            let variant_d2 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_d2_ptr);
            let variant_d3 = variant_with_ptr_from::<SafeArrayP<f64>>(psa_d3_ptr);

            let mut params = [
                variant_d3,
                variant_d2,
                variant_d1,
                variant_moment,
                variant_dir,
                VARIANT::from(beam_no),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetUNIMoments",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();

                    let dir_safe_arr = *params[4].Anonymous.Anonymous.Anonymous.pparray;
                    let moment_safe_arr = *params[3].Anonymous.Anonymous.Anonymous.pparray;
                    let d1_safe_arr = *params[2].Anonymous.Anonymous.Anonymous.pparray;
                    let d2_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let d3_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut dir_arr = Vec::with_capacity(moment_count as usize);
                    let mut moment_arr = Vec::with_capacity(moment_count as usize);
                    let mut d1_arr = Vec::with_capacity(moment_count as usize);
                    let mut d2_arr = Vec::with_capacity(moment_count as usize);
                    let mut d3_arr = Vec::with_capacity(moment_count as usize);

                    for i in 0..moment_count {
                        let mut index = i as i32;
                        let mut dir_val = 0;
                        let mut moment_val = 0.0;
                        let mut d1_val = 0.0;
                        let mut d2_val = 0.0;
                        let mut d3_val = 0.0;

                        let _ = SafeArrayGetElement(
                            dir_safe_arr,
                            &mut index,
                            &mut dir_val as *mut i32 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            moment_safe_arr,
                            &mut index,
                            &mut moment_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            d1_safe_arr,
                            &mut index,
                            &mut d1_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            d2_safe_arr,
                            &mut index,
                            &mut d2_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            d3_safe_arr,
                            &mut index,
                            &mut d3_val as *mut f64 as *mut c_void,
                        )?;

                        dir_arr.push(dir_val);
                        moment_arr.push(moment_val);
                        d1_arr.push(d1_val);
                        d2_arr.push(d2_val);
                        d3_arr.push(d3_val);
                    }

                    anyOk((result_code, dir_arr, moment_arr, d1_arr, d2_arr, d3_arr))
                }
                Err(e) => bail!("Error::Load::get_uni_moments: {}", e),
            }
        }
    }
    /// Automatically finds enclosed panels in the given boundary and adds a FLOOR LOAD.
    /// Generated floor load is applied only in the Global X direction with YRANGE option.
    /// # Parameters
    /// * `[in] varPressure` Magnitude of the pressure or concentrate load on the element.
    /// * `[in] varYMIN` Y range from which the load start (in global coordinate).
    /// * `[in] varYMAX` Y range at which the load end (in global coordinate).
    /// * `[in] varZMIN` Z range from which the load start (in global coordinate).
    /// * `[in] varZMAX` Z range at which the load end (in global coordinate).
    /// * `[in] varXMIN` X range from which the load start (in global coordinate).
    /// * `[in] varXMAX` X range at which the load end (in global coordinate).
    ///
    /// # Return values
    /// * `1` OK.
    /// * `0` General error.
    /// * `-8001` Load direction is invalid.
    pub fn add_member_floor_load(
        &self,
        pressure: f64,
        y_min: f64,
        y_max: f64,
        z_min: f64,
        z_max: f64,
        x_min: f64,
        x_max: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(x_max),
                VARIANT::from(x_min),
                VARIANT::from(z_max),
                VARIANT::from(z_min),
                VARIANT::from(y_max),
                VARIANT::from(y_min),
                VARIANT::from(pressure),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddMemberFloorLoad",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_member_floor_load: {}", e),
            }
        }
    }

    /// Automatically finds enclosed panels in the given boundary and adds a FLOOR LOAD with extended options.
    /// # Parameters
    /// * `[in] varRange` Type of the Range:
    ///   - `FloorLoadRangeType::XRange` (0) - X-RANGE
    ///   - `FloorLoadRangeType::YRange` (1) - Y-RANGE
    ///   - `FloorLoadRangeType::ZRange` (2) - Z-RANGE
    ///   - `FloorLoadRangeType::GroupLoad` (3) - Group Load
    /// * `[in] varDirection` Load direction:
    ///   - `FloorLoadDirection::GlobalX` (0) - Global X
    ///   - `FloorLoadDirection::GlobalY` (1) - Global Y
    ///   - `FloorLoadDirection::GlobalZ` (2) - Global Z
    /// * `[in] dPressure` Magnitude of the pressure or concentrate load on the element.
    /// * `[in] varGrpOrOneWay` One-Way Load (if empty or "0") or group name for Floor Group Load.
    /// * `[in] dYMIN` Y range from which the load start (in global coordinate).
    /// * `[in] dYMAX` Y range at which the load end (in global coordinate).
    /// * `[in] dZMIN` Z range from which the load start (in global coordinate).
    /// * `[in] dZMAX` Z range at which the load end (in global coordinate).
    /// * `[in] dXMIN` X range from which the load start (in global coordinate).
    /// * `[in] dXMAX` X range at which the load end (in global coordinate).
    ///
    /// # Return values
    /// * `1` OK.
    /// * `0` General error.
    pub fn add_member_floor_load_ex(
        &self,
        range_type: i32,
        direction: i32,
        pressure: f64,
        grp_or_one_way: &str,
        y_min: f64,
        y_max: f64,
        z_min: f64,
        z_max: f64,
        x_min: f64,
        x_max: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(x_max),
                VARIANT::from(x_min),
                VARIANT::from(z_max),
                VARIANT::from(z_min),
                VARIANT::from(y_max),
                VARIANT::from(y_min),
                VARIANT::from(grp_or_one_way),
                VARIANT::from(pressure),
                VARIANT::from(direction),
                VARIANT::from(range_type),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddMemberFloorLoadEx",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_member_floor_load_ex: {}", e),
            }
        }
    }

    /// Get the beam count at the specific floor.
    /// # Parameters
    /// * `[in] fMinX` X range start (in global coordinate).
    /// * `[in] fMaxX` X range end (in global coordinate).
    /// * `[in] fMinY` Y range start (in global coordinate).
    /// * `[in] fMaxY` Y range end (in global coordinate).
    /// * `[in] fMinZ` Z range start (in global coordinate).
    /// * `[in] fMaxZ` Z range end (in global coordinate).
    /// * `[in] nDirection` Direction:
    ///   - `FloorDirection::XRange` (1) - for XRange
    ///   - `FloorDirection::YRange` (2) - for YRange
    ///   - `FloorDirection::ZRange` (3) - for ZRange
    ///
    /// # Returns
    /// The beam count at the specific floor.
    pub fn get_beam_count_at_floor(
        &self,
        min_x: f64,
        max_x: f64,
        min_y: f64,
        max_y: f64,
        min_z: f64,
        max_z: f64,
        direction: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(direction),
                VARIANT::from(max_z),
                VARIANT::from(min_z),
                VARIANT::from(max_y),
                VARIANT::from(min_y),
                VARIANT::from(max_x),
                VARIANT::from(min_x),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetBeamCountAtFloor",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_count)
                }
                Err(e) => bail!("Error::Load::get_beam_count_at_floor: {}", e),
            }
        }
    }

    /// Get Influence Area at the specific floor.
    /// # Parameters
    /// * `[in] fMinX` X range start (in global coordinate).
    /// * `[in] fMaxX` X range end (in global coordinate).
    /// * `[in] fMinY` Y range start (in global coordinate).
    /// * `[in] fMaxY` Y range end (in global coordinate).
    /// * `[in] fMinZ` Z range start (in global coordinate).
    /// * `[in] fMaxZ` Z range end (in global coordinate).
    /// * `[in] nDirection` Direction:
    ///   - `FloorDirection::XRange` (1) - for XRange
    ///   - `FloorDirection::YRange` (2) - for YRange
    ///   - `FloorDirection::ZRange` (3) - for ZRange
    ///
    /// # Return values
    /// * `true` Get Influence Area Successful.
    /// * `false` Generate Error.
    ///
    /// Returns tuple of (success, beam_numbers, areas)
    pub fn get_influence_area(
        &self,
        min_x: f64,
        max_x: f64,
        min_y: f64,
        max_y: f64,
        min_z: f64,
        max_z: f64,
        direction: i32,
    ) -> Result<(bool, Vec<i32>, Vec<f64>), anyErr> {
        unsafe {
            let beam_count =
                self.get_beam_count_at_floor(min_x, max_x, min_y, max_y, min_z, max_z, direction)?;
            if beam_count <= 0 {
                return anyOk((false, Vec::new(), Vec::new()));
            }

            let mut psa_beams = SafeArrayCreateVector(VT_I4, 0, beam_count as u32);
            let psa_beams_ptr = &mut psa_beams as *mut *mut SAFEARRAY;
            let variant_beams = variant_with_ptr_from::<SafeArrayP<i32>>(psa_beams_ptr);

            let mut psa_areas = SafeArrayCreateVector(VT_R8, 0, beam_count as u32);
            let psa_areas_ptr = &mut psa_areas as *mut *mut SAFEARRAY;
            let variant_areas = variant_with_ptr_from::<SafeArrayP<f64>>(psa_areas_ptr);

            let mut params = [
                variant_areas,
                variant_beams,
                VARIANT::from(direction),
                VARIANT::from(max_z),
                VARIANT::from(min_z),
                VARIANT::from(max_y),
                VARIANT::from(min_y),
                VARIANT::from(max_x),
                VARIANT::from(min_x),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetInfluenceArea",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();

                    let beams_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let areas_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut beam_arr = Vec::with_capacity(beam_count as usize);
                    let mut area_arr = Vec::with_capacity(beam_count as usize);

                    for i in 0..beam_count {
                        let mut index = i as i32;
                        let mut beam_val = 0;
                        let mut area_val = 0.0;

                        let _ = SafeArrayGetElement(
                            beams_safe_arr,
                            &mut index as *mut i32,
                            &mut beam_val as *mut i32 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            areas_safe_arr,
                            &mut index as *mut i32,
                            &mut area_val as *mut f64 as *mut c_void,
                        )?;

                        beam_arr.push(beam_val);
                        area_arr.push(area_val);
                    }

                    anyOk((result == 1, beam_arr, area_arr))
                }
                Err(e) => bail!("Error::Load::get_influence_area: {}", e),
            }
        }
    }

    /// Adds a seismic load input direction and factor.
    /// # Parameters
    /// * `[in] varDirection` Load direction:
    ///   - `SeismicDirection::GlobalX` (0) - Global X direction
    ///   - `SeismicDirection::GlobalY` (1) - Global Y direction
    ///   - `SeismicDirection::GlobalZ` (2) - Global Z direction
    /// * `[in] varFactor` Multiplication factor to be used to multiply the seismic load.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    /// * `-8001` Load direction is invalid.
    pub fn add_seismic_load(&self, direction: i32, factor: f64) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(factor), VARIANT::from(direction)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddSeismicLoad",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_seismic_load: {}", e),
            }
        }
    }

    /// Checks if dynamic load included in specified load case.
    /// # Parameters
    /// * `[in] nLoadCase` Load case reference ID.
    ///
    /// # Return values
    /// * `true` YES (dynamic load is included).
    /// * `false` NO (dynamic load is not included).
    ///
    /// Returns `Err` if general error occurs.
    pub fn is_dynamic_load_included(&self, load_case: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_case)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "IsDynamicLoadIncluded",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    match result {
                        1 => anyOk(true),
                        0 => anyOk(false),
                        -1 => bail!("General error checking dynamic load inclusion"),
                        _ => bail!("Unexpected return value: {}", result),
                    }
                }
                Err(e) => bail!("Error::Load::is_dynamic_load_included: {}", e),
            }
        }
    }

    /// Creates a Notional load case using combinations of previously defined primary load cases and Reference load cases.
    /// # Parameters
    /// * `[in] varPrimaryLoadCaseList` Primary load case reference number ID(s) array.
    /// * `[in] varPLFactorList` Multiplication factor array of Primary load cases.
    /// * `[in] varPLDirectionList` Direction of Primary load cases (1=X, 2=Y, 3=Z).
    /// * `[in] varReferenceLoadCaseList` Reference load case reference number ID(s) array.
    /// * `[in] varRLFactorList` Multiplication factor array of Reference load cases.
    /// * `[in] varRLDirectionList` Direction of Reference load cases (1=X, 2=Y, 3=Z).
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_notional_load(
        &self,
        primary_load_cases: Vec<i32>,
        pl_factors: Vec<f64>,
        pl_directions: Vec<i32>,
        reference_load_cases: Vec<i32>,
        rl_factors: Vec<f64>,
        rl_directions: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_primary_cases = safe_array_from_vec1d::<i32>(primary_load_cases)?;
            let variant_primary_cases =
                variant_with_ptr_from::<SafeArray<i32>>(sa_primary_cases);

            let sa_pl_factors = safe_array_from_vec1d::<f64>(pl_factors)?;
            let variant_pl_factors = variant_with_ptr_from::<SafeArray<f64>>(sa_pl_factors);

            let sa_pl_directions = safe_array_from_vec1d::<i32>(pl_directions)?;
            let variant_pl_directions =
                variant_with_ptr_from::<SafeArray<i32>>(sa_pl_directions);

            let sa_reference_cases = safe_array_from_vec1d::<i32>(reference_load_cases)?;
            let variant_reference_cases =
                variant_with_ptr_from::<SafeArray<i32>>(sa_reference_cases);

            let sa_rl_factors = safe_array_from_vec1d::<f64>(rl_factors)?;
            let variant_rl_factors = variant_with_ptr_from::<SafeArray<f64>>(sa_rl_factors);

            let sa_rl_directions = safe_array_from_vec1d::<i32>(rl_directions)?;
            let variant_rl_directions =
                variant_with_ptr_from::<SafeArray<i32>>(sa_rl_directions);

            let mut params = [
                variant_rl_directions,
                variant_rl_factors,
                variant_reference_cases,
                variant_pl_directions,
                variant_pl_factors,
                variant_primary_cases,
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddNotionalLoad",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_notional_load: {}", e),
            }
        }
    }

    /// Adds a reference load item to current active load case in Load Cases Details.
    /// # Parameters
    /// * `[in] varRefLoadCaseList` Array of reference load case number ID(s) from Reference Load Definitions.
    /// * `[in] varFactorList` Factor(s) array.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_reference_load(
        &self,
        ref_load_cases: Vec<i32>,
        factors: Vec<f64>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_ref_cases = safe_array_from_vec1d::<i32>(ref_load_cases)?;
            let variant_ref_cases = variant_with_ptr_from::<SafeArray<i32>>(sa_ref_cases);

            let sa_factors = safe_array_from_vec1d::<f64>(factors)?;
            let variant_factors = variant_with_ptr_from::<SafeArray<f64>>(sa_factors);

            let mut params = [variant_factors, variant_ref_cases];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddReferenceLoad",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_reference_load: {}", e),
            }
        }
    }

    /// Creates a primary load case using combinations of previously defined primary load cases.
    /// # Parameters
    /// * `[in] varLoadCaseList` Primary load case reference number ID(s) array.
    /// * `[in] varFactorList` Multiplication factor array.
    ///
    /// # Return values
    /// * `1` If Load Case is added successfully.
    /// * `0` Otherwise.
    pub fn add_repeat_load(&self, load_cases: Vec<i32>, factors: Vec<f64>) -> Result<bool, anyErr> {
        unsafe {
            let sa_load_cases = safe_array_from_vec1d::<i32>(load_cases)?;
            let variant_load_cases = variant_with_ptr_from::<SafeArray<i32>>(sa_load_cases);

            let sa_factors = safe_array_from_vec1d::<f64>(factors)?;
            let variant_factors = variant_with_ptr_from::<SafeArray<f64>>(sa_factors);

            let mut params = [variant_factors, variant_load_cases];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddRepeatLoad",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Load::add_repeat_load: {}", e),
            }
        }
    }

    /// Begin Load Merging.
    pub fn begin_load_merging(&self) -> Result<(), anyErr> {
        let result_variant =
            unsafe { invoke_method(self.dispatch.as_ref().unwrap(), "BeginLoadMerging", &mut []) };
        match result_variant {
            Ok(_) => anyOk(()),
            Err(e) => bail!("Error::Load::begin_load_merging: {}", e),
        }
    }

    /// End Load Merging.
    pub fn end_load_merging(&self) -> Result<(), anyErr> {
        let result_variant =
            unsafe { invoke_method(self.dispatch.as_ref().unwrap(), "EndLoadMerging", &mut []) };
        match result_variant {
            Ok(_) => anyOk(()),
            Err(e) => bail!("Error::Load::end_load_merging: {}", e),
        }
    }

    /// Gets the no of factor for specified Notional load.
    /// # Parameters
    /// * `[in] nIndex` The index for Notional load.
    ///
    /// # Return values
    /// * `<Val>` The factor for specified Notional load.
    /// * `-1` General error.
    pub fn get_no_load_factor_direction_in_notional_load(&self, index: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(index)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetNoLoadFactorDirectionInNotionalLoad",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result)
                }
                Err(e) => bail!(
                    "Error::Load::get_no_load_factor_direction_in_notional_load: {}",
                    e
                ),
            }
        }
    }

    /// Returns the number of load and factor pairs associated with a given repeat load command in the active load case.
    /// # Parameters
    /// * `[in] nIndex` The index (One based) for repeat load.
    ///
    /// # Return values
    /// * `<Val>` Number of load and factor pairs associated with a given repeat load command.
    /// * `-1` In case of invalid repeat load index.
    pub fn get_no_load_factor_in_repeat_load(&self, index: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(index)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetNoLoadFactorInRepeatLoad",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result)
                }
                Err(e) => bail!("Error::Load::get_no_load_factor_in_repeat_load: {}", e),
            }
        }
    }

    /// Returns the number of reference load case - factor set(s) in specified reference load item.
    /// # Parameters
    /// * `[in] nIndex` The index for reference load item.
    ///
    /// # Return values
    /// * `<Val>` Reference load case reference ID.
    /// * `-1` General error.
    pub fn get_no_of_sets_in_reference_load(&self, index: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(index)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetNoOfSetsInReferenceLoad",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result)
                }
                Err(e) => bail!("Error::Load::get_no_of_sets_in_reference_load: {}", e),
            }
        }
    }

    /// Gets load case(s), direction(s) and factor(s) for specified Notional load.
    /// # Parameters
    /// * `[in] nIndex` The index for Notional load.
    ///
    /// # Return values
    /// * `<Val>` The size of arrays.
    /// * `-1` General error.
    ///
    /// Returns tuple of (size, load_cases, factors, directions)
    /// Note: +ve values in load_cases = Primary Load Cases, -ve values = Reference Load Cases
    pub fn get_notional_load_by_index(
        &self,
        index: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>, Vec<i32>), anyErr> {
        unsafe {
            let factor_count = self.get_no_load_factor_direction_in_notional_load(index)?;
            if factor_count <= 0 {
                return anyOk((factor_count, Vec::new(), Vec::new(), Vec::new()));
            }

            let mut psa_loads = SafeArrayCreateVector(VT_I4, 0, factor_count as u32);
            let psa_loads_ptr = &mut psa_loads as *mut *mut SAFEARRAY;
            let variant_loads = variant_with_ptr_from::<SafeArrayP<i32>>(psa_loads_ptr);

            let mut psa_factors = SafeArrayCreateVector(VT_R8, 0, factor_count as u32);
            let psa_factors_ptr = &mut psa_factors as *mut *mut SAFEARRAY;
            let variant_factors = variant_with_ptr_from::<SafeArrayP<f64>>(psa_factors_ptr);

            let mut psa_directions = SafeArrayCreateVector(VT_I4, 0, factor_count as u32);
            let psa_directions_ptr = &mut psa_directions as *mut *mut SAFEARRAY;
            let variant_directions =
                variant_with_ptr_from::<SafeArrayP<i32>>(psa_directions_ptr);

            let mut params = [
                variant_directions,
                variant_factors,
                variant_loads,
                VARIANT::from(index),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetNotionalLoadByIndex",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_size = VariantToInt32(&var as *const VARIANT).unwrap();

                    let loads_safe_arr = *params[2].Anonymous.Anonymous.Anonymous.pparray;
                    let factors_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let directions_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut loads_arr = Vec::with_capacity(factor_count as usize);
                    let mut factors_arr = Vec::with_capacity(factor_count as usize);
                    let mut directions_arr = Vec::with_capacity(factor_count as usize);

                    for i in 0..factor_count {
                        let mut index = i as i32;
                        let mut load_val = 0;
                        let mut factor_val = 0.0;
                        let mut direction_val = 0;

                        let _ = SafeArrayGetElement(
                            loads_safe_arr,
                            &mut index,
                            &mut load_val as *mut i32 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            factors_safe_arr,
                            &mut index,
                            &mut factor_val as *mut f64 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            directions_safe_arr,
                            &mut index,
                            &mut direction_val as *mut i32 as *mut c_void,
                        )?;

                        loads_arr.push(load_val);
                        factors_arr.push(factor_val);
                        directions_arr.push(direction_val);
                    }

                    anyOk((result_size, loads_arr, factors_arr, directions_arr))
                }
                Err(e) => bail!("Error::Load::get_notional_load_by_index: {}", e),
            }
        }
    }

    /// Returns the number of Notional load.
    /// # Return values
    /// * `<Val>` The number of Notional load.
    /// * `-1` General error.
    pub fn get_notional_load_count(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetNotionalLoadCount",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_count)
            }
            Err(e) => bail!("Error::Load::get_notional_load_count: {}", e),
        }
    }

    /// Gets reference load item: reference load case reference number ID(s) and corresponding factor(s) for specified reference load item.
    /// # Parameters
    /// * `[in] nIndex` The reference load and factor set number.
    ///
    /// # Return values
    /// * Reference load case reference number ID(s).
    /// * `-1` General error.
    ///
    /// Returns tuple of (result, load_cases, factors)
    pub fn get_reference_load_by_index(
        &self,
        index: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>), anyErr> {
        unsafe {
            let sets_count = self.get_no_of_sets_in_reference_load(index)?;
            if sets_count <= 0 {
                return anyOk((sets_count, Vec::new(), Vec::new()));
            }

            let mut psa_loads = SafeArrayCreateVector(VT_I4, 0, sets_count as u32);
            let psa_loads_ptr = &mut psa_loads as *mut *mut SAFEARRAY;
            let variant_loads = variant_with_ptr_from::<SafeArrayP<i32>>(psa_loads_ptr);

            let mut psa_factors = SafeArrayCreateVector(VT_R8, 0, sets_count as u32);
            let psa_factors_ptr = &mut psa_factors as *mut *mut SAFEARRAY;
            let variant_factors = variant_with_ptr_from::<SafeArrayP<f64>>(psa_factors_ptr);

            let mut params = [variant_factors, variant_loads, VARIANT::from(index)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetReferenceLoadByIndex",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();

                    let loads_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let factors_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut loads_arr = Vec::with_capacity(sets_count as usize);
                    let mut factors_arr = Vec::with_capacity(sets_count as usize);

                    for i in 0..sets_count {
                        let mut index = i as i32;
                        let mut load_val = 0;
                        let mut factor_val = 0.0;

                        let _ = SafeArrayGetElement(
                            loads_safe_arr,
                            &mut index,
                            &mut load_val as *mut i32 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            factors_safe_arr,
                            &mut index,
                            &mut factor_val as *mut f64 as *mut c_void,
                        )?;

                        loads_arr.push(load_val);
                        factors_arr.push(factor_val);
                    }

                    anyOk((result, loads_arr, factors_arr))
                }
                Err(e) => bail!("Error::Load::get_reference_load_by_index: {}", e),
            }
        }
    }

    /// Gets reference load Type Name.
    /// # Parameters
    /// * `[in] varLoadNo` The Reference Load No.
    ///
    /// # Return values
    /// Reference load Title.
    pub fn get_reference_load_case_title(&self, load_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetReferenceLoadCaseTitle",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(result)
                }
                Err(e) => bail!("Error::Load::get_reference_load_case_title: {}", e),
            }
        }
    }

    /// Returns the number of reference load case item(s) in current active load case.
    /// # Return values
    /// * `<Val>` The number of reference load case item(s).
    /// * `-1` General error.
    pub fn get_reference_load_count(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetReferenceLoadCount",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_count)
            }
            Err(e) => bail!("Error::Load::get_reference_load_count: {}", e),
        }
    }

    /// Gets reference load Type.
    /// # Parameters
    /// * `[in] varLoadNo` The Reference Load No.
    ///
    /// # Return values
    /// * `-1` General error.
    /// * Others (value can be 0 to 23) Reference load Type.
    pub fn get_reference_load_type(&self, load_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetReferenceLoadType",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result)
                }
                Err(e) => bail!("Error::Load::get_reference_load_type: {}", e),
            }
        }
    }

    /// Gets the list of load case IDs and load factors for a given repeat load command in the active load case.
    /// # Parameters
    /// * `[in] nIndex` The index for repeat load (One based).
    ///
    /// # Return values
    /// * `<Val>` Number of load and factor pairs associated with a given repeat load command.
    /// * `-1` Invalid repeat load index used.
    ///
    /// Returns tuple of (count, load_cases, factors)
    pub fn get_repeat_load_by_index(
        &self,
        index: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>), anyErr> {
        unsafe {
            let factor_count = self.get_no_load_factor_in_repeat_load(index)?;
            if factor_count <= 0 {
                return anyOk((factor_count, Vec::new(), Vec::new()));
            }

            let mut psa_loads = SafeArrayCreateVector(VT_I4, 0, factor_count as u32);
            let psa_loads_ptr = &mut psa_loads as *mut *mut SAFEARRAY;
            let variant_loads = variant_with_ptr_from::<SafeArrayP<i32>>(psa_loads_ptr);

            let mut psa_factors = SafeArrayCreateVector(VT_R8, 0, factor_count as u32);
            let psa_factors_ptr = &mut psa_factors as *mut *mut SAFEARRAY;
            let variant_factors = variant_with_ptr_from::<SafeArrayP<f64>>(psa_factors_ptr);

            let mut params = [variant_factors, variant_loads, VARIANT::from(index)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetRepeatLoadByIndex",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();

                    let loads_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let factors_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut loads_arr = Vec::with_capacity(factor_count as usize);
                    let mut factors_arr = Vec::with_capacity(factor_count as usize);

                    for i in 0..factor_count {
                        let mut index = i as i32;
                        let mut load_val = 0;
                        let mut factor_val = 0.0;

                        let _ = SafeArrayGetElement(
                            loads_safe_arr,
                            &mut index,
                            &mut load_val as *mut i32 as *mut c_void,
                        )?;
                        let _ = SafeArrayGetElement(
                            factors_safe_arr,
                            &mut index,
                            &mut factor_val as *mut f64 as *mut c_void,
                        )?;

                        loads_arr.push(load_val);
                        factors_arr.push(factor_val);
                    }

                    anyOk((result_count, loads_arr, factors_arr))
                }
                Err(e) => bail!("Error::Load::get_repeat_load_by_index: {}", e),
            }
        }
    }

    /// Returns the number of repeat load commands in the active load case.
    /// # Return values
    /// * `<Val>` The number of repeat load commands in the active load case.
    /// * `0` General error.
    pub fn get_repeat_load_count(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetRepeatLoadCount",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_count)
            }
            Err(e) => bail!("Error::Load::get_repeat_load_count: {}", e),
        }
    }
    /// Adds a wind load.
    /// # Parameters
    /// * `[in] varTypeNo` Wind Definition Type number ID.
    /// * `[in] varDirection` Wind load direction:
    ///   - `WindLoadDirection::GlobalX` (1) - Global X
    ///   - `WindLoadDirection::GlobalZ` (3) - Global Z
    ///   - `WindLoadDirection::GlobalNegativeX` (4) - Global -X
    ///   - `WindLoadDirection::GlobalNegativeZ` (6) - Global -Z
    /// * `[in] dFraction` Factor to be used to multiply the wind loads. Negative signs may be used to indicate opposite direction of resulting load (default=1.0).
    /// * `[in] varOpenStructure` Open-type of structure (true), closed-type of structure (false).
    /// * `[in] dYMIN` Ymin of GLOBAL Y range in which Wind load applied (assume Y axis is vertical).
    /// * `[in] dYMAX` Ymax of GLOBAL Y range in which Wind load applied (assume Y axis is vertical).
    /// * `[in] dZMIN` Zmin of GLOBAL Z range in which Wind load applied (assume Y axis is vertical).
    /// * `[in] dZMAX` Zmax of GLOBAL Z range in which Wind load applied (assume Y axis is vertical).
    /// * `[in] dXMIN` Xmin of GLOBAL X range in which Wind load applied (assume Y axis is vertical).
    /// * `[in] dXMAX` Xmax of GLOBAL X range in which Wind load applied (assume Y axis is vertical).
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_wind_load(
        &self,
        type_no: i32,
        direction: i32,
        fraction: f64,
        open_structure: bool,
        y_min: f64,
        y_max: f64,
        z_min: f64,
        z_max: f64,
        x_min: f64,
        x_max: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(x_max),
                VARIANT::from(x_min),
                VARIANT::from(z_max),
                VARIANT::from(z_min),
                VARIANT::from(y_max),
                VARIANT::from(y_min),
                VARIANT::from(open_structure),
                VARIANT::from(fraction),
                VARIANT::from(direction),
                VARIANT::from(type_no),
            ];

            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "AddWindLoad", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_wind_load: {}", e),
            }
        }
    }

    /// Automatically adds repeat load based on assigned design code and Category.
    /// # Parameters
    /// * `[in] varCode` Load Combination Code string name (refer to "Codes.ini").
    /// * `[in] varCategory` Load Combination Category string name (refer to corresponding rule ini file defined in "Codes.ini").
    /// * `[in] varLoadList` Load case reference ID(s), Array of Load case numbers. If empty, all load cases in current model will be considered.
    /// * `[in/out] varStartLoadCaseNo` (Repeat Load) load case reference ID with which automatically generation starts.
    /// * `[out] varGeneratedLCS` (Repeat Load) The counts of automatically generated repeat loads.
    /// * `[in] bVarReference` Whether include Reference load.
    /// * `[in] bVarNotional` Whether include Notional load. If it's True but all Directions are False, return -1.
    /// * `[in] dVarNotionalLoadFactor` If bVarNotional is valid, the value of Notional load factor.
    /// * `[in] bVarGB50017` Consider Notional load factor per GB 50017 Design code.
    /// * `[in] nVarFloor` The count of floor, it is valid when bVarGB50017 is True only.
    /// * `[in] bVarX` Consider X Direction of Notional Load.
    /// * `[in] bVarNegtiveX` Consider -X Direction of Notional Load.
    /// * `[in] bVarZ` Consider Z Direction of Notional Load.
    /// * `[in] bVarNegtiveZ` Consider -Z Direction of Notional Load.
    ///
    /// # Return values
    /// * `0` if successful.
    /// * `-1` if unsuccessful.
    ///
    /// Returns tuple of (result, start_load_case_no, generated_lcs_count)
    pub fn add_auto_combination_repeat(
        &self,
        code: &str,
        category: &str,
        load_list: Vec<i32>,
        mut start_load_case_no: i32,
        reference: bool,
        notional: bool,
        notional_load_factor: f64,
        gb50017: bool,
        floor_count: i32,
        x_direction: bool,
        negative_x: bool,
        z_direction: bool,
        negative_z: bool,
    ) -> Result<(i32, i32, i32), anyErr> {
        unsafe {
            let sa_load_list = safe_array_from_vec1d::<i32>(load_list)?;
            let variant_load_list = variant_with_ptr_from::<SafeArray<i32>>(sa_load_list);

            let start_ptr = &mut start_load_case_no as *mut i32;
            let variant_start = variant_with_ptr_from::<i32>(start_ptr);

            let generated_ptr = &mut 0i32 as *mut i32;
            let variant_generated = variant_with_ptr_from::<i32>(generated_ptr);

            let mut params = [
                VARIANT::from(negative_z),
                VARIANT::from(z_direction),
                VARIANT::from(negative_x),
                VARIANT::from(x_direction),
                VARIANT::from(floor_count),
                VARIANT::from(gb50017),
                VARIANT::from(notional_load_factor),
                VARIANT::from(notional),
                VARIANT::from(reference),
                variant_generated,
                variant_start,
                variant_load_list,
                VARIANT::from(category),
                VARIANT::from(code),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddAutoCombinationRepeat",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((result_code, *start_ptr, *generated_ptr))
                }
                Err(e) => bail!("Error::Load::add_auto_combination_repeat: {}", e),
            }
        }
    }

    /// Automatically adds load combination based on assigned design code and Category.
    /// # Parameters
    /// * `[in] varCode` Load Combination Code string name (refer to "Codes.ini").
    /// * `[in] varCategory` Load Combination Category string name (refer to corresponding rule ini file defined in "Codes.ini").
    /// * `[in] varLoadList` Load case reference ID(s), Array of Load case numbers. If empty, all load cases in current model will be considered.
    /// * `[in/out] varStartLoadCaseNo` (Combination) load case reference ID with which automatically load combination generation starts.
    ///
    /// # Return values
    /// * `0` if successful.
    /// * `-8002` load case not found.
    /// * `-8040` invalid load combination code name.
    /// * `-8041` invalid load combination category name.
    ///
    /// Returns tuple of (result, start_load_case_no)
    pub fn add_auto_load_combinations(
        &self,
        code: &str,
        category: &str,
        load_list: Vec<i32>,
        mut start_load_case_no: i32,
    ) -> Result<(i32, i32), anyErr> {
        unsafe {
            let sa_load_list = safe_array_from_vec1d::<i32>(load_list)?;
            let variant_load_list = variant_with_ptr_from::<SafeArray<i32>>(sa_load_list);

            let start_ptr = &mut start_load_case_no as *mut i32;
            let variant_start = variant_with_ptr_from::<i32>(start_ptr);

            let mut params = [
                variant_start,
                variant_load_list,
                VARIANT::from(category),
                VARIANT::from(code),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddAutoLoadCombinations",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((result_code, *start_ptr))
                }
                Err(e) => bail!("Error::Load::add_auto_load_combinations: {}", e),
            }
        }
    }

    /// Adds a primary load case with specified multiplication factor to an existing load combination.
    /// # Parameters
    /// * `[in] varLoadCombNo` (Combination) Load case reference number ID.
    /// * `[in] varLoadNo` (Primary) Load case reference number ID.
    /// * `[in] varFactor` Multiplication factor for the specified primary load case.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn add_load_and_factor_to_combination(
        &self,
        load_comb_no: i32,
        load_no: i32,
        factor: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(factor),
                VARIANT::from(load_no),
                VARIANT::from(load_comb_no),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddLoadAndFactorToCombination",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::add_load_and_factor_to_combination: {}", e),
            }
        }
    }

    /// Creates new load combination with the number and title defined.
    /// # Parameters
    /// * `[in] varLoadCombTitle` Load case string title.
    /// * `[in] varLoadCombNo` Load case reference number ID.
    ///
    /// # Return values
    /// * `<Val>` Number ID, assigned to new load combination.
    /// * `-1` General error.
    pub fn create_new_load_combination(
        &self,
        title: &str,
        load_comb_no: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_comb_no), VARIANT::from(title)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateNewLoadCombination",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_id)
                }
                Err(e) => bail!("Error::Load::create_new_load_combination: {}", e),
            }
        }
    }

    /// Gets load case reference number ID(s) and corresponding multiplication factor(s) for specified load combination.
    /// # Parameters
    /// * `[in] varLoadCombNo` (Combination) Load case reference number ID.
    ///
    /// # Return values
    /// * `true` if the method is successful.
    /// * `false` if the method is unsuccessful.
    ///
    /// Returns tuple of (success, load_nos, factors)
    /// Note: For SRSS, factors array size is no. of primary load cases + 1, where the last value is the overall multiplication factor.
    pub fn get_load_and_factor_for_combination(
        &self,
        load_comb_no: i32,
    ) -> Result<(bool, Vec<i32>, Vec<f64>), anyErr> {
        unsafe {
            let pairs_count = self.get_no_of_load_and_factor_pairs_for_combination(load_comb_no)?;
            if pairs_count <= 0 {
                return anyOk((false, Vec::new(), Vec::new()));
            }

            // For SRSS, need extra space for overall factor
            let factor_size = pairs_count + 1;

            let mut psa_loads = SafeArrayCreateVector(VT_I4, 0, pairs_count as u32);
            let psa_loads_ptr = &mut psa_loads as *mut *mut SAFEARRAY;
            let variant_loads = variant_with_ptr_from::<SafeArrayP<i32>>(psa_loads_ptr);

            let mut psa_factors = SafeArrayCreateVector(VT_R8, 0, factor_size as u32);
            let psa_factors_ptr = &mut psa_factors as *mut *mut SAFEARRAY;
            let variant_factors = variant_with_ptr_from::<SafeArrayP<f64>>(psa_factors_ptr);

            let mut params = [variant_factors, variant_loads, VARIANT::from(load_comb_no)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLoadAndFactorForCombination",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();

                    let loads_safe_arr = *params[1].Anonymous.Anonymous.Anonymous.pparray;
                    let factors_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;

                    let mut loads_arr = Vec::with_capacity(pairs_count as usize);
                    let mut factors_arr = Vec::with_capacity(factor_size as usize);

                    for i in 0..pairs_count {
                        let mut index = i as i32;
                        let mut load_val = 0;
                        let _ = SafeArrayGetElement(
                            loads_safe_arr,
                            &mut index,
                            &mut load_val as *mut i32 as *mut c_void,
                        )?;
                        loads_arr.push(load_val);
                    }

                    for i in 0..factor_size {
                        let mut index = i as i32;
                        let mut factor_val = 0.0;
                        let _ = SafeArrayGetElement(
                            factors_safe_arr,
                            &mut index,
                            &mut factor_val as *mut f64 as *mut c_void,
                        )?;
                        factors_arr.push(factor_val);
                    }

                    anyOk((result == 1, loads_arr, factors_arr))
                }
                Err(e) => bail!("Error::Load::get_load_and_factor_for_combination: {}", e),
            }
        }
    }

    /// Gets total number of combination load case(s) present in the current structure.
    /// # Returns
    /// The total number of combination load cases(s).
    pub fn get_load_combination_case_count(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLoadCombinationCaseCount",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_count)
            }
            Err(e) => bail!("Error::Load::get_load_combination_case_count: {}", e),
        }
    }

    /// Gets all load combination case number(s).
    /// # Return values
    /// * `<Val>` The number of primary load case(s).
    /// * `-1` General error.
    /// * `-106` 1 dimensional array of long expected.
    /// * `-114` OLE Exception Occurred.
    ///
    /// Returns tuple of (count, case_numbers)
    pub fn get_load_combination_case_numbers(&self) -> Result<(i32, Vec<i32>), anyErr> {
        unsafe {
            let case_count = self.get_load_combination_case_count()?;
            if case_count <= 0 {
                return anyOk((case_count, Vec::new()));
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, case_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_with_ptr_from::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLoadCombinationCaseNumbers",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    let cases_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut cases_arr = Vec::with_capacity(case_count as usize);

                    for i in 0..case_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            cases_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        cases_arr.push(value as i32);
                    }
                    anyOk((result_count, cases_arr))
                }
                Err(e) => bail!("Error::Load::get_load_combination_case_numbers: {}", e),
            }
        }
    }

    /// Gets the number of load case(s) applied with multiplication factor in specified load combination.
    /// # Parameters
    /// * `[in] varLoadCombNo` (Combination) Load case reference number ID.
    ///
    /// # Returns
    /// The number of load case(s) in specified load combination.
    pub fn get_no_of_load_and_factor_pairs_for_combination(
        &self,
        load_comb_no: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_comb_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetNoOfLoadAndFactorPairsForCombination",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_count)
                }
                Err(e) => bail!(
                    "Error::Load::get_no_of_load_and_factor_pairs_for_combination: {}",
                    e
                ),
            }
        }
    }

    /// Checks if specified load case is combination load case.
    /// # Parameters
    /// * `[in] nLoadCase` Load case reference ID.
    ///
    /// # Return values
    /// * `true` YES.
    /// * `false` NO.
    ///
    /// Returns `Err` if general error occurs.
    pub fn is_combination_case(&self, load_case: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_case)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "IsCombinationCase",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    match result {
                        1 => anyOk(true),
                        0 => anyOk(false),
                        -1 => bail!("General error checking combination case"),
                        _ => bail!("Unexpected return value: {}", result),
                    }
                }
                Err(e) => bail!("Error::Load::is_combination_case: {}", e),
            }
        }
    }

    /// Clears the load items in a specified Primary Load cases or Reference Load cases.
    /// # Parameters
    /// * `[in] varLoadCaseNos` Primary load case reference ID(s) array.
    /// * `[in] varIsReferenceLoads` If reference load case(s): true or false.
    ///
    /// # Return values
    /// * `true` OK.
    /// * `false` Failed to delete load(s).
    pub fn clear_primary_load_case(
        &self,
        load_case_nos: Vec<i32>,
        is_reference_loads: bool,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_load_cases = safe_array_from_vec1d::<i32>(load_case_nos)?;
            let variant_load_cases = variant_with_ptr_from::<SafeArray<i32>>(sa_load_cases);

            let mut params = [VARIANT::from(is_reference_loads), variant_load_cases];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "ClearPrimaryLoadCase",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Load::clear_primary_load_case: {}", e),
            }
        }
    }

    /// Clears the load items in a specified Reference Load cases.
    /// # Parameters
    /// * `[in] varLoadCaseNos` Reference load case reference ID(s) array.
    ///
    /// # Return values
    /// * `true` OK.
    /// * `false` Failed to delete load(s).
    pub fn clear_reference_load_case(&self, load_case_nos: Vec<i32>) -> Result<bool, anyErr> {
        unsafe {
            let sa_load_cases = safe_array_from_vec1d::<i32>(load_case_nos)?;
            let variant_load_cases = variant_with_ptr_from::<SafeArray<i32>>(sa_load_cases);

            let mut params = [variant_load_cases];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "ClearReferenceLoadCase",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Load::clear_reference_load_case: {}", e),
            }
        }
    }

    /// Creates a load list.
    /// # Parameters
    /// * `[in] varListType` Load list type: LoadList or LoadEnvelopeList.
    /// * `[in] varLoadCaseList` Load case reference ID(s) array.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn create_load_list(
        &self,
        list_type: i32,
        load_case_list: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_load_cases = safe_array_from_vec1d::<i32>(load_case_list)?;
            let variant_load_cases = variant_with_ptr_from::<SafeArray<i32>>(sa_load_cases);

            let mut params = [variant_load_cases, VARIANT::from(list_type)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateLoadList",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::create_load_list: {}", e),
            }
        }
    }

    /// Creates new PRIMARY load case.
    /// # Parameters
    /// * `[in] varPrimaryLoadTitle` The load case string title.
    ///
    /// # Return values
    /// * `<Val>` nLoadNo.
    /// * `-1` General error.
    /// * `-8004` Fail to create load.
    pub fn create_new_primary_load(&self, title: &str) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(title)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateNewPrimaryLoad",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_id)
                }
                Err(e) => bail!("Error::Load::create_new_primary_load: {}", e),
            }
        }
    }

    /// Creates new PRIMARY load case with specified load type.
    /// # Parameters
    /// * `[in] varPrimaryLoadTitle` The primary load case string title.
    /// * `[in] varLoadType` Type of the load.
    ///
    /// # Return values
    /// * `<Val>` nLoadNo.
    /// * `-1` General error.
    /// * `-8004` Fail to create load.
    pub fn create_new_primary_load_ex(&self, title: &str, load_type: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_type), VARIANT::from(title)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateNewPrimaryLoadEx",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_id)
                }
                Err(e) => bail!("Error::Load::create_new_primary_load_ex: {}", e),
            }
        }
    }

    /// Creates new PRIMARY load case with specified load type and load case number.
    /// # Parameters
    /// * `[in] varPrimaryLoadTitle` The primary load case string title.
    /// * `[in] varLoadType` Type of the load.
    /// * `[in] nLoadCaseNo` The load case number.
    ///
    /// # Return values
    /// * `<Val>` nLoadNo.
    /// * `0` Failed to create load.
    pub fn create_new_primary_load_ex2(
        &self,
        title: &str,
        load_type: i32,
        load_case_no: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(load_case_no),
                VARIANT::from(load_type),
                VARIANT::from(title),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateNewPrimaryLoadEx2",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_id)
                }
                Err(e) => bail!("Error::Load::create_new_primary_load_ex2: {}", e),
            }
        }
    }

    /// Deletes specified load list.
    /// # Parameters
    /// * `[in] varLoadListIndex` Load list index.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn delete_load_list(&self, load_list_index: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_list_index)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "DeleteLoadList",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::delete_load_list: {}", e),
            }
        }
    }

    /// Deletes specified Primary/Reference Load Cases.
    /// # Parameters
    /// * `[in] varPrimaryLoadCaseNos` Primary/Reference load case reference ID(s).
    /// * `[in] varIsReferenceLoads` If reference load case(s): true or false.
    ///
    /// # Return values
    /// * `true` OK.
    /// * `false` Failed to delete load(s).
    pub fn delete_primary_load_cases(
        &self,
        load_case_nos: Vec<i32>,
        is_reference_loads: bool,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_load_cases = safe_array_from_vec1d::<i32>(load_case_nos)?;
            let variant_load_cases = variant_with_ptr_from::<SafeArray<i32>>(sa_load_cases);

            let mut params = [VARIANT::from(is_reference_loads), variant_load_cases];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "DeletePrimaryLoadCases",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Load::delete_primary_load_cases: {}", e),
            }
        }
    }

    /// Deletes specified Reference Load Cases.
    /// # Parameters
    /// * `[in] varReferenceLoadCaseNos` Reference load case reference ID(s) array.
    ///
    /// # Return values
    /// * `true` OK.
    /// * `false` Failed to delete load(s).
    pub fn delete_reference_load_cases(&self, load_case_nos: Vec<i32>) -> Result<bool, anyErr> {
        unsafe {
            let sa_load_cases = safe_array_from_vec1d::<i32>(load_case_nos)?;
            let variant_load_cases = variant_with_ptr_from::<SafeArray<i32>>(sa_load_cases);

            let mut params = [variant_load_cases];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "DeleteReferenceLoadCases",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Load::delete_reference_load_cases: {}", e),
            }
        }
    }

    /// Returns the current load case number.
    /// # Return values
    /// * `<Val>` Active load case number ID.
    /// * `-1` General error.
    pub fn get_active_load(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(self.dispatch.as_ref().unwrap(), "GetActiveLoad", &mut []) };
        match result_variant {
            Ok(var) => {
                let result_id = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_id)
            }
            Err(e) => bail!("Error::Load::get_active_load: {}", e),
        }
    }

    /// Gets the list of entities that have been assigned to a load command in the active load case.
    /// # Parameters
    /// * `[in] loadType` Type of the load.
    /// * `[in] loadIndex` Load item index of specified load type (Zero based).
    ///
    /// # Return values
    /// * `<Val>` Size of entity list.
    /// * `0` General error.
    ///
    /// Returns tuple of (size, entity_list)
    pub fn get_assignment_list_for_load_type(
        &self,
        load_type: i32,
        load_index: i32,
    ) -> Result<(i32, Vec<i32>), anyErr> {
        unsafe {
            let list_size = self.get_list_size_for_load_type(load_type, load_index)?;
            if list_size <= 0 {
                return anyOk((list_size, Vec::new()));
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, list_size as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_with_ptr_from::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant, VARIANT::from(load_index), VARIANT::from(load_type)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetAssignmentListForLoadType",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_size = VariantToInt32(&var as *const VARIANT).unwrap();
                    let entities_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut entities_arr = Vec::with_capacity(list_size as usize);

                    for i in 0..list_size {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            entities_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        entities_arr.push(value);
                    }
                    anyOk((result_size, entities_arr))
                }
                Err(e) => bail!("Error::Load::get_assignment_list_for_load_type: {}", e),
            }
        }
    }

    /// Gets load attribute information of specified load case.
    /// # Parameters
    /// * `[in] lLoadCase` Load case reference ID.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn get_attribute(&self, load_case: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_case)];
            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "GetAttribute", &mut params);
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result)
                }
                Err(e) => bail!("Error::Load::get_attribute: {}", e),
            }
        }
    }

    /// Gets number of entities to which specified Load Type and load index.
    /// # Parameters
    /// * `[in] loadType` Type of the load.
    /// * `[in] loadIndex` Load item index of specified load type (Zero based).
    ///
    /// # Return values
    /// * `<Val>` The number of entities.
    pub fn get_list_size_for_load_type(
        &self,
        load_type: i32,
        load_index: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_index), VARIANT::from(load_type)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetListSizeForLoadType",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_size = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_size)
                }
                Err(e) => bail!("Error::Load::get_list_size_for_load_type: {}", e),
            }
        }
    }

    /// Returns title of the specified load case as a text string.
    /// # Parameters
    /// * `[in] varLoadNo` The load case number. Input 0 to retrieve title of current active load case.
    ///
    /// # Return values
    /// * `<VARIANT>` The load case string title.
    /// * "NONE" Load case not found.
    pub fn get_load_case_title(&self, load_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLoadCaseTitle",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(result)
                }
                Err(e) => bail!("Error::Load::get_load_case_title: {}", e),
            }
        }
    }

    /// Gets the number of load case(s) in specified load list.
    /// # Parameters
    /// * `[in] varLoadListIndex` Load list index.
    ///
    /// # Returns
    /// The number of Load Case(s) in specified Load List.
    pub fn get_load_count_in_load_list(&self, load_list_index: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_list_index)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLoadCountInLoadList",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_count)
                }
                Err(e) => bail!("Error::Load::get_load_count_in_load_list: {}", e),
            }
        }
    }

    /// Returns the number of loaditems in the specified load case.
    /// # Parameters
    /// * `[in] loadCaseNo` Load case number.
    ///
    /// # Return values
    /// * Number of load items.
    /// * `-1` General error.
    pub fn get_load_items_count(&self, load_case_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_case_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLoadItemsCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_count)
                }
                Err(e) => bail!("Error::Load::get_load_items_count: {}", e),
            }
        }
    }

    /// Returns the load item type for the specified loadIndex and loadCase.
    /// # Parameters
    /// * `[in] nloadCaseNo` Load case reference ID.
    /// * `[in] nloadItemIndex` Load item index (Zero based).
    ///
    /// # Return values
    /// * Load item type value.
    /// * `0` LoadCase/LoadItemIndex Not Found.
    pub fn get_load_item_type(
        &self,
        load_case_no: i32,
        load_item_index: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_item_index), VARIANT::from(load_case_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLoadItemType",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_type = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_type)
                }
                Err(e) => bail!("Error::Load::get_load_item_type: {}", e),
            }
        }
    }

    /// Gets the number of existing load list(s).
    /// # Return values
    /// * `<Val>` The number of load list(s).
    /// * `-1` General error.
    pub fn get_load_list_count(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(self.dispatch.as_ref().unwrap(), "GetLoadListCount", &mut []) };
        match result_variant {
            Ok(var) => {
                let result_count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_count)
            }
            Err(e) => bail!("Error::Load::get_load_list_count: {}", e),
        }
    }

    /// Gets the load case(s) in specified load list.
    /// # Parameters
    /// * `[in] varLoadListIndex` Load list index (Starts from one).
    ///
    /// # Return values
    /// * `true` Successful.
    /// * `false` Failed.
    ///
    /// Returns tuple of (success, load_cases)
    pub fn get_loads_in_load_list(&self, load_list_index: i32) -> Result<(bool, Vec<i32>), anyErr> {
        unsafe {
            let load_count = self.get_load_count_in_load_list(load_list_index)?;
            if load_count <= 0 {
                return anyOk((false, Vec::new()));
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, load_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_with_ptr_from::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant, VARIANT::from(load_list_index)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLoadsInLoadList",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    let loads_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut loads_arr = Vec::with_capacity(load_count as usize);

                    for i in 0..load_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            loads_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        loads_arr.push(value);
                    }
                    anyOk((result == 1, loads_arr))
                }
                Err(e) => bail!("Error::Load::get_loads_in_load_list: {}", e),
            }
        }
    }

    /// Returns primary load case category(s) as a long value.
    /// # Parameters
    /// * `[in] varLoadNo` Primary load case reference ID. Pass in 0 to get information about current active load case.
    ///
    /// # Return values
    /// * Load type value (0-23).
    /// * `-1` General error.
    pub fn get_load_type(&self, load_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_no)];
            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "GetLoadType", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_type = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_type)
                }
                Err(e) => bail!("Error::Load::get_load_type: {}", e),
            }
        }
    }

    /// Gets the number of load(s) with specified Load Type in active Load Case.
    /// # Parameters
    /// * `[in] loadType` Type of the load.
    ///
    /// # Return values
    /// * `<Val>` the number of load(s).
    /// * `0` Load Case not found.
    pub fn get_load_type_count(&self, load_type: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_type)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLoadTypeCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_count)
                }
                Err(e) => bail!("Error::Load::get_load_type_count: {}", e),
            }
        }
    }

    /// Gets all primary load case numbers.
    /// # Return values
    /// * `<Val>` The number of primary load case(s).
    /// * `-1` General error.
    /// * `-106` 1 dimensional array of VARIANT expected.
    /// * `-114` OLE Exception Occurred.
    ///
    /// Returns tuple of (count, case_numbers)
    pub fn get_primary_load_case_numbers(&self) -> Result<(i32, Vec<i32>), anyErr> {
        unsafe {
            // First get the count to allocate proper array size
            let primary_count = self.get_primary_load_case_count()?;
            if primary_count <= 0 {
                return anyOk((primary_count, Vec::new()));
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, primary_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_with_ptr_from::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetPrimaryLoadCaseNumbers",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_count = VariantToInt32(&var as *const VARIANT).unwrap();
                    let cases_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut cases_arr = Vec::with_capacity(primary_count as usize);

                    for i in 0..primary_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            cases_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        cases_arr.push(value);
                    }
                    anyOk((result_count, cases_arr))
                }
                Err(e) => bail!("Error::Load::get_primary_load_case_numbers: {}", e),
            }
        }
    }

    /// Removes the load attribute specified by lLoadCase.
    /// # Parameters
    /// * `[in] lLoadCase` Load case reference ID.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_attribute(&self, load_case: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_case)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "RemoveAttribute",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result)
                }
                Err(e) => bail!("Error::Load::remove_attribute: {}", e),
            }
        }
    }

    /// Sets Allowable Stress Design (ASD) load attribute.
    /// # Parameters
    /// * `[in] nLoadCase` Load case reference ID.
    /// * `[in] strength_type` Strength Type.
    /// * `[in] bIncrease` Allow 1/3 stress increase in ASD: false or true.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn set_asd_load_attribute(
        &self,
        load_case: i32,
        strength_type: i32,
        increase: bool,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(increase),
                VARIANT::from(strength_type),
                VARIANT::from(load_case),
            ];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SetASDLoadAttribute",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result)
                }
                Err(e) => bail!("Error::Load::set_asd_load_attribute: {}", e),
            }
        }
    }

    /// Set load type to load case(s) for considering load combination.
    /// # Parameters
    /// * `[in] varLoadNo` The load case reference number ID(s) array.
    /// * `[in] varLoadType` Type of the load.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn set_load_type(&self, load_nos: Vec<i32>, load_type: i32) -> Result<i32, anyErr> {
        unsafe {
            let sa_load_nos = safe_array_from_vec1d::<i32>(load_nos)?;
            let variant_load_nos = variant_with_ptr_from::<SafeArray<i32>>(sa_load_nos);

            let mut params = [VARIANT::from(load_type), variant_load_nos];

            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "SetLoadType", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Load::set_load_type: {}", e),
            }
        }
    }

    /// Sets Limit State Design (LSD) load attribute.
    /// # Parameters
    /// * `[in] nLoadCase` Load case reference ID.
    ///
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn set_lsd_load_attribute(&self, load_case: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_case)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SetLSDLoadAttribute",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result)
                }
                Err(e) => bail!("Error::Load::set_lsd_load_attribute: {}", e),
            }
        }
    }
    /// Returns the total number of primary load cases present in the current structure.
    /// # Returns
    /// The total number of primary load case(s).
    pub fn get_primary_load_case_count(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetPrimaryLoadCaseCount",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_count)
            }
            Err(e) => bail!("Error::Load::get_primary_load_case_count: {}", e),
        }
    }

    /// Makes the specified load number active, in order to add or remove load item(s).
    /// # Parameters
    /// * `[in] varLoadNo` The load case reference number ID.
    ///
    /// # Return values
    /// * `true` OK.
    /// * `false` General error.
    pub fn set_load_active(&self, load_no: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(load_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SetLoadActive",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Load::set_load_active: {}", e),
            }
        }
    }

    /// Adds a list of primary load case(s) to an existed load envelop.
    /// # Parameters
    /// * `[in] varEnvNo` Load Envelop reference ID.
    /// * `[in] varLoadCaseList` Primary load case(s) reference ID(s) array.
    ///
    /// # Return values
    /// * `true` OK.
    /// * `false` General error.
    pub fn add_load_cases_to_envelop(
        &self,
        env_no: i32,
        load_case_list: Vec<i32>,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_load_cases = safe_array_from_vec1d::<i32>(load_case_list)?;
            let variant_load_cases = variant_with_ptr_from::<SafeArray<i32>>(sa_load_cases);

            let mut params = [variant_load_cases, VARIANT::from(env_no)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AddLoadCasesToEnvelop",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Load::add_load_cases_to_envelop: {}", e),
            }
        }
    }

    /// Creates a Load Envelop with specified primary load case(s) and envelop type.
    /// # Parameters
    /// * `[in] varEnvNo` Load Envelop reference ID.
    /// * `[in] varEnvType` Type of the load envelop.
    /// * `[in] varLoadCaseList` Primary load case(s) reference ID(s) array.
    ///
    /// # Return values
    /// * `true` OK.
    /// * `false` General error.
    pub fn create_load_envelop(
        &self,
        env_no: i32,
        env_type: i32,
        load_case_list: Vec<i32>,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_load_cases = safe_array_from_vec1d::<i32>(load_case_list)?;
            let variant_load_cases = variant_with_ptr_from::<SafeArray<i32>>(sa_load_cases);

            let mut params = [
                variant_load_cases,
                VARIANT::from(env_type),
                VARIANT::from(env_no),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateLoadEnvelop",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Load::create_load_envelop: {}", e),
            }
        }
    }

    /// Deletes a specified load envelop.
    /// # Parameters
    /// * `[in] varEnvNo` Load Envelop reference ID.
    ///
    /// # Return values
    /// * `true` OK.
    /// * `false` General error.
    pub fn delete_load_envelop(&self, env_no: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(env_no)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "DeleteLoadEnvelop",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Load::delete_load_envelop: {}", e),
            }
        }
    }

    /// Returns number of Envelopes defined.
    /// # Return values
    /// Total Number of load Envelopes present.
    pub fn get_envelope_count(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(self.dispatch.as_ref().unwrap(), "GetEnvelopeCount", &mut []) };
        match result_variant {
            Ok(var) => {
                let result_count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_count)
            }
            Err(e) => bail!("Error::Load::get_envelope_count: {}", e),
        }
    }

    /// Gets the list of Loads Envelope IDs present in the staad file.
    /// # Return values
    /// * `1` OK.
    /// * `-1` Unsuccessful.
    ///
    /// Returns tuple of (result, envelope_ids)
    pub fn get_envelope_ids(&self) -> Result<(i32, Vec<i32>), anyErr> {
        unsafe {
            let envelope_count = self.get_envelope_count()?;
            if envelope_count <= 0 {
                return anyOk((envelope_count, Vec::new()));
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, envelope_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_with_ptr_from::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetEnvelopeIDs",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    let ids_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut ids_arr = Vec::with_capacity(envelope_count as usize);

                    for i in 0..envelope_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            ids_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        ids_arr.push(value);
                    }
                    anyOk((result, ids_arr))
                }
                Err(e) => bail!("Error::Load::get_envelope_ids: {}", e),
            }
        }
    }

    /// Gets the type of Envelope and number of primary load case(s) present in the entered load envelope.
    /// # Parameters
    /// * `[in] EnvNo` Load Envelope reference ID.
    ///
    /// # Return values
    /// * `1` Successful.
    /// * `-1` Unsuccessful.
    ///
    /// Returns tuple of (result, envelope_type, number_of_load_cases)
    pub fn get_load_envelope_details(&self, env_no: i32) -> Result<(i32, i32, i32), anyErr> {
        unsafe {
            let env_type_ptr = &mut 0i32 as *mut i32;
            let load_count_ptr = &mut 0i32 as *mut i32;
            let variant_env_type = variant_with_ptr_from::<i32>(env_type_ptr);
            let variant_load_count = variant_with_ptr_from::<i32>(load_count_ptr);

            let mut params = [variant_load_count, variant_env_type, VARIANT::from(env_no)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLoadEnvelopeDetails",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    let envelope_type = VariantToInt32(&params[1] as *const VARIANT)?;
                    let load_count = VariantToInt32(&params[0] as *const VARIANT)?;
                    anyOk((result, envelope_type, load_count))
                }
                Err(e) => bail!("Error::Load::get_load_envelope_details: {}", e),
            }
        }
    }

    /// Gets the list of primary load case reference Ids present in the load envelope passed.
    /// # Parameters
    /// * `[in] EnvNo` Load Envelope reference ID.
    ///
    /// # Return values
    /// * `1` Successful.
    /// * `-1` Unsuccessful.
    ///
    /// Returns tuple of (result, load_case_list)
    pub fn get_load_list_from_load_envelope(&self, env_no: i32) -> Result<(i32, Vec<i32>), anyErr> {
        unsafe {
            // First get the envelope details to know how many load cases are in it
            let (detail_result, _, load_count) = self.get_load_envelope_details(env_no)?;
            if detail_result != 1 || load_count <= 0 {
                return anyOk((detail_result, Vec::new()));
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, load_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_with_ptr_from::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant, VARIANT::from(env_no)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetLoadListfromLoadEnvelope",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    let loads_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut loads_arr = Vec::with_capacity(load_count as usize);

                    for i in 0..load_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            loads_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        loads_arr.push(value);
                    }
                    anyOk((result, loads_arr))
                }
                Err(e) => bail!("Error::Load::get_load_list_from_load_envelope: {}", e),
            }
        }
    }

    /// Removes a list of primary load case(s) from an existed load envelop.
    /// # Parameters
    /// * `[in] varEnvNo` Load Envelop reference ID.
    /// * `[in] varLoadCaseList` Primary load case(s) reference ID(s) array.
    ///
    /// # Return values
    /// * `true` OK.
    /// * `false` General error.
    pub fn remove_load_cases_from_envelop(
        &self,
        env_no: i32,
        load_case_list: Vec<i32>,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_load_cases = safe_array_from_vec1d::<i32>(load_case_list)?;
            let variant_load_cases = variant_with_ptr_from::<SafeArray<i32>>(sa_load_cases);

            let mut params = [variant_load_cases, VARIANT::from(env_no)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "RemoveLoadCasesFromEnvelop",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result == 1)
                }
                Err(e) => bail!("Error::Load::remove_load_cases_from_envelop: {}", e),
            }
        }
    }
}

unsafe impl Send for Load{}
unsafe impl Sync for Load{}

// ********** Definitions **********
// :: Wind
// AddWindDefinition
// AddWindDefinitionASCE7Parameters
// AddWindExposure
// AddWindIntensity
// ComputeWallWindPressureProfile
// ComputeWallWindPressureProfileASCE72016
// DeleteWindDefinition
// :: Seismic
// AddResponseSpectrumLoadEx
// AddSeismicDefinition
// AddSeismicDefJointWeight
// AddSeismicDefMemberWeight
// AddSeismicDefSelfWeight
// AddSeismicDefWallArea
// ModifySeismicDefinitionParams
// :: Reference Load
// CreateNewReferenceLoad
// GetReferenceLoadCaseCount
// GetReferenceLoadCaseNumbers
// SetReferenceLoadActive
// :: Direct Analysis
// AddDirectAnalysisDefinitionParameter
// DeleteDirectAnalysisDefinition
// DeleteDirectAnalysisDefinitionParameter

// ********** Load Case Details **********
// :: Load items :: SelfWeight Load
// AddSelfWeightInXYZ
// AddSelfWeightInXYZToGeometry
// :: Load items :: Nodal Load
// AddNodalLoad
// AddSupportDisplacement
// GetNodalLoadCount
// GetNodalLoadInfo
// GetNodalLoads
// :: Load items :: Member Load
// AddMemberAreaLoad
// AddMemberConcForce
// AddMemberConcMoment
// AddMemberFixedEnd
// AddMemberLinearVari
// AddMemberTrapezoidal
// AddMemberUniformForce
// AddMemberUniformMoment
// GetConcForceCount
// GetConcForces
// GetConcMomentCount
// GetConcMoments
// GetLinearVaryingLoadCount
// GetLinearVaryingLoads
// GetMemberLoadInfo
// GetTrapLoadCount
// GetTrapLoads
// GetUDLLoadCount
// GetUDLLoads
// GetUNIMomentCount
// GetUNIMoments
// :: Load items :: Floor Load
// AddMemberFloorLoad
// AddMemberFloorLoadEx
// GetBeamCountAtFloor
// GetInfluenceArea
// :: Load items :: Seismic Load
// AddSeismicLoad
// IsDynamicLoadIncluded
// :: Load items :: Wind and Snow Load
// AddWindLoad
// :: Load items :: Repeat Load
// AddNotionalLoad
// AddReferenceLoad
// AddRepeatLoad
// BeginLoadMerging
// EndLoadMerging
// GetNoLoadFactorDirectionInNotionalLoad
// GetNoLoadFactorInRepeatLoad
// GetNoOfSetsInReferenceLoad
// GetNotionalLoadByIndex
// GetNotionalLoadCount
// GetReferenceLoadByIndex
// GetReferenceLoadCaseTitle
// GetReferenceLoadCount
// GetReferenceLoadType
// GetRepeatLoadByIndex
// GetRepeatLoadCount
// :: Load Combination
// AddAutoCombinationRepeat
// AddAutoLoadCombinations
// AddLoadAndFactorToCombination
// CreateNewLoadCombination
// GetLoadAndFactorForCombination
// GetLoadCombinationCaseCount
// GetLoadCombinationCaseNumbers
// GetNoOfLoadAndFactorPairsForCombination
// IsCombinationCase
// :: Load Case Operation
// ClearPrimaryLoadCase
// ClearReferenceLoadCase
// CreateLoadList
// CreateNewPrimaryLoad
// CreateNewPrimaryLoadEx
// CreateNewPrimaryLoadEx2
// DeleteLoadList
// DeletePrimaryLoadCases
// DeleteReferenceLoadCases
// GetActiveLoad
// GetAssignmentListForLoadType
// GetAttribute
// GetListSizeForLoadType
// GetLoadCaseTitle
// GetLoadCountInLoadList
// GetLoadItemsCount
// GetLoadItemType
// GetLoadListCount
// GetLoadsInLoadList
// GetLoadType
// GetLoadTypeCount
// GetPrimaryLoadCaseNumbers
// RemoveAttribute
// SetASDLoadAttribute
// SetLoadType
// SetLSDLoadAttribute
// ::
// GetPrimaryLoadCaseCount
// SetLoadActive

// ********** Load Envelopes **********
// AddLoadCasesToEnvelop
// CreateLoadEnvelop
// DeleteLoadEnvelop
// GetEnvelopeCount
// GetEnvelopeIDs
// GetLoadEnvelopeDetails
// GetLoadListfromLoadEnvelope
// RemoveLoadCasesFromEnvelop
