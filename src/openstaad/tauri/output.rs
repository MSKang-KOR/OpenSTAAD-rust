use crate::openstaad::api::output::Output;
use crate::openstaad::tauri::store::PROCESS_STORE;
use crate::openstaad::tauri::utils::{
    ConvertedParam, MethodSignature, ParamType, StaadObject, convert_param,
};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub fn output_call(id: String, method: String, params: Vec<Value>) -> Result<Value, String> {
    let store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
    let output = match store.get(&id) {
        Some(StaadObject::Output(o)) => o,
        _ => return Err("Output not found".to_string()),
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
    let result = call_method(&output, &method, converted_params);

    result
}

// Output 메서드 시그니처 매핑
fn get_method_signatures() -> HashMap<&'static str, MethodSignature> {
    let mut signatures = HashMap::new();

    // Results availability 관련 메서드들
    signatures.insert(
        "are_results_available",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );

    // Design Results 관련 메서드들
    signatures.insert(
        "get_member_design_section_name",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_member_steel_design_max_failure_ratio",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_member_steel_design_min_failure_ratio",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_member_steel_design_ratio",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_member_steel_design_results",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::BaseUnit],
            returns_unit_scaled: true,
        },
    );
    signatures.insert(
        "get_multiple_member_steel_design_max_ratio",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_multiple_member_steel_design_ratio",
        MethodSignature {
            params: vec![ParamType::String, ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_multiple_member_steel_design_results",
        MethodSignature {
            params: vec![ParamType::String, ParamType::I32],
            returns_unit_scaled: false,
        },
    );

    // Design Parameter Block 관련 메서드들
    signatures.insert(
        "get_steel_design_parameter_block_count",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_steel_design_parameter_block_name_by_index",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "is_multiple_member_steel_design_results_available",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );

    signatures
}

fn call_method(
    output: &Arc<Output>,
    method: &str,
    params: Vec<ConvertedParam>,
) -> Result<serde_json::Value, String> {
    match method {
        // Results availability 관련 메서드들
        "are_results_available" => {
            let result = output.are_results_available().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }

        // Design Results 관련 메서드들
        "get_member_design_section_name" => {
            if let ConvertedParam::I32(member_no) = &params[0] {
                let result = output
                    .get_member_design_section_name(*member_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_member_design_section_name".to_string())
            }
        }
        "get_member_steel_design_max_failure_ratio" => {
            let result = output
                .get_member_steel_design_max_failure_ratio()
                .map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_member_steel_design_min_failure_ratio" => {
            let result = output
                .get_member_steel_design_min_failure_ratio()
                .map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_member_steel_design_ratio" => {
            if let ConvertedParam::I32(member_no) = &params[0] {
                let result = output
                    .get_member_steel_design_ratio(*member_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_member_steel_design_ratio".to_string())
            }
        }
        "get_member_steel_design_results" => {
            if let (ConvertedParam::I32(member_no), ConvertedParam::I32(base_unit)) =
                (&params[0], &params[1])
            {
                let result = output
                    .get_member_steel_design_results(*member_no, *base_unit)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_member_steel_design_results".to_string())
            }
        }
        "get_multiple_member_steel_design_max_ratio" => {
            if let ConvertedParam::I32(member_no) = &params[0] {
                let result = output
                    .get_multiple_member_steel_design_max_ratio(*member_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_multiple_member_steel_design_max_ratio".to_string())
            }
        }
        "get_multiple_member_steel_design_ratio" => {
            if let (ConvertedParam::String(param_block_name), ConvertedParam::I32(member_no)) =
                (&params[0], &params[1])
            {
                let result = output
                    .get_multiple_member_steel_design_ratio(&param_block_name, *member_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_multiple_member_steel_design_ratio".to_string())
            }
        }
        "get_multiple_member_steel_design_results" => {
            if let (ConvertedParam::String(param_block_name), ConvertedParam::I32(member_no)) =
                (&params[0], &params[1])
            {
                let result = output
                    .get_multiple_member_steel_design_results(&param_block_name, *member_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_multiple_member_steel_design_results".to_string())
            }
        }

        // Design Parameter Block 관련 메서드들
        "get_steel_design_parameter_block_count" => {
            let result = output
                .get_steel_design_parameter_block_count()
                .map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_steel_design_parameter_block_name_by_index" => {
            if let ConvertedParam::I32(index) = &params[0] {
                let result = output
                    .get_steel_design_parameter_block_name_by_index(*index)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err(
                    "Invalid parameters for get_steel_design_parameter_block_name_by_index"
                        .to_string(),
                )
            }
        }
        "is_multiple_member_steel_design_results_available" => {
            let result = output
                .is_multiple_member_steel_design_results_available()
                .map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }

        _ => Err(format!("Unknown method: {}", method)),
    }
}
