use std::collections::HashMap;

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use serde::{Deserialize, Serialize};
use windows::Win32::System::Com::IDispatch;

use crate::openstaad::{
    new_api::utils::{MethodSignature, ParamType as ptype, ResultType as rtype},
    tools::com::invoke_method,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    pub id: String,
    #[serde(skip)]
    pub staad: Option<IDispatch>,
    #[serde(skip)]
    pub dispatch: Option<IDispatch>,
    #[serde(skip)]
    pub methods: HashMap<String, MethodSignature>,
}

impl Root {
    pub fn new(staad: Option<IDispatch>) -> Self {
        let mut methods = HashMap::new();
        methods.insert(
            "GetProcessId".to_string(),
            MethodSignature {
                arguments: vec![],
                indice: vec![],
                result: vec![rtype::Integer],
                is_scaled: true,
            },
        );
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            staad: staad.clone(),
            dispatch: staad,
            methods,
        }
    }
}

// unsafe impl Send for Root{}
// unsafe impl Sync for Root{}
