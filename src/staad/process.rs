use std::time::Duration;

use anyhow::{Context, Ok as anyOk, Result, anyhow};
use windows::{
    Win32::System::{
        Com::{
            CLSIDFromProgID, COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize, IDispatch,
        },
        Ole::GetActiveObject,
    },
    core::{GUID, HSTRING, IUnknown, Interface, PCWSTR},
};

use crate::staad::root::Root;

// STAAD 백그라운드 실행 및 제어 클래스
pub struct StaadProcess {
    // pub root: Option<IDispatch>,
    pub root: Option<Root>,
    pub staad_path: String,
}

impl StaadProcess {
    pub fn new(staad_path: &str) -> Self {
        StaadProcess {
            root: None,
            staad_path: staad_path.to_string(),
        }
    }

    // STAAD.exe를 백그라운드에서 실행
    pub async fn start(&mut self) -> Result<()> {
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

        // COM 연결 시도 (최대 30초 대기)
        let mut attempts = 0;
        let max_attempts = 3;

        while attempts < max_attempts {
            match self.connect_to_staad().await {
                Ok(_) => {
                    println!("STAAD.exe와 COM 연결 성공!");
                    return anyOk(());
                }
                Err(e) => {
                    if attempts == max_attempts - 1 {
                        return Err(anyhow!("STAAD.exe COM 연결 실패: {}", e));
                    }
                    attempts += 1;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }

        anyOk(())
    }

    // STAAD COM 객체에 연결
    async fn connect_to_staad(&mut self) -> Result<()> {
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
                        let root_dispatch = _ppunk.cast::<IDispatch>();
                        if let Ok(v) = root_dispatch {
                            self.root = Some(Root::new(&v));
                        };
                    }
                }
            };

            // // 하위 객체들 초기화
            // self.initialize_sub_objects()?;

            anyOk(())
        }
    }
}

// 리소스 정리
impl Drop for StaadProcess {
    fn drop(&mut self) {
        println!("STAAD 백그라운드 프로세스를 종료합니다...");

        // // STAAD 프로세스 종료
        // if let Some(mut process) = self.staad_process.take() {
        //     let _ = process.kill();
        //     let _ = process.wait();
        // }

        // COM 정리
        unsafe {
            CoUninitialize();
        }
    }
}
