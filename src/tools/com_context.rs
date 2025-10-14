use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;
use std::thread::ThreadId;
use anyhow::Result;
use log::{info, warn};
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

/// Global COM initialization tracker per thread
static COM_INITIALIZED_THREADS: Mutex<Option<HashMap<ThreadId, usize>>> = Mutex::new(None);

/// Initialize COM for the current thread if not already initialized
pub fn initialize_com() -> Result<()> {
    let thread_id = std::thread::current().id();

    let mut map_lock = COM_INITIALIZED_THREADS.lock().unwrap();
    let map = map_lock.get_or_insert_with(HashMap::new);

    let counter = map.entry(thread_id).or_insert(0);

    if *counter == 0 {
        unsafe {
            let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            if hr.is_ok() {
                info!("COM initialized for thread {:?}", thread_id);
            } else {
                // RPC_E_CHANGED_MODE (0x80010106) means COM is already initialized
                // with a different threading model, which is acceptable
                // S_FALSE (0x00000001) means COM is already initialized in this thread
                let code = hr.0 as u32;
                if code == 0x80010106 || code == 0x00000001 {
                    info!("COM already initialized for thread {:?} (code: 0x{:08X})", thread_id, code);
                } else {
                    warn!("COM initialization warning for thread {:?}: 0x{:08X}", thread_id, code);
                }
            }
        }
    }

    *counter += 1;
    info!("COM reference count for thread {:?}: {}", thread_id, *counter);

    Ok(())
}

/// Decrement COM reference count and uninitialize if count reaches zero
pub fn uninitialize_com() {
    let thread_id = std::thread::current().id();

    let mut map_lock = COM_INITIALIZED_THREADS.lock().unwrap();
    if let Some(map) = map_lock.as_mut() {
        if let Some(counter) = map.get_mut(&thread_id) {
            *counter = counter.saturating_sub(1);

            if *counter == 0 {
                unsafe {
                    CoUninitialize();
                }
                map.remove(&thread_id);
                info!("COM uninitialized for thread {:?}", thread_id);
            } else {
                info!("COM reference count for thread {:?}: {}", thread_id, *counter);
            }
        }
    }
}

/// RAII wrapper for COM initialization
#[derive(Debug)]
pub struct ComContext {
    _private: (),
}

impl ComContext {
    /// Create a new COM context, initializing COM if necessary
    pub fn new() -> Result<Self> {
        initialize_com()?;
        Ok(Self { _private: () })
    }
}

impl Drop for ComContext {
    fn drop(&mut self) {
        uninitialize_com();
    }
}
