use crate::openstaad::api::design::Design;
use crate::openstaad::tauri::store::PROCESS_STORE;
use crate::openstaad::tauri::utils::{
    ConvertedParam, MethodSignature, ParamType, StaadObject, convert_param,
};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub fn design_call(id: String, method: String, params: Vec<Value>) -> Result<Value, String> {
    let store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
    let design = match store.get(&id) {
        Some(StaadObject::Design(d)) => d,
        _ => return Err("Design not found".to_string()),
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
    let result = call_method(&design, &method, converted_params);

    result
}

// Design 메서드 시그니처 매핑
fn get_method_signatures() -> HashMap<&'static str, MethodSignature> {
    let mut signatures = HashMap::new();

    // Design Command 관련 메서드들
    signatures.insert(
        "assign_design_command",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::String,
                ParamType::String,
                ParamType::VecI32,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "assign_design_group",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::String,
                ParamType::String,
                ParamType::I32,
                ParamType::VecI32,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "assign_design_parameter",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::String,
                ParamType::String,
                ParamType::VecI32,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_design_brief",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_design_brief_code",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_member_design_parameters",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::I32],
            returns_unit_scaled: false,
        },
    );

    signatures
}

fn call_method(
    design: &Arc<Design>,
    method: &str,
    params: Vec<ConvertedParam>,
) -> Result<serde_json::Value, String> {
    match method {
        // Design Command 관련 메서드들
        "assign_design_command" => {
            if let (
                ConvertedParam::I32(brief_ref),
                ConvertedParam::String(command_name),
                ConvertedParam::String(command_value),
                ConvertedParam::VecI32(members),
            ) = (&params[0], &params[1], &params[2], &params[3])
            {
                let result = design
                    .assign_design_command(*brief_ref, command_name, command_value, members.clone())
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for assign_design_command".to_string())
            }
        }
        "assign_design_group" => {
            if let (
                ConvertedParam::I32(brief_ref),
                ConvertedParam::String(command_name),
                ConvertedParam::String(command_value),
                ConvertedParam::I32(same_as_member),
                ConvertedParam::VecI32(members),
            ) = (&params[0], &params[1], &params[2], &params[3], &params[4])
            {
                let result = design
                    .assign_design_group(
                        *brief_ref,
                        command_name,
                        command_value,
                        *same_as_member,
                        members.clone(),
                    )
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for assign_design_group".to_string())
            }
        }
        "assign_design_parameter" => {
            if let (
                ConvertedParam::I32(brief_ref),
                ConvertedParam::String(param_name),
                ConvertedParam::String(param_value),
                ConvertedParam::VecI32(members),
            ) = (&params[0], &params[1], &params[2], &params[3])
            {
                let result = design
                    .assign_design_parameter(*brief_ref, param_name, param_value, members.clone())
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for assign_design_parameter".to_string())
            }
        }
        "create_design_brief" => {
            if let ConvertedParam::I32(design_code) = &params[0] {
                let result = design
                    .create_design_brief(*design_code)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_design_brief".to_string())
            }
        }
        "get_design_brief_code" => {
            if let ConvertedParam::I32(brief_ref) = &params[0] {
                let result = design
                    .get_design_brief_code(*brief_ref)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_design_brief_code".to_string())
            }
        }
        "get_member_design_parameters" => {
            if let (ConvertedParam::I32(brief_ref), ConvertedParam::I32(member_no)) =
                (&params[0], &params[1])
            {
                let result = design
                    .get_member_design_parameters(*brief_ref, *member_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_member_design_parameters".to_string())
            }
        }

        _ => Err(format!("Unknown method: {}", method)),
    }
}

