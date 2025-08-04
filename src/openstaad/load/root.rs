use windows::Win32::System::Com::IDispatch;

use crate::openstaad::{
    definition::Definition, element_property::ElementProperty, load_case_detail::LoadCaseDetail,
    load_envelopes::LoadEnvelopes, material::Material, root::Root, section::Section,
    specification::Specification, standard_section::StandardSection, tools::invoke::get_dispatch,
};

#[derive(Debug)]
pub struct Load<'a> {
    pub root: &'a Root,
    pub dispatch: IDispatch,
}

impl<'a> Load<'a> {
    pub fn new(root: &'a Root) -> Self {
        let dispatch =
            unsafe { get_dispatch(root.dispatch.as_ref().unwrap(), "Load", &mut []).unwrap() };
        Self { root, dispatch }
    }

    pub fn definition(&self) -> Definition {
        Definition::new(self)
    }
    pub fn load_case_detail(&self) -> LoadCaseDetail {
        LoadCaseDetail::new(self)
    }
    pub fn load_envelopes(&self) -> LoadEnvelopes {
        LoadEnvelopes::new(self)
    }
}
