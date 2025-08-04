use windows::Win32::System::Com::IDispatch;

use crate::openstaad::{
    beam::Beam, group::Group, node::Node, root::Root, tools::invoke::get_dispatch,
};

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
}
