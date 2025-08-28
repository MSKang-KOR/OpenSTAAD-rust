use anyhow::{Result, anyhow, bail};
use log::{info, warn};
use serde::Serialize;
use std::collections::HashMap;
use windows::Win32::System::Com::CoUninitialize;
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
use crate::openstaad::support::Support;
use crate::tools::invoke::{invoke_method, invoke_property};
use crate::tools::value_types::{
    InType as ptype, Input as tparam, MethodSignature, OutType as rtype,
};

fn initialize(_path: Option<String>) -> Result<IDispatch> {
    info!("Initializing COM library...");
    // let mut output = Command::new("cmd")
    //     .args(&[_path.as_ref().unwrap()])
    //     .spawn()?;
    // let _ = output.wait()?;

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
            bail!("Fail to cast IUnknown to IDispatch.")
        }
    }
}

/// A wrapper struct to manage the OpenSTAAD application instance.
/// It will handle cleanup (releasing COM objects) when it goes out of scope.
#[derive(Debug, Serialize)]
pub struct OpenStaad {
    #[serde(skip)]
    pub dispatch: IDispatch,
    pub id: u32,
    #[serde(skip)]
    pub methods: HashMap<String, MethodSignature>,
}

impl OpenStaad {
    /// Connects to OpenSTAAD and initializes the application.
    pub fn new(path: Option<String>) -> Result<Self> {
        let dispatch = initialize(path)?;
        // let std_path = "C:\\Users\\kms36\\Downloads\\staa_api_test\\Sample.STD".to_string();
        // let dispatch = initialize_background(path, std_path)?;
        let id = unsafe {
            match invoke_method(&dispatch, "GetProcessId", &mut []) {
                Ok(_var) => VariantToInt32(&_var as *const VARIANT)? as u32,
                Err(e) => bail!("GetProcessId failed: {}", e),
            }
        };
        let mut methods = HashMap::new();

        // Root OpenSTAAD API Methods (Alphabetical)
        methods.insert(
            "Analyze".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![],
            },
        );
        methods.insert(
            "AnalyzeEx".to_string(),
            MethodSignature {
                inputs: vec![ptype::Int, ptype::Int, ptype::Int],
                outputs: vec![rtype::Int],
            },
        );
        methods.insert(
            "AnalyzeModel".to_string(),
            MethodSignature {
                inputs: vec![ptype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "CloseSTAADFile".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![],
            },
        );
        methods.insert(
            "CreateNamedView".to_string(),
            MethodSignature {
                inputs: vec![ptype::Str, ptype::Int, ptype::MutInt],
                outputs: vec![rtype::Index(2)],
            },
        );
        methods.insert(
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
        methods.insert(
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
        methods.insert(
            "GetBaseUnit".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![rtype::Int],
            },
        );
        methods.insert(
            "GetCONNECTEDProjectInfo".to_string(),
            MethodSignature {
                inputs: vec![ptype::MutStr, ptype::MutStr],
                outputs: vec![rtype::Int, rtype::Index(0), rtype::Index(1)],
            },
        );
        methods.insert(
            "GetErrorMessage".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![rtype::Str],
            },
        );
        methods.insert(
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
        methods.insert(
            "GetInputUnitForForce".to_string(),
            MethodSignature {
                inputs: vec![ptype::MutStr],
                outputs: vec![rtype::Int, rtype::Index(0)],
            },
        );
        methods.insert(
            "GetInputUnitForLength".to_string(),
            MethodSignature {
                inputs: vec![ptype::MutStr],
                outputs: vec![rtype::Int, rtype::Index(0)],
            },
        );
        methods.insert(
            "GetMainWindowHandle".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![rtype::Int],
            },
        );
        methods.insert(
            "GetProcessHandle".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![rtype::Int],
            },
        );
        methods.insert(
            "GetProcessId".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![rtype::Int],
            },
        );
        methods.insert(
            "GetShortJobInfo".to_string(),
            MethodSignature {
                inputs: vec![ptype::MutStr, ptype::MutStr, ptype::MutStr],
                outputs: vec![rtype::Index(0), rtype::Index(1), rtype::Index(2)],
            },
        );
        methods.insert(
            "GetSTAADFile".to_string(),
            MethodSignature {
                inputs: vec![ptype::MutStr, ptype::Bool],
                outputs: vec![rtype::Index(0)],
            },
        );
        methods.insert(
            "GetSTAADFileFolder".to_string(),
            MethodSignature {
                inputs: vec![ptype::MutStr],
                outputs: vec![rtype::Index(0)],
            },
        );
        methods.insert(
            "IsAnalyzing".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![rtype::Int],
            },
        );
        methods.insert(
            "IsPhysicalModel".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![rtype::Int],
            },
        );
        methods.insert(
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
        methods.insert(
            "NewSTAADFile".to_string(),
            MethodSignature {
                inputs: vec![ptype::Str, ptype::Int, ptype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "OpenSTAADFile".to_string(),
            MethodSignature {
                inputs: vec![ptype::Str],
                outputs: vec![],
            },
        );
        methods.insert(
            "Quit".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![],
            },
        );
        methods.insert(
            "RemoveNamedView".to_string(),
            MethodSignature {
                inputs: vec![ptype::Str, ptype::MutInt],
                outputs: vec![rtype::Index(1)],
            },
        );
        methods.insert(
            "SaveModel".to_string(),
            MethodSignature {
                inputs: vec![ptype::Bool],
                outputs: vec![],
            },
        );
        methods.insert(
            "SaveNamedView".to_string(),
            MethodSignature {
                inputs: vec![ptype::Str, ptype::MutInt],
                outputs: vec![rtype::Index(1)],
            },
        );
        methods.insert(
            "SetCONNECTEDProjectInfo".to_string(),
            MethodSignature {
                inputs: vec![ptype::Str, ptype::Str],
                outputs: vec![rtype::Int],
            },
        );
        methods.insert(
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
        methods.insert(
            "SetInputUnitForForce".to_string(),
            MethodSignature {
                inputs: vec![ptype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "SetInputUnitForLength".to_string(),
            MethodSignature {
                inputs: vec![ptype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "SetInputUnits".to_string(),
            MethodSignature {
                inputs: vec![ptype::Int, ptype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "SetShortJobInfo".to_string(),
            MethodSignature {
                inputs: vec![ptype::Str, ptype::Str, ptype::Str],
                outputs: vec![],
            },
        );
        methods.insert(
            "SetSilentMode".to_string(),
            MethodSignature {
                inputs: vec![ptype::Int],
                outputs: vec![rtype::Int],
            },
        );
        methods.insert(
            "UpdateStructure".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![],
            },
        );
        let instance = Self {
            dispatch,
            id,
            methods,
        };
        Ok(instance)
    }

    pub fn get_command(&self) -> Result<Command> {
        info!("Accessing the 'Command' property...");
        let dispatch = unsafe { invoke_property(&self.dispatch, "Command")? };
        Ok(Command::new(dispatch))
    }
    pub fn get_design(&self) -> Result<Design> {
        info!("Accessing the 'Design' property...");
        let dispatch = unsafe { invoke_property(&self.dispatch, "Design")? };
        Ok(Design::new(dispatch))
    }
    pub fn get_geometry(&self) -> Result<Geometry> {
        info!("Accessing the 'Geometry' property...");
        let dispatch = unsafe { invoke_property(&self.dispatch, "Geometry")? };
        Ok(Geometry::new(dispatch))
    }
    pub fn get_load(&self) -> Result<Load> {
        info!("Accessing the 'Load' property...");
        let dispatch = unsafe { invoke_property(&self.dispatch, "Load")? };
        Ok(Load::new(dispatch))
    }
    pub fn get_output(&self) -> Result<Output> {
        info!("Accessing the 'Output' property...");
        let dispatch = unsafe { invoke_property(&self.dispatch, "Output")? };
        Ok(Output::new(dispatch))
    }
    pub fn get_property(&self) -> Result<Property> {
        info!("Accessing the 'Property' property...");
        let dispatch = unsafe { invoke_property(&self.dispatch, "Property")? };
        Ok(Property::new(dispatch))
    }
    pub fn get_support(&self) -> Result<Support> {
        info!("Accessing the 'Support' property...");
        let dispatch = unsafe { invoke_property(&self.dispatch, "Support")? };
        Ok(Support::new(dispatch))
    }
}

// 리소스 정리
impl Drop for OpenStaad {
    fn drop(&mut self) {
        println!("Drop OpenStaad instance.");
        unsafe {
            CoUninitialize();
        }
    }
}
