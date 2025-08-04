use windows::Win32::System::{Com::IDispatch, Variant::VARIANT};

use crate::openstaad::{root::Root, tools::invoke::{get_dispatch, invoke_method}};
use anyhow::{Error as anyErr, Ok as anyOk, Result, bail};

#[derive(Debug)]
pub struct Command<'a> {
    pub root: &'a Root,
    pub dispatch: IDispatch,
}

impl<'a> Command<'a> {
    pub fn new(root: &'a Root) -> Self {
        let dispatch =
            unsafe { get_dispatch(root.dispatch.as_ref().unwrap(), "Command", &mut []).unwrap() };
        Self { root, dispatch }
    }

    pub async fn perform_analysis(&self, print_option: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(print_option)];
            let result_variant = invoke_method(
                &self.dispatch,
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
