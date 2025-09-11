use anyhow::{Result, anyhow};
use chrono::Local;
use log::{error, info, warn};
use openstaad_rust::{
    openstaad::{
        app::OpenStaad,
        bindings::Staad,
        custom::{get_beams_table, get_nodes_table},
        execute::execute_method,
    },
    tools::{
        InType, SafeArray, SafeArrayP, invoke_method, safe_array_from_vec1d, safe_array_to_vec1d,
        variant_with_ptr_from, variant_with_ptr_to,
    },
};
use serde_json::Value;
use std::{
    ffi::{OsStr, c_void},
    fs::OpenOptions,
    os::windows::ffi::OsStrExt,
    sync::Arc,
};
use windows::Win32::System::{
    Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize, SAFEARRAY},
    Ole::{SafeArrayCreateVector, SafeArrayGetElement},
    Variant::{
        VARIANT, VT_I4, VT_R4, VT_R8, VariantToBoolean, VariantToInt32, VariantToStringAlloc,
    },
};

fn main() -> Result<()> {
    // Initialize file logging with timestamp
    setup_file_logging()?;

    let system_path =
        "C:\\Program Files\\Bentley\\Engineering\\STAAD.Pro 2025\\STAAD\\Bentley.Staad.exe"
            .to_string();
    let std_path = "C:\\Users\\kms36\\Downloads\\staa_api_test\\sample.STD";
    // let mut _openstaad = OpenStaad::new(system_path, std_path.to_string()).map_err(|e| {
    //     error!("Failed to create OpenSTAAD instance: {}", e);
    //     e
    // })?;
    let mut _openstaad = OpenStaad::new_by_activated().map_err(|e| {
        error!("Failed to create OpenSTAAD instance: {}", e);
        e
    })?;
    let _output = _openstaad.get_output()?;
    let _geometry = _openstaad.get_geometry()?;

    let mut openstaad = Staad::OpenStaad(_openstaad);
    match get_nodes_table(&mut openstaad) {
        Ok(v) => {
            info!("{:#?}", v);
        }
        Err(e) => {
            error!("get_nodes_table failed: {}", e);
            drop(openstaad);
            return Err(e);
        }
    }
    match get_beams_table(&mut openstaad) {
        Ok(v) => {
            info!("{:#?}", v);
        }
        Err(e) => {
            error!("get_beams_table failed: {}", e);
            drop(openstaad);
            return Err(e);
        }
    }

    // match test_openstaad(&openstaad, std_path.to_string()) {
    //     Ok(_) => info!("✓ Basic connection test passed"),
    //     Err(e) => {
    //         error!("✗ Basic connection test failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }

    // let output = Staad::Output(_output);
    // match test_output(&output) {
    //     Ok(_) => info!("✓ Output test passed"),
    //     Err(e) => {
    //         error!("✗ Output test failed: {}", e);
    //         return Err(e);
    //     }
    // }

    Ok(())
}

/// Test basic COM connection and interface access
fn test_openstaad(instance: &Staad, std_path: String) -> Result<()> {
    let GetProcessId = execute_method(&instance, "GetProcessId", &[])?;
    info!("GetProcessId: {:#?}", GetProcessId);

    let GetAnalysisStatus = execute_method(&instance, "GetAnalysisStatus", &[std_path.into()])?;
    info!("GetAnalysisStatus: {:#?}", GetAnalysisStatus);

    // let OpenSTAADFile = execute_method(
    //     &instance,
    //     "OpenSTAADFile",
    //     &["C:\\Users\\kms36\\Downloads\\staa_api_test\\sample.STD"
    //         .to_string()
    //         .into()],
    // )?;
    // info!("OpenSTAADFile: {:#?}", OpenSTAADFile);
    Ok(())
}

fn test_output(instance: &Staad) -> Result<()> {
    let GetNodeDisplacements =
        execute_method(&instance, "GetNodeDisplacements", &[1.into(), 351.into()])?;
    info!("GetNodeDisplacements: {:#?}", GetNodeDisplacements);

    let GetSupportReactions =
        execute_method(&instance, "GetSupportReactions", &[1051.into(), 351.into()])?;
    info!("GetSupportReactions: {:#?}", GetSupportReactions);

    let GetMemberSteelDesignResults =
        execute_method(&instance, "GetMemberSteelDesignResults", &[1.into()])?;
    info!(
        "GetMemberSteelDesignResults: {:#?}",
        GetMemberSteelDesignResults
    );
    Ok(())
}

// /// Test basic COM connection and interface access
// fn test_geometry(instance: &Staad) -> Result<()> {
//     // let AddNode = execute_method(
//     //     &instance,
//     //     "AddNode",
//     //     &[10566.979.into(), 103.650.into(), (-9475.481).into()],
//     // )?;
//     // info!("AddNode: {:#?}", AddNode);
//     // let AddMultipleNodes = execute_method(
//     //     &instance,
//     //     "AddMultipleNodes",
//     //     &[vec![
//     //         vec![10567.979, 103.650, (-9475.481)],
//     //         vec![10568.979, 103.650, (-9475.481)],
//     //         vec![10569.979, 103.650, (-9475.481)],
//     //     ]
//     //     .into()],
//     // )?;
//     // info!("AddMultipleNodes: {:#?}", AddMultipleNodes);
//     let GetNodeCount = execute_method(&instance, "GetNodeCount", &[])?;
//     info!("GetNodeCount: {:#?}", GetNodeCount);
//     let GetNodeDistance = execute_method(
//         &instance,
//         "GetNodeDistance",
//         &[1, 2].map(|x| x.into()).as_slice(),
//     )?;
//     info!("GetNodeDistance: {:#?}", GetNodeDistance);
//     let GetNodeCoordinates = execute_method(&instance, "GetNodeCoordinates", &[1.into()])?;
//     info!("GetNodeCoordinates: {:#?}", GetNodeCoordinates);
//     let GetNodeList = execute_method(&instance, "GetNodeList", &[])?;
//     info!("GetNodeList: {:#?}", GetNodeList);
//     let GetNodeIncidence_CIS2 = execute_method(&instance, "GetNodeIncidence_CIS2", &[1.into()])?;
//     info!("GetNodeIncidence_CIS2: {:#?}", GetNodeIncidence_CIS2);

//     let GetNoOfBeamsConnectedAtNode =
//         execute_method(&instance, "GetNoOfBeamsConnectedAtNode", &[1.into()])?;
//     info!(
//         "GetNoOfBeamsConnectedAtNode: {:#?}",
//         GetNoOfBeamsConnectedAtNode
//     );
//     let GetBeamsConnectedAtNode =
//         execute_method(&instance, "GetBeamsConnectedAtNode", &[1.into()])?;
//     info!("GetBeamsConnectedAtNode: {:#?}", GetBeamsConnectedAtNode);
//     let IntersectBeams = execute_method(
//         &instance,
//         "IntersectBeams",
//         &[1.into(), vec![1, 2, 3].into(), 0.1.into()],
//     )?;
//     info!("IntersectBeams: {:#?}", IntersectBeams);
//     let GetGroupCount = execute_method(&instance, "GetGroupCount", &[2.into()])?;
//     info!("GetGroupCount: {:#?}", GetGroupCount);
//     let GetGroupNames = execute_method(&instance, "GetGroupNames", &[2.into()])?;
//     info!("GetGroupNames: {:#?}", GetGroupNames);
//     Ok(())
// }

/// Setup file logging to save all logs to a timestamped txt file
fn setup_file_logging() -> Result<()> {
    use env_logger::{Builder, Target};
    use std::io::Write;

    // // Create timestamped log filename
    // let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let log_filename = format!("log.log");

    // Create or open log file
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_filename)?;

    // Initialize logger with custom format
    Builder::from_default_env()
        .target(Target::Pipe(Box::new(log_file)))
        .format(|buf, record| {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            writeln!(
                buf,
                "[{}] [{}] [{}:{}] {}",
                timestamp,
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Trace) // Log everything
        .init();

    // Also print to console that logging started
    println!("✓ Logging initialized - saving to: {}", log_filename);
    println!("✓ All logs (info, warn, error, debug, trace) will be saved to the file");
    println!("✓ Starting Staad.Pro COM connection test...\n");

    Ok(())
}
