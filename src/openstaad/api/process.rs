use anyhow::{Context, Ok as anyOk, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use windows::{
    Win32::{
        Foundation::CloseHandle,
        System::{
            Com::{
                CLSIDFromProgID, COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize,
                GetRunningObjectTable, IDispatch, IMoniker,
            },
            Ole::GetActiveObject,
            Threading::{
                GetProcessId, OpenProcess, PROCESS_QUERY_INFORMATION, WaitForSingleObject,
            },
            Variant::{VARIANT, VariantToInt32},
        },
    },
    core::{GUID, HSTRING, IUnknown, Interface, PCWSTR},
};

use crate::openstaad::{
    api::{
        command::Command, design::Design, geometry::Geometry, load::Load, output::Output,
        property::Property, root::Root, support::Support,
    },
    tools::com::invoke_method,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct StaadProcess {
    pub path: String,
    pub pid: Option<u32>,
    #[serde(skip)]
    pub staad: Option<IDispatch>,
    pub _root: Option<Arc<Root>>,
    pub _geometry: Option<Arc<Geometry>>,
    pub _property: Option<Arc<Property>>,
    pub _support: Option<Arc<Support>>,
    pub _load: Option<Arc<Load>>,
    pub _design: Option<Arc<Design>>,
    pub _output: Option<Arc<Output>>,
    pub _command: Option<Arc<Command>>,
}

impl StaadProcess {
    pub fn new(staad_path: &str) -> Self {
        StaadProcess {
            staad: None,
            pid: None,
            path: staad_path.to_string(),
            _root: None,
            _geometry: None,
            _property: None,
            _support: None,
            _load: None,
            _design: None,
            _output: None,
            _command: None,
        }
    }

    pub fn root(&mut self) -> Arc<Root> {
        if let Some(root) = &self._root {
            return root.clone();
        }
        let root = Root::new(self.staad.clone());
        let arc_root = Arc::new(root);
        self._root = Some(arc_root.clone());
        arc_root
    }
    pub fn geometry(&mut self) -> Arc<Geometry> {
        if let Some(geometry) = &self._geometry {
            return geometry.clone();
        }
        let geometry = Geometry::new(self.staad.clone());
        let arc_geometry = Arc::new(geometry);
        self._geometry = Some(arc_geometry.clone());
        arc_geometry
    }
    pub fn property(&mut self) -> Arc<Property> {
        if let Some(property) = &self._property {
            return property.clone();
        }
        let property = Property::new(self.staad.clone());
        let arc_property = Arc::new(property);
        self._property = Some(arc_property.clone());
        arc_property
    }
    pub fn support(&mut self) -> Arc<Support> {
        if let Some(support) = &self._support {
            return support.clone();
        }
        let support = Support::new(self.staad.clone());
        let arc_support = Arc::new(support);
        self._support = Some(arc_support.clone());
        arc_support
    }
    pub fn load(&mut self) -> Arc<Load> {
        if let Some(load) = &self._load {
            return load.clone();
        }
        let load = Load::new(self.staad.clone());
        let arc_load = Arc::new(load);
        self._load = Some(arc_load.clone());
        arc_load
    }
    pub fn design(&mut self) -> Arc<Design> {
        if let Some(design) = &self._design {
            return design.clone();
        }
        let design = Design::new(self.staad.clone());
        let arc_design = Arc::new(design);
        self._design = Some(arc_design.clone());
        arc_design
    }
    pub fn output(&mut self) -> Arc<Output> {
        if let Some(output) = &self._output {
            return output.clone();
        }
        let output = Output::new(self.staad.clone());
        let arc_output = Arc::new(output);
        self._output = Some(arc_output.clone());
        arc_output
    }
    pub fn command(&mut self) -> Arc<Command> {
        if let Some(command) = &self._command {
            return command.clone();
        }
        let command = Command::new(self.staad.clone());
        let arc_command = Arc::new(command);
        self._command = Some(arc_command.clone());
        arc_command
    }

    // 프로세스가 여전히 살아있는지 확인하는 메서드
    pub fn is_alive(&self) -> bool {
        if let Some(pid) = self.pid {
            unsafe {
                match OpenProcess(PROCESS_QUERY_INFORMATION, false, pid) {
                    Ok(handle) => {
                        let _ = CloseHandle(handle);
                        true
                    }
                    Err(_) => false,
                }
            }
        } else {
            // PID가 없으면 프로세스 상태를 확인할 수 없음
            false
        }
    }

    pub fn get_active_object(&self) -> Option<IDispatch> {
        unsafe {
            // COM 초기화
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            // OpenSTAAD 애플리케이션 객체 생성
            let clsid_str = PCWSTR::from_raw(HSTRING::from("StaadPro.OpenSTAAD").as_ptr());
            let clsid = CLSIDFromProgID(clsid_str).ok()?;

            let pv_reserved: Option<*mut core::ffi::c_void> = None;
            let mut ppunk: Option<IUnknown> = None;
            let _ = GetActiveObject(
                &clsid as *const GUID,
                pv_reserved,
                &mut ppunk as *mut Option<IUnknown>,
            );

            if let Some(punk) = ppunk {
                match punk.cast::<IDispatch>() {
                    Ok(staad) => Some(staad),
                    Err(_) => None,
                }
            } else {
                None
            }
        }
    }

    // STAAD.exe를 백그라운드에서 실행
    pub fn start(&mut self) -> Result<()> {
        // println!("STAAD.exe를 백그라운드에서 시작합니다...");

        // // STAAD.exe 경로 확인
        // if !Path::new(&self.staad_path).exists() {
        //     return Err(anyhow!("STAAD.exe를 찾을 수 없습니다: {}", self.staad_path));
        // }

        // // 백그라운드에서 STAAD.exe 실행
        // let child = Command::new(&self.staad_path)
        //     .args(&["/minimize", "/nologo", "/automation"]) // 최소화, 로고 없음, 자동화 모드
        //     .spawn()
        //     .context("STAAD.exe 실행 실패")?;

        // self.staad_process = Some(child);

        // // STAAD가 완전히 로드될 때까지 대기
        // println!("STAAD.exe 로딩 대기 중...");
        // tokio::time::sleep(Duration::from_secs(10)).await;

        // // COM 연결 시도 (최대 30초 대기)
        // let mut attempts = 0;
        // let max_attempts = 3;

        // while attempts < max_attempts {
        //     match self.connect_to_staad().await {
        //         Ok(_) => {
        //             println!("STAAD.exe와 COM 연결 성공!");
        //             return anyOk(());
        //         }
        //         Err(e) => {
        //             if attempts == max_attempts - 1 {
        //                 return Err(anyhow!("STAAD.exe COM 연결 실패: {}", e));
        //             }
        //             attempts += 1;
        //             tokio::time::sleep(Duration::from_secs(1)).await;
        //         }
        //     }
        // }
        match self.connect_to_staad() {
            Ok(_) => {
                println!("STAAD.exe와 COM 연결 성공!");
                let root = self.root();
                let pid = root.get_process_id()?;
                self.pid = Some(pid as u32);
                return anyOk(());
            }
            Err(e) => return Err(anyhow!("STAAD.exe COM 연결 실패: {}", e)),
        }
    }

    // Process ID를 통해 특정 STAAD.pro 프로세스와 연결
    pub fn start_with_pid(&mut self, pid: u32) -> Result<()> {
        match self.connect_to_staad_by_pid(pid) {
            Ok(_) => {
                println!("Process ID {}의 STAAD.exe와 COM 연결 성공!", pid);
                self.pid = Some(pid);
                return anyOk(());
            }
            Err(e) => {
                bail!("Process ID {}의 STAAD.exe COM 연결 실패: {}", pid, e);
            }
        }
    }

    // STAAD COM 객체에 연결
    fn connect_to_staad(&mut self) -> Result<()> {
        unsafe {
            // COM 초기화
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            // OpenSTAAD 애플리케이션 객체 생성
            let clsid_str = PCWSTR::from_raw(HSTRING::from("StaadPro.OpenSTAAD").as_ptr());
            let clsid = CLSIDFromProgID(clsid_str).context("CLSID 생성 실패")?;

            // let staad_app: IDispatch = CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER)
            //     .context("OpenSTAAD 인스턴스 생성 실패")?;

            // self.staad_app = Some(staad_app);
            let pv_reserved: Option<*mut core::ffi::c_void> = None;
            let mut ppunk: Option<IUnknown> = None;
            match GetActiveObject(
                &clsid as *const GUID,
                pv_reserved,
                &mut ppunk as *mut Option<IUnknown>,
            ) {
                Err(e) => {
                    println!("{:#?}", e);
                }
                _ => {
                    if let Some(_ppunk) = ppunk {
                        let staad_dispatch = _ppunk.cast::<IDispatch>();
                        if let Ok(_staad) = staad_dispatch {
                            self.staad = Some(_staad);
                        };
                    }
                }
            };
            anyOk(())
        }
    }

    // Process ID를 통해 특정 STAAD.pro 프로세스와 연결
    fn connect_to_staad_by_pid(&mut self, process_id: u32) -> Result<()> {
        unsafe {
            // COM 초기화
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            println!("Process ID {}의 STAAD.pro와 연결 시도...", process_id);

            // ROT를 통해 특정 프로세스 ID의 STAAD 인스턴스 찾기
            match self.find_staad_by_process_id(process_id) {
                Ok(staad_dispatch) => {
                    self.staad = Some(staad_dispatch);
                    println!("Process ID {}의 STAAD.pro와 연결 성공!", process_id);
                    anyOk(())
                }
                Err(e) => {
                    // ROT에서 찾지 못한 경우, 기존 GetActiveObject 방식으로 폴백
                    // self.connect_to_staad_fallback()
                    bail!("ROT에서 찾지 못함, GetActiveObject로 폴백 시도: {}", e);
                }
            }
        }
    }

    // ROT를 통해 특정 프로세스 ID의 STAAD 인스턴스를 찾는 헬퍼 함수
    fn find_staad_by_process_id(&self, target_pid: u32) -> Result<IDispatch> {
        unsafe {
            // Running Object Table 가져오기
            let rot = GetRunningObjectTable(0).context("ROT 가져오기 실패")?;

            // ROT의 모든 객체를 열거
            let enum_moniker = rot.EnumRunning().context("ROT 열거 실패")?;
            println!("ROT 열거 성공 {:#?}", target_pid);
            loop {
                let mut monikers: [Option<IMoniker>; 1] = [None];
                let mut fetched = 0u32;

                // 다음 moniker 가져오기
                let hr = enum_moniker.Next(&mut monikers, Some(&mut fetched));
                if hr.is_err() || fetched == 0 {
                    break;
                }

                if let Some(moniker) = &monikers[0] {
                    // moniker로부터 객체 바인드
                    if let Ok(unknown) = rot.GetObject(moniker) {
                        // IDispatch로 캐스트 시도
                        if let Ok(dispatch) = unknown.cast::<IDispatch>() {
                            // 이 객체가 STAAD인지 확인하고 프로세스 ID 매칭 확인
                            if self.is_staad_object_with_pid(&dispatch, target_pid)? {
                                return Ok(dispatch);
                            }
                        }
                    }
                }
            }

            Err(anyhow!(
                "프로세스 ID {}에 해당하는 STAAD 인스턴스를 찾을 수 없습니다",
                target_pid
            ))
        }
    }

    // 주어진 IDispatch 객체가 특정 프로세스 ID의 STAAD 객체인지 확인
    fn is_staad_object_with_pid(&self, _dispatch: &IDispatch, _target_pid: u32) -> Result<bool> {
        unsafe {
            let current_pid_result = invoke_method(_dispatch, "GetProcessId", &mut []);
            match current_pid_result {
                Ok(var) => {
                    if let Ok(cur_pid) = VariantToInt32(&var as *const VARIANT) {
                        return Ok(cur_pid as u32 == _target_pid);
                    } else {
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

    // GetActiveObject를 사용한 폴백 연결 방식
    fn connect_to_staad_fallback(&mut self) -> Result<()> {
        unsafe {
            let clsid_str = PCWSTR::from_raw(HSTRING::from("StaadPro.OpenSTAAD").as_ptr());
            let clsid = CLSIDFromProgID(clsid_str).context("CLSID 생성 실패")?;

            let pv_reserved: Option<*mut core::ffi::c_void> = None;
            let mut ppunk: Option<IUnknown> = None;

            match GetActiveObject(
                &clsid as *const GUID,
                pv_reserved,
                &mut ppunk as *mut Option<IUnknown>,
            ) {
                Err(e) => {
                    return Err(anyhow!("GetActiveObject 실패: {:#?}", e));
                }
                _ => {
                    if let Some(_ppunk) = ppunk {
                        let staad_dispatch = _ppunk.cast::<IDispatch>();
                        if let Ok(_staad) = staad_dispatch {
                            self.staad = Some(_staad);
                            println!(
                                "GetActiveObject를 통한 STAAD.pro 연결 성공 (프로세스 ID 특정 불가)"
                            );
                        } else {
                            return Err(anyhow!("IDispatch 캐스팅 실패"));
                        }
                    } else {
                        return Err(anyhow!("활성 STAAD 객체를 찾을 수 없습니다"));
                    }
                }
            };
            anyOk(())
        }
    }
}

// 리소스 정리
impl Drop for StaadProcess {
    fn drop(&mut self) {
        println!("STAAD 백그라운드 프로세스를 종료합니다...");
        // COM 정리
        unsafe {
            CoUninitialize();
        }
    }
}

unsafe impl Send for StaadProcess {}
unsafe impl Sync for StaadProcess {}
