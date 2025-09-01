pub mod openstaad;
pub mod tools;

pub use anyhow::{Context, Error, Result, anyhow, bail};
pub use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

use openstaad::bindings::Staad;
use serde_json::Value;
use tools::value_types::{InType, Input};

use crate::openstaad::app::OpenStaad;
use crate::openstaad::execute::execute_method;

static API_STORE: LazyLock<Mutex<HashMap<String, Staad>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn start_thread() {}

pub fn openstaad_rust(id: String, method: String, params: Vec<Value>) -> Result<Value, String> {
    match method.as_str() {
        "initialize" => {
            let mut store = API_STORE.lock().map_err(|e| e.to_string())?;
            let system_path = params[0].to_string();
            let std_path = params[1].to_string();
            let app_result = OpenStaad::new(system_path, std_path);
            match app_result {
                Ok(instance) => {
                    let store_id = instance.id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::OpenStaad(instance));
                    }
                    return serde_json::to_value(&store_id).map_err(|e| e.to_string());
                }
                Err(e) => {
                    return Err(format!("Fail to '{}' method: {}", method, e));
                }
            }
        }
        "command" => {
            let mut store = API_STORE.lock().map_err(|e| e.to_string())?;
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
                    return serde_json::to_value(&store_id).map_err(|e| e.to_string());
                }
                Err(e) => {
                    return Err(format!("Fail to '{}' method: {}", method, e));
                }
            }
        }
        "design" => {
            let mut store = API_STORE.lock().map_err(|e| e.to_string())?;
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
                    return serde_json::to_value(&store_id).map_err(|e| e.to_string());
                }
                Err(e) => {
                    return Err(format!("Fail to '{}' method: {}", method, e));
                }
            }
        }
        "geometry" => {
            let mut store = API_STORE.lock().map_err(|e| e.to_string())?;
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
                    return serde_json::to_value(&store_id).map_err(|e| e.to_string());
                }
                Err(e) => {
                    return Err(format!("Fail to '{}' method: {}", method, e));
                }
            }
        }
        "load" => {
            let mut store = API_STORE.lock().map_err(|e| e.to_string())?;
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
                    return serde_json::to_value(&store_id).map_err(|e| e.to_string());
                }
                Err(e) => {
                    return Err(format!("Fail to '{}' method: {}", method, e));
                }
            }
        }
        "output" => {
            let mut store = API_STORE.lock().map_err(|e| e.to_string())?;
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
                    return serde_json::to_value(&store_id).map_err(|e| e.to_string());
                }
                Err(e) => {
                    return Err(format!("Fail to '{}' method: {}", method, e));
                }
            }
        }
        "property" => {
            let mut store = API_STORE.lock().map_err(|e| e.to_string())?;
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
                    return serde_json::to_value(&store_id).map_err(|e| e.to_string());
                }
                Err(e) => {
                    return Err(format!("Fail to '{}' method: {}", method, e));
                }
            }
        }
        "support" => {
            let mut store = API_STORE.lock().map_err(|e| e.to_string())?;
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
                    return serde_json::to_value(&store_id).map_err(|e| e.to_string());
                }
                Err(e) => {
                    return Err(format!("Fail to '{}' method: {}", method, e));
                }
            }
        }
        _ => return Err("Unsupported method".to_string()),
    };
}

pub fn invoke(id: String, method: String, params: Vec<Value>) -> Result<Value, String> {
    let store = API_STORE.lock().map_err(|e| e.to_string())?;
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

    let converted_inputs: Result<Vec<Input>, Error> = InType::filter_general_type(_inputs)
        .iter()
        .enumerate()
        .map(|(i, _type)| _type.to_input_as(&params[i]))
        .collect();

    converted_inputs
}

// use openstaad_rust as openstaad_api;
