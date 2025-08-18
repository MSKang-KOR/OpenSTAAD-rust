use crate::openstaad::api::geometry::Geometry;
use crate::openstaad::tauri::store::PROCESS_STORE;
use crate::openstaad::tauri::utils::{
    ConvertedParam, MethodSignature, ParamType, StaadObject, convert_param,
};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub fn geometry_call(id: String, method: String, params: Vec<Value>) -> Result<Value, String> {
    let store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
    let geometry = match store.get(&id) {
        Some(StaadObject::Geometry(p)) => p,
        _ => return Err("Geometry not found".to_string()),
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
    let result = call_method(&geometry, &method, converted_params);

    result
}

// Geometry 메서드 시그니처 매핑
fn get_method_signatures() -> HashMap<&'static str, MethodSignature> {
    let mut signatures = HashMap::new();

    // Node 관련 메서드들
    signatures.insert(
        "add_multiple_nodes",
        MethodSignature {
            params: vec![ParamType::VecVecF64, ParamType::BaseUnit],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_node",
        MethodSignature {
            params: vec![
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_multiple_nodes",
        MethodSignature {
            params: vec![ParamType::VecI32, ParamType::VecVecF64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_node",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "delete_node",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_last_node_no",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_node_coordinates",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::BaseUnit],
            returns_unit_scaled: true,
        },
    );
    signatures.insert(
        "get_node_count",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_node_distance",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::I32, ParamType::BaseUnit],
            returns_unit_scaled: true,
        },
    );
    signatures.insert(
        "get_node_incidence",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::BaseUnit],
            returns_unit_scaled: true,
        },
    );
    signatures.insert(
        "get_node_incidence_cis2",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::BaseUnit],
            returns_unit_scaled: true,
        },
    );
    signatures.insert(
        "get_node_list",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_node_number",
        MethodSignature {
            params: vec![
                ParamType::F64,
                ParamType::F64,
                ParamType::F64,
                ParamType::BaseUnit,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_node_unique_id",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "is_orphan_node",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "set_node_coordinate",
        MethodSignature {
            params: vec![
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
        "set_node_unique_id",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::String],
            returns_unit_scaled: false,
        },
    );

    // Beam 관련 메서드들
    signatures.insert(
        "add_beam",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "add_multiple_beams",
        MethodSignature {
            params: vec![ParamType::VecVecI32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_beam",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::I32, ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "create_multiple_beams",
        MethodSignature {
            params: vec![ParamType::VecI32, ParamType::VecVecI32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "delete_beam",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_beam_length",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::BaseUnit],
            returns_unit_scaled: true,
        },
    );
    signatures.insert(
        "get_beam_list",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_beams_connected_at_node",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_last_beam_no",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_member_count",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_member_incidence",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_member_incidence_cis2",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_member_unique_id",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_no_of_beams_connected_at_node",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "is_beam",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::F64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "is_column",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::F64],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "is_z_up",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "renumber_beam",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "set_member_unique_id",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::String],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "split_beam_in_equal_parts",
        MethodSignature {
            params: vec![ParamType::I32, ParamType::I32],
            returns_unit_scaled: false,
        },
    );

    // Group 관련 메서드들
    signatures.insert(
        "create_group_ex",
        MethodSignature {
            params: vec![
                ParamType::I32,
                ParamType::String,
                ParamType::I32,
                ParamType::VecI32,
            ],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "delete_group",
        MethodSignature {
            params: vec![ParamType::String],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_group_count",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_group_count_all",
        MethodSignature {
            params: vec![],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_group_entities",
        MethodSignature {
            params: vec![ParamType::String],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_group_entity_count",
        MethodSignature {
            params: vec![ParamType::String],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "get_group_names",
        MethodSignature {
            params: vec![ParamType::I32],
            returns_unit_scaled: false,
        },
    );
    signatures.insert(
        "update_group",
        MethodSignature {
            params: vec![
                ParamType::String,
                ParamType::I32,
                ParamType::I32,
                ParamType::VecI32,
            ],
            returns_unit_scaled: false,
        },
    );

    signatures
}

fn call_method(
    geometry: &Arc<Geometry>,
    method: &str,
    params: Vec<ConvertedParam>,
) -> Result<serde_json::Value, String> {
    match method {
        // Node 관련 메서드들
        "add_multiple_nodes" => {
            if let (ConvertedParam::VecVecF64(coords), ConvertedParam::I32(base_unit)) =
                (&params[0], &params[1])
            {
                geometry
                    .add_multiple_nodes(coords.clone(), *base_unit)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for add_multiple_nodes".to_string())
            }
        }
        "add_node" => {
            if let (
                ConvertedParam::F64(x),
                ConvertedParam::F64(y),
                ConvertedParam::F64(z),
                ConvertedParam::I32(base_unit),
            ) = (&params[0], &params[1], &params[2], &params[3])
            {
                let result = geometry
                    .add_node(*x, *y, *z, *base_unit)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_node".to_string())
            }
        }
        "create_multiple_nodes" => {
            if let (ConvertedParam::VecI32(node_ids), ConvertedParam::VecVecF64(coords)) =
                (&params[0], &params[1])
            {
                geometry
                    .create_multiple_nodes(node_ids.clone(), coords.clone())
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for create_multiple_nodes".to_string())
            }
        }
        "create_node" => {
            if let (
                ConvertedParam::I32(node_no),
                ConvertedParam::F64(x),
                ConvertedParam::F64(y),
                ConvertedParam::F64(z),
            ) = (&params[0], &params[1], &params[2], &params[3])
            {
                geometry
                    .create_node(*node_no, *x, *y, *z)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for create_node".to_string())
            }
        }
        "delete_node" => {
            if let ConvertedParam::I32(node_no) = &params[0] {
                geometry.delete_node(*node_no).map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for delete_node".to_string())
            }
        }
        "get_last_node_no" => {
            let result = geometry.get_last_node_no().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_node_coordinates" => {
            if let (ConvertedParam::I32(node_no), ConvertedParam::I32(base_unit)) =
                (&params[0], &params[1])
            {
                let result = geometry
                    .get_node_coordinates(*node_no, *base_unit)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_node_coordinates".to_string())
            }
        }
        "get_node_count" => {
            let result = geometry.get_node_count().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_node_distance" => {
            if let (
                ConvertedParam::I32(node_a),
                ConvertedParam::I32(node_b),
                ConvertedParam::I32(base_unit),
            ) = (&params[0], &params[1], &params[2])
            {
                let result = geometry
                    .get_node_distance(*node_a, *node_b, *base_unit)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_node_distance".to_string())
            }
        }
        "get_node_incidence" => {
            if let (ConvertedParam::I32(node_no), ConvertedParam::I32(base_unit)) =
                (&params[0], &params[1])
            {
                let result = geometry
                    .get_node_incidence(*node_no, *base_unit)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_node_incidence".to_string())
            }
        }
        "get_node_incidence_cis2" => {
            if let (ConvertedParam::I32(node_no), ConvertedParam::I32(base_unit)) =
                (&params[0], &params[1])
            {
                let result = geometry
                    .get_node_incidence_cis2(*node_no, *base_unit)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_node_incidence_cis2".to_string())
            }
        }
        "get_node_list" => {
            let result = geometry.get_node_list().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_node_number" => {
            if let (
                ConvertedParam::F64(x),
                ConvertedParam::F64(y),
                ConvertedParam::F64(z),
                ConvertedParam::I32(base_unit),
            ) = (&params[0], &params[1], &params[2], &params[3])
            {
                let result = geometry
                    .get_node_number(*x, *y, *z, *base_unit)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_node_number".to_string())
            }
        }
        "get_node_unique_id" => {
            if let ConvertedParam::I32(node_no) = &params[0] {
                let result = geometry
                    .get_node_unique_id(*node_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_node_unique_id".to_string())
            }
        }
        "is_orphan_node" => {
            if let ConvertedParam::I32(node_no) = &params[0] {
                let result = geometry
                    .is_orphan_node(*node_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for is_orphan_node".to_string())
            }
        }
        "set_node_coordinate" => {
            if let (
                ConvertedParam::I32(node_no),
                ConvertedParam::F64(x),
                ConvertedParam::F64(y),
                ConvertedParam::F64(z),
                ConvertedParam::I32(base_unit),
            ) = (&params[0], &params[1], &params[2], &params[3], &params[4])
            {
                geometry
                    .set_node_coordinate(*node_no, *x, *y, *z, *base_unit)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for set_node_coordinate".to_string())
            }
        }
        "set_node_unique_id" => {
            if let (ConvertedParam::I32(node_no), ConvertedParam::String(unique_id)) =
                (&params[0], &params[1])
            {
                geometry
                    .set_node_unique_id(*node_no, unique_id)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for set_node_unique_id".to_string())
            }
        }

        // Beam 관련 메서드들
        "add_beam" => {
            if let (ConvertedParam::I32(node_a), ConvertedParam::I32(node_b)) =
                (&params[0], &params[1])
            {
                let result = geometry
                    .add_beam(*node_a, *node_b)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for add_beam".to_string())
            }
        }
        "add_multiple_beams" => {
            if let ConvertedParam::VecVecI32(incidences) = &params[0] {
                geometry
                    .add_multiple_beams(incidences.clone())
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for add_multiple_beams".to_string())
            }
        }
        "create_beam" => {
            if let (
                ConvertedParam::I32(beam_no),
                ConvertedParam::I32(node_a),
                ConvertedParam::I32(node_b),
            ) = (&params[0], &params[1], &params[2])
            {
                geometry
                    .create_beam(*beam_no, *node_a, *node_b)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for create_beam".to_string())
            }
        }
        "create_multiple_beams" => {
            if let (ConvertedParam::VecI32(beam_ids), ConvertedParam::VecVecI32(incidences)) =
                (&params[0], &params[1])
            {
                geometry
                    .create_multiple_beams(beam_ids.clone(), incidences.clone())
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for create_multiple_beams".to_string())
            }
        }
        "delete_beam" => {
            if let ConvertedParam::I32(beam_no) = &params[0] {
                geometry.delete_beam(*beam_no).map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for delete_beam".to_string())
            }
        }
        "get_beam_length" => {
            if let (ConvertedParam::I32(beam_no), ConvertedParam::I32(base_unit)) =
                (&params[0], &params[1])
            {
                let result = geometry
                    .get_beam_length(*beam_no, *base_unit)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_beam_length".to_string())
            }
        }
        "get_beam_list" => {
            let result = geometry.get_beam_list().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_beams_connected_at_node" => {
            if let ConvertedParam::I32(node_no) = &params[0] {
                let result = geometry
                    .get_beams_connected_at_node(*node_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_beams_connected_at_node".to_string())
            }
        }
        "get_last_beam_no" => {
            let result = geometry.get_last_beam_no().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_member_count" => {
            let result = geometry.get_member_count().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_member_incidence" => {
            if let ConvertedParam::I32(beam_no) = &params[0] {
                let result = geometry
                    .get_member_incidence(*beam_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_member_incidence".to_string())
            }
        }
        "get_member_incidence_cis2" => {
            if let ConvertedParam::I32(beam_no) = &params[0] {
                let result = geometry
                    .get_member_incidence_cis2(*beam_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_member_incidence_cis2".to_string())
            }
        }
        "get_member_unique_id" => {
            if let ConvertedParam::I32(member_no) = &params[0] {
                let result = geometry
                    .get_member_unique_id(*member_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_member_unique_id".to_string())
            }
        }
        "get_no_of_beams_connected_at_node" => {
            if let ConvertedParam::I32(node_no) = &params[0] {
                let result = geometry
                    .get_no_of_beams_connected_at_node(*node_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_no_of_beams_connected_at_node".to_string())
            }
        }
        "is_beam" => {
            if let (ConvertedParam::I32(member_no), ConvertedParam::F64(tolerance_angle)) =
                (&params[0], &params[1])
            {
                let result = geometry
                    .is_beam(*member_no, *tolerance_angle)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for is_beam".to_string())
            }
        }
        "is_column" => {
            if let (ConvertedParam::I32(member_no), ConvertedParam::F64(tolerance_angle)) =
                (&params[0], &params[1])
            {
                let result = geometry
                    .is_column(*member_no, *tolerance_angle)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for is_column".to_string())
            }
        }
        "is_z_up" => {
            let result = geometry.is_z_up().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "renumber_beam" => {
            if let (ConvertedParam::I32(old_beam_no), ConvertedParam::I32(new_beam_no)) =
                (&params[0], &params[1])
            {
                let result = geometry
                    .renumber_beam(*old_beam_no, *new_beam_no)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for renumber_beam".to_string())
            }
        }
        "set_member_unique_id" => {
            if let (ConvertedParam::I32(member_no), ConvertedParam::String(unique_id)) =
                (&params[0], &params[1])
            {
                geometry
                    .set_member_unique_id(*member_no, unique_id)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for set_member_unique_id".to_string())
            }
        }
        "split_beam_in_equal_parts" => {
            if let (ConvertedParam::I32(beam_no), ConvertedParam::I32(num_parts)) =
                (&params[0], &params[1])
            {
                geometry
                    .split_beam_in_equal_parts(*beam_no, *num_parts)
                    .map_err(|e| e.to_string())?;
                Ok(serde_json::Value::Null)
            } else {
                Err("Invalid parameters for split_beam_in_equal_parts".to_string())
            }
        }

        // Group 관련 메서드들
        "create_group_ex" => {
            if let (
                ConvertedParam::I32(group_type),
                ConvertedParam::String(group_name),
                ConvertedParam::I32(entity_count),
                ConvertedParam::VecI32(entity_list),
            ) = (&params[0], &params[1], &params[2], &params[3])
            {
                let result = geometry
                    .create_group_ex(*group_type, group_name, *entity_count, entity_list.clone())
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for create_group_ex".to_string())
            }
        }
        "delete_group" => {
            if let ConvertedParam::String(group_name) = &params[0] {
                let result = geometry
                    .delete_group(group_name)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for delete_group".to_string())
            }
        }
        "get_group_count" => {
            if let ConvertedParam::I32(group_type) = &params[0] {
                let result = geometry
                    .get_group_count(*group_type)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_group_count".to_string())
            }
        }
        "get_group_count_all" => {
            let result = geometry.get_group_count_all().map_err(|e| e.to_string())?;
            serde_json::to_value(&result).map_err(|e| e.to_string())
        }
        "get_group_entities" => {
            if let ConvertedParam::String(group_name) = &params[0] {
                let result = geometry
                    .get_group_entities(group_name)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_group_entities".to_string())
            }
        }
        "get_group_entity_count" => {
            if let ConvertedParam::String(group_name) = &params[0] {
                let result = geometry
                    .get_group_entity_count(group_name)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_group_entity_count".to_string())
            }
        }
        "get_group_names" => {
            if let ConvertedParam::I32(group_type) = &params[0] {
                let result = geometry
                    .get_group_names(*group_type)
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for get_group_names".to_string())
            }
        }
        "update_group" => {
            if let (
                ConvertedParam::String(group_name),
                ConvertedParam::I32(flag),
                ConvertedParam::I32(entity_count),
                ConvertedParam::VecI32(entity_list),
            ) = (&params[0], &params[1], &params[2], &params[3])
            {
                let result = geometry
                    .update_group(group_name, *flag, *entity_count, entity_list.clone())
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(&result).map_err(|e| e.to_string())
            } else {
                Err("Invalid parameters for update_group".to_string())
            }
        }

        _ => Err(format!("Unknown method: {}", method)),
    }
}
