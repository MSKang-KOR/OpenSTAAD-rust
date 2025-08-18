use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use crate::openstaad::tauri::utils::StaadObject;

pub static PROCESS_STORE: LazyLock<Mutex<HashMap<String, StaadObject>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
