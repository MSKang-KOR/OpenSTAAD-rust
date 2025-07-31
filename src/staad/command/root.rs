use windows::Win32::System::Com::IDispatch;

use crate::staad::{analysis::Analysis, node::Node};

#[derive(Debug)]
pub struct Command {
    pub dispatch: Option<IDispatch>,
}

impl Command {
    pub fn new(dispatch: IDispatch) -> Self {
        Self {
            dispatch: Some(dispatch),
        }
    }

    pub fn analysis(&self) -> Analysis {
        Analysis::new(self)
    }
}
