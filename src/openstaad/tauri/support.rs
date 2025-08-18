use crate::openstaad::api::support::Support;
use crate::openstaad::tauri::store::PROCESS_STORE;
use crate::openstaad::tauri::utils::{
    ConvertedParam, MethodSignature, ParamType, StaadObject, convert_param,
};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub fn support_call(id: String, method: String, params: Vec<Value>) -> Result<Value, String> {
    let store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
    let support = match store.get(&id) {
        Some(StaadObject::Support(s)) => s,
        _ => return Err("Support not found".to_string()),
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
    let result = call_method(&support, &method, converted_params);

    result
}

// Support 메서드 시그니처 매핑
fn get_method_signatures() -> HashMap<&'static str, MethodSignature> {
    let mut signatures = HashMap::new();

    // Support Assignment 관련 메서드들
    signatures.insert(
        "assign_support_to_node",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "remove_support_from_node",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );

    // Support Creation 관련 메서드들
    signatures.insert(
        "create_inclined_support",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::I32,
                ParamType::I32,
                ParamType::VecF64,
                ParamType::VecF64,
                ParamType::VecF64,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_support_fixed",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_support_fixed_but",
        MethodSignature {
            params: vec![ParamType::VecF64, ParamType::VecF64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_support_pinned",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );

    // Support Management 관련 메서드들
    signatures.insert(
        "delete_support",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_support_name",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_support_unique_id",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "set_support_unique_id",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::String],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_support_count",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );

    // Support Information 관련 메서드들
    signatures.insert(
        "get_support_information",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_support_information_ex",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_support_nodes",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );

    signatures
}

fn call_method(
    support: &Arc<Support>,
    method: &str,
    params: Vec<ConvertedParam>,
) -> Result<serde_json::Value, String> {
    match method {
        // Support Assignment 관련 메서드들
        "assign_support_to_node" => {
            if let (ConvertedParam::I32(node_no), ConvertedParam::I32(support_no)) =
                (&params[0], &params[1])
            {
                let result = support
                    .assign_support_to_node(*node_no, *support_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for assign_support_to_node".to_string())
            }
        }
        "remove_support_from_node" => {
            if let ConvertedParam::I32(node_no) = &params[0] {
                let result = support
                    .remove_support_from_node(*node_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for remove_support_from_node".to_string())
            }
        }

        // Support Creation 관련 메서드들
        "create_inclined_support" => {
            if let (
                ConvertedParam::I32(inclined_type),
                ConvertedParam::I32(ref_type),
                ConvertedParam::I32(ref_node),
                ConvertedParam::VecF64(coord),
                ConvertedParam::VecF64(release_spec),
                ConvertedParam::VecF64(spring_spec),
            ) = (
                &params[0], &params[1], &params[2], &params[3], &params[4], &params[5],
            ) {
                let result = support
                    .create_inclined_support(
                        *inclined_type,
                        *ref_type,
                        *ref_node,
                        coord.clone(),
                        release_spec.clone(),
                        spring_spec.clone(),
                    )
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_inclined_support".to_string())
            }
        }
        "create_support_fixed" => {
            let result = support.create_support_fixed().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "create_support_fixed_but" => {
            if let (ConvertedParam::VecF64(release_spec), ConvertedParam::VecF64(sprint_spec)) =
                (&params[0], &params[1])
            {
                let result = support
                    .create_support_fixed_but(release_spec.clone(), sprint_spec.clone())
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_support_fixed_but".to_string())
            }
        }
        "create_support_pinned" => {
            let result = support.create_support_pinned().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }

        // Support Management 관련 메서드들
        "delete_support" => {
            if let ConvertedParam::I32(support_no) = &params[0] {
                let result = support
                    .delete_support(*support_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for delete_support".to_string())
            }
        }
        "get_support_name" => {
            if let ConvertedParam::I32(support_no) = &params[0] {
                let result = support
                    .get_support_name(*support_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_support_name".to_string())
            }
        }
        "get_support_unique_id" => {
            if let ConvertedParam::I32(support_no) = &params[0] {
                let result = support
                    .get_support_unique_id(*support_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_support_unique_id".to_string())
            }
        }
        "set_support_unique_id" => {
            if let (ConvertedParam::I32(support_no), ConvertedParam::String(unique_id)) =
                (&params[0], &params[1])
            {
                support
                    .set_support_unique_id(*support_no, unique_id)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for set_support_unique_id".to_string())
            }
        }
        "get_support_count" => {
            let result = support.get_support_count().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }

        // Support Information 관련 메서드들
        "get_support_information" => {
            if let ConvertedParam::I32(support_node) = &params[0] {
                let result = support
                    .get_support_information(*support_node)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_support_information".to_string())
            }
        }
        "get_support_information_ex" => {
            if let ConvertedParam::I32(support_node) = &params[0] {
                let result = support
                    .get_support_information_ex(*support_node)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_support_information_ex".to_string())
            }
        }
        "get_support_nodes" => {
            let result = support.get_support_nodes().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }

        _ => Err(format!("Unknown method: {}", method)),
    }
}
