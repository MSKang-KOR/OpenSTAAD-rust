pub mod openstaad;
pub mod tools;

pub use anyhow::{Context, Error, Result, anyhow, bail};
pub use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread::{self, JoinHandle};
use tauri::AppHandle;
// use tokio::sync::{Mutex, mpsc, oneshot};
use windows::Win32::System::Variant::VARIANT;

use serde_json::{Value, json};

use crate::openstaad::app::{self, OpenStaad};
use crate::openstaad::{bindings::Staad, custom::*, execute::execute_method};
use crate::tools::{InType, Input, invoke_method};

type StorageType = Arc<Mutex<HashMap<u32, OpenStaad>>>;
pub static STAAD_MANAGER: LazyLock<Mutex<Option<StorageType>>> = LazyLock::new(|| Mutex::new(None));

// Manager 시작
pub fn start_manager() -> Result<Value, String> {
    let mut manager = STAAD_MANAGER.lock().unwrap();
    if manager.is_some() {
        return Ok(json!("Manager already started"));
    }
    *manager = Some(Arc::new(Mutex::new(HashMap::<u32, OpenStaad>::new())));

    Ok(json!("Manager started successfully"))
}

// 특정 Worker에 메시지 전송
pub fn send(msg_type: String, data: Value, app_handle: AppHandle) -> Result<Value, String> {
    match msg_type.as_str() {
        "Open" => {
            let params = data["params"]
                .as_array()
                .ok_or("Missing params parameter")?
                .clone();
            let path = params[0]
                .as_str()
                .ok_or("Missing path parameter")?
                .to_string();
            let std_path = params[1]
                .as_str()
                .ok_or("Missing std_path parameter")?
                .to_string();
            let openstaad = handle_open(path.clone(), std_path.clone())?;
            let staad_id = openstaad.id;
            {
                let manager_guard = STAAD_MANAGER.lock().map_err(|e| e.to_string())?;
                if manager_guard.is_none() {
                    return Err("STAAD_STORAGE is not started".to_string());
                }

                let manager_arc = manager_guard
                    .as_ref()
                    .ok_or("STAAD_STORAGE is not started")?;
                let mut hashmap = manager_arc.lock().map_err(|e| e.to_string())?;
                hashmap.insert(staad_id, openstaad);
            }
            Ok(json!(staad_id))
        }
        "Connect" => {
            let params = data["params"]
                .as_array()
                .ok_or("Missing params parameter")?
                .clone();
            let pid = params[0].as_u64().ok_or("Missing pid parameter")? as u32;

            let openstaad = handle_connect(pid)?;
            let staad_id = openstaad.id;
            {
                let manager_guard = STAAD_MANAGER.lock().map_err(|e| e.to_string())?;
                if manager_guard.is_none() {
                    return Err("STAAD_STORAGE is not started".to_string());
                }

                let manager_arc = manager_guard
                    .as_ref()
                    .ok_or("STAAD_STORAGE is not started")?;
                let mut hashmap = manager_arc.lock().map_err(|e| e.to_string())?;
                hashmap.insert(staad_id, openstaad);
            }
            Ok(json!(staad_id))
        }
        "Execute" => {
            let id = data["id"].as_u64().ok_or("Missing method id")? as u32;
            let method = data["method"]
                .as_str()
                .ok_or("Missing method parameter")?
                .to_string();
            let params = data["params"]
                .as_array()
                .ok_or("Missing params parameter")?
                .clone();

            let manager_guard = STAAD_MANAGER.lock().map_err(|e| e.to_string())?;
            if manager_guard.is_none() {
                return Err("STAAD_STORAGE is not started".to_string());
            }
            let manager_arc = manager_guard
                .as_ref()
                .ok_or("STAAD_STORAGE is not started")?;
            let mut hashmap_guard = manager_arc.lock().map_err(|e| e.to_string())?;
            if let Some(staad) = hashmap_guard.get_mut(&id) {
                return handle_method(staad, method, params);
            }
            Err(format!("No Staad instance opened in this worker: {}", id))
        }
        "ExecuteBatch" => {
            let id = data["id"].as_u64().ok_or("Missing method id")? as u32;
            let method = data["method"]
                .as_str()
                .ok_or("Missing method parameter")?
                .to_string();
            let params = data["params"]
                .as_array()
                .ok_or("Missing params parameter")?
                .clone();

            let manager_guard = STAAD_MANAGER.lock().map_err(|e| e.to_string())?;
            if manager_guard.is_none() {
                return Err("STAAD_STORAGE is not started".to_string());
            }
            let manager_arc = manager_guard
                .as_ref()
                .ok_or("STAAD_STORAGE is not started")?;
            let mut hashmap_guard = manager_arc.lock().map_err(|e| e.to_string())?;
            if let Some(staad) = hashmap_guard.get_mut(&id) {
                return handle_custom_method(staad, method, params, app_handle);
            }
            Err(format!("No Staad instance opened in this worker: {}", id))
        }
        "Quit" => {
            let manager_guard = STAAD_MANAGER.lock().map_err(|e| e.to_string())?;
            if manager_guard.is_none() {
                return Err("STAAD_STORAGE is not started".to_string());
            }
            let manager_arc = manager_guard
                .as_ref()
                .ok_or("STAAD_STORAGE is not started")?;
            let mut hashmap = manager_arc.lock().map_err(|e| e.to_string())?;
            // HashMap의 모든 값에 대해 Quit 메소드 실행
            for (_id, openstaad) in hashmap.iter_mut() {
                let method = "Quit";
                let instance = Staad::from_method(openstaad, method)
                    .map_err(|e| format!("Staad instance error: {}", e))?;
                execute_method(&instance, method, &[])
                    .map_err(|e| format!("Execute method error: {}", e))?;
            }
            hashmap.clear();
            Ok(json!("Quit successfully".to_string()))
        }
        _ => Err(format!("Invalid message type: {}", msg_type)),
    }
}

fn handle_open(path: String, std_path: String) -> Result<OpenStaad, String> {
    let mut openstaad =
        OpenStaad::new(path.clone(), std_path.clone()).map_err(|e| e.to_string())?;
    let root = openstaad.get_root().map_err(|e| e.to_string())?;

    // Silent mode 설정
    let _ = unsafe {
        invoke_method(&root.dispatch, "SetSilentMode", &mut [VARIANT::from(1)])
            .map_err(|e| e.to_string())
    };
    Ok(openstaad)
}

fn handle_connect(pid: u32) -> Result<OpenStaad, String> {
    let mut openstaad = OpenStaad::new_by_pid(pid).map_err(|e| e.to_string())?;
    let root = openstaad.get_root().map_err(|e| e.to_string())?;

    // Silent mode 설정
    let _ = unsafe {
        invoke_method(&root.dispatch, "SetSilentMode", &mut [VARIANT::from(1)])
            .map_err(|e| e.to_string())
    };
    Ok(openstaad)
}

fn handle_method(
    staad: &mut OpenStaad,
    method: String,
    params: Vec<Value>,
) -> Result<Value, String> {
    let instance = Staad::from_method(staad, method.as_str()).map_err(|e| e.to_string())?;
    let converted_params =
        convert_to_inputs(&instance, method.as_str(), &params).map_err(|e| e.to_string())?;
    return execute_method(&instance, method.as_str(), converted_params.as_slice())
        .map_err(|e| e.to_string());
}

fn handle_custom_method(
    openstaad: &mut OpenStaad,
    method: String,
    params: Vec<Value>,
    handle: AppHandle,
) -> Result<Value, String> {
    // let openstaad = match staad {
    //     Staad::OpenStaad(v) => Ok(v),
    //     _ => Err("Must be OpenStaad instance".to_string()),
    // }?;
    match method.as_str() {
        "get_node_table" => {
            let v = get_node_table(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "get_beam_table" => {
            let v = get_beam_table(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "get_section_list" => {
            let v = get_section_list(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "get_section_property_tables" => {
            let v = get_section_property_tables(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "get_beta_list" => {
            let v = get_beta_list(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "get_isotropic_material_list" => {
            let v = get_isotropic_material_list(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "get_orthotropic2d_material_list" => {
            let v = get_orthotropic2d_material_list(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "get_specification_list" => {
            let v = get_specification_list(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "get_support_list" => {
            let v = get_support_list(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "get_reference_load_list" => {
            let v = get_reference_load_list(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "get_load_case_list" => {
            let v = get_load_case_list(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "get_load_item_list" => {
            let v = get_load_item_list(openstaad, params[0].clone()).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "analyze" => {
            let v = analyze(openstaad, handle).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "get_design_results" => {
            let v = get_design_results(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        _ => return Err(format!("Invalid custom method name: {}", method)),
    }
}

fn convert_to_inputs(instance: &Staad, method: &str, params: &Vec<Value>) -> Result<Vec<Input>> {
    let methods = match instance {
        Staad::Root(v) => &v.methods,
        Staad::Geometry(v) => &v.methods,
        Staad::Command(v) => &v.methods,
        Staad::Design(v) => &v.methods,
        Staad::Load(v) => &v.methods,
        Staad::Output(v) => &v.methods,
        Staad::Property(v) => &v.methods,
        Staad::Support(v) => &v.methods,
        _ => bail!("[Convert] Unsupported Staad instance".to_string()),
    };

    let (_inputs, _outputs) = match methods.get(method) {
        Some(sig) => (&sig.inputs, &sig.outputs),
        None => bail!(format!(
            "[Convert] Unsupported method on {:#?}: {}",
            instance, method
        )),
    };

    let params_count = params.len();
    let required_count = InType::count_general_type(_inputs);
    if params_count != required_count {
        bail!(
            "[Convert] '{}' method takes {} arguments but {} arguments were supplied",
            method,
            required_count,
            params_count
        );
    }

    let converted_inputs: Result<Vec<Input>> = InType::filter_general_type(_inputs)
        .iter()
        .enumerate()
        .map(|(i, _type)| _type.to_input_as(&params[i]))
        .collect();

    converted_inputs
}
