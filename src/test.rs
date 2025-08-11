mod openstaad;

use openstaad::process::StaadProcess;
use windows::Win32::System::Com::CoUninitialize;
use std::sync::Mutex;
use tauri::State;
use serde::Serialize;

use crate::openstaad::api::{geometry::Geometry, root::Root};

// Tauri application state
struct AppState {
    staad_process: Mutex<Option<StaadProcess>>,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(msg: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(msg),
        }
    }
}

// Tauri commands for StaadProcess.root() methods
#[tauri::command]
fn get_base_unit(state: State<AppState>) -> ApiResponse<i32> {
    let process_guard = state.staad_process.lock().unwrap();
    if let Some(process) = process_guard.as_ref() {
        let root = process.root();
        match root.get_base_unit() {
            Ok(unit) => ApiResponse::success(unit),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    } else {
        ApiResponse::error("StaadProcess not initialized".to_string())
    }
}

#[tauri::command]
fn get_application_version(state: State<AppState>) -> ApiResponse<(String, i32, i32, i32, i32)> {
    let process_guard = state.staad_process.lock().unwrap();
    if let Some(process) = process_guard.as_ref() {
        let root = process.root();
        match root.get_application_version() {
            Ok(version) => ApiResponse::success(version),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    } else {
        ApiResponse::error("StaadProcess not initialized".to_string())
    }
}

#[tauri::command]
fn get_staad_file(state: State<AppState>, full_path: bool) -> ApiResponse<String> {
    let process_guard = state.staad_process.lock().unwrap();
    if let Some(process) = process_guard.as_ref() {
        let root = process.root();
        match root.get_staad_file(full_path) {
            Ok(file) => ApiResponse::success(file),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    } else {
        ApiResponse::error("StaadProcess not initialized".to_string())
    }
}

// Tauri commands for StaadProcess.geometry() methods
#[tauri::command]
fn get_node_list(state: State<AppState>) -> ApiResponse<Vec<i32>> {
    let process_guard = state.staad_process.lock().unwrap();
    if let Some(process) = process_guard.as_ref() {
        let geometry = process.geometry();
        match geometry.get_node_list() {
            Ok(nodes) => ApiResponse::success(nodes),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    } else {
        ApiResponse::error("StaadProcess not initialized".to_string())
    }
}

#[tauri::command]
fn get_node_count(state: State<AppState>) -> ApiResponse<i32> {
    let process_guard = state.staad_process.lock().unwrap();
    if let Some(process) = process_guard.as_ref() {
        let geometry = process.geometry();
        match geometry.get_node_count() {
            Ok(count) => ApiResponse::success(count),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    } else {
        ApiResponse::error("StaadProcess not initialized".to_string())
    }
}

#[tauri::command]
fn get_node_coordinates(state: State<AppState>, node_no: i32, base_unit: i32) -> ApiResponse<Vec<f64>> {
    let process_guard = state.staad_process.lock().unwrap();
    if let Some(process) = process_guard.as_ref() {
        let geometry = process.geometry();
        match geometry.get_node_coordinates(node_no, base_unit) {
            Ok(coords) => ApiResponse::success(coords),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    } else {
        ApiResponse::error("StaadProcess not initialized".to_string())
    }
}

#[tauri::command]
fn get_member_count(state: State<AppState>) -> ApiResponse<i32> {
    let process_guard = state.staad_process.lock().unwrap();
    if let Some(process) = process_guard.as_ref() {
        let geometry = process.geometry();
        match geometry.get_member_count() {
            Ok(count) => ApiResponse::success(count),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    } else {
        ApiResponse::error("StaadProcess not initialized".to_string())
    }
}

#[tauri::command]
fn create_node(state: State<AppState>, node_no: i32, x: f64, y: f64, z: f64) -> ApiResponse<()> {
    let process_guard = state.staad_process.lock().unwrap();
    if let Some(process) = process_guard.as_ref() {
        let geometry = process.geometry();
        match geometry.create_node(node_no, x, y, z) {
            Ok(()) => ApiResponse::success(()),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    } else {
        ApiResponse::error("StaadProcess not initialized".to_string())
    }
}

// Initialize StaadProcess command
#[tauri::command]
fn initialize_staad_process(state: State<AppState>, staad_path: String) -> ApiResponse<()> {
    let mut process_guard = state.staad_process.lock().unwrap();
    *process_guard = Some(StaadProcess::new(&staad_path));
    ApiResponse::success(())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            staad_process: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            initialize_staad_process,
            get_base_unit,
            get_application_version,
            get_staad_file,
            get_node_list,
            get_node_count,
            get_node_coordinates,
            get_member_count,
            create_node
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn test_root_methods(root: &Root) {
//     let set_silent_mode_result = root.set_silent_mode(1);
//     match set_silent_mode_result {
//         Ok(v) => println!("set_silent_mode_result success: {:#?}", v),
//         Err(e) => println!("set_silent_mode_result error: {:#?}", e),
//     }

//     let analyze_ex_result = root.analyze_ex(0, 0, 0);
//     match analyze_ex_result {
//         Ok(v) => {
//             println!("analyze_ex_result success: {:#?}", v);
//             let get_staad_file_result = root.get_staad_file(true);
//             match get_staad_file_result {
//                 Ok(v) => {
//                     println!("get_staad_file_result success: {:#?}", v);
//                     let std_path = Path::new(&v);
//                     let log_path = std_path.with_extension("log");
//                     let _watcher_handle = watch_file_background(&log_path);
//                     _watcher_handle.join().unwrap();
//                 }
//                 Err(e) => println!("get_staad_file_result error: {:#?}", e),
//             }
//         }
//         Err(e) => println!("analyze_ex_result error: {:#?}", e),
//     }
// }

// fn test_geometry_node_methods(geometry: &Geometry, base_unit: i32) {
//     let coordinates = vec![
//         vec![100., 100., 100.],
//         vec![200., 200., 200.],
//         vec![300., 300., 300.],
//     ];
//     let add_multiple_result = geometry.add_multiple_nodes(coordinates, base_unit);
//     match add_multiple_result {
//         Ok(v) => println!("add_multiple_result success: {:#?}", v),
//         Err(e) => println!("add_multiple_result error: {:#?}", e),
//     }

//     let add_node_result = geometry.add_node(10601., 113.55, -9481.441, base_unit);
//     match add_node_result {
//         Ok(v) => println!("add_node_result success: {:#?}", v),
//         Err(e) => println!("add_node_result error: {:#?}", e),
//     }

//     let node_ids = vec![2004, 2005, 2006];
//     let coordinates = vec![
//         vec![400., 400., 400.],
//         vec![500., 500., 500.],
//         vec![600., 600., 600.],
//     ];
//     let create_multiple_result = geometry.create_multiple_nodes(node_ids, coordinates);
//     match create_multiple_result {
//         Ok(v) => println!("create_multiple_result success: {:#?}", v),
//         Err(e) => println!("create_multiple_result error: {:#?}", e),
//     }

//     let create_node_result = geometry.create_node(2007, 10600., 113.55, -9481.441);
//     match create_node_result {
//         Ok(v) => println!("create_node_result success: {:#?}", v),
//         Err(e) => println!("create_node_result error: {:#?}", e),
//     }

//     let delete_node_result = geometry.delete_node(2007);
//     match delete_node_result {
//         Ok(v) => println!("delete_node_result success: {:#?}", v),
//         Err(e) => println!("delete_node_result error: {:#?}", e),
//     }

//     let get_last_node_no_result = geometry.get_last_node_no();
//     match get_last_node_no_result {
//         Ok(v) => println!("get_last_node_no_result success: {:#?}", v),
//         Err(e) => println!("get_last_node_no_result error: {:#?}", e),
//     }

//     let get_node_coordinates_result = geometry.get_node_coordinates(1789, base_unit);
//     match get_node_coordinates_result {
//         Ok(v) => println!("get_node_coordinates_result success: {:#?}", v),
//         Err(e) => println!("get_node_coordinates_result error: {:#?}", e),
//     }

//     let get_node_count_result = geometry.get_node_count();
//     match get_node_count_result {
//         Ok(v) => println!("get_node_count_result success: {:#?}", v),
//         Err(e) => println!("get_node_count_result error: {:#?}", e),
//     }

//     let get_node_distance_result = geometry.get_node_distance(1, 2, base_unit);
//     match get_node_distance_result {
//         Ok(v) => println!("get_node_distance_result success: {:#?}", v),
//         Err(e) => println!("get_node_distance_result error: {:#?}", e),
//     }

//     let get_node_incidence_result = geometry.get_node_incidence(1789, base_unit);
//     match get_node_incidence_result {
//         Ok(v) => println!("get_node_incidence_result success: {:#?}", v),
//         Err(e) => println!("get_node_incidence_result error: {:#?}", e),
//     }

//     let get_node_incidence_cis2_result = geometry.get_node_incidence_cis2(1789, base_unit);
//     match get_node_incidence_cis2_result {
//         Ok(v) => println!("get_node_incidence_cis2_result success: {:#?}", v),
//         Err(e) => println!("get_node_incidence_cis2_result error: {:#?}", e),
//     }

//     // let get_node_list_result = geometry.get_node_list();
//     // match get_node_list_result {
//     //     Ok(v) => println!("get_node_list_result success: {:#?}", v),
//     //     Err(e) => println!("get_node_list_result error: {:#?}", e),
//     // }

//     let get_node_number_result =
//         geometry.get_node_number(10578.4, 115.5, -9478.459444444, base_unit);
//     match get_node_number_result {
//         Ok(v) => println!("get_node_number_result success: {:#?}", v),
//         Err(e) => println!("get_node_number_result error: {:#?}", e),
//     }

//     let get_unique_id_result = geometry.get_node_unique_id(1789);
//     match get_unique_id_result {
//         Ok(v) => println!("get_unique_id_result success: {:#?}", v),
//         Err(e) => println!("get_unique_id_result error: {:#?}", e),
//     }

//     let is_orphan_node_result = geometry.is_orphan_node(1789);
//     match is_orphan_node_result {
//         Ok(v) => println!("is_orphan_node_result success: {:#?}", v),
//         Err(e) => println!("is_orphan_node_result error: {:#?}", e),
//     }

//     let set_node_coordinate_result =
//         geometry.set_node_coordinate(1789, 10600., 115.5, -9478.4410, base_unit);
//     match set_node_coordinate_result {
//         Ok(v) => println!("set_node_coordinate_result success: {:#?}", v),
//         Err(e) => println!("set_node_coordinate_result error: {:#?}", e),
//     }

//     let set_node_unique_id_result = geometry.set_node_unique_id(1789, "NODE_ID_1789");
//     match set_node_unique_id_result {
//         Ok(v) => println!("set_node_unique_id_result success: {:#?}", v),
//         Err(e) => println!("set_node_unique_id_result error: {:#?}", e),
//     }
// }

// fn test_output_methods(output: &Output) {
//     let are_results_available_result = output.are_results_available();
//     match are_results_available_result {
//         Ok(v) => println!("are_results_available_result success: {:#?}", v),
//         Err(e) => println!("are_results_available_result error: {:#?}", e),
//     }
//     let get_member_steel_design_ratio_result = output.get_member_steel_design_ratio(1786);
//     match get_member_steel_design_ratio_result {
//         Ok(v) => println!("get_member_steel_design_ratio_result success: {:#?}", v),
//         Err(e) => println!("get_member_steel_design_ratio_result error: {:#?}", e),
//     }
// }

// fn test_design_methods(design: &Design) {
//     let brief_code = design.get_design_brief_code(1).unwrap();
//     println!("{:#?}", brief_code);
//     let get_member_design_parameters_result = design.get_member_design_parameters(1, 1);
//     match get_member_design_parameters_result {
//         Ok(v) => println!("get_member_design_parameters_result success: {:#?}", v),
//         Err(e) => println!("get_member_design_parameters_result error: {:#?}", e),
//     }
// }

// fn test_section_methods(property: &Property) {
//     let _section = property.section();

//     let get_beta_angle_result = _section.get_beta_angle(1272);
//     match get_beta_angle_result {
//         Ok(v) => println!("get_beta_angle_result success: {:#?}", v),
//         Err(e) => println!("get_beta_angle_result error: {:#?}", e),
//     }
// }
