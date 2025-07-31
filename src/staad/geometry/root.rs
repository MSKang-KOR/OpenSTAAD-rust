use windows::Win32::System::Com::IDispatch;

use crate::staad::node::Node;

#[derive(Debug)]
pub struct Geometry {
    pub dispatch: Option<IDispatch>,
}

impl Geometry {
    pub fn new(dispatch: IDispatch) -> Self {
        Self {
            dispatch: Some(dispatch),
        }
    }

    pub fn node(&self) -> Node {
        Node::new(self)
    }
}
