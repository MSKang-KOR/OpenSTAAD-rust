use std::{mem::ManuallyDrop, ptr::null_mut, time::Duration};

use anyhow::{Context, Error, Ok as anyOk, Result, anyhow, bail};
use windows::{
    Win32::System::{
        Com::{
            CLSIDFromProgID, COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize,
            DISPATCH_METHOD, DISPATCH_PROPERTYGET, DISPPARAMS, EXCEPINFO, IDispatch,
        },
        Ole::GetActiveObject,
        Variant::VARIANT,
    },
    core::{GUID, HSTRING, IUnknown, Interface, PCWSTR},
};

// #[implement(IDispatch)]
// pub struct Dispatch {}

// impl IDispatch_Impl for Dispatch_Impl {
//     fn GetnGeomtry(&self) -> () {}
// }

// STAAD 백그라운드 실행 및 제어 클래스
pub struct StaadProcess {
    // staad_process: Option<Child>,
    pub root: Option<IDispatch>,
    pub geometry: Option<IDispatch>,
    // analysis: Option<IDispatch>,
    // post: Option<IDispatch>,
    // property: Option<IDispatch>,
    // load: Option<IDispatch>,
    pub staad_path: String,
}

impl StaadProcess {
    pub fn new(staad_path: &str) -> Self {
        StaadProcess {
            // staad_process: None,
            root: None,
            geometry: None,
            // analysis: None,
            // post: None,
            // property: None,
            // load: None,
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
                        let root = _ppunk.cast::<IDispatch>().unwrap();

                        let empty_params: &mut [VARIANT; 0] = &mut [];
                        if let Ok(v) = get_dispatch(&root, "Geometry", empty_params) {
                            self.geometry = Some(v);
                        };

                        self.root = Some(root);
                    }
                }
            };

            // // 하위 객체들 초기화
            // self.initialize_sub_objects()?;

            anyOk(())
        }
    }

    // pub fn test_code(&mut self) -> Result<()> {
    //     let app = self
    //         .root
    //         .as_ref()
    //         .context("STAAD 애플리케이션이 초기화되지 않았습니다")?;
    //     let params = [];
    //     match unsafe { invoke_method_on_object(app, "Geometry", &params) } {
    //         Ok(v) => {
    //             if let Ok(dispatch_obj) = IDispatch::try_from(&v) {
    //                 let params = [];
    //                 match unsafe {
    //                     invoke_method_on_object(&dispatch_obj, "GetLastNodeNo", &params)
    //                 } {
    //                     Ok(v) => {
    //                         let a = unsafe {
    //                             VariantToStringAlloc(&v as *const VARIANT)
    //                                 .unwrap()
    //                                 .to_string()
    //                         };
    //                         println!("{:#?}", a);
    //                     }
    //                     _ => {}
    //                 }
    //             };
    //         }
    //         Err(e) => {
    //             println!("{:#?}", e);
    //         }
    //     };
    //     anyOk(())
    // }
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

pub unsafe fn invoke_method_with_result(
    object: &IDispatch,
    method_name: &str,
    params: &mut [VARIANT],
) -> Result<VARIANT> {
    let name = &PCWSTR::from_raw(HSTRING::from(method_name).as_ptr()) as *const PCWSTR;
    let mut dispid = 0;
    let iid = &GUID::default() as *const GUID;

    unsafe {
        let _ = object
            .GetIDsOfNames(iid, name, 1, 0, &mut dispid)
            .context("Method ID 가져오기 실패");
    };

    let mut result = VARIANT::default();
    let mut excepinfo = EXCEPINFO::default();

    let dispparams = DISPPARAMS {
        rgvarg: params.as_mut_ptr(),
        cArgs: params.len() as u32,
        cNamedArgs: 0,
        rgdispidNamedArgs: null_mut(),
        // ..Default::default()
    };

    unsafe {
        let hresult = object
            .Invoke(
                dispid,
                iid,
                0,
                DISPATCH_METHOD | DISPATCH_PROPERTYGET,
                &dispparams,
                Some(&mut result),
                Some(&mut excepinfo),
                None,
            )
            .context("Method 실행 실패");
    };
    anyOk(result)
}

pub unsafe fn invoke_method_without_result(
    object: &IDispatch,
    method_name: &str,
    params: &mut [VARIANT],
) -> Result<()> {
    let name = &PCWSTR::from_raw(HSTRING::from(method_name).as_ptr()) as *const PCWSTR;
    let mut dispid = 0;
    let iid = &GUID::default() as *const GUID;

    unsafe {
        let _ = object
            .GetIDsOfNames(iid, name, 1, 0, &mut dispid)
            .context("Method ID 가져오기 실패");
    };

    let mut excepinfo = EXCEPINFO::default();

    let dispparams = DISPPARAMS {
        rgvarg: params.as_mut_ptr(),
        cArgs: params.len() as u32,
        cNamedArgs: 0,
        rgdispidNamedArgs: null_mut(),
        // ..Default::default()
    };
    unsafe {
        let _ = object
            .Invoke(
                dispid,
                iid,
                0,
                DISPATCH_METHOD,
                &dispparams,
                None,
                Some(&mut excepinfo),
                None,
            )
            .context("Method 실행 실패");
    };
    println!("asdasdf");

    anyOk(())
}

pub unsafe fn get_dispatch(
    object: &IDispatch,
    method_name: &str,
    params: &mut [VARIANT],
) -> Result<IDispatch, Error> {
    let result: Result<VARIANT, Error> =
        unsafe { invoke_method_with_result(object, method_name, params) };
    match result {
        Ok(var) => {
            match IDispatch::try_from(&var) {
                Ok(dispatch) => Ok(dispatch), // 성공 시 Ok로 감싸서 반환
                Err(_) => bail!("VARIANT을 IDispatch로 변환할 수 없습니다"),
            }
        }
        Err(e) => bail!("메서드 호출 실패: {}", e),
    }
}
