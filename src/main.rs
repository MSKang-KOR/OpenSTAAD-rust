mod openstaad;

use openstaad::api::process::StaadProcess;

use crate::openstaad::tools::com::invoke_method;

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, anyhow, bail};
use windows::{
    Win32::System::{
        Com::{
            CLSIDFromProgID, COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize, IDispatch,
        },
        Ole::GetActiveObject,
        Variant::{VARIANT, VariantToInt32},
    },
    core::{GUID, HSTRING, IUnknown, Interface, PCWSTR},
};

fn main() {
    println!("OpenSTAAD Rust Library");
    let mut process = StaadProcess::new("");
    let _ = process.start();
    let _root = process.root();

    match _root.get_base_unit() {
        Ok(base_unit) => println!("Base unit: {:#?}", base_unit),
        Err(e) => println!("Error getting base unit: {:#?}", e),
    };

    // let _geometry = process.geometry();
    // match _geometry.get_node_list() {
    //     Ok(v) => println!("Node list result: {:#?}", v),
    //     Err(e) => println!("Node list error: {:#?}", e),
    // };

    match _root.set_silent_mode(1) {
        Ok(code) => println!("set_silent_mode: {:#?}", code),
        Err(e) => println!("Error getting set_silent_mode: {:#?}", e),
    };
    match _root.analyze_ex(1, 0, 1) {
        Ok(code) => println!("analyze_ex: {:#?}", code),
        Err(e) => println!("Error getting analyze_ex: {:#?}", e),
    };

    unsafe { CoUninitialize() };

    // unsafe {
    //     // COM 초기화
    //     let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

    //     // OpenSTAAD 애플리케이션 객체 생성
    //     let clsid_str = PCWSTR::from_raw(HSTRING::from("StaadPro.OpenSTAAD").as_ptr());
    //     let clsid = CLSIDFromProgID(clsid_str)
    //         .context("CLSID 생성 실패")
    //         .unwrap();

    //     // let staad_app: IDispatch = CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER)
    //     //     .context("OpenSTAAD 인스턴스 생성 실패")?;

    //     // self.staad_app = Some(staad_app);
    //     let pv_reserved: Option<*mut core::ffi::c_void> = None;
    //     let mut ppunk: Option<IUnknown> = None;
    //     match GetActiveObject(
    //         &clsid as *const GUID,
    //         pv_reserved,
    //         &mut ppunk as *mut Option<IUnknown>,
    //     ) {
    //         Err(e) => {
    //             println!("{:#?}", e);
    //         }
    //         _ => {
    //             if let Some(_ppunk) = ppunk {
    //                 let staad_dispatch = _ppunk.cast::<IDispatch>();
    //                 if let Ok(_staad) = staad_dispatch {
    //                     let mut params = [
    //                         VARIANT::from(1), // wait
    //                         VARIANT::from(0), // hidden
    //                         VARIANT::from(1), // silent
    //                     ];
    //                     // let result_variant = invoke_method(&_staad, "AnalyzeEx", &mut params);
    //                     let _ = invoke_method(&_staad, "SetSilentMode", &mut [VARIANT::from(1)]);
    //                     let result_variant = invoke_method(&_staad, "AnalyzeEx", &mut params);
    //                     match result_variant {
    //                         Ok(var) => {
    //                             let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
    //                             println!("Result code: {:#?}", result_code);
    //                         }
    //                         Err(e) => {
    //                             println!("Error::Main::analyze: {:#?}", e)
    //                         }
    //                     };
    //                 };
    //             }
    //         }
    //     };
    // }
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
