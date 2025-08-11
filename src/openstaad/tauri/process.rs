use serde_json::Value;
use tokio::runtime::Runtime;

use crate::openstaad::{
    api::{
        command::Command, design::Design, geometry::Geometry, load::Load, output::Output,
        property::Property, root::Root, support::Support,
    },
    process::StaadProcess,
    tauri::{store::PROCESS_STORE, utils::StaadObject},
};

pub fn staad_process_start(path: String) -> Result<Value, String> {
    let mut process = StaadProcess::new(&path);
    let _id = uuid::Uuid::new_v4().to_string();
    let rt = Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(process.start()).map_err(|e| e.to_string())?;

    let _staad = &process.staad;
    match _staad {
        Some(_v) => {
            let mut store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
            store.insert(_id.clone(), StaadObject::Process(process));
            // Ok(_id)
            serde_json::to_value(&_id).map_err(|e| e.to_string())
        }
        _ => Err("Fail to start STAAD process".to_string()),
    }
}

pub fn staad_process_call(id: String, method: String) -> Result<Value, String> {
    let mut store = PROCESS_STORE.lock().map_err(|e| e.to_string())?;
    let process = match store.remove(&id) {
        Some(StaadObject::Process(p)) => p,
        _ => return Err("Process not found".to_string()),
    };

    let new_id = uuid::Uuid::new_v4().to_string();

    // process를 Box로 래핑하여 힙에 할당
    let process_box = Box::new(process);
    let process_ptr: *mut StaadProcess = Box::into_raw(process_box);
    let process_ref = unsafe { &*process_ptr };

    let result = match method.as_str() {
        "root" => {
            let root = process_ref.root();
            let static_root: Root<'static> = unsafe { std::mem::transmute(root) };
            store.insert(new_id.clone(), StaadObject::Root(static_root));
            Ok(())
        }
        "geometry" => {
            let geometry = process_ref.geometry();
            let static_geometry: Geometry<'static> = unsafe { std::mem::transmute(geometry) };
            store.insert(new_id.clone(), StaadObject::Geometry(static_geometry));
            Ok(())
        }
        "command" => {
            let command = process_ref.command();
            let static_command: Command<'static> = unsafe { std::mem::transmute(command) };
            store.insert(new_id.clone(), StaadObject::Command(static_command));
            Ok(())
        }
        "design" => {
            let design = process_ref.design();
            let static_design: Design<'static> = unsafe { std::mem::transmute(design) };
            store.insert(new_id.clone(), StaadObject::Design(static_design));
            Ok(())
        }
        "load" => {
            let load = process_ref.load();
            let static_load: Load<'static> = unsafe { std::mem::transmute(load) };
            store.insert(new_id.clone(), StaadObject::Load(static_load));
            Ok(())
        }
        "output" => {
            let output = process_ref.output();
            let static_output: Output<'static> = unsafe { std::mem::transmute(output) };
            store.insert(new_id.clone(), StaadObject::Output(static_output));
            Ok(())
        }
        "property" => {
            let property = process_ref.property();
            let static_property: Property<'static> = unsafe { std::mem::transmute(property) };
            store.insert(new_id.clone(), StaadObject::Property(static_property));
            Ok(())
        }
        "support" => {
            let support = process_ref.support();
            let static_support: Support<'static> = unsafe { std::mem::transmute(support) };
            store.insert(new_id.clone(), StaadObject::Support(static_support));
            Ok(())
        }
        _ => Err("Unknown method".to_string()),
    };

    // process를 다시 복원
    let process_back = unsafe { Box::from_raw(process_ptr) };

    match result {
        Ok(_) => {
            store.insert(id, StaadObject::Process(*process_back));
            serde_json::to_value(&new_id).map_err(|e| e.to_string())
        }
        Err(e) => {
            store.insert(id, StaadObject::Process(*process_back));
            Err(e)
        }
    }
}
