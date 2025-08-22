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

    /// Adds multiple nodes with coordinates array
    pub unsafe fn AddMultipleNodes(&self, fa_coordinates: &VARIANT) -> HRESULT;

    /// Adds multiple beams with incidences array
    pub unsafe fn AddMultipleBeams(&self, na_incidences: &VARIANT) -> HRESULT;

    /// Adds multiple plates with incidences array
    pub unsafe fn AddMultiplePlates(&self, na_incidences: &VARIANT) -> HRESULT;

    /// Adds multiple solids with incidences array
    pub unsafe fn AddMultipleSolids(&self, na_incidences: &VARIANT) -> HRESULT;

    /// Deletes a node
    pub unsafe fn DeleteNode(&self, n_node_no: &VARIANT) -> HRESULT;

    /// Deletes a beam
    pub unsafe fn DeleteBeam(&self, n_beam_no: &VARIANT) -> HRESULT;

    /// Deletes a plate
    pub unsafe fn DeletePlate(&self, n_plate_no: &VARIANT) -> HRESULT;

    /// Deletes a solid
    pub unsafe fn DeleteSolid(&self, n_solid_no: &VARIANT) -> HRESULT;

    /// Deletes a surface
    pub unsafe fn DeleteSurface(&self, n_surface_no: &VARIANT) -> HRESULT;

    /// Splits a beam at specified distances
    pub unsafe fn SplitBeam(
        &self,
        n_beam_no: &VARIANT,
        n_nodes: &VARIANT,
        fa_dist_to_nodes: &VARIANT,
    ) -> HRESULT;

    /// Splits a beam into equal parts
    pub unsafe fn SplitBeamInEqlParts(&self, n_beam_no: &VARIANT, n_parts: &VARIANT) -> HRESULT;

    /// Gets the last node number
    pub unsafe fn GetLastNodeNo(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets the last beam number
    pub unsafe fn GetLastBeamNo(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets the last plate number
    pub unsafe fn GetLastPlateNo(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets the last solid number
    pub unsafe fn GetLastSolidNo(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets the total node count
    pub unsafe fn GetNodeCount(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets the total member count
    pub unsafe fn GetMemberCount(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets the total plate count
    pub unsafe fn GetPlateCount(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets the total surface count
    pub unsafe fn GetSurfaceCount(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets the total solid count
    pub unsafe fn GetSolidCount(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets the beam length
    pub unsafe fn GetBeamLength(&self, n_beam_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Gets node distance between two nodes
    pub unsafe fn GetNodeDistance(
        &self,
        n_node_no_a: &VARIANT,
        n_node_no_b: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets node number by coordinates
    pub unsafe fn GetNodeNumber(
        &self,
        f_coord_x: &VARIANT,
        f_coord_y: &VARIANT,
        f_coord_z: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets the node list
    pub unsafe fn GetNodeList(&self, n_node_list: *mut VARIANT) -> HRESULT;

    /// Gets the beam list
    pub unsafe fn GetBeamList(&self, n_beam_list: *mut VARIANT) -> HRESULT;

    /// Gets the plate list
    pub unsafe fn GetPlateList(&self, n_plate_list: *mut VARIANT) -> HRESULT;

    /// Gets the surface list
    pub unsafe fn GetSurfaceList(&self, n_surface_list: *mut VARIANT) -> HRESULT;

    /// Gets the solid list
    pub unsafe fn GetSolidList(&self, n_solid_list: *mut VARIANT) -> HRESULT;

    /// Gets the plate node count
    pub unsafe fn GetPlateNodeCount(&self, varn_plate_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Gets the number of generated quad panels
    pub unsafe fn GetNoOfGeneratedQuadPanels(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets generated quad panel incidences
    pub unsafe fn GetGeneratedQuadPanelIncidences(
        &self,
        na_node_a_nos: *mut VARIANT,
        na_node_b_nos: *mut VARIANT,
        na_node_c_nos: *mut VARIANT,
        na_node_d_nos: *mut VARIANT,
    ) -> HRESULT;

    /// Gets the last physical member number
    pub unsafe fn GetLastPhysicalMemberNo(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets the physical member count
    pub unsafe fn GetPhysicalMemberCount(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets the pmember count
    pub unsafe fn GetPMemberCount(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets the analytical member count for physical member
    pub unsafe fn GetAnalyticalMemberCountForPhysicalMember(
        &self,
        n_phy_memb_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets analytical members for physical member
    pub unsafe fn GetAnalyticalMembersForPhysicalMember(
        &self,
        n_phy_memb_no: &VARIANT,
        n_analytical_member: &VARIANT,
        member_list: *mut VARIANT,
    ) -> HRESULT;

    /// Gets physical member attribute
    pub unsafe fn GetPhysicalMemberAttribute(
        &self,
        n_phy_memb_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets the physical member list
    pub unsafe fn GetPhysicalMemberList(&self, n_beam_list: *mut VARIANT) -> HRESULT;

    /// Gets the surface node count
    pub unsafe fn GetSurfaceNodeCount(&self, n_surface_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Gets the surface node list
    pub unsafe fn GetSurfaceNodeList(
        &self,
        n_surface_no: &VARIANT,
        n_node_no_list: *mut VARIANT,
    ) -> HRESULT;

    /// Gets the surface opening count
    pub unsafe fn GetSurfaceOpeningCount(
        &self,
        n_surface_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets the surface opening list
    pub unsafe fn GetSurfaceOpeningList(
        &self,
        n_surface_no: &VARIANT,
        n_opening_no_list: *mut VARIANT,
    ) -> HRESULT;

    /// Gets surface opening details
    pub unsafe fn GetSurfaceOpeningDetails(
        &self,
        n_surface_no: &VARIANT,
        n_opening_no: &VARIANT,
        n_opening_type: *mut VARIANT,
        n_vertex_count: *mut VARIANT,
    ) -> HRESULT;

    /// Gets surface opening vertices
    pub unsafe fn GetSurfaceOpeningVertices(
        &self,
        n_surface_no: &VARIANT,
        n_opening_no: &VARIANT,
        x_coord_list: *mut VARIANT,
        y_coord_list: *mut VARIANT,
        z_coord_list: *mut VARIANT,
    ) -> HRESULT;

    /// Gets surface edge division
    pub unsafe fn GetSurfaceEdgeDivision(
        &self,
        n_surface_no: &VARIANT,
        n_edge_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets surface opening edge division
    pub unsafe fn GetSurfaceOpeningEdgeDivision(
        &self,
        n_surface_no: &VARIANT,
        n_opening_no: &VARIANT,
        n_opening_edge_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets surface origin
    pub unsafe fn GetSurfaceOrigin(
        &self,
        n_surface_no: &VARIANT,
        x_coord: *mut VARIANT,
        y_coord: *mut VARIANT,
        z_coord: *mut VARIANT,
    ) -> HRESULT;

    /// Gets surface local axis
    pub unsafe fn GetSurfaceLocalAxis(
        &self,
        n_surface_no: &VARIANT,
        x_axis: *mut VARIANT,
        y_axis: *mut VARIANT,
        z_axis: *mut VARIANT,
    ) -> HRESULT;

    /// Gets surface center
    pub unsafe fn GetSurfaceCenter(
        &self,
        n_surface_no: &VARIANT,
        x_coord: *mut VARIANT,
        y_coord: *mut VARIANT,
        z_coord: *mut VARIANT,
    ) -> HRESULT;

    /// Gets surface gross area
    pub unsafe fn GetSurfaceGrossArea(&self, n_surface_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Gets surface net area
    pub unsafe fn GetSurfaceNetArea(&self, n_surface_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Gets surface opening area
    pub unsafe fn GetSurfaceOpeningArea(
        &self,
        n_surface_no: &VARIANT,
        n_opening_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets number of beams connected at node
    pub unsafe fn GetNoOfBeamsConnectedAtNode(
        &self,
        n_node_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets beams connected at node
    pub unsafe fn GetBeamsConnectedAtNode(
        &self,
        n_node_no: &VARIANT,
        n_beam_list: *mut VARIANT,
    ) -> HRESULT;

    /// Gets the flag for hidden entities
    pub unsafe fn GetFlagForHiddenEntities(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets attached surface for plate
    pub unsafe fn GetAttachedSurfaceForPlate(
        &self,
        n_plate_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets the PID (Property ID)
    pub unsafe fn GetPID(
        &self,
        n_entity_no: &VARIANT,
        n_entity_type: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets count of breakable beams at specific nodes
    pub unsafe fn GetCountOfBreakableBeamsAtSpecificNodes(
        &self,
        n_node_id_array: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets intersect beams count
    pub unsafe fn GetIntersectBeamsCount(
        &self,
        beam_nos_array: &VARIANT,
        d_tolerance: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets number of selected nodes
    pub unsafe fn GetNoOfSelectedNodes(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets selected nodes
    pub unsafe fn GetSelectedNodes(
        &self,
        na_node_nos: *mut VARIANT,
        n_is_sorted: &VARIANT,
    ) -> HRESULT;

    /// Gets number of selected beams
    pub unsafe fn GetNoOfSelectedBeams(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets selected beams
    pub unsafe fn GetSelectedBeams(
        &self,
        na_beam_nos: *mut VARIANT,
        n_is_sorted: &VARIANT,
    ) -> HRESULT;

    /// Gets number of selected plates
    pub unsafe fn GetNoOfSelectedPlates(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets selected plates
    pub unsafe fn GetSelectedPlates(
        &self,
        na_plate_nos: *mut VARIANT,
        n_is_sorted: &VARIANT,
    ) -> HRESULT;

    /// Gets number of selected solids
    pub unsafe fn GetNoOfSelectedSolids(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets selected solids
    pub unsafe fn GetSelectedSolids(
        &self,
        na_solid_nos: *mut VARIANT,
        n_is_sorted: &VARIANT,
    ) -> HRESULT;

    /// Gets number of selected surfaces
    pub unsafe fn GetNoOfSelectedSurfaces(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets selected surfaces
    pub unsafe fn GetSelectedSurfaces(
        &self,
        na_surface_nos: *mut VARIANT,
        n_is_sorted: &VARIANT,
    ) -> HRESULT;

    /// Gets number of selected physical members
    pub unsafe fn GetNoOfSelectedPhysicalMembers(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets selected physical members
    pub unsafe fn GetSelectedPhysicalMembers(
        &self,
        na_phy_memb_nos: *mut VARIANT,
        n_is_sorted: &VARIANT,
    ) -> HRESULT;

    /// Selects multiple nodes
    pub unsafe fn SelectMultipleNodes(&self, na_node_nos: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Selects multiple beams
    pub unsafe fn SelectMultipleBeams(&self, na_beam_nos: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Selects multiple plates
    pub unsafe fn SelectMultiplePlates(&self, na_plate_nos: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Selects multiple solids
    pub unsafe fn SelectMultipleSolids(&self, na_solid_nos: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Selects multiple physical members
    pub unsafe fn SelectMultiplePhysicalMembers(&self, na_phy_memb_nos: &VARIANT) -> HRESULT;

    /// Selects a node
    pub unsafe fn SelectNode(&self, n_node_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Selects a beam
    pub unsafe fn SelectBeam(&self, n_beam_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Selects a plate
    pub unsafe fn SelectPlate(&self, n_plate_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Selects a solid
    pub unsafe fn SelectSolid(&self, n_solid_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Selects a physical member
    pub unsafe fn SelectPhysicalMember(&self, n_phy_memb_no: &VARIANT) -> HRESULT;

    /// Clears node selection
    pub unsafe fn ClearNodeSelection(&self) -> HRESULT;

    /// Clears member selection
    pub unsafe fn ClearMemberSelection(&self) -> HRESULT;

    /// Clears plate selection
    pub unsafe fn ClearPlateSelection(&self) -> HRESULT;

    /// Clears solid selection
    pub unsafe fn ClearSolidSelection(&self) -> HRESULT;

    /// Clears surface selection
    pub unsafe fn ClearSurfaceSelection(&self) -> HRESULT;

    /// Clears physical member selection
    pub unsafe fn ClearPhysicalMemberSelection(&self) -> HRESULT;

    /// Gets node coordinates
    pub unsafe fn GetNodeCoordinates(
        &self,
        n_node_no: &VARIANT,
        f_coord_x: *mut VARIANT,
        f_coord_y: *mut VARIANT,
        f_coord_z: *mut VARIANT,
    ) -> HRESULT;

    /// Sets node coordinates
    pub unsafe fn SetNodeCoordinate(
        &self,
        n_node_no: &VARIANT,
        f_coord_x: &VARIANT,
        f_coord_y: &VARIANT,
        f_coord_z: &VARIANT,
    ) -> HRESULT;

    /// Sets node unique ID
    pub unsafe fn SetNodeUniqueID(&self, n_node_no: &VARIANT, sz_name: &VARIANT) -> HRESULT;

    /// Sets member unique ID
    pub unsafe fn SetMemberUniqueID(&self, n_memb_no: &VARIANT, sz_name: &VARIANT) -> HRESULT;

    /// Sets plate unique ID
    pub unsafe fn SetPlateUniqueID(&self, n_plate_no: &VARIANT, sz_name: &VARIANT) -> HRESULT;

    /// Sets solid unique ID
    pub unsafe fn SetSolidUniqueID(&self, n_solid_no: &VARIANT, sz_name: &VARIANT) -> HRESULT;

    /// Sets surface unique ID
    pub unsafe fn SetSurfaceUniqueID(&self, n_surface_no: &VARIANT, sz_name: &VARIANT) -> HRESULT;

    /// Sets physical member unique ID
    pub unsafe fn SetPhysicalMemberUniqueID(&self, n_phy_memb_no: &VARIANT, sz_name: &VARIANT) -> HRESULT;

    /// Gets node unique ID
    pub unsafe fn GetNodeUniqueID(&self, n_node_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Gets member unique ID
    pub unsafe fn GetMemberUniqueID(&self, n_memb_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Gets plate unique ID
    pub unsafe fn GetPlateUniqueID(&self, n_plate_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Gets solid unique ID
    pub unsafe fn GetSolidUniqueID(&self, n_solid_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Gets surface unique ID
    pub unsafe fn GetSurfaceUniqueID(&self, n_surface_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Gets physical member unique ID
    pub unsafe fn GetPhysicalMemberUniqueID(&self, n_phy_memb_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Sets PID (Property ID)
    pub unsafe fn SetPID(
        &self,
        n_entity_no: &VARIANT,
        n_entity_type: &VARIANT,
        pid: &VARIANT,
    ) -> HRESULT;

    /// Sets flag for hidden entities
    pub unsafe fn SetFlagForHiddenEntities(&self, n_flag_value: &VARIANT) -> HRESULT;

    /// Sets attached surface for plate
    pub unsafe fn SetAttachedSurfaceForPlate(
        &self,
        n_plate_no: &VARIANT,
        n_surface_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets node incidence
    pub unsafe fn GetNodeIncidence(
        &self,
        n_node_no: &VARIANT,
        f_coord_x: *mut VARIANT,
        f_coord_y: *mut VARIANT,
        f_coord_z: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets member incidence
    pub unsafe fn GetMemberIncidence(
        &self,
        n_beam_no: &VARIANT,
        n_node_a: *mut VARIANT,
        n_node_b: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets plate incidence
    pub unsafe fn GetPlateIncidence(
        &self,
        n_plate_no: &VARIANT,
        n_node_a: *mut VARIANT,
        n_node_b: *mut VARIANT,
        n_node_c: *mut VARIANT,
        n_node_d: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets solid incidence
    pub unsafe fn GetSolidIncidence(
        &self,
        n_solid_no: &VARIANT,
        n_node_a: *mut VARIANT,
        n_node_b: *mut VARIANT,
        n_node_c: *mut VARIANT,
        n_node_d: *mut VARIANT,
        n_node_e: *mut VARIANT,
        n_node_f: *mut VARIANT,
        n_node_g: *mut VARIANT,
        n_node_h: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets node incidence CIS2
    pub unsafe fn GetNodeIncidence_CIS2(
        &self,
        n_node_no: &VARIANT,
        sz_name: *mut VARIANT,
        f_coord_x: *mut VARIANT,
        f_coord_y: *mut VARIANT,
        f_coord_z: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets member incidence CIS2
    pub unsafe fn GetMemberIncidence_CIS2(
        &self,
        n_beam_no: &VARIANT,
        sz_name: *mut VARIANT,
        n_node_a: *mut VARIANT,
        n_node_b: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets plate incidence CIS2
    pub unsafe fn GetPlateIncidence_CIS2(
        &self,
        n_plate_no: &VARIANT,
        sz_name: *mut VARIANT,
        n_node_a: *mut VARIANT,
        n_node_b: *mut VARIANT,
        n_node_c: *mut VARIANT,
        n_node_d: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets solid incidence CIS2
    pub unsafe fn GetSolidIncidence_CIS2(
        &self,
        n_solid_no: &VARIANT,
        sz_name: *mut VARIANT,
        n_node_a: *mut VARIANT,
        n_node_b: *mut VARIANT,
        n_node_c: *mut VARIANT,
        n_node_d: *mut VARIANT,
        n_node_e: *mut VARIANT,
        n_node_f: *mut VARIANT,
        n_node_g: *mut VARIANT,
        n_node_h: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Creates a physical member
    pub unsafe fn CreatePhysicalMember(
        &self,
        n_analytical_members: &VARIANT,
        member_list: &VARIANT,
        member_attribute: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Attaches member to physical member
    pub unsafe fn AttachMemberToPhysicalMember(
        &self,
        n_phy_memb_no: &VARIANT,
        member_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Sets physical member attribute
    pub unsafe fn SetPhysicalMemberAttribute(
        &self,
        n_phy_memb_no: &VARIANT,
        member_attribute: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Deletes a physical member
    pub unsafe fn DeletePhysicalMember(
        &self,
        n_phy_memb_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Creates a PMember
    pub unsafe fn CreatePMember(
        &self,
        n_analytical_members: &VARIANT,
        member_list: &VARIANT,
        member_attribute: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Attaches member to PMember
    pub unsafe fn AttachMemberToPMember(
        &self,
        n_phy_memb_no: &VARIANT,
        n_member_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Creates a group
    pub unsafe fn CreateGroup(
        &self,
        group_type: &VARIANT,
        group_name: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Installs TS callback
    pub unsafe fn InstallTSCallBack(&self, dispatch_ptr: &VARIANT) -> HRESULT;

    /// Does translational repeat
    pub unsafe fn DoTranslationalRepeat(
        &self,
        var_link_bays: &VARIANT,
        var_open_base: &VARIANT,
        var_axis_dir: &VARIANT,
        var_spacing_array: &VARIANT,
        var_no_bays: &VARIANT,
        var_renumber_bay: &VARIANT,
        var_renumber_array: &VARIANT,
        var_geometry_only: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Breaks beams at specific nodes
    pub unsafe fn BreakBeamsAtSpecificNodes(
        &self,
        n_node_id_array: &VARIANT,
        n_broken_beam_id_array: *mut VARIANT,
        n_new_beam_id_array: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Creates a surface
    pub unsafe fn CreateSurface(
        &self,
        n_surface_no: &VARIANT,
        n_node_no_list: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Adds a surface
    pub unsafe fn AddSurface(&self, n_node_no_list: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Adds surface opening
    pub unsafe fn AddSurfaceOpening(
        &self,
        n_surface_no: &VARIANT,
        n_opening_no: &VARIANT,
        n_vertex_count: &VARIANT,
        x_coord_list: &VARIANT,
        y_coord_list: &VARIANT,
        z_coord_list: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Adds surface edge division
    pub unsafe fn AddSurfaceEdgeDivision(
        &self,
        n_surface_no: &VARIANT,
        n_edge_no: &VARIANT,
        n_edge_div: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Adds surface opening edge division
    pub unsafe fn AddSurfaceOpeningEdgeDivision(
        &self,
        n_surface_no: &VARIANT,
        n_opening_no: &VARIANT,
        n_opening_edge_no: &VARIANT,
        n_opening_edge_div: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Removes surface opening
    pub unsafe fn RemoveSurfaceOpening(
        &self,
        n_surface_no: &VARIANT,
        n_opening_no: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Checks if Z axis is up
    pub unsafe fn IsZUp(&self, result: *mut VARIANT) -> HRESULT;

    /// Checks if member is a beam
    pub unsafe fn IsBeam(
        &self,
        n_member_no: &VARIANT,
        d_tol_angle: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Checks if member is a column
    pub unsafe fn IsColumn(
        &self,
        n_member_no: &VARIANT,
        d_tol_angle: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Checks if there are hidden entities
    pub unsafe fn HasHiddenEntities(&self, result: *mut VARIANT) -> HRESULT;

    /// Renumbers a beam
    pub unsafe fn RenumberBeam(
        &self,
        var_beam_no_old: &VARIANT,
        var_beam_no_new: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Checks if node is orphan
    pub unsafe fn IsOrphanNode(&self, var_node_no: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Gets count of all groups
    pub unsafe fn GetGroupCountAll(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets group count by type
    pub unsafe fn GetGroupCount(&self, var_group_type: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Gets group names
    pub unsafe fn GetGroupNames(
        &self,
        var_group_type: &VARIANT,
        sz_group_name_list: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets group entity count
    pub unsafe fn GetGroupEntityCount(
        &self,
        sz_group_name: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets group entities
    pub unsafe fn GetGroupEntities(
        &self,
        sz_group_name: &VARIANT,
        var_entity_list: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Creates group extended
    pub unsafe fn CreateGroupEx(
        &self,
        var_group_type: &VARIANT,
        sz_group_name: &VARIANT,
        var_entity_count: &VARIANT,
        var_entity_list: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Deletes a group
    pub unsafe fn DeleteGroup(&self, sz_group_name: &VARIANT, result: *mut VARIANT) -> HRESULT;

    /// Updates a group
    pub unsafe fn UpdateGroup(
        &self,
        sz_group_name: &VARIANT,
        var_flag: &VARIANT,
        var_entity_count: &VARIANT,
        var_entity_list: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Defines parametric surface
    pub unsafe fn DefineParametricSurface(
        &self,
        sz_name: &BSTR,
        var_type: &VARIANT,
        node_o: &VARIANT,
        node_x: &VARIANT,
        node_3rd: &VARIANT,
        var_count: &VARIANT,
        var_vertices: &VARIANT,
        var_auto_generate: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Adds parametric surface to model
    pub unsafe fn AddParametricSurfaceToModel(
        &self,
        n_surface_id: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Commits parametric surface mesh
    pub unsafe fn CommitParametricSurfaceMesh(
        &self,
        n_surface_id: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Removes parametric surface mesh
    pub unsafe fn RemoveParametricSurfaceMesh(
        &self,
        n_surface_id: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Adds density point to surface
    pub unsafe fn AddDensityPointToSurface(
        &self,
        n_surface_id: &VARIANT,
        var_x: &VARIANT,
        var_y: &VARIANT,
        var_z: &VARIANT,
        var_density: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Adds density line to surface
    pub unsafe fn AddDensityLineToSurface(
        &self,
        n_surface_id: &VARIANT,
        var_x1: &VARIANT,
        var_y1: &VARIANT,
        var_z1: &VARIANT,
        var_density1: &VARIANT,
        var_x2: &VARIANT,
        var_y2: &VARIANT,
        var_z2: &VARIANT,
        var_density2: &VARIANT,
        var_divs: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Adds circular region to surface
    pub unsafe fn AddCircularRegionToSurface(
        &self,
        n_surface_id: &VARIANT,
        var_x: &VARIANT,
        var_y: &VARIANT,
        var_z: &VARIANT,
        var_r: &VARIANT,
        var_div: &VARIANT,
        var_density: &VARIANT,
        var_is_opening: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Adds polygonal region to surface
    pub unsafe fn AddPolygonalRegionToSurface(
        &self,
        n_surface_id: &VARIANT,
        var_vertex_count: &VARIANT,
        var_x: &VARIANT,
        var_y: &VARIANT,
        var_z: &VARIANT,
        var_densities: &VARIANT,
        var_edge_divs: &VARIANT,
        var_is_opening: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets parametric surface count
    pub unsafe fn GetParametricSurfaceCount(&self, result: *mut VARIANT) -> HRESULT;

    /// Gets parametric surface info
    pub unsafe fn GetParametricSurfaceInfo(
        &self,
        var_surface_id: &VARIANT,
        var_name: *mut VARIANT,
        var_s_type: *mut VARIANT,
        var_bp_count: *mut VARIANT,
        var_dp_count: *mut VARIANT,
        var_dl_count: *mut VARIANT,
        var_opening_count: *mut VARIANT,
        var_region_count: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Finds parametric surface by name
    pub unsafe fn FindParametricSurfaceByName(
        &self,
        var_name: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets parametric surface mesh info
    pub unsafe fn GetParametricSurfaceMeshInfo(
        &self,
        n_surface_id: &VARIANT,
        var_node_count: *mut VARIANT,
        var_element_count: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets parametric surface mesh data
    pub unsafe fn GetParametricSurfaceMeshData(
        &self,
        n_surface_id: &VARIANT,
        var_generated_nodes: *mut VARIANT,
        var_generated_elements: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Sets parametric surface unique ID
    pub unsafe fn SetParametricSurfaceUniqueID(
        &self,
        sz_name: &VARIANT,
        sz_uid: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets parametric surface unique ID
    pub unsafe fn GetParametricSurfaceUniqueID(
        &self,
        sz_name: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets area of plates
    pub unsafe fn GetAreaOfPlates(
        &self,
        var_plate_nos: &VARIANT,
        var_plates_area: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Creates multiple plates
    pub unsafe fn CreateMultiplePlates(
        &self,
        n_plate_ids: &VARIANT,
        n_plate_incidences: &VARIANT,
    ) -> HRESULT;

    /// Sets parametric surface sub type
    pub unsafe fn SetParametricSurfaceSubType(
        &self,
        sz_name: &VARIANT,
        sz_sub_type: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Gets parametric surface sub type
    pub unsafe fn GetParametricSurfaceSubType(
        &self,
        sz_name: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Sets check for identical entity
    pub unsafe fn SetCheckForIdenticalEntity(
        &self,
        entity_type: i32,
        b_enable: i32,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Creates multiple nodes
    pub unsafe fn CreateMultipleNodes(
        &self,
        n_node_id_array: &VARIANT,
        d_coord_array: &VARIANT,
    ) -> HRESULT;

    /// Creates multiple beams
    pub unsafe fn CreateMultipleBeams(
        &self,
        n_beam_id_array: &VARIANT,
        n_beam_incidence_array: &VARIANT,
    ) -> HRESULT;

    /// Gets parametric surface info extended
    pub unsafe fn GetParametricSurfaceInfoEx(
        &self,
        n_surface_id: i32,
        str_name: *mut BSTR,
        n_surface_type: *mut i32,
        str_surface_sub_type: *mut BSTR,
        n_vertices: *mut i32,
        meshsize: *mut f64,
        i_div_opt: *mut i32,
        method: *mut i32,
        is_quad: *mut i32,
        origin_node: *mut i32,
        node_on_x_axis: *mut i32,
        node_towards_y_axis: *mut i32,
        n_circular_openings: *mut i32,
        n_polygonal_opening: *mut i32,
        n_circular_regions: *mut i32,
        n_polygonal_regions: *mut i32,
        n_density_points: *mut i32,
        n_density_lines: *mut i32,
    ) -> i32;

    /// Merges beams
    pub unsafe fn MergeBeams(
        &self,
        n_beam_id_array: &VARIANT,
        var_beam_no: &VARIANT,
        var_property_no: &VARIANT,
        var_beta_angle: &VARIANT,
        var_material_name: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Intersects beams
    pub unsafe fn IntersectBeams(
        &self,
        method: &VARIANT,
        beam_nos_array: &VARIANT,
        d_tolerance: &VARIANT,
        new_beam_nos_array: *mut VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;

    /// Merges nodes
    pub unsafe fn MergeNodes(
        &self,
        var_node_no: &VARIANT,
        n_node_id_array: &VARIANT,
        result: *mut VARIANT,
    ) -> HRESULT;
}
