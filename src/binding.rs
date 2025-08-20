#![allow(non_camel_case_types, non_snake_case, dead_code)]
//! Contains the manually translated COM interface definitions from `OpenStaadUI.IDL`.
//! This file is crucial for enabling the `windows-rs` crate to understand and
//! interact with the custom OpenSTAAD COM objects. Each interface is defined
//! as a Rust trait with the `#[interface]` attribute, which
//! specifies its unique GUID.

use windows::Win32::Foundation::VARIANT_BOOL;
use windows::Win32::System::Com::{IDispatch, IDispatch_Impl, IDispatch_Vtbl};
use windows::Win32::System::Variant::VARIANT;
use windows::core::Result;
use windows_core::{BSTR, HRESULT, IUnknown, interface};

/// Main OpenSTAAD UI interface for controlling the application
#[interface("3F5B8055-31C6-446E-8BED-FEE43E09D4CC")]
pub unsafe trait IOpenSTAADUI: IDispatch {
    /// Access to geometry operations
    pub unsafe fn get_Geometry(&self, geometry: *mut *mut IOSGeometryUI) -> HRESULT;

    /// User control property (getter)
    pub unsafe fn get_UserControl(&self, user_control: *mut VARIANT_BOOL) -> HRESULT;

    /// User control property (setter)
    pub unsafe fn put_UserControl(&self, user_control: VARIANT_BOOL) -> HRESULT;

    /// Window interface access
    pub unsafe fn get_Window(&self, window: *mut *mut IDispatch) -> HRESULT;

    /// View interface access
    pub unsafe fn get_View(&self, view: *mut *mut IDispatch) -> HRESULT;

    /// Output interface access
    pub unsafe fn get_Output(&self, output: *mut *mut IDispatch) -> HRESULT;

    /// Property interface access
    pub unsafe fn get_Property(&self, property: *mut *mut IDispatch) -> HRESULT;

    /// Load interface access
    pub unsafe fn get_Load(&self, load: *mut *mut IDispatch) -> HRESULT;

    /// Table interface access
    pub unsafe fn get_Table(&self, table: *mut *mut IDispatch) -> HRESULT;

    /// Support interface access
    pub unsafe fn get_Support(&self, support: *mut *mut IDispatch) -> HRESULT;
}

/// Geometry operations interface for creating and managing structural elements
#[interface("C052FED9-A2D6-42E3-A271-2C6FB8461711")]
pub unsafe trait IOSGeometryUI: IDispatch {
    /// Creates a node with specified coordinates
    pub unsafe fn CreateNode(
        &self,
        node_no: &VARIANT,
        coord_x: &VARIANT,
        coord_y: &VARIANT,
        coord_z: &VARIANT,
    ) -> HRESULT;

    /// Creates a beam between two nodes
    pub unsafe fn CreateBeam(
        &self,
        beam_no: &VARIANT,
        node_a: &VARIANT,
        node_b: &VARIANT,
    ) -> HRESULT;

    /// Creates a plate element
    pub unsafe fn CreatePlate(
        &self,
        plate_no: &VARIANT,
        node_a: &VARIANT,
        node_b: &VARIANT,
        node_c: &VARIANT,
        node_d: &VARIANT,
    ) -> HRESULT;

    /// Creates a solid element
    pub unsafe fn CreateSolid(
        &self,
        solid_no: &VARIANT,
        node_a: &VARIANT,
        node_b: &VARIANT,
        node_c: &VARIANT,
        node_d: &VARIANT,
        node_e: &VARIANT,
        node_f: &VARIANT,
        node_g: &VARIANT,
        node_h: &VARIANT,
    ) -> HRESULT;

    /// Adds a node and returns its number
    pub unsafe fn AddNode(
        &self,
        coord_x: &VARIANT,
        coord_y: &VARIANT,
        coord_z: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Adds a beam and returns its number
    pub unsafe fn AddBeam(
        &self,
        node_a: &VARIANT,
        node_b: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Adds a plate and returns its number
    pub unsafe fn AddPlate(
        &self,
        node_a: &VARIANT,
        node_b: &VARIANT,
        node_c: &VARIANT,
        node_d: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Adds a solid and returns its number
    pub unsafe fn AddSolid(
        &self,
        node_a: &VARIANT,
        node_b: &VARIANT,
        node_c: &VARIANT,
        node_d: &VARIANT,
        node_e: &VARIANT,
        node_f: &VARIANT,
        node_g: &VARIANT,
        node_h: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;
}
