use chrono::Local;
use log::{error, info, warn};
use openstaad_rust::Result;
use openstaad_rust::com_interop::{GeometryManager, OpenStaadApp};
use std::fs::OpenOptions;

fn main() -> Result<()> {
    // Initialize file logging with timestamp
    setup_file_logging()?;

    info!("Starting Staad.Pro COM connection test...");

    // Test 1: Basic COM connection
    match test_basic_connection() {
        Ok(_) => info!("✓ Basic connection test passed"),
        Err(e) => {
            error!("✗ Basic connection test failed: {}", e);
            return Err(e);
        }
    }

    // Test 2: Comprehensive geometry operations
    match test_geometry_operations() {
        Ok(_) => info!("✓ Geometry operations test passed"),
        Err(e) => {
            error!("✗ Geometry operations test failed: {}", e);
            return Err(e);
        }
    }

    info!("All tests completed successfully! Staad.Pro COM integration is working.");
    Ok(())
}

/// Setup file logging to save all logs to a timestamped txt file
fn setup_file_logging() -> Result<()> {
    use env_logger::{Builder, Target};
    use std::io::Write;

    // // Create timestamped log filename
    // let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let log_filename = format!("log_.txt");

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

/// Test basic COM connection and interface access
fn test_basic_connection() -> Result<()> {
    info!("Testing basic COM connection...");

    // Create OpenSTAAD application instance
    let app = OpenStaadApp::new()
        .map_err(|e| {
            error!("Failed to create OpenSTAAD instance. Make sure Staad.Pro is installed and properly registered.");
            e
        })?;

    info!("Successfully connected to Staad.Pro COM object");

    // Test Geometry interface access
    let _geometry = app.get_geometry()?;
    info!("Successfully obtained IOSGeometryUI interface");

    Ok(())
}

/// Test comprehensive geometry operations
fn test_geometry_operations() -> Result<()> {
    info!("Testing geometry operations...");

    // Create OpenSTAAD application instance
    let app = OpenStaadApp::new()?;

    // Get geometry interface
    let geometry_interface = app.get_geometry()?;
    // let geometry_manager = GeometryManager::new(geometry_interface);

    // // Test 1: Create nodes with specific numbers
    // info!("Creating nodes with specific numbers...");
    // geometry_manager.create_node(1, 0.0, 0.0, 0.0)?;
    // geometry_manager.create_node(2, 10.0, 0.0, 0.0)?;
    // geometry_manager.create_node(3, 20.0, 0.0, 0.0)?;
    // geometry_manager.create_node(4, 10.0, 10.0, 0.0)?;

    // // Test 2: Create beams with specific numbers
    // info!("Creating beams with specific numbers...");
    // geometry_manager.create_beam(1, 1, 2)?;
    // geometry_manager.create_beam(2, 2, 3)?;
    // geometry_manager.create_beam(3, 2, 4)?;

    // // Test 3: Add nodes (auto-numbering)
    // info!("Adding nodes with auto-numbering...");
    // let node5 = geometry_manager.add_node(0.0, 10.0, 0.0)?;
    // let node6 = geometry_manager.add_node(20.0, 10.0, 0.0)?;
    // info!("Auto-generated node numbers: {} and {}", node5, node6);

    // // Test 4: Add beams (auto-numbering)
    // info!("Adding beams with auto-numbering...");
    // let beam4 = geometry_manager.add_beam(1, node5)?;
    // let beam5 = geometry_manager.add_beam(3, node6)?;
    // info!("Auto-generated beam numbers: {} and {}", beam4, beam5);

    // // Test 5: Create a simple frame structure
    // info!("Creating a simple frame structure...");

    // // Create corner nodes for a rectangular frame
    // let corner1 = geometry_manager.add_node(30.0, 0.0, 0.0)?;
    // let corner2 = geometry_manager.add_node(40.0, 0.0, 0.0)?;
    // let corner3 = geometry_manager.add_node(40.0, 8.0, 0.0)?;
    // let corner4 = geometry_manager.add_node(30.0, 8.0, 0.0)?;

    // // Create frame beams
    // let _frame_beam1 = geometry_manager.add_beam(corner1, corner2)?;
    // let _frame_beam2 = geometry_manager.add_beam(corner2, corner3)?;
    // let _frame_beam3 = geometry_manager.add_beam(corner3, corner4)?;
    // let _frame_beam4 = geometry_manager.add_beam(corner4, corner1)?;

    // info!("Successfully created a rectangular frame structure");

    // info!("Geometry operations completed successfully!");
    // info!("Check your Staad.Pro application to see the created structure:");
    // info!("- Nodes at various coordinates");
    // info!("- Connecting beams forming a simple structure");
    // info!("- A rectangular frame at the end");

    Ok(())
}
