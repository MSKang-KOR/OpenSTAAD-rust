use anyhow::{Result, anyhow};
use chrono::Local;
use log::{error, info, warn};
use openstaad_rust::{
    openstaad::{app::OpenStaad, bindings::Staad, custom::*, execute::execute_method},
    tools::{
        InType, SafeArray, SafeArrayP, invoke_method, sa_to_vec1d, safe_array_from_vec1d,
        variant_with_ptr_from, variant_with_ptr_to,
    },
};
use serde_json::{Value, json};
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
    // match get_nodes_table(&mut openstaad) {
    //     Ok(v) => {
    //         info!("get_nodes_table {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("get_nodes_table failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match get_beams_table(&mut openstaad) {
    //     Ok(v) => {
    //         info!("get_beams_table {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("get_beams_table failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match get_section_list(&mut openstaad) {
    //     Ok(v) => {
    //         info!("get_section_list {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("get_section_list failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match get_section_property_tables(&mut openstaad) {
    //     Ok(v) => {
    //         info!("get_section_property_table {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("get_section_property_table failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match get_beta_list(&mut openstaad) {
    //     Ok(v) => {
    //         info!("get_beta_list {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("get_beta_list failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match get_orthotropic2d_material_list(&mut openstaad) {
    //     Ok(v) => {
    //         info!("get_orthotropic2d_material_list {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("get_orthotropic2d_material_list failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match get_specification_list(&mut openstaad) {
    //     Ok(v) => {
    //         info!("get_specification_list {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("get_specification_list failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match get_support_list(&mut openstaad) {
    //     Ok(v) => {
    //         info!("get_support_list {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("get_support_list failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match get_reference_load_list(&mut openstaad) {
    //     Ok(v) => {
    //         info!("get_reference_load_list {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("get_reference_load_list failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match get_load_case_list(&mut openstaad) {
    //     Ok(v) => {
    //         info!("get_load_case_list {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("get_load_case_list failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match get_load_item_list(&mut openstaad, json!(2)) {
    //     Ok(v) => {
    //         info!("get_load_item_list {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("get_load_item_list failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match analyze(&mut openstaad) {
    //     Ok(v) => {
    //         info!("get_load_item_list {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("get_load_item_list failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }

    Ok(())
}

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
