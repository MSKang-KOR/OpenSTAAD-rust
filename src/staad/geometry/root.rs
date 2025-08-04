use windows::Win32::System::Com::IDispatch;

use crate::staad::{beam::Beam, group::Group, node::Node, root::Root, utils::get_dispatch};

#[derive(Debug)]
pub struct Geometry<'a> {
    pub root: &'a Root,
    pub dispatch: IDispatch,
}

impl<'a> Geometry<'a> {
    pub fn new(root: &'a Root) -> Self {
        let dispatch =
            unsafe { get_dispatch(root.dispatch.as_ref().unwrap(), "Geometry", &mut []).unwrap() };
        Self { root, dispatch }
    }

    pub fn node(&self) -> Node {
        Node::new(self)
    }

    pub fn beam(&self) -> Beam {
        Beam::new(self)
    }

    pub fn group(&self) -> Group {
        Group::new(self)
    }
}
