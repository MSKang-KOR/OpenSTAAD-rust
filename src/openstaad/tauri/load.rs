use crate::openstaad::api::load::Load;
use crate::openstaad::tauri::store::PROCESS_STORE;
use crate::openstaad::tauri::utils::{
    ConvertedParam, MethodSignature, ParamType, StaadObject, convert_param,
};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub fn load_call(id: String, method: String, params: Vec<Value>) -> Result<Value, String> {
    let store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
    let load = match store.get(&id) {
        Some(StaadObject::Load(l)) => l,
        _ => return Err("Load not found".to_string()),
    };

    // 메서드 시그니처 매핑 가져오기
    let signatures = get_method_signatures();
    let signature = match signatures.get(method.as_str()) {
        Some(sig) => sig,
        None => return Err(format!("Unknown method: {}", method)),
    };

    // 파라미터 개수 검증
    if params.len() != signature.params.len() {
        return Err(format!(
            "Parameter count mismatch: expected {}, got {}",
            signature.params.len(),
            params.len()
        ));
    }

    // 파라미터 변환
    let converted_params: Result<Vec<ConvertedParam>, String> = params
        .iter()
        .zip(signature.params.iter())
        .map(|(param, param_type)| convert_param(param, param_type))
        .collect();

    let converted_params = converted_params?;

    // 동적 메서드 호출
    let result = call_method(&load, &method, converted_params);

    result
}

// Load 메서드 시그니처 매핑
fn get_method_signatures() -> HashMap<&'static str, MethodSignature> {
    let mut signatures = HashMap::new();

    // Wind Definition 관련 메서드들
    signatures.insert(
        "add_wind_definition",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::String],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "delete_wind_definition",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_wind_definition_asce7_parameters",
        MethodSignature {
            params: vec![
                ParamType::I32, // type_no
                ParamType::I32, // code
                ParamType::F64, // wind_speed
                ParamType::F64, // height_above_sea_lvl
                ParamType::I32, // bldg_class
                ParamType::I32, // bldg_type
                ParamType::I32, // exp_cat
                ParamType::Bool, // escarpment
                ParamType::I32, // wall_type
                ParamType::Bool, // is_flexible
                ParamType::VecF64, // escarpment_data
                ParamType::VecF64, // bldg_data
                ParamType::VecI32, // units_data
                ParamType::VecI32, // factors_user_input
                ParamType::VecF64, // factors
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_wind_exposure",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::F64, ParamType::VecI32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_wind_intensity",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::VecF64, ParamType::VecF64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_wind_intensity_single",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::F64, ParamType::F64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "compute_wall_wind_pressure_profile",
        MethodSignature {
            params: vec![
                ParamType::I32, // code
                ParamType::F64, // wind_speed
                ParamType::I32, // bldg_class
                ParamType::I32, // bldg_type
                ParamType::I32, // exp_cat
                ParamType::Bool, // escarpment
                ParamType::VecI32, // unit_data
                ParamType::VecF64, // escarpment_data
                ParamType::VecF64, // bldg_data
                ParamType::I32, // wall_type
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "compute_wall_wind_pressure_profile_asce7_2016",
        MethodSignature {
            params: vec![
                ParamType::F64, // wind_speed
                ParamType::F64, // height_above_sea_lvl
                ParamType::I32, // bldg_class
                ParamType::I32, // bldg_type
                ParamType::I32, // exp_cat
                ParamType::Bool, // escarpment
                ParamType::VecI32, // unit_data
                ParamType::VecF64, // escarpment_data
                ParamType::VecF64, // bldg_data
                ParamType::I32, // wall_type
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_wind_load",
        MethodSignature {
            params: vec![
                ParamType::I32, // type_no
                ParamType::I32, // direction
                ParamType::F64, // fraction
                ParamType::Bool, // open_structure
                ParamType::F64, // y_min
                ParamType::F64, // y_max
                ParamType::F64, // z_min
                ParamType::F64, // z_max
                ParamType::F64, // x_min
                ParamType::F64, // x_max
            ],
            returns_unit_scaled: false,
        },
    );

    // Self Weight 관련 메서드들
    signatures.insert(
        "add_self_weight_in_xyz",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::F64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_self_weight_in_xyz_to_geometry",
        MethodSignature {
            params: vec![
                ParamType::VecI32,
                ParamType::I32,
                ParamType::F64,
            ],
            returns_unit_scaled: false,
        },
    );

    // Nodal Load 관련 메서드들
    signatures.insert(
        "add_nodal_load",
        MethodSignature {
            params: vec![
                ParamType::VecI32,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_support_displacement",
        MethodSignature {
            params: vec![
                ParamType::VecI32,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_nodal_load_count",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );

    // Member Load 관련 메서드들
    signatures.insert(
        "add_member_area_load",
        MethodSignature {
            params: vec![ParamType::VecI32, ParamType::F64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_member_conc_force",
        MethodSignature {
            params: vec![
                ParamType::VecI32,
                ParamType::I32,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_member_conc_moment",
        MethodSignature {
            params: vec![
                ParamType::VecI32,
                ParamType::I32,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_member_uniform_force",
        MethodSignature {
            params: vec![
                ParamType::VecI32,
                ParamType::I32,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_member_uniform_moment",
        MethodSignature {
            params: vec![
                ParamType::VecI32,
                ParamType::I32,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );

    // Load Case Management 관련 메서드들
    signatures.insert(
        "create_new_primary_load",
        MethodSignature {
            params: vec![ParamType::String],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_new_primary_load_ex",
        MethodSignature {
            params: vec![ParamType::String, ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "delete_primary_load_cases",
        MethodSignature {
            params: vec![ParamType::VecI32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_active_load",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_load_case_title",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_primary_load_case_count",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_primary_load_case_numbers",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "set_load_active",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );

    // Seismic Load 관련 메서드들
    signatures.insert(
        "add_response_spectrum_load_ex",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::I32,
                ParamType::VecString,
                ParamType::VecF64,
                ParamType::OptionVecString,
                ParamType::OptionVecF64,
                ParamType::OptionVecF64,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_seismic_definition",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::Bool],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_seismic_def_joint_weight",
        MethodSignature {
            params: vec![ParamType::F64, ParamType::VecI32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_seismic_def_member_weight",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::I32,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::VecI32,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_seismic_def_self_weight",
        MethodSignature {
            params: vec![ParamType::F64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_seismic_def_wall_area",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::String, ParamType::VecF64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "modify_seismic_definition_params",
        MethodSignature {
            params: vec![ParamType::String, ParamType::F64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_seismic_load",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::F64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "is_dynamic_load_included",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );

    // Load Combination 관련 메서드들
    signatures.insert(
        "create_new_load_combination",
        MethodSignature {
            params: vec![ParamType::String, ParamType::VecI32, ParamType::VecF64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_load_and_factor_to_combination",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::I32, ParamType::F64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_load_combination_case_count",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "is_combination_case",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );

    signatures
}

fn call_method(
    load: &Arc<Load>,
    method: &str,
    params: Vec<ConvertedParam>,
) -> Result<serde_json::Value, String> {
    match method {
        // Wind Definition 관련 메서드들
        "add_wind_definition" => {
            if let (ConvertedParam::I32(type_no), ConvertedParam::String(type_name)) =
                (&params[0], &params[1])
            {
                let result = load
                    .add_wind_definition(*type_no, type_name)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_wind_definition".to_string())
            }
        }
        "delete_wind_definition" => {
            if let ConvertedParam::I32(type_no) = &params[0] {
                let result = load
                    .delete_wind_definition(*type_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for delete_wind_definition".to_string())
            }
        }
        "add_wind_definition_asce7_parameters" => {
            if let (
                ConvertedParam::I32(type_no),
                ConvertedParam::I32(code),
                ConvertedParam::F64(wind_speed),
                ConvertedParam::F64(height_above_sea_lvl),
                ConvertedParam::I32(bldg_class),
                ConvertedParam::I32(bldg_type),
                ConvertedParam::I32(exp_cat),
                ConvertedParam::Bool(escarpment),
                ConvertedParam::I32(wall_type),
                ConvertedParam::Bool(is_flexible),
                ConvertedParam::VecF64(escarpment_data),
                ConvertedParam::VecF64(bldg_data),
                ConvertedParam::VecI32(units_data),
                ConvertedParam::VecI32(factors_user_input),
                ConvertedParam::VecF64(factors),
            ) = (
                &params[0], &params[1], &params[2], &params[3], &params[4],
                &params[5], &params[6], &params[7], &params[8], &params[9],
                &params[10], &params[11], &params[12], &params[13], &params[14],
            ) {
                let result = load
                    .add_wind_definition_asce7_parameters(
                        *type_no,
                        *code,
                        *wind_speed,
                        *height_above_sea_lvl,
                        *bldg_class,
                        *bldg_type,
                        *exp_cat,
                        *escarpment,
                        *wall_type,
                        *is_flexible,
                        escarpment_data.clone(),
                        bldg_data.clone(),
                        units_data.clone(),
                        factors_user_input.clone(),
                        factors.clone(),
                    )
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_wind_definition_asce7_parameters".to_string())
            }
        }
        "add_wind_exposure" => {
            if let (
                ConvertedParam::I32(type_no),
                ConvertedParam::F64(exposure_factor),
                ConvertedParam::VecI32(node_array),
            ) = (&params[0], &params[1], &params[2]) {
                let result = load
                    .add_wind_exposure(*type_no, *exposure_factor, node_array.clone())
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_wind_exposure".to_string())
            }
        }
        "add_wind_intensity" => {
            if let (
                ConvertedParam::I32(type_no),
                ConvertedParam::VecF64(intensity),
                ConvertedParam::VecF64(height),
            ) = (&params[0], &params[1], &params[2]) {
                let result = load
                    .add_wind_intensity(*type_no, intensity.clone(), height.clone())
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_wind_intensity".to_string())
            }
        }
        "add_wind_intensity_single" => {
            if let (
                ConvertedParam::I32(type_no),
                ConvertedParam::F64(intensity),
                ConvertedParam::F64(height),
            ) = (&params[0], &params[1], &params[2]) {
                let result = load
                    .add_wind_intensity_single(*type_no, *intensity, *height)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_wind_intensity_single".to_string())
            }
        }
        "compute_wall_wind_pressure_profile" => {
            if let (
                ConvertedParam::I32(code),
                ConvertedParam::F64(wind_speed),
                ConvertedParam::I32(bldg_class),
                ConvertedParam::I32(bldg_type),
                ConvertedParam::I32(exp_cat),
                ConvertedParam::Bool(escarpment),
                ConvertedParam::VecI32(unit_data),
                ConvertedParam::VecF64(escarpment_data),
                ConvertedParam::VecF64(bldg_data),
                ConvertedParam::I32(wall_type),
            ) = (
                &params[0], &params[1], &params[2], &params[3], &params[4],
                &params[5], &params[6], &params[7], &params[8], &params[9],
            ) {
                let result = load
                    .compute_wall_wind_pressure_profile(
                        *code,
                        *wind_speed,
                        *bldg_class,
                        *bldg_type,
                        *exp_cat,
                        *escarpment,
                        unit_data.clone(),
                        escarpment_data.clone(),
                        bldg_data.clone(),
                        *wall_type,
                    )
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for compute_wall_wind_pressure_profile".to_string())
            }
        }
        "compute_wall_wind_pressure_profile_asce7_2016" => {
            if let (
                ConvertedParam::F64(wind_speed),
                ConvertedParam::F64(height_above_sea_lvl),
                ConvertedParam::I32(bldg_class),
                ConvertedParam::I32(bldg_type),
                ConvertedParam::I32(exp_cat),
                ConvertedParam::Bool(escarpment),
                ConvertedParam::VecI32(unit_data),
                ConvertedParam::VecF64(escarpment_data),
                ConvertedParam::VecF64(bldg_data),
                ConvertedParam::I32(wall_type),
            ) = (
                &params[0], &params[1], &params[2], &params[3], &params[4],
                &params[5], &params[6], &params[7], &params[8], &params[9],
            ) {
                let result = load
                    .compute_wall_wind_pressure_profile_asce7_2016(
                        *wind_speed,
                        *height_above_sea_lvl,
                        *bldg_class,
                        *bldg_type,
                        *exp_cat,
                        *escarpment,
                        unit_data.clone(),
                        escarpment_data.clone(),
                        bldg_data.clone(),
                        *wall_type,
                    )
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for compute_wall_wind_pressure_profile_asce7_2016".to_string())
            }
        }
        "add_wind_load" => {
            if let (
                ConvertedParam::I32(type_no),
                ConvertedParam::I32(direction),
                ConvertedParam::F64(fraction),
                ConvertedParam::Bool(open_structure),
                ConvertedParam::F64(y_min),
                ConvertedParam::F64(y_max),
                ConvertedParam::F64(z_min),
                ConvertedParam::F64(z_max),
                ConvertedParam::F64(x_min),
                ConvertedParam::F64(x_max),
            ) = (
                &params[0], &params[1], &params[2], &params[3], &params[4],
                &params[5], &params[6], &params[7], &params[8], &params[9],
            ) {
                let result = load
                    .add_wind_load(
                        *type_no,
                        *direction,
                        *fraction,
                        *open_structure,
                        *y_min,
                        *y_max,
                        *z_min,
                        *z_max,
                        *x_min,
                        *x_max,
                    )
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_wind_load".to_string())
            }
        }

        // Self Weight 관련 메서드들
        "add_self_weight_in_xyz" => {
            if let (ConvertedParam::I32(direction), ConvertedParam::F64(load_factor)) =
                (&params[0], &params[1])
            {
                let result = load
                    .add_self_weight_in_xyz(*direction, *load_factor)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_self_weight_in_xyz".to_string())
            }
        }
        "add_self_weight_in_xyz_to_geometry" => {
            if let (
                ConvertedParam::VecI32(geom_nos),
                ConvertedParam::I32(direction),
                ConvertedParam::F64(load_factor),
            ) = (&params[0], &params[1], &params[2])
            {
                let result = load
                    .add_self_weight_in_xyz_to_geometry(geom_nos.clone(), *direction, *load_factor)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_self_weight_in_xyz_to_geometry".to_string())
            }
        }

        // Nodal Load 관련 메서드들
        "add_nodal_load" => {
            if let (
                ConvertedParam::VecI32(node_list),
                ConvertedParam::F64(fx),
                ConvertedParam::F64(fy),
                ConvertedParam::F64(fz),
                ConvertedParam::F64(mx),
                ConvertedParam::F64(my),
                ConvertedParam::F64(mz),
            ) = (
                &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6],
            ) {
                let result = load
                    .add_nodal_load(node_list.clone(), *fx, *fy, *fz, *mx, *my, *mz)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_nodal_load".to_string())
            }
        }

        // Load Case Management 관련 메서드들
        "create_new_primary_load" => {
            if let ConvertedParam::String(title) = &params[0] {
                let result = load
                    .create_new_primary_load(title)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_new_primary_load".to_string())
            }
        }
        "get_active_load" => {
            let result = load.get_active_load().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_primary_load_case_count" => {
            let result = load
                .get_primary_load_case_count()
                .map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "set_load_active" => {
            if let ConvertedParam::I32(load_no) = &params[0] {
                let result = load.set_load_active(*load_no).map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for set_load_active".to_string())
            }
        }

        // // Seismic Load 관련 메서드들
        // "add_response_spectrum_load_ex" => {
        //     if let (VecString
        //         ConvertedParam::I32(rsa_code),
        //         ConvertedParam::I32(rsa_combination),
        //         ConvertedParam::VecString(set1_names),
        //         ConvertedParam::VecF64(set1_vals),
        //         ConvertedParam::OptionVecString(set2_names),
        //         ConvertedParam::OptionVecF64(set2_vals),
        //         ConvertedParam::OptionVecF64(spectrum_data_pairs),
        //     ) = (&params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6]) {
        //         let result = load
        //             .add_response_spectrum_load_ex(
        //                 *rsa_code,
        //                 *rsa_combination,
        //                 set1_names.clone(),
        //                 set1_vals.clone(),
        //                 set2_names.clone(),
        //                 set2_vals.clone(),
        //                 spectrum_data_pairs.clone(),
        //             )
        //             .map_err(|e| e.to_string())?;
        //         serde_json::to_value(&result).map_err(|e| e.to_string())
        //     } else {
        //         Err("Invalid parameters for add_response_spectrum_load_ex".to_string())
        //     }
        // }
        "add_seismic_definition" => {
            if let (ConvertedParam::I32(seismic_type), ConvertedParam::Bool(accidental)) = (&params[0], &params[1]) {
                let result = load
                    .add_seismic_definition(*seismic_type, *accidental)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_seismic_definition".to_string())
            }
        }
        "add_seismic_def_joint_weight" => {
            if let (ConvertedParam::F64(weight), ConvertedParam::VecI32(node_array)) = (&params[0], &params[1]) {
                let result = load
                    .add_seismic_def_joint_weight(*weight, node_array.clone())
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_seismic_def_joint_weight".to_string())
            }
        }
        "add_seismic_def_member_weight" => {
            if let (
                ConvertedParam::I32(seismic_type),
                ConvertedParam::I32(load_type),
                ConvertedParam::F64(weight),
                ConvertedParam::F64(start_dist),
                ConvertedParam::F64(end_dist),
                ConvertedParam::VecI32(member_array),
            ) = (&params[0], &params[1], &params[2], &params[3], &params[4], &params[5]) {
                let result = load
                    .add_seismic_def_member_weight(
                        *seismic_type,
                        *load_type,
                        *weight,
                        *start_dist,
                        *end_dist,
                        member_array.clone(),
                    )
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_seismic_def_member_weight".to_string())
            }
        }
        "add_seismic_def_self_weight" => {
            if let ConvertedParam::F64(weight_factor) = &params[0] {
                let result = load
                    .add_seismic_def_self_weight(*weight_factor)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_seismic_def_self_weight".to_string())
            }
        }
        "add_seismic_def_wall_area" => {
            if let (
                ConvertedParam::I32(type_no),
                ConvertedParam::String(direction),
                ConvertedParam::VecF64(member_array),
            ) = (&params[0], &params[1], &params[2]) {
                let result = load
                    .add_seismic_def_wall_area(*type_no, direction, member_array.clone())
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_seismic_def_wall_area".to_string())
            }
        }
        "modify_seismic_definition_params" => {
            if let (ConvertedParam::String(param_name), ConvertedParam::F64(value)) = (&params[0], &params[1]) {
                let result = load
                    .modify_seismic_definition_params(param_name, *value)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for modify_seismic_definition_params".to_string())
            }
        }
        "add_seismic_load" => {
            if let (ConvertedParam::I32(direction), ConvertedParam::F64(factor)) = (&params[0], &params[1]) {
                let result = load
                    .add_seismic_load(*direction, *factor)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_seismic_load".to_string())
            }
        }
        "is_dynamic_load_included" => {
            if let ConvertedParam::I32(load_case) = &params[0] {
                let result = load
                    .is_dynamic_load_included(*load_case)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for is_dynamic_load_included".to_string())
            }
        }

        _ => Err(format!("Unknown method: {}", method)),
    }
}
