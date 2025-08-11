use crate::openstaad::tools::com::{get_dispatch, invoke_method};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use serde::{Deserialize, Serialize};
use windows::Win32::System::{Com::IDispatch, Variant::VARIANT};

#[derive(Debug, Serialize, Deserialize)]
pub struct Command<'a> {
    #[serde(skip)]
    pub staad: Option<&'a IDispatch>,
    #[serde(skip)]
    pub dispatch: Option<IDispatch>,
}

impl<'a> Command<'a> {
    pub fn new(staad: Option<&'a IDispatch>) -> Self {
        let _command = unsafe { get_dispatch(staad.unwrap(), "Command", &mut []).unwrap() };
        Self {
            staad,
            dispatch: Some(_command),
        }
    }

    pub async fn perform_analysis(&self, print_option: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(print_option)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "PerforAnalysis",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::get_node_coordinates: {}", e),
            }
        }
    }
}

unsafe impl<'a> Send for Command<'a> {}
unsafe impl<'a> Sync for Command<'a> {}
