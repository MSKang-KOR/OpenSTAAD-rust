mod staad;

use staad::process::StaadProcess;

use std::{path::Path, thread::sleep, time::Duration};

use crate::staad::{geometry::root::Geometry, notify::watch_file_background, root::Root};

#[tokio::main]
async fn main() {
    // StaadBackgroundAnalyzer
    println!("Hello, world!");
    let mut process = StaadProcess::new("");
    match process.start().await {
        Err(e) => {
            println!("{:#?}", e)
        }
        _ => {
            // let _ = _staad.test_code();
            if let Some(root) = process.root.as_ref() {
                let base_unit = root.get_base_unit().unwrap();
                println!("base unit: {:#?}", base_unit);

                // let _root = process.root.as_ref().unwrap();
                // test_root_methods(_root);

                let _geometry = root.geometry();
                test_geometry_node_methods(&_geometry, base_unit);

                // let _command = root.command();
                // let _design = root.design();
                // let _output = root.output();
            }
        }
    }
}

fn test_geometry_node_methods(geometry: &Geometry, base_unit: i32) {
    let node = geometry.node();
    let coordinates = vec![
        vec![100., 100., 100.],
        vec![200., 200., 200.],
        vec![300., 300., 300.],
    ];
    let add_multiple_result = node.add_multiple_nodes(coordinates, base_unit);
    match add_multiple_result {
        Ok(v) => println!("add_multiple_result success: {:#?}", v),
        Err(e) => println!("add_multiple_result error: {:#?}", e),
    }

    let add_node_result = node.add_node(10601., 113.55, -9481.441, base_unit);
    match add_node_result {
        Ok(v) => println!("add_node_result success: {:#?}", v),
        Err(e) => println!("add_node_result error: {:#?}", e),
    }

    let node_ids = vec![2004, 2005, 2006];
    let coordinates = vec![
        vec![400., 400., 400.],
        vec![500., 500., 500.],
        vec![600., 600., 600.],
    ];
    let create_multiple_result = node.create_multiple_nodes(node_ids, coordinates);
    match create_multiple_result {
        Ok(v) => println!("create_multiple_result success: {:#?}", v),
        Err(e) => println!("create_multiple_result error: {:#?}", e),
    }

    let create_node_result = node.create_node(2007, 10600., 113.55, -9481.441);
    match create_node_result {
        Ok(v) => println!("create_node_result success: {:#?}", v),
        Err(e) => println!("create_node_result error: {:#?}", e),
    }

    let delete_node_result = node.delete_node(2007);
    match delete_node_result {
        Ok(v) => println!("delete_node_result success: {:#?}", v),
        Err(e) => println!("delete_node_result error: {:#?}", e),
    }

    let get_last_node_no_result = node.get_last_node_no();
    match get_last_node_no_result {
        Ok(v) => println!("get_last_node_no_result success: {:#?}", v),
        Err(e) => println!("get_last_node_no_result error: {:#?}", e),
    }

    let get_node_coordinates_result = node.get_node_coordinates(1789, base_unit);
    match get_node_coordinates_result {
        Ok(v) => println!("get_node_coordinates_result success: {:#?}", v),
        Err(e) => println!("get_node_coordinates_result error: {:#?}", e),
    }

    let get_node_count_result = node.get_node_count();
    match get_node_count_result {
        Ok(v) => println!("get_node_count_result success: {:#?}", v),
        Err(e) => println!("get_node_count_result error: {:#?}", e),
    }

    let get_node_distance_result = node.get_node_distance(1, 2, base_unit);
    match get_node_distance_result {
        Ok(v) => println!("get_node_distance_result success: {:#?}", v),
        Err(e) => println!("get_node_distance_result error: {:#?}", e),
    }

    let get_node_incidence_result = node.get_node_incidence(1789, base_unit);
    match get_node_incidence_result {
        Ok(v) => println!("get_node_incidence_result success: {:#?}", v),
        Err(e) => println!("get_node_incidence_result error: {:#?}", e),
    }

    let get_node_incidence_cis2_result = node.get_node_incidence_cis2(1789, base_unit);
    match get_node_incidence_cis2_result {
        Ok(v) => println!("get_node_incidence_cis2_result success: {:#?}", v),
        Err(e) => println!("get_node_incidence_cis2_result error: {:#?}", e),
    }

    // let get_node_list_result = node.get_node_list();
    // match get_node_list_result {
    //     Ok(v) => println!("get_node_list_result success: {:#?}", v),
    //     Err(e) => println!("get_node_list_result error: {:#?}", e),
    // }

    let get_node_number_result = node.get_node_number(10578.4, 115.5, -9478.459444444, base_unit);
    match get_node_number_result {
        Ok(v) => println!("get_node_number_result success: {:#?}", v),
        Err(e) => println!("get_node_number_result error: {:#?}", e),
    }

    let get_unique_id_result = node.get_node_unique_id(1789);
    match get_unique_id_result {
        Ok(v) => println!("get_unique_id_result success: {:#?}", v),
        Err(e) => println!("get_unique_id_result error: {:#?}", e),
    }

    let is_orphan_node_result = node.is_orphan_node(1789);
    match is_orphan_node_result {
        Ok(v) => println!("is_orphan_node_result success: {:#?}", v),
        Err(e) => println!("is_orphan_node_result error: {:#?}", e),
    }

    let set_node_coordinate_result =
        node.set_node_coordinate(1789, 10600., 115.5, -9478.4410, base_unit);
    match set_node_coordinate_result {
        Ok(v) => println!("set_node_coordinate_result success: {:#?}", v),
        Err(e) => println!("set_node_coordinate_result error: {:#?}", e),
    }

    let set_node_unique_id_result = node.set_node_unique_id(1789, "NODE_ID_1789");
    match set_node_unique_id_result {
        Ok(v) => println!("set_node_unique_id_result success: {:#?}", v),
        Err(e) => println!("set_node_unique_id_result error: {:#?}", e),
    }
}

async fn test_root_methods(root: &Root) {
    let set_silent_mode_result = root.set_silent_mode(1);
    match set_silent_mode_result {
        Ok(v) => println!("set_silent_mode_result success: {:#?}", v),
        Err(e) => println!("set_silent_mode_result error: {:#?}", e),
    }

    let analyze_ex_result = root.analyze_ex(1, 1, 0).await;
    match analyze_ex_result {
        Ok(v) => {
            println!("analyze_ex_result success: {:#?}", v);
        }
        Err(e) => println!("analyze_ex_result error: {:#?}", e),
    }

    let get_staad_file_result = root.get_staad_file(true);
    match get_staad_file_result {
        Ok(v) => {
            println!("get_staad_file_result success: {:#?}", v);
            let std_path = Path::new(&v);
            let log_path = std_path.with_extension("log");
            println!("log path: {:#?}", log_path);
            // let log_handle = monitor_log_file_with_notify(log_path.to_str().unwrap()).await;

            let _watcher_handle = watch_file_background(&log_path);

            for i in 1..=5 {
                sleep(Duration::from_secs(2));
                println!("[MAIN] 메인 작업 진행 중... {}/5", i);
            }

            println!("End");

            _watcher_handle.join().unwrap();
        }
        Err(e) => println!("get_staad_file_result error: {:#?}", e),
    }
}
