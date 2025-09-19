pub mod openstaad;
pub mod tools;

pub use anyhow::{Context, Error, Result, anyhow, bail};
pub use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::thread::{self, JoinHandle};
use tauri::AppHandle;
use tokio::sync::{mpsc, oneshot};
use windows::Win32::System::Variant::VARIANT;

use openstaad::bindings::Staad;
use serde_json::Value;

use crate::openstaad::app::OpenStaad;
use crate::openstaad::custom::*;
use crate::openstaad::execute::execute_method;
use crate::tools::{InType, Input, invoke_method};

// 메시지 타입 정의
#[derive(Debug)]
pub enum ThreadMessage {
    Initialize {
        path: String,
        std_path: String,
        response_tx: oneshot::Sender<Result<Value, String>>,
    },
    Instance {
        id: String,
        property: String,
        response_tx: oneshot::Sender<Result<Value, String>>,
    },
    Method {
        id: String,
        method: String,
        params: Vec<Value>,
        response_tx: oneshot::Sender<Result<Value, String>>,
    },
    CustomMethod {
        id: String,
        method: String,
        params: Vec<Value>,
        response_tx: oneshot::Sender<Result<Value, String>>,
    },
    Shutdown,
}

// 스레드 매니저 구조체
pub struct OpenStaadThreadManager {
    sender: mpsc::UnboundedSender<ThreadMessage>,
    thread_handle: Option<JoinHandle<()>>,
}

static THREAD_MANAGER: LazyLock<Mutex<Option<OpenStaadThreadManager>>> =
    LazyLock::new(|| Mutex::new(None));

// 백그라운드 스레드 시작
pub fn start(handle: AppHandle) -> Result<(), String> {
    let mut manager = THREAD_MANAGER.lock().map_err(|e| e.to_string())?;

    if manager.is_some() {
        return Err("Thread already started".to_string());
    }

    let (tx, mut rx) = mpsc::unbounded_channel::<ThreadMessage>();

    let thread_handle = thread::spawn(move || {
        // 스레드별 API_STORE
        let mut local_store: HashMap<String, Staad> = HashMap::new();

        while let Some(message) = rx.blocking_recv() {
            match message {
                ThreadMessage::Initialize {
                    path,
                    std_path,
                    response_tx,
                } => {
                    let result = handle_initialize(&mut local_store, path, std_path);
                    let _ = response_tx.send(result);
                }
                ThreadMessage::Instance {
                    id,
                    property,
                    response_tx,
                } => {
                    let result = handle_instance(&mut local_store, id, property);
                    let _ = response_tx.send(result);
                }
                ThreadMessage::Method {
                    id,
                    method,
                    params,
                    response_tx,
                } => {
                    let result = handle_method(&local_store, id, method, params);
                    let _ = response_tx.send(result);
                }
                ThreadMessage::CustomMethod {
                    id,
                    method,
                    params,
                    response_tx,
                } => {
                    let result =
                        handle_custom_method(&mut local_store, id, method, params, handle.clone());
                    let _ = response_tx.send(result);
                }
                ThreadMessage::Shutdown => {
                    break;
                }
            }
        }
    });

    *manager = Some(OpenStaadThreadManager {
        sender: tx,
        thread_handle: Some(thread_handle),
    });

    Ok(())
}

// 스레드로 메시지 전송하는 함수
pub async fn send(msg: String, data: Value) -> Result<Value, String> {
    let sender = {
        let manager = THREAD_MANAGER.lock().map_err(|e| e.to_string())?;
        match &*manager {
            Some(m) => m.sender.clone(),
            None => return Err("Thread not started. Call start_thread() first.".to_string()),
        }
    }; // MutexGuard가 여기서 드롭됨

    let (response_tx, response_rx) = oneshot::channel();

    let message = match msg.as_str() {
        "Initialize" => {
            let path = data["path"].as_str().ok_or("Missing path")?;
            let std_path = data["std_path"].as_str().ok_or("Missing std_path")?;
            ThreadMessage::Initialize {
                path: path.to_string(),
                std_path: std_path.to_string(),
                response_tx,
            }
        }
        "Instance" => {
            let id = data["id"].as_str().ok_or("Missing id")?.to_string();
            let property = data["property"]
                .as_str()
                .ok_or("Missing property")?
                .to_string();
            ThreadMessage::Instance {
                id,
                property,
                response_tx,
            }
        }
        "Method" => {
            let id = data["id"].as_str().ok_or("Missing id")?.to_string();
            let method = data["method"].as_str().ok_or("Missing method")?.to_string();
            let params = data["params"].as_array().ok_or("Missing params")?.clone();
            ThreadMessage::Method {
                id,
                method,
                params,
                response_tx,
            }
        }
        "CustomMethod" => {
            let id = data["id"].as_str().ok_or("Missing id")?.to_string();
            let method = data["method"].as_str().ok_or("Missing method")?.to_string();
            let params = data["params"].as_array().ok_or("Missing params")?.clone();
            ThreadMessage::CustomMethod {
                id,
                method,
                params,
                response_tx,
            }
        }
        _ => return Err("Unknown message type".to_string()),
    };

    sender.send(message).map_err(|e| e.to_string())?;

    match response_rx.await {
        Ok(result) => match result {
            Ok(value) => {
                match msg.as_str() {
                    "Initialize" | "Instance" => {
                        // String을 JSON Value로 변환
                        serde_json::to_value(value).map_err(|e| e.to_string())
                    }
                    "Method" | "CustomMethod" => Ok(value),
                    _ => Err("Unexpected result type".to_string()),
                }
            }
            Err(e) => Err(e),
        },
        Err(_) => Err("Thread communication failed".to_string()),
    }
}

// 스레드 종료
pub fn shutdown() -> Result<(), String> {
    let mut manager = THREAD_MANAGER.lock().map_err(|e| e.to_string())?;

    if let Some(m) = manager.take() {
        let _ = m.sender.send(ThreadMessage::Shutdown);
        if let Some(handle) = m.thread_handle {
            let _ = handle.join();
        }
    }

    Ok(())
}

// 백그라운드 스레드에서 사용할 핸들러 함수들
fn handle_initialize(
    store: &mut HashMap<String, Staad>,
    path: String,
    std_path: String,
) -> Result<Value, String> {
    // let app_result = OpenStaad::new(path, std_path);
    let app_result = OpenStaad::new_by_activated();
    match app_result {
        Ok(instance) => {
            let _ = unsafe {
                invoke_method(&instance.dispatch, "SetSilentMode", &mut [VARIANT::from(1)])
            };
            let store_id = instance.id.to_string();
            if !store.contains_key(&store_id) {
                store.insert(store_id.clone(), Staad::OpenStaad(instance));
            }
            serde_json::to_value(&store_id).map_err(|e| e.to_string())
        }
        Err(e) => Err(format!("Failed to initialize: {}", e)),
    }
}

fn handle_instance(
    store: &mut HashMap<String, Staad>,
    id: String,
    instance_type: String,
) -> Result<Value, String> {
    match instance_type.as_str() {
        "command" => {
            let openstaad = match store.get_mut(&id) {
                Some(Staad::OpenStaad(p)) => p,
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad.get_command();
            match _result {
                Ok(instance) => {
                    let store_id = openstaad.command.as_ref().unwrap().id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Command(instance));
                    }
                    serde_json::to_value(&store_id).map_err(|e| e.to_string())
                }
                Err(e) => Err(format!("Fail to get command: {}", e)),
            }
        }
        "design" => {
            let openstaad = match store.get_mut(&id) {
                Some(Staad::OpenStaad(p)) => p,
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad.get_design();
            match _result {
                Ok(instance) => {
                    let store_id = openstaad.design.as_ref().unwrap().id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Design(instance));
                    }
                    serde_json::to_value(&store_id).map_err(|e| e.to_string())
                }
                Err(e) => Err(format!("Fail to get design: {}", e)),
            }
        }
        "geometry" => {
            let openstaad = match store.get_mut(&id) {
                Some(Staad::OpenStaad(p)) => p,
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad.get_geometry();
            match _result {
                Ok(instance) => {
                    let store_id = openstaad.geometry.as_ref().unwrap().id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Geometry(instance));
                    }
                    serde_json::to_value(&store_id).map_err(|e| e.to_string())
                }
                Err(e) => Err(format!("Fail to get geometry: {}", e)),
            }
        }
        "load" => {
            let openstaad = match store.get_mut(&id) {
                Some(Staad::OpenStaad(p)) => p,
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad.get_load();
            match _result {
                Ok(instance) => {
                    let store_id = openstaad.load.as_ref().unwrap().id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Load(instance));
                    }
                    serde_json::to_value(&store_id).map_err(|e| e.to_string())
                }
                Err(e) => Err(format!("Fail to get load: {}", e)),
            }
        }
        "output" => {
            let openstaad = match store.get_mut(&id) {
                Some(Staad::OpenStaad(p)) => p,
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad.get_output();
            match _result {
                Ok(instance) => {
                    let store_id = openstaad.output.as_ref().unwrap().id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Output(instance));
                    }
                    serde_json::to_value(&store_id).map_err(|e| e.to_string())
                }
                Err(e) => Err(format!("Fail to get output: {}", e)),
            }
        }
        "property" => {
            let openstaad = match store.get_mut(&id) {
                Some(Staad::OpenStaad(p)) => p,
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad.get_property();
            match _result {
                Ok(instance) => {
                    let store_id = openstaad.property.as_ref().unwrap().id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Property(instance));
                    }
                    serde_json::to_value(&store_id).map_err(|e| e.to_string())
                }
                Err(e) => Err(format!("Fail to get property: {}", e)),
            }
        }
        "support" => {
            let openstaad = match store.get_mut(&id) {
                Some(Staad::OpenStaad(p)) => p,
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad.get_support();
            match _result {
                Ok(instance) => {
                    let store_id = openstaad.support.as_ref().unwrap().id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Support(instance));
                    }
                    serde_json::to_value(&store_id).map_err(|e| e.to_string())
                }
                Err(e) => Err(format!("Fail to get support: {}", e)),
            }
        }
        _ => Err("Unsupported instance type".to_string()),
    }
}

fn handle_method(
    store: &HashMap<String, Staad>,
    id: String,
    method: String,
    params: Vec<Value>,
) -> Result<Value, String> {
    let arc = store.get(&id);
    if let Some(instance) = arc {
        let converted_params =
            convert_to_inputs(instance, method.as_str(), &params).map_err(|e| e.to_string())?;
        return execute_method(instance, method.as_str(), converted_params.as_slice())
            .map_err(|e| e.to_string());
    } else {
        return Err("Instance not found".to_string());
    }
}

fn handle_custom_method(
    store: &mut HashMap<String, Staad>,
    id: String,
    method: String,
    params: Vec<Value>,
    handle: AppHandle,
) -> Result<Value, String> {
    let arc = store.get_mut(&id);
    match arc {
        Some(instance) => match method.as_str() {
            "get_node_table" => {
                let v = get_node_table(instance).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            "get_beam_table" => {
                let v = get_beam_table(instance).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            "get_section_list" => {
                let v = get_section_list(instance).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            "get_section_property_tables" => {
                let v = get_section_property_tables(instance).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            "get_beta_list" => {
                let v = get_beta_list(instance).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            "get_isotropic_material_list" => {
                let v = get_isotropic_material_list(instance).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            "get_orthotropic2d_material_list" => {
                let v = get_orthotropic2d_material_list(instance).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            "get_specification_list" => {
                let v = get_specification_list(instance).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            "get_support_list" => {
                let v = get_support_list(instance).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            "get_reference_load_list" => {
                let v = get_reference_load_list(instance).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            "get_load_case_list" => {
                let v = get_load_case_list(instance).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            "get_load_item_list" => {
                let v =
                    get_load_item_list(instance, params[0].clone()).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            "analyze" => {
                let v = analyze(instance, handle).map_err(|e| e.to_string())?;
                serde_json::to_value(v).map_err(|e| e.to_string())
            }
            _ => return Err(format!("Invalid custom method name: {}", method)),
        },
        _ => Err(format!("OpenStaad instance is not initialized: {}", id)),
    }
}

fn convert_to_inputs(instance: &Staad, method: &str, params: &Vec<Value>) -> Result<Vec<Input>> {
    let methods = match instance {
        Staad::OpenStaad(v) => &v.methods,
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
