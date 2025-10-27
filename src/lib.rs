pub mod bindings;
pub mod openstaad;
pub mod parser;
pub mod tools;

pub use anyhow::{Context, Error, Result, anyhow, bail};
pub use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::PathBuf;
use tauri::AppHandle;
use windows::Win32::System::Variant::VARIANT;

use serde_json::Value;

use crate::bindings::Staad;
use crate::openstaad::app::{self, OpenStaad};
use crate::parser::parsing_std;
use crate::tools::{InType, Input, custom::*, execute_method, invoke_method};

pub fn handle_open(path: PathBuf, file_name: String) -> Result<OpenStaad, String> {
    let mut openstaad = OpenStaad::new(path, file_name).map_err(|e| e.to_string())?;
    let root = openstaad.get_root().map_err(|e| e.to_string())?;

    // Silent mode 설정
    let _ = unsafe {
        invoke_method(&root.dispatch, "SetSilentMode", &mut [VARIANT::from(1)])
            .map_err(|e| e.to_string())
    };
    Ok(openstaad)
}

pub fn handle_connect(pid: u32) -> Result<OpenStaad, String> {
    let mut openstaad = OpenStaad::new_by_pid(pid).map_err(|e| e.to_string())?;
    let root = openstaad.get_root().map_err(|e| e.to_string())?;

    // Silent mode 설정
    let _ = unsafe {
        invoke_method(&root.dispatch, "SetSilentMode", &mut [VARIANT::from(1)])
            .map_err(|e| e.to_string())
    };
    Ok(openstaad)
}

pub fn handle_method(
    staad: &mut OpenStaad,
    method: String,
    params: Vec<Value>,
    handle: AppHandle,
) -> Result<Value, String> {
    let instance = Staad::from_method(staad, method.as_str()).map_err(|e| e.to_string())?;
    let converted_params =
        convert_to_inputs(&instance, method.as_str(), &params).map_err(|e| e.to_string())?;
    let result = execute_method(&instance, method.as_str(), converted_params.as_slice())
        .map_err(|e| e.to_string());
    if let Ok(_) = result {
        match method.as_str() {
            "OpenSTAADFile" => {
                let file_name = params[0]
                    .as_str()
                    .ok_or("OpenSTAADFile: Missing file_name parameter")?
                    .to_string();
                staad.file_name = Some(file_name);
            }
            "CloseSTAADFile" => {
                staad.file_name = None;
                staad.command = None;
                staad.design = None;
                staad.geometry = None;
                staad.load = None;
                staad.output = None;
                staad.property = None;
                staad.support = None;
            }
            _ => {}
        }
    }
    return result;
}

pub fn handle_custom_method(
    openstaad: &mut OpenStaad,
    method: String,
    params: Vec<Value>,
    handle: AppHandle,
) -> Result<Value, String> {
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
        "open_staad_file" => {
            let v = open_staad_file(openstaad, params[0].clone()).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "close_staad_file" => {
            let v = close_staad_file(openstaad).map_err(|e| e.to_string())?;
            serde_json::to_value(v).map_err(|e| e.to_string())
        }
        "parsing_std" => {
            let std_path = params[0]
                .as_str()
                .ok_or("Missing std_path parameter")?
                .to_string();
            let content = read_to_string(std_path).map_err(|e| e.to_string())?;
            parsing_std(content).map_err(|e| e.to_string())
            // serde_json::to_value(v).map_err(|e| e.to_string())
        }
        // "parsing_loadings" => {
        //     let std_path = params[0]
        //         .as_str()
        //         .ok_or("Missing std_path parameter")?
        //         .to_string();
        //     let content = read_to_string(std_path).map_err(|e| e.to_string())?;
        //     let v = parsing_loadings(content).map_err(|e| e.to_string())?;
        //     serde_json::to_value(v).map_err(|e| e.to_string())
        // }
        // "parsing_specifications" => {
        //     let std_path = params[0]
        //         .as_str()
        //         .ok_or("Missing std_path parameter")?
        //         .to_string();
        //     let content = read_to_string(std_path).map_err(|e| e.to_string())?;
        //     let v = parsing_specifications(content).map_err(|e| e.to_string())?;
        //     serde_json::to_value(v).map_err(|e| e.to_string())
        // }
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
