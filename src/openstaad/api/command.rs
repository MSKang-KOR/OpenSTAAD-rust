use crate::openstaad::tools::com::{get_dispatch, invoke_method};

use anyhow::{Error as anyErr, Ok as anyOk, Result, bail};
use serde::{Deserialize, Serialize};
use windows::Win32::System::{Com::IDispatch, Variant::VARIANT};

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    #[serde(skip)]
    pub dispatch: Option<IDispatch>,
    pub id: String,
}

impl Command {
    pub fn new(staad: Option<IDispatch>) -> Self {
        let _command =
            unsafe { get_dispatch(staad.as_ref().unwrap(), "Command", &mut []).unwrap() };
        Self {
            dispatch: Some(_command),
            id: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub fn perform_analysis(&self, print_option: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(print_option)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "PerformAnalysis",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::get_node_coordinates: {}", e),
            }
        }
    }
}

unsafe impl Send for Command {}
unsafe impl Sync for Command {}
