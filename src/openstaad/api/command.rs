use crate::openstaad::tools::invoke::{get_dispatch, invoke_method};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use windows::Win32::System::{Com::IDispatch, Variant::VARIANT};

#[derive(Debug)]
pub struct Command<'a> {
    pub staad: &'a IDispatch,
    pub dispatch: IDispatch,
}

impl<'a> Command<'a> {
    pub fn new(staad: &'a IDispatch) -> Self {
        let _command = unsafe { get_dispatch(staad, "Command", &mut []).unwrap() };
        Self {
            staad,
            dispatch: _command,
        }
    }

    pub async fn perform_analysis(&self, print_option: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(print_option)];
            let result_variant = invoke_method(&self.dispatch, "PerforAnalysis", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Geometry::get_node_coordinates: {}", e),
            }
        }
    }
}
