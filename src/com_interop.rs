// src-openstaad/com_interop.rs
// This module handles all direct interaction with the OpenSTAAD COM API.
// It's the bridge between the Rust world and the proprietary OpenSTAAD world.
// NOTE: This code is highly platform-specific and will only compile and run on Windows
// with OpenSTAAD installed.

use anyhow::{Context, Result, anyhow, bail};
use log::{info, warn};
use std::{ffi::c_void, mem, ptr::null_mut};
use windows::core::GUID;
use windows::{
    Win32::System::{
        Com::{
            CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED, COINIT_MULTITHREADED, CoCreateInstance,
            CoInitializeEx, DISPATCH_METHOD, DISPATCH_PROPERTYGET, DISPPARAMS, IDispatch,
        },
        Variant::{VARIANT, VT_DISPATCH, VariantClear},
    },
    core::{BSTR, HSTRING, Interface, PCWSTR},
};

use crate::binding::{IOSGeometryUI, IOpenSTAADUI};

/// Initializes the COM library for the current thread and creates an instance
/// of the OpenSTAAD application.
fn initialize_openstaad() -> Result<IDispatch> {
    info!("Initializing COM library...");
    // OpenSTAAD COM server is Single-Threaded Apartment (STA), so we must
    // initialize the thread as an STA. Using MTA would incur significant
    // performance penalties due to cross-apartment marshalling for every call.
    unsafe {
        if CoInitializeEx(None, COINIT_APARTMENTTHREADED).is_err() {
            return Err(anyhow!("CoInitializeEx failed"));
        }
    }

    info!("Creating OpenSTAAD instance...");
    let clsid = unsafe {
        // ProgID for OpenSTAAD, as per the documentation.
        let prog_id = HSTRING::from("StaadPro.OpenSTAAD");
        windows::Win32::System::Com::CLSIDFromProgID(PCWSTR(prog_id.as_ptr()))
            .map_err(|e| anyhow!("CLSIDFromProgID failed: {}", e))?
    };
    println!("clsid: {:#?}", clsid);

    let openstaad_app = unsafe {
        CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER)
            .map_err(|e| anyhow!("CoCreateInstance failed: {}", e))?
    };

    info!("OpenSTAAD instance created successfully.");
    Ok(openstaad_app)
}

/// A wrapper struct to manage the OpenSTAAD application instance.
/// It will handle cleanup (releasing COM objects) when it goes out of scope.
pub struct OpenStaadApp {
    // The IDispatch interface is a generic way to call methods on a COM object.
    app: IDispatch,
}

impl OpenStaadApp {
    /// Connects to OpenSTAAD and initializes the application.
    pub fn new() -> Result<Self> {
        let app = initialize_openstaad()?;
        let instance = Self { app };
        Ok(instance)
    }

    /// Gets the Geometry interface from the OpenSTAAD UI.
    pub fn get_geometry(&self) -> Result<IOSGeometryUI> {
        info!("Accessing the 'Geometry' property...");

        let prop_name = HSTRING::from("Geometry");
        let mut disp_id = 0;
        println!("{:#?}", self.app);

        let riid = &GUID::default() as *const GUID;
        unsafe {
            // 1. Get the Dispatch ID for the "Geometry" property.
            self.app
                .GetIDsOfNames(
                    &GUID::default(),
                    &PCWSTR(prop_name.as_ptr()),
                    1,
                    0,
                    &mut disp_id,
                )
                .map_err(|e| anyhow!("GetIDsOfNames for Geometry failed: {}", e))?;
        }

        // Prepare to receive the result.
        let mut result_variant = VARIANT::default();

        // No arguments needed to get a property.
        let disp_params = DISPPARAMS::default();

        info!("Invoking 'Geometry' property get...");
        unsafe {
            // 2. Invoke the property get.
            self.app
                .Invoke(
                    disp_id,
                    &GUID::default(),
                    0,
                    DISPATCH_PROPERTYGET,
                    &disp_params,
                    Some(&mut result_variant),
                    None,
                    None,
                )
                .map_err(|e| anyhow!("Invoke Geometry property failed: {}", e))?;
        }

        // 3. Extract the IDispatch pointer from the resulting VARIANT.
        unsafe {
            if result_variant.vt() != VT_DISPATCH {
                return Err(anyhow!(
                    "Expected IDispatch from Geometry property, got variant type: {}",
                    result_variant.vt().0
                ));
            }

            let dispatch_ptr = &result_variant.Anonymous.Anonymous.Anonymous.pdispVal;
            if dispatch_ptr.is_none() {
                return Err(anyhow!("Geometry property returned null IDispatch"));
            }

            let dispatch = dispatch_ptr
                .as_ref()
                .ok_or_else(|| anyhow!("Invalid IDispatch pointer in VARIANT"))?
                .clone();

            let geometry_interface: IOSGeometryUI = unsafe { dispatch.cast()? };

            info!("Successfully accessed 'Geometry' property.");

            VariantClear(&mut result_variant)?;

            Ok(geometry_interface)
        }
    }
}

/// Helper struct for managing geometry operations
pub struct GeometryManager {
    geometry: IOSGeometryUI,
}

impl GeometryManager {
    /// Creates a new GeometryManager with the given geometry interface
    pub fn new(geometry: IOSGeometryUI) -> Self {
        Self { geometry }
    }

    /// Creates a node with the specified number and coordinates
    pub fn create_node(&self, node_no: i32, x: f64, y: f64, z: f64) -> Result<()> {
        info!("Creating node {} at ({}, {}, {})", node_no, x, y, z);

        let node_variant = VARIANT::from(node_no);
        let x_variant = VARIANT::from(x);
        let y_variant = VARIANT::from(y);
        let z_variant = VARIANT::from(z);

        unsafe {
            let _ = self
                .geometry
                .CreateNode(&node_variant, &x_variant, &y_variant, &z_variant);
        }

        info!("Successfully created node {}", node_no);
        Ok(())
    }

    /// Creates a beam with the specified number between two nodes
    pub fn create_beam(&self, beam_no: i32, node_a: i32, node_b: i32) -> Result<()> {
        info!(
            "Creating beam {} between nodes {} and {}",
            beam_no, node_a, node_b
        );

        let beam_variant = VARIANT::from(beam_no);
        let node_a_variant = VARIANT::from(node_a);
        let node_b_variant = VARIANT::from(node_b);

        unsafe {
            let _ = self
                .geometry
                .CreateBeam(&beam_variant, &node_a_variant, &node_b_variant);
        }

        info!("Successfully created beam {}", beam_no);
        Ok(())
    }

    /// Adds a node and returns its automatically assigned number
    pub fn add_node(&self, x: f64, y: f64, z: f64) -> Result<i32> {
        info!("Adding node at ({}, {}, {})", x, y, z);

        let x_variant = VARIANT::from(x);
        let y_variant = VARIANT::from(y);
        let z_variant = VARIANT::from(z);
        let mut result_variant = VARIANT::default();

        unsafe {
            let _ = self
                .geometry
                .AddNode(&x_variant, &y_variant, &z_variant, &mut result_variant);
        }

        // Extract the node number from the result
        let node_no = unsafe {
            let value = result_variant.Anonymous.Anonymous.Anonymous.lVal;
            VariantClear(&mut result_variant)?;
            value
        };

        info!("Successfully added node with number {}", node_no);
        Ok(node_no)
    }

    /// Adds a beam and returns its automatically assigned number
    pub fn add_beam(&self, node_a: i32, node_b: i32) -> Result<i32> {
        info!("Adding beam between nodes {} and {}", node_a, node_b);

        let node_a_variant = VARIANT::from(node_a);
        let node_b_variant = VARIANT::from(node_b);
        let mut result_variant = VARIANT::default();

        unsafe {
            let _ = self
                .geometry
                .AddBeam(&node_a_variant, &node_b_variant, &mut result_variant);
        }

        // Extract the beam number from the result
        let beam_no = unsafe {
            let value = result_variant.Anonymous.Anonymous.Anonymous.lVal;
            VariantClear(&mut result_variant)?;
            value
        };

        info!("Successfully added beam with number {}", beam_no);
        Ok(beam_no)
    }
}
