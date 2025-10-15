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
    let std_path = "C:\\Users\\kms36\\Downloads\\staa_api_test\\360-PAR-01_Case2.STD";
    let mut _openstaad =
        OpenStaad::new(system_path.clone(), std_path.to_string()).map_err(|e| {
            error!("Failed to create OpenSTAAD instance: {}", e);
            e
        })?;
    // let mut _openstaad = OpenStaad::new_by_activated().map_err(|e| {
    //     error!("Failed to create OpenSTAAD instance: {}", e);
    //     e
    // })?;
    let _root = _openstaad.get_root()?;

    let root = Staad::Root(_root);
    let _ = execute_method(&root, "OpenSTAADFile", &[std_path.to_string().into()]);

    // let primary_loads = get_load_case_list(&mut _openstaad)?;
    // for load in primary_loads {
    //     let item = get_load_item_list(&mut _openstaad, load.id);
    //     match item {
    //         Ok(v) => {
    //             info!("{:#?}", v);
    //         }
    //         Err(e) => {
    //             error!("{:#?}", e);
    //         }
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
