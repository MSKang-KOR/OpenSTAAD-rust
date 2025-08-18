use crate::openstaad::{
    api::process::StaadProcess,
    tauri::{store::PROCESS_STORE, utils::StaadObject},
};
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub fn process_call(id: String, method: String, params: Vec<Value>) -> Result<Value, String> {
    match method.as_str() {
        "start" => {
            let path = &params[0].to_string();
            let mut process = StaadProcess::new(path);
            if let Ok(_) = process.start() {
                let mut store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
                let _pid = process.pid.unwrap();
                let store_id = _pid.to_string();
                if store.contains_key(&store_id) {
                    if !process.is_alive() {
                        store.remove(&store_id);
                        store.insert(
                            store_id.clone(),
                            StaadObject::Process(Arc::new(Mutex::new(process))),
                        );
                    } else {
                        drop(process);
                    }
                    return serde_json::to_value(&store_id).map_err(|e| e.to_string());
                }
                store.insert(
                    store_id.clone(),
                    StaadObject::Process(Arc::new(Mutex::new(process))),
                );
                return serde_json::to_value(&store_id).map_err(|e| e.to_string());
            }
            return Err(format!("Fail to '{}' method", method));
        }
        "start_with_pid" => {
            let path = params[0].to_string();
            let pid = params[1].as_i64().unwrap() as u32;
            let mut process = StaadProcess::new(&path);
            if let Ok(_) = process.start_with_pid(pid) {
                let mut store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
                let _pid = process.pid.unwrap();
                let store_id = _pid.to_string();
                if store.contains_key(&store_id) {
                    if !process.is_alive() {
                        store.remove(&store_id);
                        store.insert(
                            store_id.clone(),
                            StaadObject::Process(Arc::new(Mutex::new(process))),
                        );
                    } else {
                        drop(process);
                    }
                    return serde_json::to_value(&store_id).map_err(|e| e.to_string());
                }
                store.insert(
                    store_id.clone(),
                    StaadObject::Process(Arc::new(Mutex::new(process))),
                );
                return serde_json::to_value(&store_id).map_err(|e| e.to_string());
            }
            return Err(format!("Fail to '{}' method", method));
        }
        "root" => {
            let mut store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
            let process_arc = match store.get(&id) {
                Some(StaadObject::Process(p)) => Arc::clone(p),
                _ => return Err("Process not found".to_string()),
            };
            let root = {
                let mut process = process_arc.lock().map_err(|e| e.to_string())?;
                process.root()
            };
            let root_id = root.id.clone();
            if !store.contains_key(&root_id) {
                store.insert(root_id.clone(), StaadObject::Root(root));
            }
            return serde_json::to_value(&root_id).map_err(|e| e.to_string());
        }
        "geometry" => {
            let mut store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
            let process_arc = match store.get(&id) {
                Some(StaadObject::Process(p)) => Arc::clone(p),
                _ => return Err("Process not found".to_string()),
            };
            let geometry = {
                let mut process = process_arc.lock().map_err(|e| e.to_string())?;
                process.geometry()
            };
            let geometry_id = geometry.id.clone();
            if !store.contains_key(&geometry_id) {
                store.insert(geometry_id.clone(), StaadObject::Geometry(geometry));
            }
            return serde_json::to_value(&geometry_id).map_err(|e| e.to_string());
        }
        "command" => {
            let mut store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
            let process_arc = match store.get(&id) {
                Some(StaadObject::Process(p)) => Arc::clone(p),
                _ => return Err("Process not found".to_string()),
            };
            let command = {
                let mut process = process_arc.lock().map_err(|e| e.to_string())?;
                process.command()
            };
            let command_id = command.id.clone();
            if !store.contains_key(&command_id) {
                store.insert(command_id.clone(), StaadObject::Command(command));
            }
            return serde_json::to_value(&command_id).map_err(|e| e.to_string());
        }
        "design" => {
            let mut store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
            let process_arc = match store.get(&id) {
                Some(StaadObject::Process(p)) => Arc::clone(p),
                _ => return Err("Process not found".to_string()),
            };
            let design = {
                let mut process = process_arc.lock().map_err(|e| e.to_string())?;
                process.design()
            };
            let design_id = design.id.clone();
            if !store.contains_key(&design_id) {
                store.insert(design_id.clone(), StaadObject::Design(design));
            }
            return serde_json::to_value(&design_id).map_err(|e| e.to_string());
        }
        "load" => {
            let mut store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
            let process_arc = match store.get(&id) {
                Some(StaadObject::Process(p)) => Arc::clone(p),
                _ => return Err("Process not found".to_string()),
            };
            let load = {
                let mut process = process_arc.lock().map_err(|e| e.to_string())?;
                process.load()
            };
            let load_id = load.id.clone();
            if !store.contains_key(&load_id) {
                store.insert(load_id.clone(), StaadObject::Load(load));
            }
            return serde_json::to_value(&load_id).map_err(|e| e.to_string());
        }
        "output" => {
            let mut store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
            let process_arc = match store.get(&id) {
                Some(StaadObject::Process(p)) => Arc::clone(p),
                _ => return Err("Process not found".to_string()),
            };
            let output = {
                let mut process = process_arc.lock().map_err(|e| e.to_string())?;
                process.output()
            };
            let output_id = output.id.clone();
            if !store.contains_key(&output_id) {
                store.insert(output_id.clone(), StaadObject::Output(output));
            }
            return serde_json::to_value(&output_id).map_err(|e| e.to_string());
        }
        "property" => {
            let mut store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
            let process_arc = match store.get(&id) {
                Some(StaadObject::Process(p)) => Arc::clone(p),
                _ => return Err("Process not found".to_string()),
            };
            let property = {
                let mut process = process_arc.lock().map_err(|e| e.to_string())?;
                process.property()
            };
            let property_id = property.id.clone();
            if !store.contains_key(&property_id) {
                store.insert(property_id.clone(), StaadObject::Property(property));
            }
            return serde_json::to_value(&property_id).map_err(|e| e.to_string());
        }
        "support" => {
            let mut store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
            let process_arc = match store.get(&id) {
                Some(StaadObject::Process(p)) => Arc::clone(p),
                _ => return Err("Process not found".to_string()),
            };
            let support = {
                let mut process = process_arc.lock().map_err(|e| e.to_string())?;
                process.support()
            };
            let support_id = support.id.clone();
            if !store.contains_key(&support_id) {
                store.insert(support_id.clone(), StaadObject::Support(support));
            }
            return serde_json::to_value(&support_id).map_err(|e| e.to_string());
        }
        _ => return Err("Unknown method".to_string()),
    };
}
