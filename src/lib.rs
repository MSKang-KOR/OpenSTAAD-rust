pub mod openstaad;
pub mod tools;

pub mod binding;

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

pub fn openstaad_rust(id: String, method: String, params: Vec<Value>) -> Result<Value, String> {
    match method.as_str() {
        "initialize" => {
            let mut store = API_STORE.lock().map_err(|e| e.to_string())?;
            let path = params[0].to_string();
            let app_result = OpenStaad::new(Some(path));
            match app_result {
                Ok(app) => {
                    let store_id = app.id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::OpenStaad(Arc::new(app)));
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
            let openstaad_arc = match store.get(&id) {
                Some(Staad::OpenStaad(p)) => Arc::clone(p),
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad_arc.get_command();
            match _result {
                Ok(app) => {
                    let store_id = app.id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Command(Arc::new(app)));
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
            let openstaad_arc = match store.get(&id) {
                Some(Staad::OpenStaad(p)) => Arc::clone(p),
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad_arc.get_design();
            match _result {
                Ok(app) => {
                    let store_id = app.id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Design(Arc::new(app)));
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
            let openstaad_arc = match store.get(&id) {
                Some(Staad::OpenStaad(p)) => Arc::clone(p),
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad_arc.get_geometry();
            match _result {
                Ok(app) => {
                    let store_id = app.id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Geometry(Arc::new(app)));
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
            let openstaad_arc = match store.get(&id) {
                Some(Staad::OpenStaad(p)) => Arc::clone(p),
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad_arc.get_load();
            match _result {
                Ok(app) => {
                    let store_id = app.id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Load(Arc::new(app)));
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
            let openstaad_arc = match store.get(&id) {
                Some(Staad::OpenStaad(p)) => Arc::clone(p),
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad_arc.get_output();
            match _result {
                Ok(app) => {
                    let store_id = app.id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Output(Arc::new(app)));
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
            let openstaad_arc = match store.get(&id) {
                Some(Staad::OpenStaad(p)) => Arc::clone(p),
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad_arc.get_property();
            match _result {
                Ok(app) => {
                    let store_id = app.id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Property(Arc::new(app)));
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
            let openstaad_arc = match store.get(&id) {
                Some(Staad::OpenStaad(p)) => Arc::clone(p),
                _ => return Err("OpenStaad not found".to_string()),
            };
            let _result = openstaad_arc.get_support();
            match _result {
                Ok(app) => {
                    let store_id = app.id.to_string();
                    if !store.contains_key(&store_id) {
                        store.insert(store_id.clone(), Staad::Support(Arc::new(app)));
                    }
                    return serde_json::to_value(&store_id).map_err(|e| e.to_string());
                }
                Err(e) => {
                    return Err(format!("Fail to '{}' method: {}", method, e));
                }
            }
        }
        "invoke" => {
            let mut store = API_STORE.lock().map_err(|e| e.to_string())?;
            let arc = store.get(&id);
            if let Some(instance) = arc {
                let converted_params = convert_to_inputs(instance, method.as_str(), &params)
                    .map_err(|e| e.to_string())?;
                return execute_method(instance, method.as_str(), converted_params.as_slice())
                    .map_err(|e| e.to_string());
            } else {
                return Err(format!("Instance with '{}' method not found", method));
            }
        }
        _ => return Err("Unsupported method".to_string()),
    };
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
        None => bail!(format!("[Convert] Unsupported method: {}", method)),
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
