use anyhow::{Result, anyhow, bail};
use log::{info, warn};
use serde::Serialize;
use windows::Win32::System::Com::{
    CLSCTX_LOCAL_SERVER, CLSIDFromProgID, COINIT_SPEED_OVER_MEMORY, CoCreateInstance,
    CoUninitialize, GetRunningObjectTable, IMoniker,
};
use windows::Win32::System::Ole::GetActiveObject;
use windows::Win32::System::Variant::VariantToInt32;
use windows::Win32::UI::WindowsAndMessaging::{self, SW_HIDE, SWP_HIDEWINDOW};
use windows::{
    Win32::System::{
        Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, IDispatch},
        Variant::VARIANT,
    },
    core::{GUID, HSTRING, PCWSTR},
};
use windows_core::{IUnknown, Interface, PWSTR};

use crate::openstaad::command::Command;
use crate::openstaad::design::Design;
use crate::openstaad::geometry::Geometry;
use crate::openstaad::load::Load;
use crate::openstaad::output::Output;
use crate::openstaad::property::Property;
use crate::openstaad::support::Support;
use crate::tools::invoke::{self, invoke_method, invoke_property};
use crate::tools::value_types::{
    InType as ptype, Input as tparam, MethodSignature, OutType as rtype,
};
use std::mem;
use std::os::windows::process::CommandExt;
use std::ptr;
use std::{
    collections::HashMap, ffi::OsStr, os::windows::ffi::OsStrExt, path::Path, sync::Arc, thread,
    time::Duration,
};
use windows::Win32::Foundation::{CloseHandle, FALSE};
use windows::Win32::System::Threading::STARTF_USESHOWWINDOW;
use windows::Win32::System::Threading::{
    CREATE_NO_WINDOW, CreateProcessW, GetExitCodeProcess, PROCESS_INFORMATION, STARTUPINFOW,
};

/// A wrapper struct to manage the OpenSTAAD application instance.
/// It will handle cleanup (releasing COM objects) when it goes out of scope.
#[derive(Debug, Clone, Serialize)]
pub struct OpenStaad {
    #[serde(skip)]
    pub dispatch: IDispatch,
    pub id: u32,
    #[serde(skip)]
    pub methods: HashMap<String, MethodSignature>,
    pub command: Option<Arc<Command>>,
    pub design: Option<Arc<Design>>,
    pub geometry: Option<Arc<Geometry>>,
    pub load: Option<Arc<Load>>,
    pub output: Option<Arc<Output>>,
    pub property: Option<Arc<Property>>,
    pub support: Option<Arc<Support>>,
}

impl OpenStaad {
    /// Connects to OpenSTAAD and initializes the application.
    pub fn new(system_path: String, std_path: String) -> Result<Self> {
        // let dispatch = get_active_object(system_path, std_path)?;
        let (id, dispatch) = initialize(system_path)?;

        // let prog_id = HSTRING::from("StaadPro.OpenSTAAD");

        // let clsid = unsafe {
        //     CLSIDFromProgID(PCWSTR(prog_id.as_ptr())).map_err(|e| anyhow!("CLSID_ERR: {}", e))?
        // };
        // let _cmd = std::process::Command::new(&system_path)
        //     .args(&[std_path.as_str(), "/h"])
        //     .spawn()?;

        // let _hwnd = unsafe {
        //     WindowsAndMessaging::FindWindowW(PCWSTR(prog_id.as_ptr()), PCWSTR(prog_id.as_ptr()))
        // }
        // .unwrap();

        // unsafe {
        //     let _ = WindowsAndMessaging::ShowWindow(_hwnd, SW_HIDE);
        // };

        // return Err(anyhow!("asdd"));

        // unsafe {
        //     if CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_SPEED_OVER_MEMORY).is_err() {
        //         bail!("COM already initialized or initialization failed");
        //     }
        // }

        // let dispatch = unsafe {
        //     CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER)
        //         .map_err(|e| anyhow!("CoCreateInstance failed: {}", e))?
        // };

        let mut methods = HashMap::new();
        let _ = set_methods(&mut methods);

        let instance = Self {
            dispatch,
            id,
            methods,
            command: None,
            design: None,
            geometry: None,
            load: None,
            output: None,
            property: None,
            support: None,
        };
        Ok(instance)
    }
    pub fn new_by_activated() -> Result<Self> {
        let dispatch = get_active_object()?;
        let id = unsafe {
            match invoke_method(&dispatch, "GetProcessId", &mut []) {
                Ok(_var) => VariantToInt32(&_var as *const VARIANT)? as u32,
                Err(e) => {
                    bail!(
                        "[OpenStaad::get_active_staad] Failed to get process ID: {}",
                        e
                    );
                }
            }
        };

        let mut methods = HashMap::new();
        let _ = set_methods(&mut methods);
        let instance = Self {
            dispatch,
            id,
            methods,
            command: None,
            design: None,
            geometry: None,
            load: None,
            output: None,
            property: None,
            support: None,
        };
        Ok(instance)
    }

    pub fn get_command(&mut self) -> Result<Arc<Command>> {
        info!("Accessing the 'Command' property...");
        if self.command.is_none() {
            let dispatch = unsafe { invoke_property(&self.dispatch, "Geometry")? };
            self.command = Some(Arc::new(Command::new(dispatch)));
        }
        Ok(Arc::clone(self.command.as_ref().unwrap()))
    }
    pub fn get_design(&mut self) -> Result<Arc<Design>> {
        info!("Accessing the 'Design' property...");
        if self.design.is_none() {
            let dispatch = unsafe { invoke_property(&self.dispatch, "Geometry")? };
            self.design = Some(Arc::new(Design::new(dispatch)));
        }
        Ok(Arc::clone(self.design.as_ref().unwrap()))
    }
    pub fn get_geometry(&mut self) -> Result<Arc<Geometry>> {
        info!("Accessing the 'Geometry' property...");
        if self.geometry.is_none() {
            let dispatch = unsafe { invoke_property(&self.dispatch, "Geometry")? };
            self.geometry = Some(Arc::new(Geometry::new(dispatch)));
        }
        Ok(Arc::clone(self.geometry.as_ref().unwrap()))
    }
    pub fn get_load(&mut self) -> Result<Arc<Load>> {
        info!("Accessing the 'Load' property...");
        if self.load.is_none() {
            let dispatch = unsafe { invoke_property(&self.dispatch, "Geometry")? };
            self.load = Some(Arc::new(Load::new(dispatch)));
        }
        Ok(Arc::clone(self.load.as_ref().unwrap()))
    }
    pub fn get_output(&mut self) -> Result<Arc<Output>> {
        info!("Accessing the 'Output' property...");
        if self.output.is_none() {
            let dispatch = unsafe { invoke_property(&self.dispatch, "Geometry")? };
            self.output = Some(Arc::new(Output::new(dispatch)));
        }
        Ok(Arc::clone(self.output.as_ref().unwrap()))
    }
    pub fn get_property(&mut self) -> Result<Arc<Property>> {
        info!("Accessing the 'Property' property...");
        if self.property.is_none() {
            let dispatch = unsafe { invoke_property(&self.dispatch, "Geometry")? };
            self.property = Some(Arc::new(Property::new(dispatch)));
        }
        Ok(Arc::clone(self.property.as_ref().unwrap()))
    }
    pub fn get_support(&mut self) -> Result<Arc<Support>> {
        info!("Accessing the 'Support' property...");
        if self.support.is_none() {
            let dispatch = unsafe { invoke_property(&self.dispatch, "Geometry")? };
            self.support = Some(Arc::new(Support::new(dispatch)));
        }
        Ok(Arc::clone(self.support.as_ref().unwrap()))
    }
}

// 리소스 정리
impl Drop for OpenStaad {
    fn drop(&mut self) {
        info!("Dropping OpenStaad instance with ID: {}", self.id);

        // // Quit 메서드 호출하여 정상 종료 시도
        // match unsafe { invoke_method(&self.dispatch, "Quit", &mut []) } {
        //     Ok(_) => info!("Successfully called Quit method"),
        //     Err(e) => warn!("Failed to call Quit method: {}", e),
        // }

        // COM 정리 및 약간의 대기
        unsafe {
            CoUninitialize();
        }

        // 프로세스 정리를 위한 짧은 대기
        thread::sleep(Duration::from_millis(100));
        info!("OpenStaad instance dropped");
    }
}

fn get_active_object() -> Result<IDispatch> {
    info!("Initializing COM library...");
    unsafe {
        if CoInitializeEx(None, COINIT_APARTMENTTHREADED).is_err() {
            return Err(anyhow!("CoInitializeEx failed"));
        }
    }

    info!("Creating OpenSTAAD instance...");
    let clsid = unsafe {
        // ProgID for OpenSTAAD, as per the documentation.
        let prog_id = HSTRING::from("StaadPro.OpenSTAAD");
        windows::Win32::System::Com::CLSIDFromProgID(PCWSTR(prog_id.as_ptr()))
            .map_err(|e| anyhow!("CLSIDFromProgID failed: {}", e))?
    };

    let pv_reserved: Option<*mut core::ffi::c_void> = None;
    let mut ppunk: Option<IUnknown> = None;
    unsafe {
        match GetActiveObject(
            &clsid as *const GUID,
            pv_reserved,
            &mut ppunk as *mut Option<IUnknown>,
        ) {
            Ok(_) => {
                info!("Success connect to active STAAD.Pro.");
            }
            Err(e) => {
                info!(
                    "No active STAAD.Pro instance found. Attempting to launch STAAD.Pro in background..."
                );
            }
        };
    }

    // let openstaad_app = unsafe {
    //     CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER)
    //         .map_err(|e| anyhow!("CoCreateInstance failed: {}", e))?
    // };
    match ppunk {
        Some(v) => {
            let dispatch = v.cast::<IDispatch>()?;
            info!("OpenSTAAD instance created successfully.");
            return Ok(dispatch);
        }
        _ => {
            unsafe {
                CoUninitialize();
            }
            bail!("Fail to cast IUnknown to IDispatch.")
        }
    }
}

fn initialize(system_path: String) -> Result<(u32, IDispatch)> {
    let pid = run_no_window(system_path)?;
    info!("Started STAAD.Pro process with PID: {}", pid);

    unsafe {
        // COM 초기화
        let com_result = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if com_result.is_err() {
            warn!(
                "COM already initialized or initialization failed: {:?}",
                com_result
            );
        }
        // 프로세스가 완전히 시작될 때까지 잠시 대기
        let mut attempts = 0;
        let max_attempts = 5;
        loop {
            attempts += 1;
            match find_staad_by_process_id(pid) {
                Ok(dispatch) => {
                    info!("Successfully connected to STAAD instance with PID: {}", pid);
                    return Ok((pid, dispatch));
                }
                Err(e) => {
                    if attempts > max_attempts {
                        let _ = CoUninitialize();
                        bail!("Failed to find STAAD by PID {}: {}", pid, e);
                    }
                    warn!(
                        "Attempt to initialize with {}(pid) {} failed: {}",
                        pid, attempts, e
                    );
                    thread::sleep(Duration::from_millis(3000));
                }
            }
        }
    }
}

fn find_staad_by_process_id(target_pid: u32) -> Result<IDispatch> {
    unsafe {
        let rot = GetRunningObjectTable(0)?;
        let enum_moniker = rot.EnumRunning()?;

        loop {
            let mut monikers: [Option<IMoniker>; 1] = [None];
            let mut fetched = 0u32;

            let hr = enum_moniker.Next(&mut monikers, Some(&mut fetched));
            if hr.is_err() || fetched == 0 {
                break;
            }

            if let Some(moniker) = &monikers[0] {
                if let Ok(unknown) = rot.GetObject(moniker) {
                    if let Ok(dispatch) = unknown.cast::<IDispatch>() {
                        if is_staad_object_with_pid(&dispatch, target_pid)? {
                            info!("Found STAAD object with PID: {}", target_pid);
                            return Ok(dispatch);
                        }
                    }
                }
            }
        }

        bail!("No STAAD instance found with PID: {}", target_pid)
    }
}

fn is_staad_object_with_pid(dispatch: &IDispatch, target_pid: u32) -> Result<bool> {
    unsafe {
        match invoke_method(dispatch, "GetProcessId", &mut []) {
            Ok(var) => {
                if let Ok(current_pid) = VariantToInt32(&var as *const VARIANT) {
                    Ok(current_pid as u32 == target_pid)
                } else {
                    Ok(false)
                }
            }
            Err(_) => Ok(false),
        }
    }
}

fn run_no_window(exe_path: String) -> Result<u32> {
    // 명령줄 생성
    let command_line = format!("\"{}\"", exe_path.as_str());
    let mut command_line_wide: Vec<u16> = OsStr::new(&command_line)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    // STARTUPINFO 구조체 초기화
    let mut startup_info: STARTUPINFOW = unsafe { mem::zeroed() };
    startup_info.cb = mem::size_of::<STARTUPINFOW>() as u32;
    startup_info.dwFlags = STARTF_USESHOWWINDOW;
    // startup_info.wShowWindow = SW_HIDE as u16; // 창 숨김

    // PROCESS_INFORMATION 구조체 초기화
    let mut process_info: PROCESS_INFORMATION = unsafe { mem::zeroed() };

    // CreateProcessW 호출
    let result = unsafe {
        CreateProcessW(
            None,                                        // lpApplicationName
            Some(PWSTR(command_line_wide.as_mut_ptr())), // lpCommandLine
            None,                                        // lpProcessAttributes
            None,                                        // lpThreadAttributes
            false,                                       // bInheritHandles
            CREATE_NO_WINDOW,                            // dwCreationFlags
            None,                                        // lpEnvironment
            None,                                        // lpCurrentDirectory
            &mut startup_info,                           // lpStartupInfo
            &mut process_info,                           // lpProcessInformation
        )
    };

    if result.is_err() {
        bail!("CreateProcessW failed");
    }

    println!(
        "Process created successfully. PID: {}",
        process_info.dwProcessId
    );

    // 핸들 정리 (대기하지 않음)
    unsafe {
        // WaitForSingleObject(process_info.hProcess, INFINITE);

        // // 종료 코드 가져오기
        // let mut exit_code: u32 = 0;
        // GetExitCodeProcess(process_info.hProcess, &mut exit_code);

        // 핸들 정리
        let _ = CloseHandle(process_info.hProcess);
        let _ = CloseHandle(process_info.hThread);

        // Ok(exit_code)
        Ok(process_info.dwProcessId)
    }
}

fn set_methods(store: &mut HashMap<String, MethodSignature>) {
    // Root OpenSTAAD API Methods (Alphabetical)
    store.insert(
        "Analyze".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![],
        },
    );
    store.insert(
        "AnalyzeEx".to_string(),
        MethodSignature {
            inputs: vec![ptype::Int, ptype::Int, ptype::Int],
            outputs: vec![rtype::Int],
        },
    );
    store.insert(
        "AnalyzeModel".to_string(),
        MethodSignature {
            inputs: vec![ptype::Int],
            outputs: vec![],
        },
    );
    store.insert(
        "CloseSTAADFile".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![],
        },
    );
    store.insert(
        "CreateNamedView".to_string(),
        MethodSignature {
            inputs: vec![ptype::Str, ptype::Int, ptype::MutInt],
            outputs: vec![rtype::Index(2)],
        },
    );
    store.insert(
        "GetAnalysisStatus".to_string(),
        MethodSignature {
            inputs: vec![ptype::Str, ptype::MutInt, ptype::MutInt, ptype::MutDouble],
            outputs: vec![
                rtype::Int,
                rtype::Index(1),
                rtype::Index(2),
                rtype::Index(3),
            ],
        },
    );
    store.insert(
        "GetApplicationVersion".to_string(),
        MethodSignature {
            inputs: vec![ptype::MutInt, ptype::MutInt, ptype::MutInt, ptype::MutInt],
            outputs: vec![
                rtype::Str,
                rtype::Index(0),
                rtype::Index(1),
                rtype::Index(2),
                rtype::Index(3),
            ],
        },
    );
    store.insert(
        "GetBaseUnit".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![rtype::Int],
        },
    );
    store.insert(
        "GetCONNECTEDProjectInfo".to_string(),
        MethodSignature {
            inputs: vec![ptype::MutStr, ptype::MutStr],
            outputs: vec![rtype::Int, rtype::Index(0), rtype::Index(1)],
        },
    );
    store.insert(
        "GetErrorMessage".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![rtype::Str],
        },
    );
    store.insert(
        "GetFullJobInfo".to_string(),
        MethodSignature {
            inputs: vec![
                ptype::MutStr,
                ptype::MutStr,
                ptype::MutStr,
                ptype::MutStr,
                ptype::MutStr,
                ptype::MutStr,
                ptype::MutStr,
                ptype::MutStr,
                ptype::MutStr,
                ptype::MutStr,
                ptype::MutStr,
                ptype::MutStr,
            ],
            outputs: vec![
                rtype::Index(0),
                rtype::Index(1),
                rtype::Index(2),
                rtype::Index(3),
                rtype::Index(4),
                rtype::Index(5),
                rtype::Index(6),
                rtype::Index(7),
                rtype::Index(8),
                rtype::Index(9),
                rtype::Index(10),
                rtype::Index(11),
            ],
        },
    );
    store.insert(
        "GetInputUnitForForce".to_string(),
        MethodSignature {
            inputs: vec![ptype::MutStr],
            outputs: vec![rtype::Int, rtype::Index(0)],
        },
    );
    store.insert(
        "GetInputUnitForLength".to_string(),
        MethodSignature {
            inputs: vec![ptype::MutStr],
            outputs: vec![rtype::Int, rtype::Index(0)],
        },
    );
    store.insert(
        "GetMainWindowHandle".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![rtype::Int],
        },
    );
    store.insert(
        "GetProcessHandle".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![rtype::Int],
        },
    );
    store.insert(
        "GetProcessId".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![rtype::Int],
        },
    );
    store.insert(
        "GetShortJobInfo".to_string(),
        MethodSignature {
            inputs: vec![ptype::MutStr, ptype::MutStr, ptype::MutStr],
            outputs: vec![rtype::Index(0), rtype::Index(1), rtype::Index(2)],
        },
    );
    store.insert(
        "GetSTAADFile".to_string(),
        MethodSignature {
            inputs: vec![ptype::MutStr, ptype::Bool],
            outputs: vec![rtype::Index(0)],
        },
    );
    store.insert(
        "GetSTAADFileFolder".to_string(),
        MethodSignature {
            inputs: vec![ptype::MutStr],
            outputs: vec![rtype::Index(0)],
        },
    );
    store.insert(
        "IsAnalyzing".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![rtype::Int],
        },
    );
    store.insert(
        "IsPhysicalModel".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![rtype::Int],
        },
    );
    store.insert(
        "ModifyNamedView".to_string(),
        MethodSignature {
            inputs: vec![
                ptype::Str,
                ptype::Int,
                ptype::Int,
                ptype::Int,
                ptype::Int,
                ptype::MutInt,
            ],
            outputs: vec![rtype::Index(5)],
        },
    );
    store.insert(
        "NewSTAADFile".to_string(),
        MethodSignature {
            inputs: vec![ptype::Str, ptype::Int, ptype::Int],
            outputs: vec![],
        },
    );
    store.insert(
        "OpenSTAADFile".to_string(),
        MethodSignature {
            inputs: vec![ptype::Str],
            outputs: vec![],
        },
    );
    store.insert(
        "Quit".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![],
        },
    );
    store.insert(
        "RemoveNamedView".to_string(),
        MethodSignature {
            inputs: vec![ptype::Str, ptype::MutInt],
            outputs: vec![rtype::Index(1)],
        },
    );
    store.insert(
        "SaveModel".to_string(),
        MethodSignature {
            inputs: vec![ptype::Bool],
            outputs: vec![],
        },
    );
    store.insert(
        "SaveNamedView".to_string(),
        MethodSignature {
            inputs: vec![ptype::Str, ptype::MutInt],
            outputs: vec![rtype::Index(1)],
        },
    );
    store.insert(
        "SetCONNECTEDProjectInfo".to_string(),
        MethodSignature {
            inputs: vec![ptype::Str, ptype::Str],
            outputs: vec![rtype::Int],
        },
    );
    store.insert(
        "SetFullJobInfo".to_string(),
        MethodSignature {
            inputs: vec![
                ptype::Str,
                ptype::Str,
                ptype::Str,
                ptype::Str,
                ptype::Str,
                ptype::Str,
                ptype::Str,
                ptype::Str,
                ptype::Str,
                ptype::Str,
                ptype::Str,
                ptype::Str,
            ],
            outputs: vec![],
        },
    );
    store.insert(
        "SetInputUnitForForce".to_string(),
        MethodSignature {
            inputs: vec![ptype::Int],
            outputs: vec![],
        },
    );
    store.insert(
        "SetInputUnitForLength".to_string(),
        MethodSignature {
            inputs: vec![ptype::Int],
            outputs: vec![],
        },
    );
    store.insert(
        "SetInputUnits".to_string(),
        MethodSignature {
            inputs: vec![ptype::Int, ptype::Int],
            outputs: vec![],
        },
    );
    store.insert(
        "SetShortJobInfo".to_string(),
        MethodSignature {
            inputs: vec![ptype::Str, ptype::Str, ptype::Str],
            outputs: vec![],
        },
    );
    store.insert(
        "SetSilentMode".to_string(),
        MethodSignature {
            inputs: vec![ptype::Int],
            outputs: vec![rtype::Int],
        },
    );
    store.insert(
        "UpdateStructure".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![],
        },
    );
}
