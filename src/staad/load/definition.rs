use crate::staad::{
    geometry::root::Geometry,
    load::root::Load,
    property::root::Property,
    safe_array::{safe_array_from_vec1d, safe_array_from_vec2d},
    utils::invoke_method_with_result,
    variant::{SafeArray, SafeArrayP, variant_from_raw_pointer},
};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use std::ffi::c_void;
use windows::Win32::System::{
    Com::SAFEARRAY,
    Ole::{SafeArrayCreateVector, SafeArrayGetElement, SafeArrayPutElement},
    Variant::{VARIANT, VT_I4, VT_R8, VariantToDouble, VariantToInt32, VariantToStringAlloc},
};
use windows_core::BSTR;

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

// :: Direct Analysis

#[derive(Debug)]
pub struct Definition<'a> {
    pub load: &'a Load<'a>,
}

impl<'a> Definition<'a> {
    pub fn new(load: &'a Load<'a>) -> Self {
        Self { load }
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
            let result_variant =
                invoke_method_with_result(&self.load.dispatch, "AddWindDefinition", &mut params);
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

            let variant_escarpment = variant_from_raw_pointer::<SafeArray<f64>>(sa_escarpment);
            let variant_bldg = variant_from_raw_pointer::<SafeArray<f64>>(sa_bldg);
            let variant_units = variant_from_raw_pointer::<SafeArray<i32>>(sa_units);
            let variant_factors_user = variant_from_raw_pointer::<SafeArray<i32>>(sa_factors_user);
            let variant_factors = variant_from_raw_pointer::<SafeArray<f64>>(sa_factors);

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

            let result_variant = invoke_method_with_result(
                &self.load.dispatch,
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
            let variant_nodes = variant_from_raw_pointer::<SafeArray<i32>>(sa_nodes);

            let mut params = [
                variant_nodes,
                VARIANT::from(exposure_factor),
                VARIANT::from(type_no),
            ];
            let result_variant =
                invoke_method_with_result(&self.load.dispatch, "AddWindExposure", &mut params);
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
            let variant_intensity = variant_from_raw_pointer::<SafeArray<f64>>(sa_intensity);
            let variant_height = variant_from_raw_pointer::<SafeArray<f64>>(sa_height);

            let mut params = [variant_height, variant_intensity, VARIANT::from(type_no)];
            let result_variant =
                invoke_method_with_result(&self.load.dispatch, "AddWindIntensity", &mut params);
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
            let result_variant =
                invoke_method_with_result(&self.load.dispatch, "AddWindIntensity", &mut params);
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

            let variant_unit = variant_from_raw_pointer::<SafeArray<i32>>(sa_unit);
            let variant_escarpment = variant_from_raw_pointer::<SafeArray<f64>>(sa_escarpment);
            let variant_bldg = variant_from_raw_pointer::<SafeArray<f64>>(sa_bldg);

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

            let result_variant = invoke_method_with_result(
                &self.load.dispatch,
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

            let variant_unit = variant_from_raw_pointer::<SafeArray<i32>>(sa_unit);
            let variant_escarpment = variant_from_raw_pointer::<SafeArray<f64>>(sa_escarpment);
            let variant_bldg = variant_from_raw_pointer::<SafeArray<f64>>(sa_bldg);

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

            let result_variant = invoke_method_with_result(
                &self.load.dispatch,
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
            let result_variant =
                invoke_method_with_result(&self.load.dispatch, "DeleteWindDefinition", &mut params);
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
            let variant_set1_names = variant_from_raw_pointer::<SafeArray<BSTR>>(sa_set1_names);
            let variant_set1_vals = variant_from_raw_pointer::<SafeArray<f64>>(sa_set1_vals);

            let (variant_set2_names, variant_set2_vals, variant_spectrum_data) =
                match (set2_names, set2_vals, spectrum_data_pairs) {
                    (Some(names), Some(vals), None) => {
                        let sa_set2_names = safe_array_from_vec1d::<String>(names)?;
                        let sa_set2_vals = safe_array_from_vec1d::<f64>(vals)?;
                        (
                            variant_from_raw_pointer::<SafeArray<BSTR>>(sa_set2_names),
                            variant_from_raw_pointer::<SafeArray<f64>>(sa_set2_vals),
                            VARIANT::default(), // NULL
                        )
                    }
                    (None, None, Some(data)) => {
                        let sa_spectrum = safe_array_from_vec1d::<f64>(data)?;
                        (
                            VARIANT::default(), // NULL
                            VARIANT::default(), // NULL
                            variant_from_raw_pointer::<SafeArray<f64>>(sa_spectrum),
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

            let result_variant = invoke_method_with_result(
                &self.load.dispatch,
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
            let result_variant =
                invoke_method_with_result(&self.load.dispatch, "AddSeismicDefinition", &mut params);
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
            let variant_nodes = variant_from_raw_pointer::<SafeArray<i32>>(sa_nodes);

            let mut params = [variant_nodes, VARIANT::from(weight)];
            let result_variant = invoke_method_with_result(
                &self.load.dispatch,
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
            let variant_members = variant_from_raw_pointer::<SafeArray<i32>>(sa_members);

            let mut params = [
                variant_members,
                VARIANT::from(end_dist),
                VARIANT::from(start_dist),
                VARIANT::from(weight),
                VARIANT::from(load_type),
                VARIANT::from(seismic_type),
            ];

            let result_variant = invoke_method_with_result(
                &self.load.dispatch,
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
            let result_variant = invoke_method_with_result(
                &self.load.dispatch,
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
            let variant_members = variant_from_raw_pointer::<SafeArray<f64>>(sa_members);

            let mut params = [
                variant_members,
                VARIANT::from(direction),
                VARIANT::from(type_no),
            ];
            let result_variant = invoke_method_with_result(
                &self.load.dispatch,
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
            let result_variant = invoke_method_with_result(
                &self.load.dispatch,
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
}
