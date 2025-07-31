use crate::staad::{
    command::root::Command, utils::invoke_method_with_result, variant::variant_from_raw_pointer,
};
use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use windows::Win32::System::Variant::VARIANT;

#[derive(Debug)]
pub struct Analysis<'a> {
    pub command: &'a Command,
}

impl<'a> Analysis<'a> {
    pub fn new(command: &'a Command) -> Self {
        Self { command }
    }

    pub async fn perform_analysis(&self, print_option: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(print_option)];
            let result_variant = invoke_method_with_result(
                self.command.dispatch.as_ref().unwrap(),
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
