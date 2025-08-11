use crate::openstaad::api::root::Root;
use crate::openstaad::tauri::store::PROCESS_STORE;
use crate::openstaad::tauri::utils::{
    ConvertedParam, MethodSignature, ParamType, StaadObject, convert_param,
};
use serde_json::Value;
use std::collections::HashMap;

pub fn root_call(id: String, method: String, params: Vec<Value>) -> Result<Value, String> {
    let store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
    let root = match store.get(&id) {
        Some(StaadObject::Root(r)) => r,
        _ => return Err("Root not found".to_string()),
    };

    // 메서드 시그니처 매핑 가져오기
    let signatures = get_root_method_signatures();
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

    // Root 객체를 안전하게 참조하기 위한 Box 처리
    let _box = Box::new(root);
    let _ptr = Box::into_raw(_box);
    let _ref = unsafe { &*_ptr };

    // 동적 메서드 호출
    let result = call_root_method(_ref, &method, converted_params);

    // 메모리 정리
    let _back = unsafe { Box::from_raw(_ptr) };

    result
}

// Root 메서드 시그니처 매핑
fn get_root_method_signatures() -> HashMap<&'static str, MethodSignature> {
    let mut signatures = HashMap::new();

    // Analysis 관련 메서드들
    signatures.insert(
        "analyze",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "analyze_ex",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::I32, ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "analyze_model",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_analysis_status",
        MethodSignature {
            params: vec![ParamType::String],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "is_analyzing",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );

    // File 관련 메서드들
    signatures.insert(
        "close_staad_file",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_staad_file",
        MethodSignature {
            params: vec![ParamType::Bool],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_staad_file_folder",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "new_staad_file",
        MethodSignature {
            params: vec![ParamType::String, ParamType::I32, ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "open_staad_file",
        MethodSignature {
            params: vec![ParamType::String],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "save_model",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );

    // Application 관련 메서드들
    signatures.insert(
        "get_application_version",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_error_message",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_main_window_handle",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_process_handle",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_process_id",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "quit",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "set_silent_mode",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "update_structure",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );

    // Unit 관련 메서드들
    signatures.insert(
        "get_base_unit",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_input_unit_for_force",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_input_unit_for_length",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "set_input_unit_for_force",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "set_input_unit_for_length",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "set_input_units",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::I32],
            returns_unit_scaled: false,
        },
    );

    // Job Info 관련 메서드들
    signatures.insert(
        "get_full_job_info",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_short_job_info",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "set_full_job_info",
        MethodSignature {
            params: vec![
                ParamType::String, // job_name
                ParamType::String, // job_client
                ParamType::String, // engg_name
                ParamType::String, // e_date
                ParamType::String, // job_number
                ParamType::String, // revision
                ParamType::String, // part
                ParamType::String, // reference
                ParamType::String, // checker_name
                ParamType::String, // c_date
                ParamType::String, // approver_name
                ParamType::String, // a_date
                ParamType::String, // comments
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "set_short_job_info",
        MethodSignature {
            params: vec![ParamType::String, ParamType::String, ParamType::String],
            returns_unit_scaled: false,
        },
    );

    // Connected Project 관련 메서드들
    signatures.insert(
        "get_connected_project_info",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "set_connected_project_info",
        MethodSignature {
            params: vec![ParamType::String, ParamType::String],
            returns_unit_scaled: false,
        },
    );

    // View 관련 메서드들
    signatures.insert(
        "create_named_view",
        MethodSignature {
            params: vec![ParamType::String, ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "modify_named_view",
        MethodSignature {
            params: vec![
                ParamType::String,
                ParamType::I32,
                ParamType::I32,
                ParamType::I32,
                ParamType::I32,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "remove_named_view",
        MethodSignature {
            params: vec![ParamType::String],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "save_named_view",
        MethodSignature {
            params: vec![ParamType::String],
            returns_unit_scaled: false,
        },
    );

    // Model 관련 메서드들
    signatures.insert(
        "is_physical_model",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );

    signatures
}

fn call_root_method(
    root: &Root,
    method: &str,
    params: Vec<ConvertedParam>,
) -> Result<serde_json::Value, String> {
    match method {
        // Analysis 관련 메서드들
        "analyze" => {
            root.analyze().map_err(|e| e.to_string())?;
            Ok(serde_json::Value::Null)
        }
        "analyze_ex" => {
            if let (
                ConvertedParam::I32(silent),
                ConvertedParam::I32(hidden),
                ConvertedParam::I32(wait),
            ) = (&params[0], &params[1], &params[2])
            {
                let result = root
                    .analyze_ex(*silent, *hidden, *wait)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for analyze_ex".to_string())
            }
        }
        "analyze_model" => {
            if let ConvertedParam::I32(engine) = &params[0] {
                root.analyze_model(*engine).map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for analyze_model".to_string())
            }
        }
        "get_analysis_status" => {
            if let ConvertedParam::String(model_path) = &params[0] {
                let result = root
                    .get_analysis_status(model_path)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_analysis_status".to_string())
            }
        }
        "is_analyzing" => {
            let result = root.is_analyzing().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }

        // File 관련 메서드들
        "close_staad_file" => {
            root.close_staad_file().map_err(|e| e.to_string())?;
            Ok(serde_json::Value::Null)
        }
        "get_staad_file" => {
            if let ConvertedParam::Bool(full_path) = &params[0] {
                let result = root.get_staad_file(*full_path).map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_staad_file".to_string())
            }
        }
        "get_staad_file_folder" => {
            let result = root.get_staad_file_folder().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "new_staad_file" => {
            if let (
                ConvertedParam::String(file_name),
                ConvertedParam::I32(len_unit),
                ConvertedParam::I32(force_unit),
            ) = (&params[0], &params[1], &params[2])
            {
                root.new_staad_file(file_name, *len_unit, *force_unit)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for new_staad_file".to_string())
            }
        }
        "open_staad_file" => {
            if let ConvertedParam::String(file_name) = &params[0] {
                root.open_staad_file(file_name).map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for open_staad_file".to_string())
            }
        }
        "save_model" => {
            if let ConvertedParam::I32(silent) = &params[0] {
                root.save_model(*silent).map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for save_model".to_string())
            }
        }

        // Application 관련 메서드들
        "get_application_version" => {
            let result = root.get_application_version().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_error_message" => {
            let result = root.get_error_message().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_main_window_handle" => {
            let result = root.get_main_window_handle().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_process_handle" => {
            let result = root.get_process_handle().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_process_id" => {
            let result = root.get_process_id().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "quit" => {
            root.quit().map_err(|e| e.to_string())?;
            Ok(serde_json::Value::Null)
        }
        "set_silent_mode" => {
            if let ConvertedParam::I32(flag) = &params[0] {
                let result = root.set_silent_mode(*flag).map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for set_silent_mode".to_string())
            }
        }
        "update_structure" => {
            root.update_structure().map_err(|e| e.to_string())?;
            Ok(serde_json::Value::Null)
        }

        // Unit 관련 메서드들
        "get_base_unit" => {
            let result = root.get_base_unit().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_input_unit_for_force" => {
            let result = root.get_input_unit_for_force().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_input_unit_for_length" => {
            let result = root
                .get_input_unit_for_length()
                .map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "set_input_unit_for_force" => {
            if let ConvertedParam::I32(unit) = &params[0] {
                root.set_input_unit_for_force(*unit)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for set_input_unit_for_force".to_string())
            }
        }
        "set_input_unit_for_length" => {
            if let ConvertedParam::I32(unit) = &params[0] {
                root.set_input_unit_for_length(*unit)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for set_input_unit_for_length".to_string())
            }
        }
        "set_input_units" => {
            if let (ConvertedParam::I32(length_unit), ConvertedParam::I32(force_unit)) =
                (&params[0], &params[1])
            {
                root.set_input_units(*length_unit, *force_unit)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for set_input_units".to_string())
            }
        }

        // Job Info 관련 메서드들
        "get_full_job_info" => {
            let result = root.get_full_job_info().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_short_job_info" => {
            let result = root.get_short_job_info().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "set_full_job_info" => {
            if let (
                ConvertedParam::String(job_name),
                ConvertedParam::String(job_client),
                ConvertedParam::String(engg_name),
                ConvertedParam::String(e_date),
                ConvertedParam::String(job_number),
                ConvertedParam::String(revision),
                ConvertedParam::String(part),
                ConvertedParam::String(reference),
                ConvertedParam::String(checker_name),
                ConvertedParam::String(c_date),
                ConvertedParam::String(approver_name),
                ConvertedParam::String(a_date),
                ConvertedParam::String(comments),
            ) = (
                &params[0],
                &params[1],
                &params[2],
                &params[3],
                &params[4],
                &params[5],
                &params[6],
                &params[7],
                &params[8],
                &params[9],
                &params[10],
                &params[11],
                &params[12],
            ) {
                root.set_full_job_info(
                    job_name,
                    job_client,
                    engg_name,
                    e_date,
                    job_number,
                    revision,
                    part,
                    reference,
                    checker_name,
                    c_date,
                    approver_name,
                    a_date,
                    comments,
                )
                .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for set_full_job_info".to_string())
            }
        }
        "set_short_job_info" => {
            if let (
                ConvertedParam::String(job_name),
                ConvertedParam::String(job_client),
                ConvertedParam::String(engg_name),
            ) = (&params[0], &params[1], &params[2])
            {
                root.set_short_job_info(job_name, job_client, engg_name)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for set_short_job_info".to_string())
            }
        }

        // Connected Project 관련 메서드들
        "get_connected_project_info" => {
            let result = root
                .get_connected_project_info()
                .map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "set_connected_project_info" => {
            if let (ConvertedParam::String(proj_id), ConvertedParam::String(name)) =
                (&params[0], &params[1])
            {
                let result = root
                    .set_connected_project_info(proj_id, name)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for set_connected_project_info".to_string())
            }
        }

        // View 관련 메서드들
        "create_named_view" => {
            if let (ConvertedParam::String(name), ConvertedParam::I32(flag)) =
                (&params[0], &params[1])
            {
                let result = root
                    .create_named_view(name, *flag)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_named_view".to_string())
            }
        }
        "modify_named_view" => {
            if let (
                ConvertedParam::String(name),
                ConvertedParam::I32(entities),
                ConvertedParam::I32(entity_array),
                ConvertedParam::I32(array_qualifier),
                ConvertedParam::I32(modify_flag),
            ) = (&params[0], &params[1], &params[2], &params[3], &params[4])
            {
                let result = root
                    .modify_named_view(
                        name,
                        *entities,
                        *entity_array,
                        *array_qualifier,
                        *modify_flag,
                    )
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for modify_named_view".to_string())
            }
        }
        "remove_named_view" => {
            if let ConvertedParam::String(name) = &params[0] {
                let result = root.remove_named_view(name).map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for remove_named_view".to_string())
            }
        }
        "save_named_view" => {
            if let ConvertedParam::String(name) = &params[0] {
                let result = root.save_named_view(name).map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for save_named_view".to_string())
            }
        }

        // Model 관련 메서드들
        "is_physical_model" => {
            let result = root.is_physical_model().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }

        _ => Err(format!("Unknown method: {}", method)),
    }
}
