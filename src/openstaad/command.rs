use serde::Serialize;
use std::collections::HashMap;
use windows::Win32::System::Com::IDispatch;

use crate::tools::value_types::{InType as itype, MethodSignature, OutType as otype};
use std::fmt::{Debug, Formatter, Result};

#[derive(Clone, Serialize)]
pub struct Command {
    pub id: String,
    #[serde(skip)]
    pub dispatch: IDispatch,
    #[serde(skip)]
    pub methods: HashMap<String, MethodSignature>,
}
impl Debug for Command {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "")
    }
}

impl Command {
    pub fn new(dispatch: IDispatch) -> Self {
        let mut methods = HashMap::new();

        // Command::Analysis Commands
        methods.insert(
            "DeleteAllAnalysisCommands".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "DeleteCheckIrregularitiesCommand".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "DeleteCheckSoftStoryCommand".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "DeleteFloorDiaphragmBaseCommand".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "PerformAnalysis".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "PerformBucklingAnalysis".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "PerformBucklingAnalysisEx".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "PerformCableAnalysis".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "PerformCableAnalysisEx".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::VecInt, itype::VecDouble, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "PerformDirectAnalysis".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::VecDouble, itype::VecInt, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "PerformNonlinearAnalysisEx".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::VecDouble,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "PerformPDeltaAnalysisEx".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::Int, itype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "PerformPDeltaAnalysisNoConverge".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "SetCheckIrregularitiesCommand".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "SetCheckSoftStoryCommand".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "SetFloorDiaphragmBaseCommand".to_string(),
            MethodSignature {
                inputs: vec![itype::Double],
                outputs: vec![otype::Int],
            },
        );
        // Command::Steel Design Commands
        methods.insert(
            "CreateSteelDesignCommand".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::VecInt,
                    itype::VecDouble,
                    itype::VecStr,
                    itype::VecInt,
                ],
                outputs: vec![],
            },
        );

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            dispatch,
            methods,
        }
    }
}

unsafe impl Send for Command {}
unsafe impl Sync for Command {}
