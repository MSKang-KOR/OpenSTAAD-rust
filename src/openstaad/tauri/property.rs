use crate::openstaad::api::property::Property;
use crate::openstaad::tauri::store::PROCESS_STORE;
use crate::openstaad::tauri::utils::{
    ConvertedParam, MethodSignature, ParamType, StaadObject, convert_param,
};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub fn property_call(id: String, method: String, params: Vec<Value>) -> Result<Value, String> {
    let store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
    let property = match store.get(&id) {
        Some(StaadObject::Property(p)) => p,
        _ => return Err("Property not found".to_string()),
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
    let result = call_method(&property, &method, converted_params);

    result
}

// Property 메서드 시그니처 매핑
fn get_method_signatures() -> HashMap<&'static str, MethodSignature> {
    let mut signatures = HashMap::new();

    // Property Creation from Table 관련 메서드들
    signatures.insert(
        "create_angle_property_from_table",
        MethodSignature {
            params: vec![
                ParamType::String,
                ParamType::String,
                ParamType::String,
                ParamType::VecI32,
                ParamType::VecF64,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_beam_property_from_table",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::String,
                ParamType::I32,
                ParamType::F64,
                ParamType::F64,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_beam_property_from_table_ex",
        MethodSignature {
            params: vec![
                ParamType::String,
                ParamType::String,
                ParamType::VecI32,
                ParamType::VecF64,
                ParamType::I32,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_channel_property_from_table",
        MethodSignature {
            params: vec![
                ParamType::String,
                ParamType::String,
                ParamType::VecI32,
                ParamType::VecF64,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_plate_thickness_property",
        MethodSignature {
            params: vec![ParamType::VecF64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_tee_property_from_table",
        MethodSignature {
            params: vec![
                ParamType::String,
                ParamType::String,
                ParamType::VecI32,
                ParamType::VecF64,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_wide_flange_property_from_table",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::String,
                ParamType::I32,
                ParamType::VecF64,
            ],
            returns_unit_scaled: false,
        },
    );

    // Prismatic Property Creation 관련 메서드들
    signatures.insert(
        "create_prismatic_tee_property",
        MethodSignature {
            params: vec![
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_prismatic_trapezoidal_property",
        MethodSignature {
            params: vec![
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
            ],
            returns_unit_scaled: false,
        },
    );

    // UPT Property 관련 메서드들
    signatures.insert(
        "add_upt_property_channel",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::String,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::I32,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_upt_property_general",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::String,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
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
        "add_upt_property_isection",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::String,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::I32,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_upt_property_wide_flange",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::String,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::I32,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );

    // UPT Table 관련 메서드들
    signatures.insert(
        "create_property_from_upt_table",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::String,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_upt_table",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );

    signatures
}

fn call_method(
    property: &Arc<Property>,
    method: &str,
    params: Vec<ConvertedParam>,
) -> Result<serde_json::Value, String> {
    match method {
        // Property Creation from Table 관련 메서드들
        "create_beam_property_from_table" => {
            if let (
                ConvertedParam::I32(country),
                ConvertedParam::String(section_name),
                ConvertedParam::I32(type_spec),
                ConvertedParam::F64(add_spec_1),
                ConvertedParam::F64(add_spec2),
            ) = (&params[0], &params[1], &params[2], &params[3], &params[4])
            {
                let result = property
                    .create_beam_property_from_table(
                        *country,
                        section_name,
                        *type_spec,
                        *add_spec_1,
                        *add_spec2,
                    )
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_beam_property_from_table".to_string())
            }
        }
        "create_wide_flange_property_from_table" => {
            if let (
                ConvertedParam::I32(country),
                ConvertedParam::String(section_spec),
                ConvertedParam::I32(type_spec),
                ConvertedParam::VecF64(specs),
            ) = (&params[0], &params[1], &params[2], &params[3])
            {
                let result = property
                    .create_wide_flange_property_from_table(
                        *country,
                        section_spec,
                        *type_spec,
                        specs.clone(),
                    )
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_wide_flange_property_from_table".to_string())
            }
        }
        "create_plate_thickness_property" => {
            if let ConvertedParam::VecF64(thickness) = &params[0] {
                let result = property
                    .create_plate_thickness_property(thickness.clone())
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_plate_thickness_property".to_string())
            }
        }

        // Prismatic Property Creation 관련 메서드들
        "create_prismatic_tee_property" => {
            if let (
                ConvertedParam::F64(yd),
                ConvertedParam::F64(zd),
                ConvertedParam::F64(yb),
                ConvertedParam::F64(zb),
            ) = (&params[0], &params[1], &params[2], &params[3])
            {
                let result = property
                    .create_prismatic_tee_property(*yd, *zd, *yb, *zb)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_prismatic_tee_property".to_string())
            }
        }
        "create_prismatic_trapezoidal_property" => {
            if let (ConvertedParam::F64(yd), ConvertedParam::F64(zd), ConvertedParam::F64(zb)) =
                (&params[0], &params[1], &params[2])
            {
                let result = property
                    .create_prismatic_trapezoidal_property(*yd, *zd, *zb)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_prismatic_trapezoidal_property".to_string())
            }
        }

        // UPT Table 관련 메서드들
        "create_upt_table" => {
            if let ConvertedParam::I32(table_type) = &params[0] {
                let result = property
                    .create_upt_table(*table_type)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_upt_table".to_string())
            }
        }
        "create_property_from_upt_table" => {
            if let (ConvertedParam::I32(table_id), ConvertedParam::String(section_name)) =
                (&params[0], &params[1])
            {
                let result = property
                    .create_property_from_upt_table(*table_id, section_name)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_property_from_upt_table".to_string())
            }
        }

        _ => Err(format!("Unknown method: {}", method)),
    }
}
