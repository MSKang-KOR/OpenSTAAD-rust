use serde::Serialize;
use std::collections::HashMap;
use windows::Win32::System::Com::IDispatch;

use crate::tools::value_types::{InType as itype, MethodSignature, OutType as otype};

use std::fmt::{Debug, Formatter, Result};

#[derive(Clone, Serialize)]
pub struct Design {
    pub id: String,
    #[serde(skip)]
    pub dispatch: IDispatch,
    #[serde(skip)]
    pub methods: HashMap<String, MethodSignature>,
}
impl Debug for Design {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Design Struct")
    }
}

impl Design {
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

unsafe impl Send for Design {}
unsafe impl Sync for Design {}

fn set_methods(methods: &mut HashMap<String, MethodSignature>) {
    // Design: Steel
    methods.insert(
        "AssignDesignCommand".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::Str, itype::VecInt],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AssignDesignGroup".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Str,
                itype::Int,
                itype::VecInt,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AssignDesignParameter".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::Str, itype::VecInt],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateDesignBrief".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetDesignBriefCode".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetMemberDesignParameters".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Int, itype::MemberSteelDgnParams],
            outputs: vec![otype::Int],
        },
    );
}
