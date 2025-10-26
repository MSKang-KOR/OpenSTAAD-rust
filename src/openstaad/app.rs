use anyhow::{Context, Result, anyhow, bail};
use log::{info, warn};
use serde::Serialize;
use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM, STILL_ACTIVE};
use windows::Win32::System::Com::{GetRunningObjectTable, IMoniker};
use windows::Win32::System::Ole::GetActiveObject;
use windows::Win32::System::Threading::{
    CREATE_NO_WINDOW, CreateProcessW, GetExitCodeProcess, OpenProcess, PROCESS_INFORMATION,
    PROCESS_TERMINATE, STARTF_USESHOWWINDOW, STARTUPINFOW, TerminateProcess,
};
use windows::Win32::System::Variant::VariantToInt32;
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowThreadProcessId, SW_HIDE, ShowWindow,
};
use windows::{
    Win32::System::{Com::IDispatch, Variant::VARIANT},
    core::{GUID, HSTRING, PCWSTR, PWSTR},
};
use windows_core::{BOOL, BSTR, IUnknown, Interface};

use crate::openstaad::command::Command;
use crate::openstaad::design::Design;
use crate::openstaad::geometry::Geometry;
use crate::openstaad::load::Load;
use crate::openstaad::output::Output;
use crate::openstaad::property::Property;
use crate::openstaad::root::Root;
use crate::openstaad::support::Support;
use crate::tools::invoke::{invoke_method, invoke_property};
use crate::tools::{ComContext, variant_with_ptr_from, variant_with_ptr_to};
use std::path::{Path, PathBuf};
use std::{ffi::OsStr, mem, os::windows::ffi::OsStrExt, sync::Arc, thread, time::Duration};

#[derive(Debug, Clone, Serialize)]
pub struct OpenStaad {
    pub id: u32,
    pub path: PathBuf,
    pub file_name: Option<String>,
    pub root: Option<Arc<Root>>,
    pub command: Option<Arc<Command>>,
    pub design: Option<Arc<Design>>,
    pub geometry: Option<Arc<Geometry>>,
    pub load: Option<Arc<Load>>,
    pub output: Option<Arc<Output>>,
    pub property: Option<Arc<Property>>,
    pub support: Option<Arc<Support>>,
    #[serde(skip)]
    _com_context: Option<Arc<ComContext>>,
}

impl OpenStaad {
    /// Connects to OpenSTAAD and initializes the application.
    pub fn new(system_path: PathBuf, path: PathBuf, file_name: String) -> Result<Self> {
        let com_context = Arc::new(ComContext::new()?);
        let system_path_str = system_path
            .to_str()
            .ok_or(anyhow!("system_path is invalid: {:#?}", path))?
            .to_string();
        let std_path = path
            .join(file_name.clone())
            .to_str()
            .ok_or(anyhow!("system_path is invalid: {:#?}", path))?
            .to_string();

        let (id, dispatch) = initialize(system_path_str, std_path)?;
        let instance = Self {
            id,
            path,
            file_name: Some(file_name),
            root: Some(Arc::new(Root::new(dispatch))),
            command: None,
            design: None,
            geometry: None,
            load: None,
            output: None,
            property: None,
            support: None,
            _com_context: Some(com_context),
        };
        Ok(instance)
    }
    // pub fn new_by_activated() -> Result<Self> {
    //     let com_context = Arc::new(ComContext::new()?);
    //     let dispatch = get_active_object()?;
    //     let id_var = unsafe { invoke_method(&dispatch, "GetProcessId", &mut [])? };
    //     let id = unsafe { VariantToInt32(&id_var as *const VARIANT)? as u32 };

    //     let instance = Self {
    //         id,
    //         root: Some(Arc::new(Root::new(dispatch))),
    //         command: None,
    //         design: None,
    //         geometry: None,
    //         load: None,
    //         output: None,
    //         property: None,
    //         support: None,
    //         _com_context: Some(com_context),
    //     };
    //     Ok(instance)
    // }

    pub fn new_by_pid(pid: u32) -> Result<Self> {
        let com_context = ComContext::new()?;
        let dispatch = get_object_by_pid(pid)?;

        let mut mut_bstr = Box::new(BSTR::default());
        let bstr_ptr = mut_bstr.as_mut() as *mut BSTR;
        let bstr_var = variant_with_ptr_from::<BSTR>(bstr_ptr);
        let mut variants = vec![bstr_var, VARIANT::from(true)];
        let params: &mut [VARIANT] = &mut variants[..];
        let _ = unsafe { invoke_method(&dispatch, "GetSTAADFile", params)? };
        let full_path = variant_with_ptr_to::<BSTR>(&mut params[0]);
        let full_pathbuf = PathBuf::from(full_path);
        let path = full_pathbuf
            .parent()
            .ok_or(anyhow!(
                "Fail to full_pathbuf to parent: {:#?}",
                full_pathbuf
            ))?
            .to_path_buf();
        let file_name = full_pathbuf
            .file_name()
            .ok_or(anyhow!(
                "Fail to full_pathbuf to file_name: {:#?}",
                full_pathbuf
            ))?
            .to_str()
            .ok_or(anyhow!(
                "Fail to full_pathbuf str to file_name: {:#?}",
                full_pathbuf
            ))?
            .to_string();

        let instance = Self {
            id: pid,
            path,
            file_name: Some(file_name),
            root: Some(Arc::new(Root::new(dispatch))),
            command: None,
            design: None,
            geometry: None,
            load: None,
            output: None,
            property: None,
            support: None,
            _com_context: Some(Arc::new(com_context)),
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

fn initialize(exe_path: String, std_path: String) -> Result<(u32, IDispatch)> {
    // COM is already initialized by ComContext
    if !std::path::Path::new(&exe_path).exists() {
        bail!("파일이 존재하지 않습니다: {}", exe_path);
    }

    let pid = spawn_hidden_process(&exe_path, &std_path)?;
    info!("Started STAAD.Pro process with PID: {}", pid);

    info!("Waiting for STAAD.Pro process to initialize...");
    thread::sleep(Duration::from_millis(5000));

    let mut attempts = 0;
    let max_attempts = 5;
    loop {
        attempts += 1;

        // 먼저 ROT 방식 시도
        match find_staad_by_process_id(pid) {
            Ok(dispatch) => {
                info!("Success to connect Staad.Pro with pid {} via ROT", pid);
                let _ = waiting(&dispatch)?;
                return Ok((pid, dispatch));
            }
            Err(e) => {
                if attempts > max_attempts {
                    let _ = std::process::Command::new("taskkill")
                        .args(&["/PID", pid.to_string().as_str()])
                        .spawn()?;
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
    // COM is already initialized by ComContext
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
    // COM is already initialized by ComContext
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

/// Spawns a hidden process using Windows CreateProcessW API
fn spawn_hidden_process(exe_path: &str, std_path: &str) -> Result<u32> {
    // Build command line with arguments
    let command_line = format!("\"{}\" \"{}\" /s", exe_path, std_path);
    let mut command_line_wide: Vec<u16> = OsStr::new(&command_line)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    // Initialize STARTUPINFOW with hidden window settings
    let mut startup_info = STARTUPINFOW {
        cb: mem::size_of::<STARTUPINFOW>() as u32,
        dwFlags: STARTF_USESHOWWINDOW,
        ..unsafe { mem::zeroed() }
    };

    let mut process_info = unsafe { mem::zeroed::<PROCESS_INFORMATION>() };

    // Create process with no window
    unsafe {
        CreateProcessW(
            None,
            Some(PWSTR(command_line_wide.as_mut_ptr())),
            None,
            None,
            false,
            CREATE_NO_WINDOW,
            None,
            None,
            &startup_info,
            &mut process_info,
        )
        .context("Failed to create hidden process")?;

        let pid = process_info.dwProcessId;
        info!("Process created successfully. PID: {}", pid);

        // Verify process is running
        thread::sleep(Duration::from_millis(500));

        let mut exit_code = 0u32;
        if GetExitCodeProcess(process_info.hProcess, &mut exit_code).is_ok() {
            if exit_code == STILL_ACTIVE.0 as u32 {
                info!("Process {} is running successfully", pid);
            } else {
                warn!("Process {} exited with code: {}", pid, exit_code);
            }
        } else {
            warn!("Failed to check process status for PID: {}", pid);
        }

        // Hide all windows belonging to this process
        hide_process_windows(pid);

        // Clean up handles
        let _ = CloseHandle(process_info.hProcess);
        let _ = CloseHandle(process_info.hThread);

        Ok(pid)
    }
}

/// Hides all windows belonging to a specific process
fn hide_process_windows(target_pid: u32) {
    unsafe {
        let _ = EnumWindows(Some(enum_windows_callback), LPARAM(target_pid as isize));
    }
}

/// Callback function for EnumWindows to hide windows of target process
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let target_pid = lparam.0 as u32;
    let mut window_pid = 0u32;

    unsafe {
        GetWindowThreadProcessId(hwnd, Some(&mut window_pid));

        if window_pid == target_pid {
            ShowWindow(hwnd, SW_HIDE);
            info!("Hidden window for PID: {}", target_pid);
        }
    }

    BOOL::from(true) // Continue enumeration
}

// 리소스 정리
impl Drop for OpenStaad {
    fn drop(&mut self) {
        info!("Dropping OpenStaad instance with ID: {}", self.id);

        // Close the application gracefully via COM if possible
        if let Some(root) = &self.root {
            unsafe {
                let _ = invoke_method(&root.dispatch, "Quit", &mut []);
            }
        }

        // Wait a bit for graceful shutdown
        thread::sleep(Duration::from_millis(100));

        // Force terminate if still running
        if is_process_running(self.id) {
            warn!("Process {} still running, force terminating...", self.id);
            let _ = terminate_process(self.id);
        }

        // COM cleanup is handled automatically by ComContext's Drop implementation
        info!("OpenStaad instance dropped");
    }
}

/// Checks if a process is still running
fn is_process_running(pid: u32) -> bool {
    unsafe {
        if let Ok(handle) = OpenProcess(PROCESS_TERMINATE, false, pid) {
            let mut exit_code = 0u32;
            if GetExitCodeProcess(handle, &mut exit_code).is_ok() {
                let _ = CloseHandle(handle);
                return exit_code == STILL_ACTIVE.0 as u32;
            }
            let _ = CloseHandle(handle);
        }
        false
    }
}

/// Terminates a process forcefully
fn terminate_process(pid: u32) -> Result<()> {
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, false, pid)
            .context("Failed to open process for termination")?;

        TerminateProcess(handle, 1).context("Failed to terminate process")?;

        info!("Process {} terminated successfully", pid);
        let _ = CloseHandle(handle);
        Ok(())
    }
}
