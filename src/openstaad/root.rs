use serde::Serialize;
use std::collections::HashMap;
use windows::Win32::System::Com::IDispatch;

use crate::tools::value_types::{InType as itype, MethodSignature, OutType as otype};
use std::fmt::{Debug, Formatter, Result};

#[derive(Clone, Serialize)]
pub struct Root {
    pub id: String,
    #[serde(skip)]
    pub dispatch: IDispatch,
    #[serde(skip)]
    pub methods: HashMap<String, MethodSignature>,
}
impl Debug for Root {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "")
    }
}

impl Root {
    pub fn new(dispatch: IDispatch) -> Self {
        let mut methods = HashMap::new();
        let _ = set_methods(&mut methods);
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            dispatch,
            methods,
        }
    }
}

unsafe impl Send for Root {}
unsafe impl Sync for Root {}

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
            inputs: vec![itype::Int, itype::Int, itype::Int],
            outputs: vec![otype::Int],
        },
    );
    store.insert(
        "AnalyzeModel".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
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
            inputs: vec![itype::Str, itype::Int, itype::MutInt],
            outputs: vec![otype::Index(2)],
        },
    );
    store.insert(
        "GetAnalysisStatus".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::MutInt, itype::MutInt, itype::MutDouble],
            outputs: vec![
                otype::Int,
                otype::Index(1),
                otype::Index(2),
                otype::Index(3),
            ],
        },
    );
    store.insert(
        "GetApplicationVersion".to_string(),
        MethodSignature {
            inputs: vec![itype::MutInt, itype::MutInt, itype::MutInt, itype::MutInt],
            outputs: vec![
                otype::Str,
                otype::Index(0),
                otype::Index(1),
                otype::Index(2),
                otype::Index(3),
            ],
        },
    );
    store.insert(
        "GetBaseUnit".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    store.insert(
        "GetCONNECTEDProjectInfo".to_string(),
        MethodSignature {
            inputs: vec![itype::MutStr, itype::MutStr],
            outputs: vec![otype::Int, otype::Index(0), otype::Index(1)],
        },
    );
    store.insert(
        "GetErrorMessage".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Str],
        },
    );
    store.insert(
        "GetFullJobInfo".to_string(),
        MethodSignature {
            inputs: vec![
                itype::MutStr,
                itype::MutStr,
                itype::MutStr,
                itype::MutStr,
                itype::MutStr,
                itype::MutStr,
                itype::MutStr,
                itype::MutStr,
                itype::MutStr,
                itype::MutStr,
                itype::MutStr,
                itype::MutStr,
            ],
            outputs: vec![
                otype::Index(0),
                otype::Index(1),
                otype::Index(2),
                otype::Index(3),
                otype::Index(4),
                otype::Index(5),
                otype::Index(6),
                otype::Index(7),
                otype::Index(8),
                otype::Index(9),
                otype::Index(10),
                otype::Index(11),
            ],
        },
    );
    store.insert(
        "GetInputUnitForForce".to_string(),
        MethodSignature {
            inputs: vec![itype::MutStr],
            outputs: vec![otype::Int, otype::Index(0)],
        },
    );
    store.insert(
        "GetInputUnitForLength".to_string(),
        MethodSignature {
            inputs: vec![itype::MutStr],
            outputs: vec![otype::Int, otype::Index(0)],
        },
    );
    store.insert(
        "GetMainWindowHandle".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    store.insert(
        "GetProcessHandle".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    store.insert(
        "GetProcessId".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    store.insert(
        "GetShortJobInfo".to_string(),
        MethodSignature {
            inputs: vec![itype::MutStr, itype::MutStr, itype::MutStr],
            outputs: vec![otype::Index(0), otype::Index(1), otype::Index(2)],
        },
    );
    store.insert(
        "GetSTAADFile".to_string(),
        MethodSignature {
            inputs: vec![itype::MutStr, itype::Bool],
            outputs: vec![otype::Index(0)],
        },
    );
    store.insert(
        "GetSTAADFileFolder".to_string(),
        MethodSignature {
            inputs: vec![itype::MutStr],
            outputs: vec![otype::Index(0)],
        },
    );
    store.insert(
        "IsAnalyzing".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    store.insert(
        "IsPhysicalModel".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    store.insert(
        "ModifyNamedView".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Str,
                itype::Int,
                itype::Int,
                itype::Int,
                itype::Int,
                itype::MutInt,
            ],
            outputs: vec![otype::Index(5)],
        },
    );
    store.insert(
        "NewSTAADFile".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Int, itype::Int],
            outputs: vec![],
        },
    );
    store.insert(
        "OpenSTAADFile".to_string(),
        MethodSignature {
            inputs: vec![itype::Str],
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
            inputs: vec![itype::Str, itype::MutInt],
            outputs: vec![otype::Index(1)],
        },
    );
    store.insert(
        "SaveModel".to_string(),
        MethodSignature {
            inputs: vec![itype::Bool],
            outputs: vec![],
        },
    );
    store.insert(
        "SaveNamedView".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::MutInt],
            outputs: vec![otype::Index(1)],
        },
    );
    store.insert(
        "SetCONNECTEDProjectInfo".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str],
            outputs: vec![otype::Int],
        },
    );
    store.insert(
        "SetFullJobInfo".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Str,
                itype::Str,
                itype::Str,
                itype::Str,
                itype::Str,
                itype::Str,
                itype::Str,
                itype::Str,
                itype::Str,
                itype::Str,
                itype::Str,
                itype::Str,
            ],
            outputs: vec![],
        },
    );
    store.insert(
        "SetInputUnitForForce".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![],
        },
    );
    store.insert(
        "SetInputUnitForLength".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![],
        },
    );
    store.insert(
        "SetInputUnits".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Int],
            outputs: vec![],
        },
    );
    store.insert(
        "SetShortJobInfo".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str, itype::Str],
            outputs: vec![],
        },
    );
    store.insert(
        "SetSilentMode".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
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
