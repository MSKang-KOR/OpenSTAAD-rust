use crate::openstaad::api::command::Command;
use crate::openstaad::tauri::store::PROCESS_STORE;
use crate::openstaad::tauri::utils::{
    ConvertedParam, MethodSignature, ParamType, StaadObject, convert_param,
};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub fn command_call(id: String, method: String, params: Vec<Value>) -> Result<Value, String> {
    let store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
    let command = match store.get(&id) {
        Some(StaadObject::Command(c)) => c,
        _ => return Err("Command not found".to_string()),
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
    let result = call_method(&command, &method, converted_params);

    result
}

// Command 메서드 시그니처 매핑
fn get_method_signatures() -> HashMap<&'static str, MethodSignature> {
    let mut signatures = HashMap::new();

    // Analysis 관련 메서드들
    signatures.insert(
        "perform_analysis",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );

    signatures
}

fn call_method(
    command: &Arc<Command>,
    method: &str,
    params: Vec<ConvertedParam>,
) -> Result<serde_json::Value, String> {
    match method {
        // Analysis 관련 메서드들
        "perform_analysis" => {
            if let ConvertedParam::I32(print_option) = &params[0] {
                command.perform_analysis(*print_option).map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for perform_analysis".to_string())
            }
        }

        _ => Err(format!("Unknown method: {}", method)),
    }
}