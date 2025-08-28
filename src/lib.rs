pub mod openstaad;
pub mod tools;

pub mod binding;

pub use anyhow::{Context, Result, bail};
use log::{error, info};
pub use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

use openstaad::bindings::Staad;
use serde_json::Value;
use tools::value_types::{InType, Input, OutType};

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
        // "invoke" => {
        //     let mut store = API_STORE.lock().map_err(|e| e.to_string())?;
        //     let arc = store.get(&id);
        //     if let Some(instance) = arc {
        //         execute_method(instance, method, params);
        //     } else {
        //         return Err(format!("Instance with '{}' method not found", method));
        //     }
        // }
        _ => return Err("Unsupported method".to_string()),
    };
}
