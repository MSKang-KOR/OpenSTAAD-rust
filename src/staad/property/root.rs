use windows::Win32::System::Com::IDispatch;

use crate::staad::{
    element_property::ElementProperty, material::Material, root::Root, section::Section,
    specification::Specification, standard_section::StandardSection, utils::get_dispatch,
};

#[derive(Debug)]
pub struct Property<'a> {
    pub root: &'a Root,
    pub dispatch: IDispatch,
}

impl<'a> Property<'a> {
    pub fn new(root: &'a Root) -> Self {
        let dispatch =
            unsafe { get_dispatch(root.dispatch.as_ref().unwrap(), "Property", &mut []).unwrap() };
        Self { root, dispatch }
    }

    pub fn section(&self) -> Section {
        Section::new(self)
    }
    pub fn element_property(&self) -> ElementProperty {
        ElementProperty::new(self)
    }
    pub fn material(&self) -> Material {
        Material::new(self)
    }
    pub fn specification(&self) -> Specification {
        Specification::new(self)
    }
    pub fn standard_section(&self) -> StandardSection {
        StandardSection::new(self)
    }
}
