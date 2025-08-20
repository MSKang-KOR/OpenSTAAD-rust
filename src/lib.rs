// // src/lib.rs
// pub mod openstaad;

// // Re-export the main StaadProcess struct
// // pub use openstaad::process::StaadProcess;

// // Re-export tauri utilities and functions directly at root level
// pub use openstaad::tauri::utils::{self, *};

// // Re-export tauri command functions directly
// pub use openstaad::tauri::command::command_call;
// pub use openstaad::tauri::design::design_call;
// pub use openstaad::tauri::geometry::geometry_call;
// pub use openstaad::tauri::load::load_call;
// pub use openstaad::tauri::output::output_call;
// pub use openstaad::tauri::process::process_call;
// pub use openstaad::tauri::property::property_call;
// pub use openstaad::tauri::root::{analyze_background, root_call};
// pub use openstaad::tauri::support::support_call;

pub mod binding;
pub mod com_interop;

pub use anyhow::{Context, Result, bail};
use log::{error, info};
pub use serde::{Deserialize, Serialize};

use crate::binding::{IOSGeometryUI, IOpenSTAADUI};
