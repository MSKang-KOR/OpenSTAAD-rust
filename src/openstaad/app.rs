use anyhow::{Context, Result, anyhow, bail};
use log::{info, warn};
use serde::Serialize;
use windows::Win32::System::Com::{CoUninitialize, GetRunningObjectTable, IMoniker};
use windows::Win32::System::Ole::GetActiveObject;
use windows::Win32::System::Variant::VariantToInt32;
use windows::{
    Win32::System::{
        Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, IDispatch},
        Variant::VARIANT,
    },
    core::{GUID, HSTRING, PCWSTR},
};
use windows_core::{IUnknown, Interface};

use crate::openstaad::command::Command;
use crate::openstaad::design::Design;
use crate::openstaad::geometry::Geometry;
use crate::openstaad::load::Load;
use crate::openstaad::output::Output;
use crate::openstaad::property::Property;
use crate::openstaad::root::Root;
use crate::openstaad::support::Support;
use crate::tools::invoke::{invoke_method, invoke_property};
use crate::tools::value_types::{InType as ptype, MethodSignature, OutType as rtype};
use std::{collections::HashMap, sync::Arc, thread, time::Duration};

#[derive(Debug, Clone, Serialize)]
pub struct OpenStaad {
    #[serde(skip)]
    pub id: u32,
    #[serde(skip)]
    pub root: Option<Arc<Root>>,
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
        let (id, dispatch) = initialize(system_path, std_path)?;
        let instance = Self {
            id,
            root: Some(Arc::new(Root::new(dispatch))),
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

        let instance = Self {
            id,
            root: Some(Arc::new(Root::new(dispatch))),
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

    pub fn new_by_pid(pid: u32) -> Result<Self> {
        let dispatch = get_object_by_pid(pid)?;
        let instance = Self {
            id: pid,
            root: Some(Arc::new(Root::new(dispatch))),
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

    pub fn get_root(&mut self) -> Result<Arc<Root>> {
        Ok(Arc::clone(self.root.as_ref().unwrap()))
    }
    pub fn get_command(&mut self) -> Result<Arc<Command>> {
        if self.command.is_none() {
            let dispatch =
                unsafe { invoke_property(&self.root.as_ref().unwrap().dispatch, "Command")? };
            self.command = Some(Arc::new(Command::new(dispatch)));
        }
        Ok(Arc::clone(self.command.as_ref().unwrap()))
    }
    pub fn get_design(&mut self) -> Result<Arc<Design>> {
        if self.design.is_none() {
            let dispatch =
                unsafe { invoke_property(&self.root.as_ref().unwrap().dispatch, "Design")? };
            self.design = Some(Arc::new(Design::new(dispatch)));
        }
        Ok(Arc::clone(self.design.as_ref().unwrap()))
    }
    pub fn get_geometry(&mut self) -> Result<Arc<Geometry>> {
        if self.geometry.is_none() {
            let dispatch =
                unsafe { invoke_property(&self.root.as_ref().unwrap().dispatch, "Geometry")? };
            self.geometry = Some(Arc::new(Geometry::new(dispatch)));
        }
        Ok(Arc::clone(self.geometry.as_ref().unwrap()))
    }
    pub fn get_load(&mut self) -> Result<Arc<Load>> {
        if self.load.is_none() {
            let dispatch =
                unsafe { invoke_property(&self.root.as_ref().unwrap().dispatch, "Load")? };
            self.load = Some(Arc::new(Load::new(dispatch)));
        }
        Ok(Arc::clone(self.load.as_ref().unwrap()))
    }
    pub fn get_output(&mut self) -> Result<Arc<Output>> {
        if self.output.is_none() {
            let dispatch =
                unsafe { invoke_property(&self.root.as_ref().unwrap().dispatch, "Output")? };
            self.output = Some(Arc::new(Output::new(dispatch)));
        }
        Ok(Arc::clone(self.output.as_ref().unwrap()))
    }
    pub fn get_property(&mut self) -> Result<Arc<Property>> {
        if self.property.is_none() {
            let dispatch =
                unsafe { invoke_property(&self.root.as_ref().unwrap().dispatch, "Property")? };
            self.property = Some(Arc::new(Property::new(dispatch)));
        }
        Ok(Arc::clone(self.property.as_ref().unwrap()))
    }
    pub fn get_support(&mut self) -> Result<Arc<Support>> {
        if self.support.is_none() {
            let dispatch =
                unsafe { invoke_property(&self.root.as_ref().unwrap().dispatch, "Support")? };
            self.support = Some(Arc::new(Support::new(dispatch)));
        }
        Ok(Arc::clone(self.support.as_ref().unwrap()))
    }
}

fn initialize(system_path: String, std_path: String) -> Result<(u32, IDispatch)> {
    // let pid = run_no_window(system_path)?;
    // info!("Started STAAD.Pro process with PID: {}", pid);
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    }
    if !std::path::Path::new(&system_path).exists() {
        bail!("파일이 존재하지 않습니다: {}", system_path);
    }

    let child = std::process::Command::new(&system_path)
        .args(&[std_path.as_str(), "/s"])
        .spawn()?;
    let _pid = child.id();

    info!("Waiting for STAAD.Pro process to initialize...");
    thread::sleep(Duration::from_millis(5000));

    let mut attempts = 0;
    let max_attempts = 5;
    loop {
        attempts += 1;

        // 먼저 ROT 방식 시도
        match find_staad_by_process_id(_pid) {
            Ok(dispatch) => {
                info!("Success to connect Staad.Pro with pid {} via ROT", _pid);
                let _ = waiting(&dispatch)?;
                return Ok((_pid, dispatch));
            }
            Err(e) => {
                if attempts > max_attempts {
                    let _ = std::process::Command::new("taskkill")
                        .args(&["/PID", _pid.to_string().as_str()])
                        .spawn()?;
                    // unsafe {
                    //     let _ = CoUninitialize();
                    // }
                    bail!(
                        "Staas.Pro가 정상적으로 실행되지 않았거나 STD 파일을 열 수 없어 종료합니다."
                    );
                }
                info!(
                    "ROT approach failed: {}. Trying CoCreateInstance approach...",
                    e
                );
                thread::sleep(Duration::from_millis(3000));
            }
        }
    }
}

fn get_active_object() -> Result<IDispatch> {
    info!("Creating OpenSTAAD instance...");
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    };
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
                    "No active STAAD.Pro instance found. Attempting to launch STAAD.Pro in background: {}",
                    e
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
            bail!("Fail to cast IUnknown to IDispatch.")
        }
    }
}

fn get_object_by_pid(_pid: u32) -> Result<IDispatch> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    }
    match find_staad_by_process_id(_pid) {
        Ok(dispatch) => {
            info!("Success to connect Staad.Pro with pid {} via ROT", _pid);
            return Ok(dispatch);
        }
        Err(e) => {
            bail!("ROT approach failed: {}", e)
        }
    }
}

fn find_staad_by_process_id(target_pid: u32) -> Result<IDispatch> {
    unsafe {
        let rot = GetRunningObjectTable(0).context("ROT 가져오기 실패")?;

        // ROT의 모든 객체를 열거
        let enum_moniker = rot.EnumRunning().context("ROT 열거 실패")?;
        loop {
            let mut monikers: [Option<IMoniker>; 1] = [None];
            let mut fetched = 0u32;

            // 다음 moniker 가져오기
            let hr = enum_moniker.Next(&mut monikers, Some(&mut fetched));
            if hr.is_err() || fetched == 0 {
                break;
            }
            if let Some(moniker) = &monikers[0] {
                if let Ok(unknown) = rot.GetObject(moniker) {
                    // IDispatch로 캐스트 시도
                    if let Ok(dispatch) = unknown.cast::<IDispatch>() {
                        // 이 객체가 STAAD인지 확인 (GetProcessId 메서드 존재 여부로 판단)
                        if let Ok(true) = is_staad_object_with_pid(&dispatch, target_pid) {
                            info!(
                                "대상 PID {}와 일치하는 STAAD 인스턴스를 찾았습니다",
                                target_pid
                            );
                            return Ok(dispatch);
                        }
                    }
                } else {
                    info!("Failed to get object: {}", target_pid);
                }
            }
        }
        Err(anyhow!(
            "프로세스 ID {}에 해당하는 STAAD 인스턴스를 찾을 수 없습니다",
            target_pid
        ))
    }
}

fn is_staad_object_with_pid(_dispatch: &IDispatch, _target_pid: u32) -> Result<bool> {
    unsafe {
        let current_pid_result = invoke_method(_dispatch, "GetProcessId", &mut []);
        match current_pid_result {
            Ok(var) => {
                if let Ok(cur_pid) = VariantToInt32(&var as *const VARIANT) {
                    info!("TargetPid: {}, CurrentPid: {}", _target_pid, cur_pid);
                    return Ok(cur_pid as u32 == _target_pid);
                } else {
                    warn!("PID를 추출할 수 없는 VARIANT: {:#?}", var);
                    return Ok(false);
                }
            }
            Err(_) => {
                // GetProcessId 메서드가 없는 객체는 STAAD 객체가 아님
                return Ok(false);
            }
        }
    }
}

pub fn waiting(dispatch: &IDispatch) -> Result<bool> {
    unsafe {
        let mut attempts = 0;
        let max_attempts = 10;
        let delay = 3;
        loop {
            attempts += 1;
            let result_var = invoke_method(dispatch, "GetCONNECTEDProjectInfo", &mut []);
            match result_var {
                Ok(var) => {
                    if attempts > max_attempts {
                        bail!("Fail to load std as max attempts");
                    }
                    let _bool = VariantToInt32(&var as *const VARIANT)?;
                    if _bool == 1 {
                        return Ok(true);
                    }
                    info!("Waiting for loading std..: {}", attempts);
                    thread::sleep(Duration::from_millis(delay * 1000));
                }
                Err(e) => {
                    if attempts > max_attempts {
                        bail!("Fail to load std as max attempts: {}", e);
                    } else {
                        info!("Waiting for loading std with err...: {}", e);
                        thread::sleep(Duration::from_millis(delay * 1000));
                    }
                }
            }
        }
    }
}

// fn run_no_window(exe_path: String) -> Result<u32> {
//     // 명령줄 생성
//     let command_line = format!("\"{}\"", exe_path.as_str());
//     let mut command_line_wide: Vec<u16> = OsStr::new(&command_line)
//         .encode_wide()
//         .chain(std::iter::once(0))
//         .collect();

//     // STARTUPINFO 구조체 초기화
//     let mut startup_info: STARTUPINFOW = unsafe { mem::zeroed() };
//     startup_info.cb = mem::size_of::<STARTUPINFOW>() as u32;
//     startup_info.dwFlags = STARTF_USESHOWWINDOW;
//     // startup_info.wShowWindow = SW_HIDE as u16; // 창 숨김

//     // PROCESS_INFORMATION 구조체 초기화
//     let mut process_info: PROCESS_INFORMATION = unsafe { mem::zeroed() };

//     // CreateProcessW 호출
//     let result = unsafe {
//         CreateProcessW(
//             None,                                        // lpApplicationName
//             Some(PWSTR(command_line_wide.as_mut_ptr())), // lpCommandLine
//             None,                                        // lpProcessAttributes
//             None,                                        // lpThreadAttributes
//             false,                                       // bInheritHandles
//             CREATE_NO_WINDOW,                            // dwCreationFlags
//             None,                                        // lpEnvironment
//             None,                                        // lpCurrentDirectory
//             &mut startup_info,                           // lpStartupInfo
//             &mut process_info,                           // lpProcessInformation
//         )
//     };

//     if result.is_err() {
//         bail!("CreateProcessW failed");
//     }

//     let pid = process_info.dwProcessId;
//     info!("Process created successfully. PID: {}", pid);

//     // 프로세스가 실제로 시작되었는지 확인
//     unsafe {
//         // 짧은 시간 대기 후 프로세스 상태 확인
//         thread::sleep(Duration::from_millis(500));

//         let mut exit_code: u32 = 0;
//         let exit_result = GetExitCodeProcess(process_info.hProcess, &mut exit_code);

//         if exit_result.is_ok() {
//             if exit_code == 259 {
//                 // STILL_ACTIVE
//                 info!("Process {} is running successfully", pid);
//             } else {
//                 warn!("Process {} exited with code: {}", pid, exit_code);
//             }
//         } else {
//             warn!("Failed to check process status for PID: {}", pid);
//         }

//         // 핸들 정리
//         let _ = CloseHandle(process_info.hProcess);
//         let _ = CloseHandle(process_info.hThread);

//         Ok(pid)
//     }
// }

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
