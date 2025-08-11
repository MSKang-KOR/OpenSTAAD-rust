// src/lib.rs
pub mod openstaad;

// Re-export the main StaadProcess struct
pub use openstaad::process::StaadProcess;

// Re-export tauri utilities and functions directly at root level
pub use openstaad::tauri::utils::{self, *};

// Re-export tauri command functions directly
pub use openstaad::tauri::geometry::geometry_call;
pub use openstaad::tauri::process::{staad_process_call, staad_process_start};
pub use openstaad::tauri::root::root_call;

// Re-export commonly used types for convenience
pub use anyhow::{Error as AnyhowError, Result as AnyhowResult};
pub use serde::{Deserialize, Serialize};
pub use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
