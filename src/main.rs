use anyhow::{Result, anyhow};
use chrono::Local;
use log::{error, info, warn};
use openstaad_rust::openstaad::{
    app::OpenStaad, bindings::Staad, custom::*, execute::execute_method,
};
use std::fs::OpenOptions;

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
    let _design = _openstaad.get_design()?;

    let mut openstaad = Staad::OpenStaad(_openstaad);
    let mut output = Staad::Output(_output);
    let mut design = Staad::Design(_design);
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
    match get_design_results(&mut openstaad) {
        Ok(v) => {
            info!("get_design_results {:#?}", v);
        }
        Err(e) => {
            error!("get_design_results failed: {}", e);
            drop(openstaad);
            return Err(e);
        }
    }

    // match execute_method(&openstaad, "SetSilentMode", &[1.into()]) {
    //     Ok(v) => {
    //         info!("SetSilentMode {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("SetSilentMode failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match execute_method(&openstaad, "AnalyzeEx", &[1.into(), 0.into(), 1.into()]) {
    //     Ok(v) => {
    //         info!("AnalyzeEx {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("AnalyzeEx failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match execute_method(&openstaad, "GetSTAADFile", &[true.into()]) {
    //     Ok(v) => {
    //         info!("GetSTAADFile {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("GetSTAADFile failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match execute_method(&output, "AreResultsAvailable", &[]) {
    //     Ok(v) => {
    //         info!("AreResultsAvailable {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("AreResultsAvailable failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }

    // match execute_method(&output, "GetMemberSteelDesignRatio", &[1.into()]) {
    //     Ok(v) => {
    //         info!("GetMemberSteelDesignRatio {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("GetMemberSteelDesignRatio failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match execute_method(
    //     &output,
    //     "GetMemberEndForces",
    //     &[1.into(), 1.into(), 1.into(), 0.into()],
    // ) {
    //     Ok(v) => {
    //         info!("GetMemberEndForces {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("GetMemberEndForces failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match execute_method(&output, "GetMultipleMemberSteelDesignMaxRatio", &[1.into()]) {
    //     Ok(v) => {
    //         info!("GetMultipleMemberSteelDesignMaxRatio {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("GetMultipleMemberSteelDesignMaxRatio failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match execute_method(&design, "GetDesignBriefCode", &[1.into()]) {
    //     Ok(v) => {
    //         info!("GetDesignBriefCode {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("GetDesignBriefCode failed: {}", e);
    //         drop(openstaad);
    //         return Err(e);
    //     }
    // }
    // match execute_method(&design, "GetMemberDesignParameters", &[1.into(), 1.into()]) {
    //     Ok(v) => {
    //         info!("GetMemberDesignParameters {:#?}", v);
    //     }
    //     Err(e) => {
    //         error!("GetMemberDesignParameters failed: {}", e);
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
