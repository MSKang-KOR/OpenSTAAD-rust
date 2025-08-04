use crate::openstaad::tools::{
    invoke::{get_dispatch, invoke_method},
    safe_array::safe_array_from_vec1d,
    variant::{SafeArray, SafeArrayP, variant_from_raw_pointer},
};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use std::ffi::c_void;
use windows::Win32::System::{
    Com::{IDispatch, SAFEARRAY},
    Ole::{SafeArrayCreateVector, SafeArrayGetElement, SafeArrayPutElement},
    Variant::{
        VARIANT, VT_BSTR, VT_I4, VT_R8, VariantToDouble, VariantToInt32, VariantToStringAlloc,
    },
};
use windows_core::BSTR;

#[derive(Debug)]
pub struct Property<'a> {
    pub staad: &'a IDispatch,
    pub dispatch: IDispatch,
}

impl<'a> Property<'a> {
    pub fn new(staad: &'a IDispatch) -> Self {
        let _property = unsafe { get_dispatch(staad, "Property", &mut []).unwrap() };
        Self {
            staad,
            dispatch: _property,
        }
    }
    /// Creates angle property from database.
    /// # Parameters
    /// * `[in] Country` The value for the specified country(Type: long/Integer)
    /// * `[in] SectionName` Name of the section(Type: string).
    /// * `[in] TypeSpec` The type specification number(Type: long/Integer):
    ///   - 0: ST - Single section from the standard built-in tables
    ///   - 1: RA - Single angle with reverse Y-Z axes
    ///   - 3: LD - Double angle with long legs back-to-back
    ///   - 4: SD - Double angle with short legs back-to-back
    ///   - 12: SA - Double angle in a star arrangement (heel to heel)[for Aluminium]
    /// * `[in] AddSpec_1` Additional specification value(Type: double/float) SP
    /// # Return values
    /// * `<Val>` The assigned section property ID.
    /// * `0` Library Error: Unable to create property.
    pub fn create_angle_property_from_table(
        &self,
        country: i32,
        section_name: &str,
        type_spec: i32,
        add_spec_1: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(add_spec_1),
                VARIANT::from(type_spec),
                VARIANT::from(section_name),
                VARIANT::from(country),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "CreateAnglePropertyFromTable", &mut params);
            match result_variant {
                Ok(var) => {
                    let property_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(property_id)
                }
                Err(e) => bail!("Error::Property::create_angle_property_from_table: {}", e),
            }
        }
    }

    /// Creates beam property from table.
    /// # Parameters
    /// * `[in] Country` The value for the specified country (Type: long/Integer).
    /// * `[in] SectionName` Name of the section(Type: string).
    /// * `[in] TypeSpec` The type specification number(Type: long/Integer):
    ///   - 0: ST
    ///   - 2: D - Double profile
    ///   - 5: T - Tee section cut from I shaped section (for aluminium)
    /// * `[in] AddSpec_1` clear Spacing for Double profile(Type: double/float)
    /// * `[in] AddSpec_2` please set it with 0.0(Type: double/float)
    /// # Return values
    /// * `<Val>` The assigned section property ID.
    /// * `0` Library Error: Unable to create property.
    pub fn create_beam_property_from_table(
        &self,
        country: i32,
        section_name: &str,
        type_spec: i32,
        add_spec_1: f64,
        add_spec_2: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(add_spec_2),
                VARIANT::from(add_spec_1),
                VARIANT::from(type_spec),
                VARIANT::from(section_name),
                VARIANT::from(country),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "CreateBeamPropertyFromTable", &mut params);
            match result_variant {
                Ok(var) => {
                    let property_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(property_id)
                }
                Err(e) => bail!("Error::Property::create_beam_property_from_table: {}", e),
            }
        }
    }

    /// Creates beam property from table.
    /// # Parameters
    /// * `[in] Country` The value for the specified country(Type: long/Integer)
    /// * `[in] SectionName` Name of the section(Type: string).
    /// * `[in] TypeSpec` The type specification number(Type: long/Integer):
    ///   - 1: Plate Strip
    ///   - 2: Solid Rect
    ///   - 3: Solid Round
    ///   - 4: Round
    ///   - 5: Cable
    /// # Return values
    /// * `<Val>` The assigned section property ID.
    /// * `0` Library Error: Unable to create property.
    pub fn create_beam_property_from_table_ex(
        &self,
        country: i32,
        section_name: &str,
        type_spec: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(type_spec),
                VARIANT::from(section_name),
                VARIANT::from(country),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "CreateBeamPropertyFromTableEx", &mut params);
            match result_variant {
                Ok(var) => {
                    let property_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(property_id)
                }
                Err(e) => bail!("Error::Property::create_beam_property_from_table_ex: {}", e),
            }
        }
    }

    /// Creates channel property from database.
    /// # Parameters
    /// * `[in] Country` The value for the specified country(Type: long/Integer)
    /// * `[in] SectionName` Name of the section(Type: string).
    /// * `[in] TypeSpec` The type specification number(Type: long/Integer):
    ///   - -1: Define
    ///   - 0: ST
    ///   - 1: RA
    ///   - 2: D
    ///   - 3: LD
    ///   - 4: SD
    ///   - 5: T (for aluminium)
    ///   - 6: CM
    ///   - 7: TC
    ///   - 8: BC
    ///   - 9: TB
    ///   - 10: BA (for aluminium)
    ///   - 11: FR
    ///   - 12: SA (for aluminium)
    /// * `[in] AddSpec_1` Additional specification value(Type: double/float) SP
    /// # Return values
    /// * `<Val>` The assigned section property ID.
    /// * `0` Library Error: Unable to create property.
    /// * `-6004` Section is not found in profile database.
    /// * `-6005` Section data for a section is not found.
    /// * `-6006` Invalid section type.
    pub fn create_channel_property_from_table(
        &self,
        country: i32,
        section_name: &str,
        type_spec: i32,
        add_spec_1: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(add_spec_1),
                VARIANT::from(type_spec),
                VARIANT::from(section_name),
                VARIANT::from(country),
            ];
            let result_variant = invoke_method(
                &self.dispatch,
                "CreateChannelPropertyFromTable",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let property_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(property_id)
                }
                Err(e) => bail!("Error::Property::create_channel_property_from_table: {}", e),
            }
        }
    }

    /// Creates plate uniform or nonuniform thickness property.
    /// # Parameters
    /// * `[in] faThickness` The thickness for all nodes.
    /// # Return values
    /// * `<Val>` The assigned section property ID.
    /// * `-106` faThickness dimensional array error.
    /// * `-6003` Library error: Unable to create property.
    pub fn create_plate_thickness_property(&self, thickness: Vec<f64>) -> Result<i32, anyErr> {
        unsafe {
            let sa_thickness = safe_array_from_vec1d::<f64>(thickness)?;
            let variant_thickness = variant_from_raw_pointer::<SafeArray<f64>>(sa_thickness);

            let mut params = [variant_thickness];
            let result_variant =
                invoke_method(&self.dispatch, "CreatePlateThicknessProperty", &mut params);
            match result_variant {
                Ok(var) => {
                    let property_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(property_id)
                }
                Err(e) => bail!("Error::Property::create_plate_thickness_property: {}", e),
            }
        }
    }

    /// Creates prismatic tee property.
    /// # Parameters
    /// * `[in] varfYD` Total depth of section (top fiber of flange to bottom fiber of web).
    /// * `[in] varfZD` Width of flange.
    /// * `[in] varfYB` Depth of stem.
    /// * `[in] varfZB` Width of stem.
    /// # Return values
    /// * `<Val>` The assigned section property ID.
    /// * `0` Library Error: Unable to create property.
    pub fn create_prismatic_tee_property(
        &self,
        yd: f64,
        zd: f64,
        yb: f64,
        zb: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(zb),
                VARIANT::from(yb),
                VARIANT::from(zd),
                VARIANT::from(yd),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "CreatePrismaticTeeProperty", &mut params);
            match result_variant {
                Ok(var) => {
                    let property_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(property_id)
                }
                Err(e) => bail!("Error::Property::create_prismatic_tee_property: {}", e),
            }
        }
    }

    /// Creates prismatic trapezoidal section property.
    /// # Parameters
    /// * `[in] varfYD` Total depth of section.
    /// * `[in] varfZD` Width of section at top fiber.
    /// * `[in] varfZB` Width of section at bottom fiber.
    /// # Return values
    /// * `<Val>` The assigned section property ID.
    /// * `0` Library Error: Unable to create property.
    pub fn create_prismatic_trapezoidal_property(
        &self,
        yd: f64,
        zd: f64,
        zb: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(zb), VARIANT::from(zd), VARIANT::from(yd)];
            let result_variant = invoke_method(
                &self.dispatch,
                "CreatePrismaticTrapezoidalProperty",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let property_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(property_id)
                }
                Err(e) => bail!(
                    "Error::Property::create_prismatic_trapezoidal_property: {}",
                    e
                ),
            }
        }
    }

    /// Creates tee property from database.
    /// # Parameters
    /// * `[in] Country` The value for the specified country(Type: long/Integer)
    /// * `[in] SectionName` Name of the section(Type: string).
    /// * `[in] TypeSpec` The type specification number(Type: long/Integer):
    ///   - -1: Define
    ///   - 0: ST
    ///   - 5: T From Wide Flange
    /// # Return values
    /// * `<Val>` The assigned section property ID.
    /// * `0` Library Error: Unable to create property.
    /// * `-6004` Section is not found in profile database.
    /// * `-6005` Section data for a section is not found.
    /// * `-6006` Invalid section type.
    pub fn create_tee_property_from_table(
        &self,
        country: i32,
        section_name: &str,
        type_spec: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(type_spec),
                VARIANT::from(section_name),
                VARIANT::from(country),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "CreateTeePropertyFromTable", &mut params);
            match result_variant {
                Ok(var) => {
                    let property_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(property_id)
                }
                Err(e) => bail!("Error::Property::create_tee_property_from_table: {}", e),
            }
        }
    }

    /// Creates wide flange member property from table with data for all specs.
    /// # Parameters
    /// * `[in] nCountry` The country CODE. (American = 1, Australian = 2, British = 3, Canadian = 4, Chinese = 5, Dutch = 6, European = 7, French = 8, German = 9, Indian = 10, Japanese = 11, Russian = 12, SouthAfrican = 13, Spanish = 14, Venezuelan = 15, Korean = 16).
    /// * `[in] SectionName` Name of the section.
    /// * `[in] nTypeSpec` The type specification number. (ST = 0; D = 2; T = 5; CM = 6; TC = 7; BC = 8; TB = 9).
    /// * `[in] varSpecs` The specification values in array:
    ///   - Index 0: SP/CT/WP - SP:-Spacing for double-I, double-C, double-L; CT:-Conc. thickness for composite-I; WP:-Width of top cover plate for TC,TB and bottom cover plate for BC
    ///   - Index 1: FC/TH - FC:-Concrete grade for composite-I; TH:-Thickness of top cover plate for TC,TB and bottom cover plate for BC
    ///   - Index 2: CW/BW - CW:-Concrete width for composite-I; BW:-Width of bottom cover plate for TB
    ///   - Index 3: CD/BT - CD:-Concrete density for composite-I; BT:-Thickness of bottom cover plate for TB
    /// # Return values
    /// * `<Val>` The assigned section property ID.
    /// * `-1` General Error.
    pub fn create_wide_flange_property_from_table(
        &self,
        country: i32,
        section_name: &str,
        type_spec: i32,
        specs: Vec<f64>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_specs = safe_array_from_vec1d::<f64>(specs)?;
            let variant_specs = variant_from_raw_pointer::<SafeArray<f64>>(sa_specs);

            let mut params = [
                variant_specs,
                VARIANT::from(type_spec),
                VARIANT::from(section_name),
                VARIANT::from(country),
            ];
            let result_variant = invoke_method(
                &self.dispatch,
                "CreateWideFlangePropertyFromTable",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let property_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(property_id)
                }
                Err(e) => bail!(
                    "Error::Property::create_wide_flange_property_from_table: {}",
                    e
                ),
            }
        }
    }
    /// Add channel type to an defined UPT section.
    /// # Parameters
    /// * `[in] nTableRef` The existing table number ID.
    /// * `[in] varSectionName` UPT section string name.
    /// * `[in] varAX` Cross section area (AX).
    /// * `[in] varD` Depth of the section (D).
    /// * `[in] varTW` Thickness of web (TW).
    /// * `[in] varWF` Width of the top flange (WF).
    /// * `[in] varTF` Thickness of top flange (TF).
    /// * `[in] varIZ` Torsional constant (IZ).
    /// * `[in] varIY` Moment of inertia about local y-axis (IY).
    /// * `[in] varIX` Moment of inertia about local z-axis (IX).
    /// * `[in] varCZ` Value CZ
    /// * `[in] varAY` Shear area in local y-axis. If zero, shear deformation is ignored in the analysis (AY).
    /// * `[in] varAZ` Shear area in local z-axis. If zero, shear deformation is ignored in the analysis (AZ).
    /// # Return values
    /// * `0` OK.
    /// * `-6032` Unable to add section varSectionName in UPT nTableRef.
    /// * `-6045` Section with same name exits in UPT nTableRef.
    pub fn add_upt_property_channel(
        &self,
        table_ref: i32,
        section_name: &str,
        ax: f64,
        d: f64,
        tw: f64,
        wf: f64,
        tf: f64,
        iz: f64,
        iy: f64,
        ix: f64,
        cz: f64,
        ay: f64,
        az: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(az),
                VARIANT::from(ay),
                VARIANT::from(cz),
                VARIANT::from(ix),
                VARIANT::from(iy),
                VARIANT::from(iz),
                VARIANT::from(tf),
                VARIANT::from(wf),
                VARIANT::from(tw),
                VARIANT::from(d),
                VARIANT::from(ax),
                VARIANT::from(section_name),
                VARIANT::from(table_ref),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "AddUPTPropertyCHANNEL", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::add_upt_property_channel: {}", e),
            }
        }
    }

    /// Add double angle type to an defined UPT section.
    /// # Parameters
    /// * `[in] nTableRef` The existing table number ID.
    /// * `[in] varSectionName` UPT section string name.
    /// * `[in] varD` Depth of angle (D).
    /// * `[in] varWF` Width of angle (WF).
    /// * `[in] varTF` Thickness of flanges (TF).
    /// * `[in] varSP` Distance between two angle (SP).
    /// * `[in] varIZ` Torsional constant (IZ).
    /// * `[in] varIY` Moment of inertia about local y-axis (IY).
    /// * `[in] varIX` Moment of inertia about local z-axis (IX).
    /// * `[in] varCY` Distance from z axis to the top of section (CY).
    /// * `[in] varAY` Shear area in local y-axis. If zero, shear deformation is ignored in the analysis (AY).
    /// * `[in] varAZ` Shear area in local z-axis. If zero, shear deformation is ignored in the analysis (AZ).
    /// # Return values
    /// * `0` OK.
    /// * `-6032` Unable to add section varSectionName in UPT nTableRef.
    /// * `-6045` Section with same name exits in UPT nTableRef.
    pub fn add_upt_property_double_angle(
        &self,
        table_ref: i32,
        section_name: &str,
        d: f64,
        wf: f64,
        tf: f64,
        sp: f64,
        iz: f64,
        iy: f64,
        ix: f64,
        cy: f64,
        ay: f64,
        az: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(az),
                VARIANT::from(ay),
                VARIANT::from(cy),
                VARIANT::from(ix),
                VARIANT::from(iy),
                VARIANT::from(iz),
                VARIANT::from(sp),
                VARIANT::from(tf),
                VARIANT::from(wf),
                VARIANT::from(d),
                VARIANT::from(section_name),
                VARIANT::from(table_ref),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "AddUPTPropertyDOUBLEANGLE", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::add_upt_property_double_angle: {}", e),
            }
        }
    }

    /// Add general type to an defined UPT section.
    /// # Parameters
    /// * `[in] nTableRef` The existing table number ID.
    /// * `[in] varSectionName` UPT section string name.
    /// * All other parameters are general section properties (AX, D, TD, B, TB, IZ, IY, IX, SZ, SY, AY, AZ, PZ, PY, HSS, DEE)
    /// # Return values
    /// * `0` OK.
    /// * `-6032` Unable to add section varSectionName in UPT nTableRef.
    /// * `-6045` Section with same name exits in UPT nTableRef.
    pub fn add_upt_property_general(
        &self,
        table_ref: i32,
        section_name: &str,
        ax: f64,
        d: f64,
        td: f64,
        b: f64,
        tb: f64,
        iz: f64,
        iy: f64,
        ix: f64,
        sz: f64,
        sy: f64,
        ay: f64,
        az: f64,
        pz: f64,
        py: f64,
        hss: f64,
        dee: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(dee),
                VARIANT::from(hss),
                VARIANT::from(py),
                VARIANT::from(pz),
                VARIANT::from(az),
                VARIANT::from(ay),
                VARIANT::from(sy),
                VARIANT::from(sz),
                VARIANT::from(ix),
                VARIANT::from(iy),
                VARIANT::from(iz),
                VARIANT::from(tb),
                VARIANT::from(b),
                VARIANT::from(td),
                VARIANT::from(d),
                VARIANT::from(ax),
                VARIANT::from(section_name),
                VARIANT::from(table_ref),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "AddUPTPropertyGENERAL", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::add_upt_property_general: {}", e),
            }
        }
    }

    /// Add I type to an defined UPT section.
    /// # Parameters
    /// * `[in] nTableRef` The existing table number ID.
    /// * `[in] varSectionName` UPT section string name.
    /// * All other parameters are I-section properties (DWW, TWW, DWW1, BFF, TFF, BFF1, TFF1, AYF, AZF, XIF)
    /// # Return values
    /// * `0` OK.
    /// * `-6032` Unable to add section varSectionName in UPT nTableRef.
    /// * `-6045` Section with same name exits in UPT nTableRef.
    pub fn add_upt_property_isection(
        &self,
        table_ref: i32,
        section_name: &str,
        dww: f64,
        tww: f64,
        dww1: f64,
        bff: f64,
        tff: f64,
        bff1: f64,
        tff1: f64,
        ayf: f64,
        azf: f64,
        xif: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(xif),
                VARIANT::from(azf),
                VARIANT::from(ayf),
                VARIANT::from(tff1),
                VARIANT::from(bff1),
                VARIANT::from(tff),
                VARIANT::from(bff),
                VARIANT::from(dww1),
                VARIANT::from(tww),
                VARIANT::from(dww),
                VARIANT::from(section_name),
                VARIANT::from(table_ref),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "AddUPTPropertyISECTION", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::add_upt_property_isection: {}", e),
            }
        }
    }

    /// Add tee type to an defined UPT section.
    /// # Parameters
    /// * `[in] nTableRef` The existing table number ID.
    /// * `[in] varSectionName` UPT section string name.
    /// * All other parameters are tee section properties (AX, D, WF, TF, TW, IZ, IY, IX, CY, AY, AZ)
    /// # Return values
    /// * `0` OK.
    /// * `-6032` Unable to add section varSectionName in UPT nTableRef.
    /// * `-6045` Section with same name exits in UPT nTableRef.
    pub fn add_upt_property_tee(
        &self,
        table_ref: i32,
        section_name: &str,
        ax: f64,
        d: f64,
        wf: f64,
        tf: f64,
        tw: f64,
        iz: f64,
        iy: f64,
        ix: f64,
        cy: f64,
        ay: f64,
        az: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(az),
                VARIANT::from(ay),
                VARIANT::from(cy),
                VARIANT::from(ix),
                VARIANT::from(iy),
                VARIANT::from(iz),
                VARIANT::from(tw),
                VARIANT::from(tf),
                VARIANT::from(wf),
                VARIANT::from(d),
                VARIANT::from(ax),
                VARIANT::from(section_name),
                VARIANT::from(table_ref),
            ];
            let result_variant = invoke_method(&self.dispatch, "AddUPTPropertyTEE", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::add_upt_property_tee: {}", e),
            }
        }
    }

    /// Add wide flange type to an defined UPT section.
    /// # Parameters
    /// * `[in] nTableRef` The existing table number ID.
    /// * `[in] varSectionName` UPT section string name.
    /// * All other parameters are wide flange properties (AX, D, TW, WF, TF, IZ, IY, IX, AY, AZ)
    /// # Return values
    /// * `0` OK.
    /// * `-6032` Unable to add section varSectionName in UPT nTableRef.
    /// * `-6045` Section with same name exits in UPT nTableRef.
    pub fn add_upt_property_wide_flange(
        &self,
        table_ref: i32,
        section_name: &str,
        ax: f64,
        d: f64,
        tw: f64,
        wf: f64,
        tf: f64,
        iz: f64,
        iy: f64,
        ix: f64,
        ay: f64,
        az: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(az),
                VARIANT::from(ay),
                VARIANT::from(ix),
                VARIANT::from(iy),
                VARIANT::from(iz),
                VARIANT::from(tf),
                VARIANT::from(wf),
                VARIANT::from(tw),
                VARIANT::from(d),
                VARIANT::from(ax),
                VARIANT::from(section_name),
                VARIANT::from(table_ref),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "AddUPTPropertyWIDEFLANGE", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::add_upt_property_wide_flange: {}", e),
            }
        }
    }

    /// Add wide flange type with additional composite and bottom steel plate to an defined UPT section.
    /// # Parameters
    /// * `[in] nTableRef` The existing table number ID.
    /// * `[in] varSectionName` UPT section string name.
    /// * `[in] varPropSpecArray` VARIANT double array of Profile Specifications data (size 12, 16, or 19)
    /// # Return values
    /// * `TRUE/1` OK.
    /// * `FALSE/0` Error
    pub fn add_upt_property_wide_flange_composite(
        &self,
        table_ref: i32,
        section_name: &str,
        prop_spec_array: Vec<f64>,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_specs = safe_array_from_vec1d::<f64>(prop_spec_array)?;
            let variant_specs = variant_from_raw_pointer::<SafeArray<f64>>(sa_specs);

            let mut params = [
                variant_specs,
                VARIANT::from(section_name),
                VARIANT::from(table_ref),
            ];
            let result_variant = invoke_method(
                &self.dispatch,
                "AddUPTPropertyWIDEFLANGECOMPOSITE",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let success = VariantToInt32(&var as *const VARIANT).unwrap() > 0;
                    anyOk(success)
                }
                Err(e) => bail!(
                    "Error::Property::add_upt_property_wide_flange_composite: {}",
                    e
                ),
            }
        }
    }

    /// Add unequal wide flange to an defined UPT section.
    /// # Parameters
    /// * `[in] nTableRef` The existing table number ID.
    /// * `[in] varSectionName` UPT section string name(Type: string).
    /// * `[in] varPropSpecArray` section property array (Type: double array with 12 size allocated).
    /// # Return values
    /// * `TRUE` Add unequal wide flange successful.
    /// * `FALSE` Add unequal wide flange generate error.
    pub fn add_upt_property_wide_flange_unequal(
        &self,
        table_ref: i32,
        section_name: &str,
        prop_spec_array: Vec<f64>,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_specs = safe_array_from_vec1d::<f64>(prop_spec_array)?;
            let variant_specs = variant_from_raw_pointer::<SafeArray<f64>>(sa_specs);

            let mut params = [
                variant_specs,
                VARIANT::from(section_name),
                VARIANT::from(table_ref),
            ];
            let result_variant = invoke_method(
                &self.dispatch,
                "AddUPTPropertyWIDEFLANGEUNEQUAL",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let success = VariantToInt32(&var as *const VARIANT).unwrap() > 0;
                    anyOk(success)
                }
                Err(e) => bail!(
                    "Error::Property::add_upt_property_wide_flange_unequal: {}",
                    e
                ),
            }
        }
    }

    /// Creates a section property from User Provided Table (UPT).
    /// # Parameters
    /// * `[in] nTableID` The existing table number ID.
    /// * `[in] strSectionName` UPT section string name.
    /// # Return values
    /// * `<Val>` Section property number ID.
    /// * `-1` General error.
    pub fn create_property_from_upt_table(
        &self,
        table_id: i32,
        section_name: &str,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(section_name), VARIANT::from(table_id)];
            let result_variant =
                invoke_method(&self.dispatch, "CreatePropertyFromUPTTable", &mut params);
            match result_variant {
                Ok(var) => {
                    let property_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(property_id)
                }
                Err(e) => bail!("Error::Property::create_property_from_upt_table: {}", e),
            }
        }
    }

    /// Creates user provided table (UPT).
    /// # Parameters
    /// * `[in] nTableType` Type of the table:
    ///   - 1: scUserTableWideFlangeTitle
    ///   - 2: scUserTableChannelTitle
    ///   - 3: scUserTableAngleTitle
    ///   - 4: scUserTableDoubleAngleTitle
    ///   - 5: scUserTableTeeTitle
    ///   - 6: scUserTablePipeTitle
    ///   - 7: scUserTableTubeTitle
    ///   - 8: scUserTableGeneralTitle
    ///   - 9: scUserTableIsectionTitle
    ///   - 10: scUserTablePrismaticTitle
    /// # Return values
    /// * `<Val>` User Provided Table (UPT) number ID.
    /// * `-6031` Cannot create UPT. Unknown table type specified.
    pub fn create_upt_table(&self, table_type: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(table_type)];
            let result_variant = invoke_method(&self.dispatch, "CreateUPTTable", &mut params);
            match result_variant {
                Ok(var) => {
                    let table_id = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(table_id)
                }
                Err(e) => bail!("Error::Property::create_upt_table: {}", e),
            }
        }
    }

    /// Get Profile Points coordinate from User Provided general section Table (UPT).
    /// # Parameters
    /// * `[in] varTableRef` The existing table number ID(Type: Long).
    /// * `[in] strSectionName` UPT section string name(Type: String).
    /// * `[in] varIsInner` (Reserved for inner points, set it to false)(Type: Boolean).
    /// * `[out] varZP` Profile Points coordinate in Z(Type: double array).
    /// * `[out] varYP` Profile Points coordinate in Y(Type: double array).
    /// # Return values
    /// * `<Val>` Profile Points Count.
    /// * `0` General error.
    pub fn get_upt_general_profile_boundary_points(
        &self,
        table_ref: i32,
        section_name: &str,
        is_inner: bool,
        point_count: i32,
    ) -> Result<(i32, Vec<f64>, Vec<f64>), anyErr> {
        unsafe {
            let mut zp_sa = SafeArrayCreateVector(VT_R8, 0, point_count as u32);
            let mut yp_sa = SafeArrayCreateVector(VT_R8, 0, point_count as u32);

            let zp_sa_ptr = &mut zp_sa as *mut *mut SAFEARRAY;
            let yp_sa_ptr = &mut yp_sa as *mut *mut SAFEARRAY;

            let mut params = [
                variant_from_raw_pointer::<SafeArrayP<f64>>(yp_sa_ptr),
                variant_from_raw_pointer::<SafeArrayP<f64>>(zp_sa_ptr),
                VARIANT::from(is_inner),
                VARIANT::from(section_name),
                VARIANT::from(table_ref),
            ];

            let result_variant = invoke_method(
                &self.dispatch,
                "GetUptGeneralProfileBoundaryPoints",
                &mut params,
            );

            match result_variant {
                Ok(var) => {
                    let points_count = VariantToInt32(&var as *const VARIANT).unwrap();

                    let mut zp_points = Vec::with_capacity(points_count as usize);
                    let mut yp_points = Vec::with_capacity(points_count as usize);

                    for i in 0..points_count {
                        let mut index = i as i32;
                        let mut zp_value = 0.0;
                        let mut yp_value = 0.0;

                        let _ = SafeArrayGetElement(
                            zp_sa,
                            &mut index as *mut i32,
                            &mut zp_value as *mut f64 as *mut c_void,
                        )?;

                        let _ = SafeArrayGetElement(
                            yp_sa,
                            &mut index as *mut i32,
                            &mut yp_value as *mut f64 as *mut c_void,
                        )?;

                        zp_points.push(zp_value);
                        yp_points.push(yp_value);
                    }

                    anyOk((points_count, zp_points, yp_points))
                }
                Err(e) => bail!(
                    "Error::Property::get_upt_general_profile_boundary_points: {}",
                    e
                ),
            }
        }
    }

    /// Get profile points count from user provided general section table (UPT).
    /// # Parameters
    /// * `[in] varTableRef` The existing table number ID(Type: Long).
    /// * `[in] varSectionName` UPT section string name(Type: String).
    /// * `[out] varCountOfOuter` Count of outer profile points(Type: Long).
    /// * `[out] varCountOfInner` Count of inner profile points(Reserved, not be used now)(Type: Long).
    /// # Return values
    /// * `True` Get profile points count successful.
    /// * `False` General error.
    pub fn get_upt_general_profile_points_count(
        &self,
        table_ref: i32,
        section_name: &str,
    ) -> Result<(bool, i32, i32), anyErr> {
        unsafe {
            let count_outer_ptr = &mut 0i32 as *mut i32;
            let count_inner_ptr = &mut 0i32 as *mut i32;

            let mut params = [
                variant_from_raw_pointer::<i32>(count_inner_ptr),
                variant_from_raw_pointer::<i32>(count_outer_ptr),
                VARIANT::from(section_name),
                VARIANT::from(table_ref),
            ];

            let result_variant = invoke_method(
                &self.dispatch,
                "GetUptGeneralProfilePointsCount",
                &mut params,
            );

            match result_variant {
                Ok(var) => {
                    let success = VariantToInt32(&var as *const VARIANT).unwrap() > 0;
                    anyOk((success, *count_outer_ptr, *count_inner_ptr))
                }
                Err(e) => bail!(
                    "Error::Property::get_upt_general_profile_points_count: {}",
                    e
                ),
            }
        }
    }

    /// Stress Location in local coordinate from User Provided general section Table (UPT).
    /// # Parameters
    /// * `[in] varTableRef` The existing table number ID(Type: Long).
    /// * `[in] varSectionName` UPT section string name(Type: String).
    /// * `[out] varZP` Stress Location coordinate in Z(Type: double array of size 4).
    /// * `[out] varYP` Stress Location coordinate in Y(Type: double array of size 4).
    /// # Return values
    /// * `<Val>` Stress Location Count.
    /// * `0` General error.
    pub fn get_upt_general_stress_location_points(
        &self,
        table_ref: i32,
        section_name: &str,
    ) -> Result<(i32, Vec<f64>, Vec<f64>), anyErr> {
        unsafe {
            let mut zp_sa = SafeArrayCreateVector(VT_R8, 0, 4);
            let mut yp_sa = SafeArrayCreateVector(VT_R8, 0, 4);

            let zp_sa_ptr = &mut zp_sa as *mut *mut SAFEARRAY;
            let yp_sa_ptr = &mut yp_sa as *mut *mut SAFEARRAY;

            let mut params = [
                variant_from_raw_pointer::<SafeArrayP<f64>>(yp_sa_ptr),
                variant_from_raw_pointer::<SafeArrayP<f64>>(zp_sa_ptr),
                VARIANT::from(section_name),
                VARIANT::from(table_ref),
            ];

            let result_variant = invoke_method(
                &self.dispatch,
                "GetUptGeneralStressLocationPoints",
                &mut params,
            );

            match result_variant {
                Ok(var) => {
                    let location_count = VariantToInt32(&var as *const VARIANT).unwrap();

                    let mut zp_locations = Vec::with_capacity(4);
                    let mut yp_locations = Vec::with_capacity(4);

                    for i in 0..4 {
                        let mut index = i as i32;
                        let mut zp_value = 0.0;
                        let mut yp_value = 0.0;

                        let _ = SafeArrayGetElement(
                            zp_sa,
                            &mut index as *mut i32,
                            &mut zp_value as *mut f64 as *mut c_void,
                        )?;

                        let _ = SafeArrayGetElement(
                            yp_sa,
                            &mut index as *mut i32,
                            &mut yp_value as *mut f64 as *mut c_void,
                        )?;

                        zp_locations.push(zp_value);
                        yp_locations.push(yp_value);
                    }

                    anyOk((location_count, zp_locations, yp_locations))
                }
                Err(e) => bail!(
                    "Error::Property::get_upt_general_stress_location_points: {}",
                    e
                ),
            }
        }
    }

    /// Assign beta angle to beam(s).
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID(s) of VARIANT array.
    /// * `[in] varfBetaAngle` The beta angle in degrees.
    /// # Return values
    /// * `1` OK.
    /// * `0` General Error
    /// * `0` 1 dimensional array of long expected.
    /// * `0` Library Error: Unable to assign BETA angle.
    pub fn assign_beta_angle(&self, beam_nos: Vec<i32>, beta_angle: f64) -> Result<i32, anyErr> {
        unsafe {
            let sa_beams = safe_array_from_vec1d::<i32>(beam_nos)?;
            let variant_beams = variant_from_raw_pointer::<SafeArray<i32>>(sa_beams);

            let mut params = [VARIANT::from(beta_angle), variant_beams];
            let result_variant = invoke_method(&self.dispatch, "AssignBetaAngle", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::assign_beta_angle: {}", e),
            }
        }
    }

    /// Retrieve beta angle of the specified beam member.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `<Val>` Beta angle.
    /// * `-3001` Cannot find member varnBeamNo.
    pub fn get_beta_angle(&self, beam_no: i32) -> Result<f64, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(&self.dispatch, "GetBetaAngle", &mut params);
            match result_variant {
                Ok(var) => {
                    let beta_angle = VariantToDouble(&var as *const VARIANT).unwrap();
                    anyOk(beta_angle)
                }
                Err(e) => bail!("Error::Property::get_beta_angle: {}", e),
            }
        }
    }

    /// Get beam section string name.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Returns
    /// * The section string name.
    pub fn get_beam_section_name(&self, beam_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(&self.dispatch, "GetBeamSectionName", &mut params);
            match result_variant {
                Ok(var) => {
                    let result = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(result)
                }
                Err(e) => bail!("Error::Property::get_beam_section_name: {}", e),
            }
        }
    }

    /// Returns the section property reference number of the specified beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Int).
    /// # Return values
    /// * Section property ref number assigned to the specified beam. Zero if not found.
    pub fn get_beam_section_property_ref_no(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant =
                invoke_method(&self.dispatch, "GetBeamSectionPropertyRefNo", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::get_beam_section_property_ref_no: {}", e),
            }
        }
    }

    /// Gets the section property type number of the specified beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number.
    /// # Return values
    /// * `<Val>` The section property type number.
    /// * `0` Error.
    pub fn get_beam_section_property_type_no(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant =
                invoke_method(&self.dispatch, "GetBeamSectionPropertyTypeNo", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::get_beam_section_property_type_no: {}", e),
            }
        }
    }

    /// Returns the section property Values of the specified beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Int/Long).
    /// * `[out] varPropType` Number referring to the property type table (Type: Long).
    /// * `[out] varProperties` A double VARIANT array for section property parameters (Type: Array size of 24 doubles).
    /// # Return values
    /// * `FALSE` Get the property Values generate Error.
    /// * `TRUE` Get the property Values Successful.
    pub fn get_beam_section_property_values_ex(
        &self,
        beam_no: i32,
    ) -> Result<(i32, Vec<f64>), anyErr> {
        unsafe {
            let prop_type_ptr = &mut 0i32 as *mut i32;
            let mut prop_values = vec![0.0f64; 24];
            let prop_values_ptr = prop_values.as_mut_ptr();

            let sa_props = SafeArrayCreateVector(VT_R8, 0, 24);
            for i in 0..24 {
                let mut index = i as i32;
                let _ = SafeArrayPutElement(
                    sa_props,
                    &mut index as *mut i32,
                    &mut prop_values[i as usize] as *mut f64 as *mut c_void,
                )?;
            }

            let mut params = [
                variant_from_raw_pointer::<SafeArray<f64>>(sa_props),
                variant_from_raw_pointer::<i32>(prop_type_ptr),
                VARIANT::from(beam_no),
            ];

            let result_variant = invoke_method(
                &self.dispatch,
                "GetBeamSectionPropertyValuesEx",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    if result_code == 1 {
                        // Extract values from SafeArray
                        let mut final_values = Vec::with_capacity(24);
                        for i in 0..24 {
                            let mut index = i as i32;
                            let mut value = 0.0f64;
                            let _ = SafeArrayGetElement(
                                sa_props,
                                &mut index as *mut i32,
                                &mut value as *mut f64 as *mut c_void,
                            )?;
                            final_values.push(value);
                        }
                        anyOk((*prop_type_ptr, final_values))
                    } else {
                        bail!(
                            "Error::Property::get_beam_section_property_values_ex: Failed to get property values"
                        )
                    }
                }
                Err(e) => bail!(
                    "Error::Property::get_beam_section_property_values_ex: {}",
                    e
                ),
            }
        }
    }

    /// Returns the total count of Section Property values.
    /// # Return values
    /// * `<Val>` The total count of Section Property values.
    pub fn get_count_of_section_property_values_ex(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "GetCountofSectionPropertyValuesEx", &mut []) };
        match result_variant {
            Ok(var) => {
                let count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(count)
            }
            Err(e) => bail!(
                "Error::Property::get_count_of_section_property_values_ex: {}",
                e
            ),
        }
    }

    /// Get The country CODE for the specified member.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `<Val>` The country CODE.
    /// * `-3001` Cannot find member varnBeamNo.
    /// * `-6022` No property is attached to the member/element.
    pub fn get_country_table_no(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(&self.dispatch, "GetCountryTableNo", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::get_country_table_no: {}", e),
            }
        }
    }

    /// Get section assigned beam count.
    /// # Parameters
    /// * `[in] varProfRefNo` Assign Profile Type.
    /// # Return values
    /// * `<Val>` The section table number.
    /// * `-3001` Cannot find member varnBeamNo.
    /// * `-6004` Section not found in profile database.
    /// * `-6022` No property is attached to the member/element.
    pub fn get_section_property_assigned_beam_count(
        &self,
        prof_ref_no: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(prof_ref_no)];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetSectionPropertyAssignedBeamCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::get_section_property_assigned_beam_count: {}",
                    e
                ),
            }
        }
    }

    /// Get section assigned beam list.
    /// # Parameters
    /// * `[in] varProfRefNo` Assign Profile Type.
    /// * `[out] nBeamList` List of beam.
    /// # Return values
    /// * `<Val>` The section table number.
    /// * `-3001` Cannot find member varnBeamNo.
    /// * `-6004` Section not found in profile database.
    /// * `-6022` No property is attached to the member/element.
    pub fn get_section_property_assigned_beam_list(
        &self,
        prof_ref_no: i32,
    ) -> Result<Vec<i32>, anyErr> {
        unsafe {
            let beam_count = self.get_section_property_assigned_beam_count(prof_ref_no)?;
            let mut psa = SafeArrayCreateVector(VT_I4, 0, beam_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant, VARIANT::from(prof_ref_no)];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetSectionPropertyAssignedBeamList",
                &mut params,
            );
            match result_variant {
                Ok(_) => {
                    let beam_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut beam_arr = Vec::with_capacity(beam_count as usize);
                    for i in 0..beam_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            beam_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        beam_arr.push(value as i32);
                    }
                    anyOk(beam_arr)
                }
                Err(e) => bail!(
                    "Error::Property::get_section_property_assigned_beam_list: {}",
                    e
                ),
            }
        }
    }

    /// Return total number of different sectional properties exist in the current STAAD file.
    /// # Returns
    /// * The total number of different sectional properties.
    pub fn get_section_property_count(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "GetSectionPropertyCount", &mut []) };
        match result_variant {
            Ok(var) => {
                let count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(count)
            }
            Err(e) => bail!("Error::Property::get_section_property_count: {}", e),
        }
    }

    /// Return the country reference number for the section property reference number specified.
    /// # Parameters
    /// * `[in] varSecRefNo` The assigned section property ID (Type: long/Integer).
    /// # Return values
    /// * `<Val>` Country CODE (Type: long/Integer).
    /// * `-6025` No property is defined in the model.
    pub fn get_section_property_country(&self, sec_ref_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(sec_ref_no)];
            let result_variant =
                invoke_method(&self.dispatch, "GetSectionPropertyCountry", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::get_section_property_country: {}", e),
            }
        }
    }

    /// Gets the list of Section Property Reference IDs.
    /// # Parameters
    /// * `[out] nPropList` List of Section Property reference IDs (Type: Long Array).
    /// # Return values
    /// * `FALSE` Get Section Property reference IDs generate Error.
    /// * `TRUE` Get Section Property reference IDs Successful.
    pub fn get_section_property_list(&self) -> Result<Vec<i32>, anyErr> {
        unsafe {
            let prop_count = self.get_section_property_count()?;
            let mut psa = SafeArrayCreateVector(VT_I4, 0, prop_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant];
            let result_variant =
                invoke_method(&self.dispatch, "GetSectionPropertyList", &mut params);
            match result_variant {
                Ok(_) => {
                    let prop_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut prop_arr = Vec::with_capacity(prop_count as usize);
                    for i in 0..prop_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            prop_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        prop_arr.push(value as i32);
                    }
                    anyOk(prop_arr)
                }
                Err(e) => bail!("Error::Property::get_section_property_list: {}", e),
            }
        }
    }

    /// Get the property name for the specified section property reference number.
    /// # Parameters
    /// * `[in] varSecRefNo` The assigned section property ID.
    /// * `[out] varstrName` Identification title of material.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    /// * `-6025` No property is defined in the model.
    pub fn get_section_property_name(&self, sec_ref_no: i32) -> Result<(i32, String), anyErr> {
        unsafe {
            let name_ptr = &mut BSTR::default() as *mut BSTR;
            let mut params = [
                variant_from_raw_pointer::<BSTR>(name_ptr),
                VARIANT::from(sec_ref_no),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "GetSectionPropertyName", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    let name_bstr = &*name_ptr;
                    anyOk((result_code, name_bstr.to_string()))
                }
                Err(e) => bail!("Error::Property::get_section_property_name: {}", e),
            }
        }
    }

    /// Return the section property type for the specified section property reference number.
    /// # Parameters
    /// * `[in] varSecRefNo` The assigned section property ID.
    /// # Return values
    /// * `<Val>` Number referring to Section Type Code table.
    /// * `-6025` No property is defined in the model.
    pub fn get_section_property_type(&self, sec_ref_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(sec_ref_no)];
            let result_variant =
                invoke_method(&self.dispatch, "GetSectionPropertyType", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::get_section_property_type: {}", e),
            }
        }
    }

    /// Retrieve long member properties of the specified beam member.
    /// # Parameters
    /// * `[in] varProfRefNo` Assign Profile Type.
    /// * `[out] varfWidth` Width of the section (WID).
    /// * `[out] varfDepth` Depth of the section (DEP).
    /// * `[out] varfAx` Cross section area (Ax).
    /// * `[out] varfAy` Shear area in local y-axis (Ay).
    /// * `[out] varfAz` Shear area in local z-axis (Az).
    /// * `[out] varfIx` Moment of inertia about local z-axis (Ix).
    /// * `[out] varfIy` Moment of inertia about local y-axis (Iy).
    /// * `[out] varfIz` Torsional constant (Iz).
    /// * `[out] varfTf` Thickness of top flange (Tf).
    /// * `[out] varfTw` Thickness of web (Tw).
    /// # Return values
    /// * `0` OK.
    /// * `-3001` Cannot find member nMemberNo.
    /// * `-6022` No property is attached to the member/element.
    pub fn get_section_property_values(&self, prof_ref_no: i32) -> Result<(i32, Vec<f64>), anyErr> {
        unsafe {
            let width_ptr = &mut 0.0f64 as *mut f64;
            let depth_ptr = &mut 0.0f64 as *mut f64;
            let ax_ptr = &mut 0.0f64 as *mut f64;
            let ay_ptr = &mut 0.0f64 as *mut f64;
            let az_ptr = &mut 0.0f64 as *mut f64;
            let ix_ptr = &mut 0.0f64 as *mut f64;
            let iy_ptr = &mut 0.0f64 as *mut f64;
            let iz_ptr = &mut 0.0f64 as *mut f64;
            let tf_ptr = &mut 0.0f64 as *mut f64;
            let tw_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(tw_ptr),
                variant_from_raw_pointer::<f64>(tf_ptr),
                variant_from_raw_pointer::<f64>(iz_ptr),
                variant_from_raw_pointer::<f64>(iy_ptr),
                variant_from_raw_pointer::<f64>(ix_ptr),
                variant_from_raw_pointer::<f64>(az_ptr),
                variant_from_raw_pointer::<f64>(ay_ptr),
                variant_from_raw_pointer::<f64>(ax_ptr),
                variant_from_raw_pointer::<f64>(depth_ptr),
                variant_from_raw_pointer::<f64>(width_ptr),
                VARIANT::from(prof_ref_no),
            ];

            let result_variant =
                invoke_method(&self.dispatch, "GetSectionPropertyValues", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    let values = vec![
                        *width_ptr, *depth_ptr, *ax_ptr, *ay_ptr, *az_ptr, *ix_ptr, *iy_ptr,
                        *iz_ptr, *tf_ptr, *tw_ptr,
                    ];
                    anyOk((result_code, values))
                }
                Err(e) => bail!("Error::Property::get_section_property_values: {}", e),
            }
        }
    }

    /// Get all parameters of a specified section property by section property ID.
    /// # Parameters
    /// * `[in] propNo` Section property ID.
    /// * `[out] propType` Number referring to the property type table.
    /// * `[out] propValues` A double VARIANT array for section property parameters.
    /// # Return values
    /// * `1` OK.
    /// * `0` Error.
    pub fn get_section_property_values_ex(&self, prop_no: i32) -> Result<(i32, Vec<f64>), anyErr> {
        unsafe {
            let count = self.get_count_of_section_property_values_ex()?;
            let prop_type_ptr = &mut 0i32 as *mut i32;

            let mut psa = SafeArrayCreateVector(VT_R8, 0, count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<f64>>(psa_ptr);

            let mut params = [
                variant,
                variant_from_raw_pointer::<i32>(prop_type_ptr),
                VARIANT::from(prop_no),
            ];

            let result_variant =
                invoke_method(&self.dispatch, "GetSectionPropertyValuesEx", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    if result_code == 1 {
                        let prop_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                        let mut prop_values = Vec::with_capacity(count as usize);
                        for i in 0..count {
                            let mut index = i as i32;
                            let mut value = 0.0f64;
                            let _ = SafeArrayGetElement(
                                prop_safe_arr,
                                &mut index as *mut i32,
                                &mut value as *mut f64 as *mut c_void,
                            )?;
                            prop_values.push(value);
                        }
                        anyOk((*prop_type_ptr, prop_values))
                    } else {
                        bail!(
                            "Error::Property::get_section_property_values_ex: Failed to get property values"
                        )
                    }
                }
                Err(e) => bail!("Error::Property::get_section_property_values_ex: {}", e),
            }
        }
    }

    /// Get section table number.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `<Val>` The section table number.
    /// * `-3001` Cannot find member varnBeamNo.
    /// * `-6004` Section not found in profile database.
    /// * `-6022` No property is attached to the member/element.
    pub fn get_section_table_no(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(&self.dispatch, "GetSectionTableNo", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::get_section_table_no: {}", e),
            }
        }
    }

    /// Get the Shape Code with specific Country and specific Section Name.
    /// # Parameters
    /// * `[in] Country` Country ID (Type: long/Integer).
    /// * `[in] SectionName` Section Name (Type: String).
    /// # Return values
    /// * `<Val>` Shape Code (Type: long/Integer)
    /// * `-1` Get the Shape Code generate Error.
    pub fn get_shape_code(&self, country: i32, section_name: &str) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(section_name), VARIANT::from(country)];
            let result_variant = invoke_method(&self.dispatch, "GetShapeCode", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::get_shape_code: {}", e),
            }
        }
    }

    /// Create a section Property From User Table.
    /// # Parameters
    /// * `[in] SectionName` Section name (Type: string)
    /// * `[in] TableNo` Table Id (Type: long/integer)
    /// # Returns
    /// * The section property Ref ID.
    pub fn create_property_from_user_table(
        &self,
        section_name: &str,
        table_no: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(table_no), VARIANT::from(section_name)];
            let result_variant =
                invoke_method(&self.dispatch, "CreatePropertyFromUserTable", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_property_from_user_table: {}", e),
            }
        }
    }

    /// Create User Provided Table (UPT) specified by table number ID and Table Type.
    /// # Parameters
    /// * `[in] nTableRef` A new table number ID.
    /// * `[in] nTableType` Type of the table.
    /// # Return values
    /// * `0` Create new User Provided Table generate Error.
    /// * `<Val>` Table number ID.
    pub fn create_upt_table_ex(&self, table_no: i32, table_type: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(table_type), VARIANT::from(table_no)];
            let result_variant = invoke_method(&self.dispatch, "CreateUPTTableEx", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_upt_table_ex: {}", e),
            }
        }
    }

    /// Search for the UPT table by table type number.
    /// # Parameters
    /// * `[in] varTableType` Table type number.
    /// # Return values
    /// * `<Val>` Table number ID.
    /// * `-6036` Cannot find UPT. Unknown table type varTableType specified.
    pub fn find_upt_table(&self, table_type: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(table_type)];
            let result_variant = invoke_method(&self.dispatch, "FindUPTTable", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::find_upt_table: {}", e),
            }
        }
    }

    /// Get the number of UPT tables.
    /// # Returns
    /// * The number of UPT tables.
    pub fn get_user_provided_table_count(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "GetUserProvidedTableCount", &mut []) };
        match result_variant {
            Ok(var) => {
                let count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(count)
            }
            Err(e) => bail!("Error::Property::get_user_provided_table_count: {}", e),
        }
    }

    /// Get the UPT table ID list.
    /// # Parameters
    /// * `[out] nTableList` UPT table ID list (Type: Long Array).
    /// # Return values
    /// * `TRUE` Retrieves the UPT Table list successfully.
    /// * `FALSE` Unable to retrieve the UPT table list.
    pub fn get_user_provided_table_list(&self) -> Result<Vec<i32>, anyErr> {
        unsafe {
            let table_count = self.get_user_provided_table_count()?;
            let mut psa = SafeArrayCreateVector(VT_I4, 0, table_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant];
            let result_variant =
                invoke_method(&self.dispatch, "GetUserProvidedTableList", &mut params);
            match result_variant {
                Ok(_) => {
                    let table_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut table_arr = Vec::with_capacity(table_count as usize);
                    for i in 0..table_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            table_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        table_arr.push(value as i32);
                    }
                    anyOk(table_arr)
                }
                Err(e) => bail!("Error::Property::get_user_provided_table_list: {}", e),
            }
        }
    }

    /// Get section user provided table number ID by user table index.
    /// # Parameters
    /// * `[in] varSecRefNo` User Provided Table (UPT) index.
    /// # Return values
    /// * `<Val>` User Provided Table (UPT) number ID.
    /// * `-1` General error.
    pub fn get_user_provided_table_no(&self, sec_ref_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(sec_ref_no)];
            let result_variant =
                invoke_method(&self.dispatch, "GetUserProvidedTableNo", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::get_user_provided_table_no: {}", e),
            }
        }
    }

    /// Get the number of section defined in specified User Provided Table (UPT).
    /// # Parameters
    /// * `[in] nTableNo` The User Provided Table (UPT) number ID.
    /// # Returns
    /// * The number of section(s) in given UPT.
    pub fn get_user_provided_table_section_count(&self, table_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(table_no)];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetUserProvidedTableSectionCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(count)
                }
                Err(e) => bail!(
                    "Error::Property::get_user_provided_table_section_count: {}",
                    e
                ),
            }
        }
    }

    /// Get the list of section names in specified User Provided Table (UPT).
    /// # Parameters
    /// * `[in] nTableNo` The User Provided Table (UPT) number ID.
    /// * `[out] sectionList` CSStringList with indexes and corresponding section string names included.
    /// # Returns
    /// * The number of section(s) in given UPT.
    pub fn get_user_provided_table_section_list(
        &self,
        table_no: i32,
    ) -> Result<(i32, Vec<String>), anyErr> {
        unsafe {
            let section_count = self.get_user_provided_table_section_count(table_no)?;
            let mut psa = SafeArrayCreateVector(VT_BSTR, 0, section_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<BSTR>>(psa_ptr);

            let mut params = [variant, VARIANT::from(table_no)];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetUserProvidedTableSectionList",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    let section_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut section_arr = Vec::with_capacity(section_count as usize);
                    for i in 0..section_count {
                        let mut index = i as i32;
                        let mut value = BSTR::default();
                        let _ = SafeArrayGetElement(
                            section_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut BSTR as *mut c_void,
                        )?;
                        section_arr.push(value.to_string());
                    }
                    anyOk((result_code, section_arr))
                }
                Err(e) => bail!(
                    "Error::Property::get_user_provided_table_section_list: {}",
                    e
                ),
            }
        }
    }

    /// Get the section type and section properties of specified UPT section.
    /// # Parameters
    /// * `[in] nTableNo` The User Provided Table (UPT) number ID.
    /// * `[in] sectionName` UPT section string name given to this section property.
    /// * `[out] sectionType` UPT Section Type from the property type table.
    /// * `[out] propertyVals` A double VARIANT array for section property parameters.
    /// # Return values
    /// * `TRUE` Successful.
    /// * `FALSE` Unsuccessful.
    pub fn get_user_provided_table_section_properties(
        &self,
        table_no: i32,
        section_name: &str,
    ) -> Result<(i32, Vec<f64>), anyErr> {
        unsafe {
            let count = self.get_count_of_section_property_values_ex()?;
            let section_type_ptr = &mut 0i32 as *mut i32;

            let mut psa = SafeArrayCreateVector(VT_R8, 0, count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<f64>>(psa_ptr);

            let mut params = [
                variant,
                variant_from_raw_pointer::<i32>(section_type_ptr),
                VARIANT::from(section_name),
                VARIANT::from(table_no),
            ];

            let result_variant = invoke_method(
                &self.dispatch,
                "GetUserProvidedTableSectionProperties",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    if result_code == 1 {
                        let prop_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                        let mut prop_values = Vec::with_capacity(count as usize);
                        for i in 0..count {
                            let mut index = i as i32;
                            let mut value = 0.0f64;
                            let _ = SafeArrayGetElement(
                                prop_safe_arr,
                                &mut index as *mut i32,
                                &mut value as *mut f64 as *mut c_void,
                            )?;
                            prop_values.push(value);
                        }
                        anyOk((*section_type_ptr, prop_values))
                    } else {
                        bail!(
                            "Error::Property::get_user_provided_table_section_properties: Failed to get section properties"
                        )
                    }
                }
                Err(e) => bail!(
                    "Error::Property::get_user_provided_table_section_properties: {}",
                    e
                ),
            }
        }
    }

    /// Get the user provided table section property count in specified User Provided Table (UPT).
    /// # Parameters
    /// * `[in] nTableNo` The User Provided Table (UPT) number ID.
    /// * `[in] sectionName` UPT section string name given to this section property.
    /// # Returns
    /// * The number of section(s) in given UPT.
    pub fn get_user_provided_table_section_property_count(
        &self,
        table_no: i32,
        section_name: &str,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(section_name), VARIANT::from(table_no)];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetUserProvidedTableSectionPropertyCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(count)
                }
                Err(e) => bail!(
                    "Error::Property::get_user_provided_table_section_property_count: {}",
                    e
                ),
            }
        }
    }

    /// Get the user provided table section type in specified User Provided Table (UPT).
    /// # Parameters
    /// * `[in] nTableNo` The User Provided Table (UPT) number ID.
    /// * `[out] sectionType` Number referring to Section Type Code table.
    /// # Returns
    /// * The number of section(s) in given UPT.
    pub fn get_user_provided_table_section_type(
        &self,
        table_no: i32,
    ) -> Result<(i32, i32), anyErr> {
        unsafe {
            let section_type_ptr = &mut 0i32 as *mut i32;
            let mut params = [
                variant_from_raw_pointer::<i32>(section_type_ptr),
                VARIANT::from(table_no),
            ];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetUserProvidedTableSectionType",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((result_code, *section_type_ptr))
                }
                Err(e) => bail!(
                    "Error::Property::get_user_provided_table_section_type: {}",
                    e
                ),
            }
        }
    }

    /// Remove a property from User Provided Table (UPT) if exist.
    /// # Parameters
    /// * `[in] nTableRef` The existing table number ID (Long).
    /// * `[in] varSectionName` UPT section string name (String).
    /// # Return values
    /// * `0` Error
    /// * `1` OK. Property has been deleted.
    /// * `-100` Invalid Table Number or Section Name.
    pub fn remove_property_from_upt_table(
        &self,
        table_ref: i32,
        section_name: &str,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(section_name), VARIANT::from(table_ref)];
            let result_variant =
                invoke_method(&self.dispatch, "RemovePropertyFromUPTTable", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::remove_property_from_upt_table: {}", e),
            }
        }
    }

    /// Remove the whole User Provided Table (UPT) specified by table number ID.
    /// # Parameters
    /// * `[in] nTableRef` The existing table number ID.
    /// # Return values
    /// * `TRUE/1` OK.
    /// * `FALSE/0` Error.
    pub fn remove_upt_table(&self, table_ref: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(table_ref)];
            let result_variant = invoke_method(&self.dispatch, "RemoveUPTTable", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::remove_upt_table: {}", e),
            }
        }
    }

    /// Assign beam property.
    /// # Parameters
    /// * `[in] nBeamNo` The beam number IDs in VARIANT type.
    /// * `[in] nProperty` The number ID identifying a property.
    /// # Return values
    /// * `0` OS: OK.
    /// * `-106` nBeamNo array dimension error.
    /// * `-3006` Invalid member number ID(s).
    /// * `-6001` Invalid section The assigned section property ID.
    /// * `-6002` Library Error: Property Assign.
    pub fn assign_beam_property(
        &self,
        beam_nos: Vec<i32>,
        property_id: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_beams = safe_array_from_vec1d::<i32>(beam_nos)?;
            let variant_beams = variant_from_raw_pointer::<SafeArray<i32>>(sa_beams);

            let mut params = [VARIANT::from(property_id), variant_beams];
            let result_variant = invoke_method(&self.dispatch, "AssignBeamProperty", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::assign_beam_property: {}", e),
            }
        }
    }

    /// Assign specifications to plate(s).
    /// # Parameters
    /// * `[in] varnPlateNo` The plate number ID(s) in VARIANT array.
    /// * `[in] varnSpecNo` The specification number ID.
    /// # Return values
    /// * `0` OK
    /// * `-106` 1 dimensional array of long expected.
    /// * `-6017` Library Error: Unable to assign specification.
    pub fn assign_element_spec_to_plate(
        &self,
        plate_nos: Vec<i32>,
        spec_no: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_plates = safe_array_from_vec1d::<i32>(plate_nos)?;
            let variant_plates = variant_from_raw_pointer::<SafeArray<i32>>(sa_plates);

            let mut params = [VARIANT::from(spec_no), variant_plates];
            let result_variant =
                invoke_method(&self.dispatch, "AssignElementSpecToPlate", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::assign_element_spec_to_plate: {}", e),
            }
        }
    }

    /// Assign specifications to beam(s).
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID(s) in VARIANT array.
    /// * `[in] varnSpecNo` The specification number ID.
    /// # Return values
    /// * `0` OK
    /// * `-106` 1 dimensional array of long expected.
    /// * `-6017` Library Error: Unable to assign specification.
    pub fn assign_member_spec_to_beam(
        &self,
        beam_nos: Vec<i32>,
        spec_no: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_beams = safe_array_from_vec1d::<i32>(beam_nos)?;
            let variant_beams = variant_from_raw_pointer::<SafeArray<i32>>(sa_beams);

            let mut params = [VARIANT::from(spec_no), variant_beams];
            let result_variant =
                invoke_method(&self.dispatch, "AssignMemberSpecToBeam", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::assign_member_spec_to_beam: {}", e),
            }
        }
    }

    /// Assign thickness to a plate or a set of plates. The API will skip the plate numbers that are not found if the list contains a combination of valid and invalid plates.
    /// # Parameters
    /// * `[in] nPlateNo` The plate number ID/s. (VARIANT / VARIANT Array).
    /// * `[in] nProperty` The assigned section property ID. (Long)
    /// # Return values
    /// * `0` OK.
    /// * `-1` Error.
    /// * `-106` nPlateNo array dimension error.
    /// * `-113` nPlateNo type error (Long or Int Expected)
    /// * `-4009` All the plate numbers are invalid.
    /// * `-4008` Some of the plate numbers are invalid.
    /// * `-6001` The assigned section property ID is invalid.
    pub fn assign_plate_thickness(
        &self,
        plate_nos: Vec<i32>,
        property_id: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_plates = safe_array_from_vec1d::<i32>(plate_nos)?;
            let variant_plates = variant_from_raw_pointer::<SafeArray<i32>>(sa_plates);

            let mut params = [VARIANT::from(property_id), variant_plates];
            let result_variant = invoke_method(&self.dispatch, "AssignPlateThickness", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::assign_plate_thickness: {}", e),
            }
        }
    }

    /// Assign thickness to a single plate.
    /// # Parameters
    /// * `[in] plate_no` The plate number ID.
    /// * `[in] property_id` The assigned section property ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` Error.
    /// * `-6001` The assigned section property ID is invalid.
    pub fn assign_plate_thickness_single(
        &self,
        plate_no: i32,
        property_id: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(property_id), VARIANT::from(plate_no)];
            let result_variant = invoke_method(&self.dispatch, "AssignPlateThickness", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::assign_plate_thickness_single: {}", e),
            }
        }
    }

    /// Create "Assign Profile" property.
    /// # Parameters
    /// * `[in] varnAssignType` Profile type number ID.
    /// * Type of Profile:
    ///   - Angle: 0
    ///   - Double Angle: 1
    ///   - Beam: 2
    ///   - Column: 3
    ///   - Channel: 4
    /// # Return values
    /// * `<Val>` The assigned section property ID.
    /// * `0` Library Error: Unable to create property.
    /// * `-6008` Invalid assign profile type.
    pub fn create_assign_profile_property(&self, assign_type: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(assign_type)];
            let result_variant =
                invoke_method(&self.dispatch, "CreateAssignProfileProperty", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_assign_profile_property: {}", e),
            }
        }
    }

    /// Updates all the section properties that have been designed with a SELECT MEMBER command.
    /// # Return values
    /// * `1` if assignment is successful.
    /// * `0` if assignment is unsuccessful.
    /// # Remarks
    /// Assignment will fail if there are no members in the model or design results are not available. This API will not work with physical model.
    pub fn update_properties_to_design_section(&self) -> Result<bool, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "UpdatePropertiesToDesignSection", &mut []) };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code == 1)
            }
            Err(e) => bail!(
                "Error::Property::update_properties_to_design_section: {}",
                e
            ),
        }
    }
    /// Add a control/dependent joint specification to specified node(s).
    /// # Parameters
    /// * `[in] varControlNode` Set node (number ID) control node.
    /// * `[in] varRegid_XY_YZ_ZX` Set plate rigid: all directions rigid (= 0), XY plate rigid (= 1), YZ plate rigid (= 2), ZX plate rigid (= 3), specific define FX, FY, FZ, MX, MY, MZ rigid (= others).
    /// * `[in] varFX` Rigid in X direction translation (Rigid = 1, Not Rigid = 0).
    /// * `[in] varFY` Rigid in Y direction translation (Rigid = 1, Not Rigid = 0).
    /// * `[in] varFZ` Rigid in Z direction translation (Rigid = 1, Not Rigid = 0).
    /// * `[in] varMX` Rigid in X direction rotation (Rigid = 1, Not Rigid = 0).
    /// * `[in] varMY` Rigid in Y direction rotation (Rigid = 1, Not Rigid = 0).
    /// * `[in] varMZ` Rigid in Z direction rotation (Rigid = 1, Not Rigid = 0).
    /// * `[in] varDependentNodeArray` Nodes number ID VARIANT array.
    /// # Return values
    /// * `0` OK.
    /// * `-106` 1 dimensional array of long expected.
    /// * `-6029` Library Error: Unable to create CONTROL/DEPENDENT specification.
    pub fn add_control_dependent_relation(
        &self,
        control_node: i32,
        regid_xy_yz_zx: i32,
        fx: i32,
        fy: i32,
        fz: i32,
        mx: i32,
        my: i32,
        mz: i32,
        dependent_nodes: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_nodes = safe_array_from_vec1d::<i32>(dependent_nodes)?;
            let variant_nodes = variant_from_raw_pointer::<SafeArray<i32>>(sa_nodes);

            let mut params = [
                variant_nodes,
                VARIANT::from(mz),
                VARIANT::from(my),
                VARIANT::from(mx),
                VARIANT::from(fz),
                VARIANT::from(fy),
                VARIANT::from(fx),
                VARIANT::from(regid_xy_yz_zx),
                VARIANT::from(control_node),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "AddControlDependentRelation", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::add_control_dependent_relation: {}", e),
            }
        }
    }

    /// Create ELEMENT INPLANE ROTATION specification.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-6019` Library Error: Unable to create ELEMENT INPLANE ROTATION specification.
    pub fn create_element_ignore_inplane_rotn_spec(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                &self.dispatch,
                "CreateElementIgnoreInplaneRotnSpec",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!(
                "Error::Property::create_element_ignore_inplane_rotn_spec: {}",
                e
            ),
        }
    }

    /// Creates ELEMENT NODE RELEASE specification.
    /// # Parameters
    /// * `[in] varNode` The node number ID to be released.
    /// * `[in] varDOFRelease` Degrees of freedom: No Release (= 0) or Release (= 1) for FX, FY, FZ, MX, MY and MZ.
    /// # Return values
    /// * `0` OK
    /// * `-106` 1 dimensional array of long for varDOFRelease expected.
    /// * `-108` Array size is smaller than expected (size should be 6).
    /// * `-6021` Library Error: Unable to create ELEMENT NODE RELEASE specification.
    pub fn create_element_node_release_spec(
        &self,
        node: i32,
        dof_releases: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            if dof_releases.len() != 6 {
                bail!(
                    "Error::Property::create_element_node_release_spec: DOF release array must have 6 elements"
                );
            }

            let sa_dof = safe_array_from_vec1d::<i32>(dof_releases)?;
            let variant_dof = variant_from_raw_pointer::<SafeArray<i32>>(sa_dof);

            let mut params = [variant_dof, VARIANT::from(node)];
            let result_variant =
                invoke_method(&self.dispatch, "CreateElementNodeReleaseSpec", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_element_node_release_spec: {}", e),
            }
        }
    }

    /// Create MEMBER COMPRESSION specification.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-6013` Library Error: Unable to create MEMBER COMPRESSION specification.
    pub fn create_member_compression_spec(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "CreateMemberCompressionSpec", &mut []) };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!("Error::Property::create_member_compression_spec: {}", e),
        }
    }

    /// Create ELEMENT IGNORE STIFFNESS specification.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-6014` Library Error: Unable to create IGNORE STIFFNESS specification.
    pub fn create_member_ignore_stiff_spec(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "CreateMemberIgnoreStiffSpec", &mut []) };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!("Error::Property::create_member_ignore_stiff_spec: {}", e),
        }
    }

    /// Create MEMBER INACTIVE specification.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-6011` Library Error: Unable to create MEMBER INACTIVE specification.
    pub fn create_member_inactive_spec(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "CreateMemberInactiveSpec", &mut []) };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!("Error::Property::create_member_inactive_spec: {}", e),
        }
    }

    /// Creates MEMBER RELEASE specification with partial release factors.
    /// # Parameters
    /// * `[in] varLocation` The offset location at START (= 0) or END (= 1) of the member.
    /// * `[in] varDOFRelease` Degrees of freedom: No Release (= 0) or Release (= 1) for FX, FY, FZ, MX, MY and MZ.
    /// * `[in] varFactor` The partial release factor in respective DOFs.
    /// # Return values
    /// * `0` OK
    /// * `-106` 1 dimensional array of long for varDOFRelease and 1 dimensional array of double for varFactor expected.
    /// * `-108` Array size is smaller than expected (size should be 6).
    /// * `-6020` Library Error: Unable to create MEMBER RELEASE specification.
    pub fn create_member_partial_release_spec(
        &self,
        location: i32,
        dof_releases: Vec<i32>,
        factors: Vec<f64>,
    ) -> Result<i32, anyErr> {
        unsafe {
            if dof_releases.len() != 6 || factors.len() != 6 {
                bail!(
                    "Error::Property::create_member_partial_release_spec: DOF release and factor arrays must have 6 elements each"
                );
            }

            let sa_dof = safe_array_from_vec1d::<i32>(dof_releases)?;
            let sa_factors = safe_array_from_vec1d::<f64>(factors)?;
            let variant_dof = variant_from_raw_pointer::<SafeArray<i32>>(sa_dof);
            let variant_factors = variant_from_raw_pointer::<SafeArray<f64>>(sa_factors);

            let mut params = [variant_factors, variant_dof, VARIANT::from(location)];
            let result_variant = invoke_method(
                &self.dispatch,
                "CreateMemberPartialReleaseSpec",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_member_partial_release_spec: {}", e),
            }
        }
    }

    /// Creates MEMBER RELEASE specification with spring constants.
    /// # Parameters
    /// * `[in] varLocation` The offset location at START (= 0) or END (= 1) of the member.
    /// * `[in] varDOFRelease` Degrees of freedom: No Release (= 0) or Release (= 1) for FX, FY, FZ, MX, MY and MZ.
    /// * `[in] varSpringConst` The variable spring constants KFX, KFY, KFZ, KMX, KMY and KMZ.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-106` 1 dimensional array of long for varDOFRelease and 1 dimensional array of double for varSpringConst expected.
    /// * `-108` Array size is smaller than expected (size should be 6).
    /// * `-6020` Library Error: Unable to create MEMBER RELEASE specification.
    pub fn create_member_release_spec(
        &self,
        location: i32,
        dof_releases: Vec<i32>,
        spring_constants: Vec<f64>,
    ) -> Result<i32, anyErr> {
        unsafe {
            if dof_releases.len() != 6 || spring_constants.len() != 6 {
                bail!(
                    "Error::Property::create_member_release_spec: DOF release and spring constant arrays must have 6 elements each"
                );
            }

            let sa_dof = safe_array_from_vec1d::<i32>(dof_releases)?;
            let sa_springs = safe_array_from_vec1d::<f64>(spring_constants)?;
            let variant_dof = variant_from_raw_pointer::<SafeArray<i32>>(sa_dof);
            let variant_springs = variant_from_raw_pointer::<SafeArray<f64>>(sa_springs);

            let mut params = [variant_springs, variant_dof, VARIANT::from(location)];
            let result_variant =
                invoke_method(&self.dispatch, "CreateMemberReleaseSpec", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_member_release_spec: {}", e),
            }
        }
    }

    /// Create MEMBER TENSION specification.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-6012` Library Error: Unable to create MEMBER TENSION specification.
    pub fn create_member_tension_spec(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "CreateMemberTensionSpec", &mut []) };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!("Error::Property::create_member_tension_spec: {}", e),
        }
    }

    /// Create MEMBER TRUSS specification.
    /// # Return values
    /// * `<Val>` The assigned specification number ID.
    /// * `-6010` Library Error: Unable to create MEMBER TRUSS specification.
    pub fn create_member_truss_spec(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "CreateMemberTrussSpec", &mut []) };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!("Error::Property::create_member_truss_spec: {}", e),
        }
    }

    /// Delete all control/dependent joint specifications from model.
    /// # Return values
    /// * `0` OK Successfully deleted.
    /// * `-1` ERROR Delete unsuccessful.
    pub fn delete_all_control_dependent_relations(&self) -> Result<i32, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                &self.dispatch,
                "DeleteAllControlDependentRelations",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!(
                "Error::Property::delete_all_control_dependent_relations: {}",
                e
            ),
        }
    }

    /// Delete MEMBER RELEASE specification.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Long/Integer).
    /// * `[in] varLocation` The Release location at START (= 0) or END (= 1) of the member.
    /// # Return values
    /// * `FALSE` Delete Member Release specification failed
    /// * `TRUE` Delete Member Release specification Successful.
    pub fn delete_member_release_spec(&self, beam_no: i32, location: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(location), VARIANT::from(beam_no)];
            let result_variant =
                invoke_method(&self.dispatch, "DeleteMemberReleaseSpec", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::delete_member_release_spec: {}", e),
            }
        }
    }

    /// Delete specification based on the specification number passed.
    /// # Parameters
    /// * `[in] varnSpecNo` The specification number.
    /// # Return values
    /// * `FALSE` Delete specification Failed.
    /// * `TRUE` Delete specification Successful.
    pub fn delete_member_spec(&self, spec_no: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(spec_no)];
            let result_variant = invoke_method(&self.dispatch, "DeleteMemberSpec", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::delete_member_spec: {}", e),
            }
        }
    }

    /// Delete property based the on the property ID passed.
    /// # Parameters
    /// * `[in] nProperty` Property ID (Type: Long).
    /// # Return values
    /// * `FALSE` Delete Property Generate Error.
    /// * `TRUE` Delete Property Successful.
    pub fn delete_property(&self, property_id: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(property_id)];
            let result_variant = invoke_method(&self.dispatch, "DeleteProperty", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::delete_property: {}", e),
            }
        }
    }

    /// Returns the alpha angle of the section in radian.
    /// @ALPHA = Gets the angle between the principal axis and geometric axis of the section
    /// # Parameters
    /// * `[in] nPropNo` The specified property ID.
    /// * `[out] dAlpha` alpha angle returned (in Radian).
    pub fn get_alpha_angle_for_section(&self, prop_no: i32) -> Result<f64, anyErr> {
        unsafe {
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let mut params = [
                variant_from_raw_pointer::<f64>(alpha_ptr),
                VARIANT::from(prop_no),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "GetAlphaAngleForSection", &mut params);
            match result_variant {
                Ok(_) => anyOk(*alpha_ptr),
                Err(e) => bail!("Error::Property::get_alpha_angle_for_section: {}", e),
            }
        }
    }

    /// Returns the location of the Centroid of the section.
    /// Gets the location of the Centroid of the specified section.
    /// @The Cez and Cey are distances to centroid from top left outer edge to the centroid in terms of Y axis and Z axis respectively.
    /// # Parameters
    /// * `[in] nPropNo` The specified property ID.
    /// * `[out] dCey` returns offset value of Centroid along Y axis.
    /// * `[out] dCez` returns offset value of Centroid along Z axis.
    pub fn get_centroid_location_for_section(&self, prop_no: i32) -> Result<(f64, f64), anyErr> {
        unsafe {
            let cey_ptr = &mut 0.0f64 as *mut f64;
            let cez_ptr = &mut 0.0f64 as *mut f64;
            let mut params = [
                variant_from_raw_pointer::<f64>(cez_ptr),
                variant_from_raw_pointer::<f64>(cey_ptr),
                VARIANT::from(prop_no),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "GetCentroidLocationForSection", &mut params);
            match result_variant {
                Ok(_) => anyOk((*cey_ptr, *cez_ptr)),
                Err(e) => bail!("Error::Property::get_centroid_location_for_section: {}", e),
            }
        }
    }

    /// Returns the total number of inactive members in the current model.
    /// # Return values
    /// * `<Val>` The total number of inactive member(s).
    pub fn get_inactive_member_count(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "GetInactiveMemberCount", &mut []) };
        match result_variant {
            Ok(var) => {
                let count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(count)
            }
            Err(e) => bail!("Error::Property::get_inactive_member_count: {}", e),
        }
    }

    /// Populates a list of the member ID(s) of all the inactive members in the current model.
    /// # Parameters
    /// * `[out] nInactiveMemList` VARIANT array of LONG type, for storing returned member number ID(s) of all the members that are inactive.
    pub fn get_inactive_member_list(&self) -> Result<Vec<i32>, anyErr> {
        unsafe {
            let member_count = self.get_inactive_member_count()?;
            let mut psa = SafeArrayCreateVector(VT_I4, 0, member_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant];
            let result_variant =
                invoke_method(&self.dispatch, "GetInactiveMemberList", &mut params);
            match result_variant {
                Ok(_) => {
                    let member_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut member_arr = Vec::with_capacity(member_count as usize);
                    for i in 0..member_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            member_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        member_arr.push(value as i32);
                    }
                    anyOk(member_arr)
                }
                Err(e) => bail!("Error::Property::get_inactive_member_list: {}", e),
            }
        }
    }

    /// Get releases for the specified member at the specified end.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// * `[in] varnEnd` Member Start end (= 0); member End end (= 1).
    /// * `[out] varnReleaseArray` Translational release VARIANT array with 6 elements for 6 DOFs.
    /// * `[out] varfSpringConstArray` Rotational releases VARIANT array with 6 elements for 6 DOFs.
    /// * `[out] varfMPFactor` Partial moment release factor (same for MX, MY and MZ).
    /// * `[out] varfMPFactorArray` Rotational releases VARIANT array with 3 elements for 3 rotational DOFs.
    /// # Return values
    /// * `1` OK.
    /// * `0` General error.
    pub fn get_member_release_spec_ex(
        &self,
        beam_no: i32,
        end: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>, f64, Vec<f64>), anyErr> {
        unsafe {
            let mut release_array = vec![0i32; 6];
            let mut spring_const_array = vec![0.0f64; 6];
            let mp_factor_ptr = &mut 0.0f64 as *mut f64;
            let mut mp_factor_array = vec![0.0f64; 3];

            let sa_release = SafeArrayCreateVector(VT_I4, 0, 6);
            let sa_spring = SafeArrayCreateVector(VT_R8, 0, 6);
            let sa_mp_factor = SafeArrayCreateVector(VT_R8, 0, 3);

            let mut params = [
                variant_from_raw_pointer::<SafeArray<f64>>(sa_mp_factor),
                variant_from_raw_pointer::<f64>(mp_factor_ptr),
                variant_from_raw_pointer::<SafeArray<f64>>(sa_spring),
                variant_from_raw_pointer::<SafeArray<i32>>(sa_release),
                VARIANT::from(end),
                VARIANT::from(beam_no),
            ];

            let result_variant =
                invoke_method(&self.dispatch, "GetMemberReleaseSpecEx", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();

                    // Extract release array
                    for i in 0..6 {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            sa_release,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        release_array[i] = value;
                    }

                    // Extract spring constant array
                    for i in 0..6 {
                        let mut index = i as i32;
                        let mut value = 0.0f64;
                        let _ = SafeArrayGetElement(
                            sa_spring,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        spring_const_array[i] = value;
                    }

                    // Extract MP factor array
                    for i in 0..3 {
                        let mut index = i as i32;
                        let mut value = 0.0f64;
                        let _ = SafeArrayGetElement(
                            sa_mp_factor,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        mp_factor_array[i] = value;
                    }

                    anyOk((
                        result_code,
                        release_array,
                        spring_const_array,
                        *mp_factor_ptr,
                        mp_factor_array,
                    ))
                }
                Err(e) => bail!("Error::Property::get_member_release_spec_ex: {}", e),
            }
        }
    }

    /// Get the type of specification attached to specified member.
    /// # Parameters
    /// * `[in] varMembNo` The member number ID.
    /// * `[out] SpecCode` Value referring to type of member specification.
    /// # Return values
    /// * `TRUE/1` OK.
    /// * `FALSE/0` Error.
    pub fn get_member_spec_code(&self, member_no: i32) -> Result<(bool, i32), anyErr> {
        unsafe {
            let spec_code_ptr = &mut 0i32 as *mut i32;
            let mut params = [
                variant_from_raw_pointer::<i32>(spec_code_ptr),
                VARIANT::from(member_no),
            ];
            let result_variant = invoke_method(&self.dispatch, "GetMemberSpecCode", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((result_code == 1, *spec_code_ptr))
                }
                Err(e) => bail!("Error::Property::get_member_spec_code: {}", e),
            }
        }
    }

    /// Get Property Unique ID.
    /// # Parameters
    /// * `[in] nPropNo` Property number (Type: Long).
    /// # Returns
    /// * Property Unique ID (Type: String).
    pub fn get_property_unique_id(&self, prop_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(prop_no)];
            let result_variant = invoke_method(&self.dispatch, "GetPropertyUniqueID", &mut params);
            match result_variant {
                Ok(var) => {
                    let result = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(result)
                }
                Err(e) => bail!("Error::Property::get_property_unique_id: {}", e),
            }
        }
    }

    /// Remove all element node release specification from the model.
    /// # Return values
    /// * `1` OK.
    /// * `0` No element release specification present
    pub fn remove_all_element_node_release_spec(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "RemoveAllElementNodeReleaseSpec", &mut []) };
        match result_variant {
            Ok(var) => {
                let result_code = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(result_code)
            }
            Err(e) => bail!(
                "Error::Property::remove_all_element_node_release_spec: {}",
                e
            ),
        }
    }

    /// Remove beam property.
    /// # Parameters
    /// * `[in] beamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_beam_property_helper(&self, beam_no: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant =
                invoke_method(&self.dispatch, "RemoveBeamPropertyHelper", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 0)
                }
                Err(e) => bail!("Error::Property::remove_beam_property_helper: {}", e),
            }
        }
    }

    /// Remove element ignore in plane rotation specification from plate.
    /// # Parameters
    /// * `[in] varnPlateNo` The plate number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_element_ignore_inplane_rotn_spec_from_plate(
        &self,
        plate_no: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(plate_no)];
            let result_variant = invoke_method(
                &self.dispatch,
                "RemoveElementIgnoreInplaneRotnSpecFromPlate",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::remove_element_ignore_inplane_rotn_spec_from_plate: {}",
                    e
                ),
            }
        }
    }

    /// Removes the member cable specification from a particular member at the provided location type (Tension or Length).
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Long/Integer).
    /// * `[in] varLocation` The Cable location at Tension (= 0) or Length (= 1) of the member.
    /// # Return values
    /// * `FALSE` Remove Member Cable specification failed
    /// * `TRUE` Remove Member Cable specification Successful.
    pub fn remove_member_cable_spec_from_beam(
        &self,
        beam_no: i32,
        location: i32,
    ) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(location), VARIANT::from(beam_no)];
            let result_variant =
                invoke_method(&self.dispatch, "RemoveMemberCableSpecFromBeam", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::remove_member_cable_spec_from_beam: {}", e),
            }
        }
    }

    /// Remove member compression specification from beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_member_compression_spec_from_beam(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.dispatch,
                "RemoveMemberCompressionSpecFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::remove_member_compression_spec_from_beam: {}",
                    e
                ),
            }
        }
    }

    /// Remove member ignore stiff specification from beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_member_ignore_stiff_spec_from_beam(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.dispatch,
                "RemoveMemberIgnoreStiffSpecFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::remove_member_ignore_stiff_spec_from_beam: {}",
                    e
                ),
            }
        }
    }

    /// Remove member inactive specification from beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_member_inactive_spec_from_beam(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.dispatch,
                "RemoveMemberInactiveSpecFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::remove_member_inactive_spec_from_beam: {}",
                    e
                ),
            }
        }
    }

    /// Removes the member specification from a particular member at the provided location (Start or End).
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Long/Integer).
    /// * `[in] varLocation` The Release location at START (= 0) or END (= 1) of the member.
    /// # Return values
    /// * `FALSE` Remove Member Release specification failed
    /// * `TRUE` Remove Member Release specification Successful.
    pub fn remove_member_release_spec_from_beam(
        &self,
        beam_no: i32,
        location: i32,
    ) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(location), VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.dispatch,
                "RemoveMemberReleaseSpecFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!(
                    "Error::Property::remove_member_release_spec_from_beam: {}",
                    e
                ),
            }
        }
    }

    /// Remove member tension specification from beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_member_tension_spec_from_beam(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(
                &self.dispatch,
                "RemoveMemberTensionSpecFromBeam",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::remove_member_tension_spec_from_beam: {}",
                    e
                ),
            }
        }
    }

    /// Remove member truss specification from beam.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_member_truss_spec_from_beam(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant =
                invoke_method(&self.dispatch, "RemoveMemberTrussSpecFromBeam", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::remove_member_truss_spec_from_beam: {}", e),
            }
        }
    }

    /// Remove property from beam.
    /// # Parameters
    /// * `[in] nBeamNo` The beam number ID.
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn remove_property_from_beam(&self, beam_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant =
                invoke_method(&self.dispatch, "RemovePropertyFromBeam", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::remove_property_from_beam: {}", e),
            }
        }
    }

    /// Set Property Unique ID to specification property number.
    /// # Parameters
    /// * `[in] nPropNo` Property number (Type: Long).
    /// * `[in] szID` Property Unique ID (Type: String).
    pub fn set_property_unique_id(&self, prop_no: i32, unique_id: &str) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(unique_id), VARIANT::from(prop_no)];
            let result_variant = invoke_method(&self.dispatch, "SetPropertyUniqueID", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Property::set_property_unique_id: {}", e),
            }
        }
    }
    /// Creates isotropic material aluminum.
    /// # Parameters
    /// * `[in] strName` Identification title of material.
    /// * `[in] dE` Modulus of elasticity (E).
    /// * `[in] dPoisson` Poisson's ratio (POI).
    /// * `[in] dG` Shear modulus (G).
    /// * `[in] dDensity` Weight density (DEN).
    /// * `[in] dAlpha` Coefficient of thermal expansion (ALP).
    /// * `[in] dCrDamp` Damping ratio (DAMP).
    /// * `[in] bPhysical` Identifies if the material is for physical member.
    /// # Return values
    /// * `1` Material is updated as a material with that name was already present.
    /// * `0` Material is created.
    /// * `-1` General Error
    pub fn create_isotropic_material_aluminum(
        &self,
        name: &str,
        e: f64,
        poisson: f64,
        g: f64,
        density: f64,
        alpha: f64,
        cr_damp: f64,
        physical: bool,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(physical as i32),
                VARIANT::from(cr_damp),
                VARIANT::from(alpha),
                VARIANT::from(density),
                VARIANT::from(g),
                VARIANT::from(poisson),
                VARIANT::from(e),
                VARIANT::from(name),
            ];
            let result_variant = invoke_method(
                &self.dispatch,
                "CreateIsotropicMaterialAluminum",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_isotropic_material_aluminum: {}", e),
            }
        }
    }

    /// Creates isotropic material concrete.
    /// # Parameters
    /// * `[in] strName` Identification title of material.
    /// * `[in] dE` Modulus of elasticity (E).
    /// * `[in] dPoisson` Poisson's ratio (POI).
    /// * `[in] dG` Shear modulus (G).
    /// * `[in] dDensity` Weight density (DEN).
    /// * `[in] dAlpha` Coefficient of thermal expansion (ALP).
    /// * `[in] dCrDamp` Damping ratio (DAMP).
    /// * `[in] dfcu` Compressive strength (Fcu).
    /// * `[in] bPhysical` Identifies if the material is for physical member.
    /// # Return values
    /// * `1` Material is updated as a material with that name was already present.
    /// * `0` Material is created.
    /// * `-1` General Error
    pub fn create_isotropic_material_concrete(
        &self,
        name: &str,
        e: f64,
        poisson: f64,
        g: f64,
        density: f64,
        alpha: f64,
        cr_damp: f64,
        fcu: f64,
        physical: bool,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(physical as i32),
                VARIANT::from(fcu),
                VARIANT::from(cr_damp),
                VARIANT::from(alpha),
                VARIANT::from(density),
                VARIANT::from(g),
                VARIANT::from(poisson),
                VARIANT::from(e),
                VARIANT::from(name),
            ];
            let result_variant = invoke_method(
                &self.dispatch,
                "CreateIsotropicMaterialConcrete",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_isotropic_material_concrete: {}", e),
            }
        }
    }

    /// Creates isotropic material properties.
    /// # Parameters
    /// * `[in] strName` Material Name.
    /// * `[out] varE` Modulus of elasticity (E) VARIANT array (of size 3).
    /// * `[out] varPoisson` Poisson's ratio (POI) VARIANT array (of size 3).
    /// * `[out] varG` Shear modulus (G) VARIANT array (of size 3).
    /// * `[out] varDensity` Weight density (DEN) VARIANT array (of size 3).
    /// * `[out] varAlpha` Coefficient of thermal expansion (ALP) VARIANT array (of size 3).
    /// * `[out] varCrDamp` Damping ratio (DAMP) VARIANT array (of size 3).
    /// # Return values
    /// * `1` Material is updated as a material with that name was already present.
    /// * `0` Material is created.
    /// * `-1` General Error
    pub fn create_isotropic_material_properties(
        &self,
        name: &str,
        e: f64,
        poisson: f64,
        g: f64,
        density: f64,
        alpha: f64,
        cr_damp: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(cr_damp),
                VARIANT::from(alpha),
                VARIANT::from(density),
                VARIANT::from(g),
                VARIANT::from(poisson),
                VARIANT::from(e),
                VARIANT::from(name),
            ];
            let result_variant = invoke_method(
                &self.dispatch,
                "CreateIsotropicMaterialProperties",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::create_isotropic_material_properties: {}",
                    e
                ),
            }
        }
    }

    /// Creates isotropic material property extended.
    /// # Parameters
    /// * `[in] strName` Identification title of material.
    /// * `[in] dE` Modulus of elasticity (E).
    /// * `[in] dPoisson` Poisson's ratio (POI).
    /// * `[in] dG` Shear modulus (G).
    /// * `[in] dDensity` Weight density (DEN).
    /// * `[in] dAlpha` Coefficient of thermal expansion (ALP).
    /// * `[in] dCrDamp` Damping ratio (DAMP).
    /// * `[in] dFy` Yield stress (Fy).
    /// * `[in] dFu` Tensile strength (Fu).
    /// * `[in] dRy` Yield strength ratio (Ry).
    /// * `[in] dRt` Tensile strength ratio (Rt).
    /// * `[in] dFcu` Compressive strength (Fcu).
    /// # Return values
    /// * `1` Material is updated as a material with that name was already present.
    /// * `0` Material is created.
    /// * `-1` General Error
    pub fn create_isotropic_material_properties_ex(
        &self,
        name: &str,
        e: f64,
        poisson: f64,
        g: f64,
        density: f64,
        alpha: f64,
        cr_damp: f64,
        fy: f64,
        fu: f64,
        ry: f64,
        rt: f64,
        fcu: f64,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(fcu),
                VARIANT::from(rt),
                VARIANT::from(ry),
                VARIANT::from(fu),
                VARIANT::from(fy),
                VARIANT::from(cr_damp),
                VARIANT::from(alpha),
                VARIANT::from(density),
                VARIANT::from(g),
                VARIANT::from(poisson),
                VARIANT::from(e),
                VARIANT::from(name),
            ];
            let result_variant = invoke_method(
                &self.dispatch,
                "CreateIsotropicMaterialPropertiesEx",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::create_isotropic_material_properties_ex: {}",
                    e
                ),
            }
        }
    }

    /// Creates isotropic material steel.
    /// # Parameters
    /// * `[in] strName` Identification title of material.
    /// * `[in] dE` Modulus of elasticity (E).
    /// * `[in] dPoisson` Poisson's ratio (POI).
    /// * `[in] dG` Shear modulus (G).
    /// * `[in] dDensity` Weight density (DEN).
    /// * `[in] dAlpha` Coefficient of thermal expansion (ALP).
    /// * `[in] dCrDamp` Damping ratio (DAMP).
    /// * `[in] dfu` Tensile strength (Fu).
    /// * `[in] dfy` Yield stress (Fy).
    /// * `[in] drt` Tensile strength ratio (Rt).
    /// * `[in] dry` Yield strength ratio (Ry).
    /// * `[in] bPhysical` Identifies if the material is for physical member.
    /// # Return values
    /// * `1` Material is updated as a material with that name was already present.
    /// * `0` Material is created.
    /// * `-1` General Error
    pub fn create_isotropic_material_steel(
        &self,
        name: &str,
        e: f64,
        poisson: f64,
        g: f64,
        density: f64,
        alpha: f64,
        cr_damp: f64,
        fu: f64,
        fy: f64,
        rt: f64,
        ry: f64,
        physical: bool,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(physical as i32),
                VARIANT::from(ry),
                VARIANT::from(rt),
                VARIANT::from(fy),
                VARIANT::from(fu),
                VARIANT::from(cr_damp),
                VARIANT::from(alpha),
                VARIANT::from(density),
                VARIANT::from(g),
                VARIANT::from(poisson),
                VARIANT::from(e),
                VARIANT::from(name),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "CreateIsotropicMaterialSteel", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_isotropic_material_steel: {}", e),
            }
        }
    }

    /// Creates isotropic material timber.
    /// # Parameters
    /// * `[in] strName` Identification title of material.
    /// * `[in] dE` Modulus of elasticity (E).
    /// * `[in] dPoisson` Poisson's ratio (POI).
    /// * `[in] dG` Shear modulus (G).
    /// * `[in] dDensity` Weight density (DEN).
    /// * `[in] dAlpha` Coefficient of thermal expansion (ALP).
    /// * `[in] dCrDamp` Damping ratio (DAMP).
    /// * `[in] bPhysical` Identifies if the material is for physical member.
    /// # Return values
    /// * `1` Material is updated as a material with that name was already present.
    /// * `0` Material is created.
    /// * `-1` General Error
    pub fn create_isotropic_material_timber(
        &self,
        name: &str,
        e: f64,
        poisson: f64,
        g: f64,
        density: f64,
        alpha: f64,
        cr_damp: f64,
        physical: bool,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(physical as i32),
                VARIANT::from(cr_damp),
                VARIANT::from(alpha),
                VARIANT::from(density),
                VARIANT::from(g),
                VARIANT::from(poisson),
                VARIANT::from(e),
                VARIANT::from(name),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "CreateIsotropicMaterialTimber", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::create_isotropic_material_timber: {}", e),
            }
        }
    }

    /// Delete Material.
    /// # Parameters
    /// * `[in] varMaterialName` Material Name (Type: String).
    /// # Return values
    /// * `FALSE` Delete Material Generate Error.
    /// * `TRUE` Delete Material Successful.
    pub fn delete_material(&self, material_name: &str) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(material_name)];
            let result_variant = invoke_method(&self.dispatch, "DeleteMaterial", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::delete_material: {}", e),
            }
        }
    }

    /// Get beam material string name.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// # Returns
    /// * The beam material string name.
    pub fn get_beam_material_name(&self, beam_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(beam_no)];
            let result_variant = invoke_method(&self.dispatch, "GetBeamMaterialName", &mut params);
            match result_variant {
                Ok(var) => {
                    let result = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(result)
                }
                Err(e) => bail!("Error::Property::get_beam_material_name: {}", e),
            }
        }
    }

    /// Get entity material string name.
    /// # Parameters
    /// * `[in] varnPlateNo` The plate number ID.
    /// # Returns
    /// * The entity material string name.
    pub fn get_element_material_name(&self, plate_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(plate_no)];
            let result_variant =
                invoke_method(&self.dispatch, "GetElementMaterialName", &mut params);
            match result_variant {
                Ok(var) => {
                    let result = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(result)
                }
                Err(e) => bail!("Error::Property::get_element_material_name: {}", e),
            }
        }
    }

    /// Get isotropic material assigned beam count.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of the material.
    /// # Return values
    /// * `0` OK.
    /// * `-6023` Material not found.
    pub fn get_isotropic_material_assigned_beam_count(
        &self,
        material_name: &str,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(material_name)];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetIsotropicMaterialAssignedBeamCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_assigned_beam_count: {}",
                    e
                ),
            }
        }
    }

    /// Get isotropic material assigned beam list.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of the material.
    /// * `[out] nBeamList` List of beam.
    /// # Return values
    /// * `0` OK.
    /// * `-6023` Material not found.
    pub fn get_isotropic_material_assigned_beam_list(
        &self,
        material_name: &str,
    ) -> Result<Vec<i32>, anyErr> {
        unsafe {
            let beam_count = self.get_isotropic_material_assigned_beam_count(material_name)?;
            if beam_count <= 0 {
                return anyOk(Vec::new());
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, beam_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant, VARIANT::from(material_name)];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetIsotropicMaterialAssignedBeamList",
                &mut params,
            );
            match result_variant {
                Ok(_) => {
                    let beam_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut beam_arr = Vec::with_capacity(beam_count as usize);
                    for i in 0..beam_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            beam_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        beam_arr.push(value as i32);
                    }
                    anyOk(beam_arr)
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_assigned_beam_list: {}",
                    e
                ),
            }
        }
    }

    /// Get the count of plates assigned with the specific isotropic material.
    /// # Parameters
    /// * `[in] strMaterialName` Material Name (Type: String).
    /// # Returns
    /// * Count of plates assigned with the specific isotropic material (Type: Long).
    pub fn get_isotropic_material_assigned_plate_count(
        &self,
        material_name: &str,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(material_name)];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetIsotropicMaterialAssignedPlateCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(count)
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_assigned_plate_count: {}",
                    e
                ),
            }
        }
    }

    /// Get the count of solids assigned with the specified isotropic material.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of the material.
    /// # Returns
    /// * Count of solids assigned with the specified isotropic material (Type: Long).
    pub fn get_isotropic_material_assigned_solid_count(
        &self,
        material_name: &str,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(material_name)];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetIsotropicMaterialAssignedSolidCount",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let count = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(count)
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_assigned_solid_count: {}",
                    e
                ),
            }
        }
    }

    /// Get isotropic material assigned solid list.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of the material (type: string).
    /// * `[out] nSolidList` List of solid (type: long array).
    /// # Return values
    /// * `true` Get solid list successful.
    /// * `false` Generate Error.
    pub fn get_isotropic_material_assigned_solid_list(
        &self,
        material_name: &str,
    ) -> Result<Vec<i32>, anyErr> {
        unsafe {
            let solid_count = self.get_isotropic_material_assigned_solid_count(material_name)?;
            if solid_count <= 0 {
                return anyOk(Vec::new());
            }

            let mut psa = SafeArrayCreateVector(VT_I4, 0, solid_count as u32);
            let psa_ptr = &mut psa as *mut *mut SAFEARRAY;
            let variant = variant_from_raw_pointer::<SafeArrayP<i32>>(psa_ptr);

            let mut params = [variant, VARIANT::from(material_name)];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetIsotropicMaterialAssignedSolidList",
                &mut params,
            );
            match result_variant {
                Ok(_) => {
                    let solid_safe_arr = *params[0].Anonymous.Anonymous.Anonymous.pparray;
                    let mut solid_arr = Vec::with_capacity(solid_count as usize);
                    for i in 0..solid_count {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            solid_safe_arr,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        solid_arr.push(value as i32);
                    }
                    anyOk(solid_arr)
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_assigned_solid_list: {}",
                    e
                ),
            }
        }
    }

    /// Get the number of isotropic material present in the current structure.
    /// # Return values
    /// * `<Val>` The number of isotropic material.
    pub fn get_isotropic_material_count(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "GetIsotropicMaterialCount", &mut []) };
        match result_variant {
            Ok(var) => {
                let count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(count)
            }
            Err(e) => bail!("Error::Property::get_isotropic_material_count: {}", e),
        }
    }

    /// Get the properties for the specified isotropic material number.
    /// # Parameters
    /// * `[in] varMatNo` Zero based index of the material.
    /// * `[out] varE` Modulus of elasticity (E).
    /// * `[out] varPoisson` Poisson's ratio (POI).
    /// * `[out] varG` Shear modulus (G).
    /// * `[out] varDensity` Weight density (DEN).
    /// * `[out] varAlpha` Coefficient of thermal expansion (ALP).
    /// * `[out] varCrDamp` Damping ratio (DAMP).
    /// # Return values
    /// * `VARIANT` Material string name.
    /// * `NULL` Cannot find material varMatNo.
    pub fn get_isotropic_material_properties(
        &self,
        mat_no: i32,
    ) -> Result<(String, f64, f64, f64, f64, f64, f64), anyErr> {
        unsafe {
            let e_ptr = &mut 0.0f64 as *mut f64;
            let poisson_ptr = &mut 0.0f64 as *mut f64;
            let g_ptr = &mut 0.0f64 as *mut f64;
            let density_ptr = &mut 0.0f64 as *mut f64;
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let cr_damp_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(cr_damp_ptr),
                variant_from_raw_pointer::<f64>(alpha_ptr),
                variant_from_raw_pointer::<f64>(density_ptr),
                variant_from_raw_pointer::<f64>(g_ptr),
                variant_from_raw_pointer::<f64>(poisson_ptr),
                variant_from_raw_pointer::<f64>(e_ptr),
                VARIANT::from(mat_no),
            ];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetIsotropicMaterialProperties",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let material_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk((
                        material_name,
                        *e_ptr,
                        *poisson_ptr,
                        *g_ptr,
                        *density_ptr,
                        *alpha_ptr,
                        *cr_damp_ptr,
                    ))
                }
                Err(e) => bail!("Error::Property::get_isotropic_material_properties: {}", e),
            }
        }
    }

    /// Gets isotropic material properties and if material assigned to element(s) or not.
    /// # Parameters
    /// * `[in] varMatNo` Material number ID.
    /// * `[out] varE` Modulus of elasticity (E).
    /// * `[out] varPoisson` Poisson's ratio (POI).
    /// * `[out] varG` Shear modulus (G).
    /// * `[out] varDensity` Weight density (DEN).
    /// * `[out] varAlpha` Coefficient of thermal expansion (ALP).
    /// * `[out] varCrDamp` Damping ratio (DAMP).
    /// * `[out] varAssigned` Material assigned to elements or not: unassigned (= 1), assigned (=2).
    /// # Return values
    /// * `VARIANT` Material string name.
    /// * `NULL` Cannot find material varMatNo.
    pub fn get_isotropic_material_properties_assigned(
        &self,
        mat_no: i32,
    ) -> Result<(String, f64, f64, f64, f64, f64, f64, i32), anyErr> {
        unsafe {
            let e_ptr = &mut 0.0f64 as *mut f64;
            let poisson_ptr = &mut 0.0f64 as *mut f64;
            let g_ptr = &mut 0.0f64 as *mut f64;
            let density_ptr = &mut 0.0f64 as *mut f64;
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let cr_damp_ptr = &mut 0.0f64 as *mut f64;
            let assigned_ptr = &mut 0i32 as *mut i32;

            let mut params = [
                variant_from_raw_pointer::<i32>(assigned_ptr),
                variant_from_raw_pointer::<f64>(cr_damp_ptr),
                variant_from_raw_pointer::<f64>(alpha_ptr),
                variant_from_raw_pointer::<f64>(density_ptr),
                variant_from_raw_pointer::<f64>(g_ptr),
                variant_from_raw_pointer::<f64>(poisson_ptr),
                variant_from_raw_pointer::<f64>(e_ptr),
                VARIANT::from(mat_no),
            ];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetIsotropicMaterialPropertiesAssigned",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let material_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk((
                        material_name,
                        *e_ptr,
                        *poisson_ptr,
                        *g_ptr,
                        *density_ptr,
                        *alpha_ptr,
                        *cr_damp_ptr,
                        *assigned_ptr,
                    ))
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_properties_assigned: {}",
                    e
                ),
            }
        }
    }

    /// Get the properties for the specified isotropic material number.
    /// # Parameters
    /// * `[in] varMatNo` Zero based index of the material (Type: Long).
    /// * Extended properties including strength parameters.
    /// # Returns
    /// * The specified material Name and all properties.
    pub fn get_isotropic_material_properties_ex(
        &self,
        mat_no: i32,
    ) -> Result<
        (
            String,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
        ),
        anyErr,
    > {
        unsafe {
            let e_ptr = &mut 0.0f64 as *mut f64;
            let poisson_ptr = &mut 0.0f64 as *mut f64;
            let g_ptr = &mut 0.0f64 as *mut f64;
            let density_ptr = &mut 0.0f64 as *mut f64;
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let cr_damp_ptr = &mut 0.0f64 as *mut f64;
            let fy_ptr = &mut 0.0f64 as *mut f64;
            let fu_ptr = &mut 0.0f64 as *mut f64;
            let ry_ptr = &mut 0.0f64 as *mut f64;
            let rt_ptr = &mut 0.0f64 as *mut f64;
            let fcu_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(fcu_ptr),
                variant_from_raw_pointer::<f64>(rt_ptr),
                variant_from_raw_pointer::<f64>(ry_ptr),
                variant_from_raw_pointer::<f64>(fu_ptr),
                variant_from_raw_pointer::<f64>(fy_ptr),
                variant_from_raw_pointer::<f64>(cr_damp_ptr),
                variant_from_raw_pointer::<f64>(alpha_ptr),
                variant_from_raw_pointer::<f64>(density_ptr),
                variant_from_raw_pointer::<f64>(g_ptr),
                variant_from_raw_pointer::<f64>(poisson_ptr),
                variant_from_raw_pointer::<f64>(e_ptr),
                VARIANT::from(mat_no),
            ];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetIsotropicMaterialPropertiesEx",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let material_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk((
                        material_name,
                        *e_ptr,
                        *poisson_ptr,
                        *g_ptr,
                        *density_ptr,
                        *alpha_ptr,
                        *cr_damp_ptr,
                        *fy_ptr,
                        *fu_ptr,
                        *ry_ptr,
                        *rt_ptr,
                        *fcu_ptr,
                    ))
                }
                Err(e) => bail!(
                    "Error::Property::get_isotropic_material_properties_ex: {}",
                    e
                ),
            }
        }
    }

    /// Get material constants based on specific material name.
    /// # Parameters
    /// * `[in] varstrMaterialName` Identification title of the material.
    /// * Material properties as output parameters.
    /// # Return values
    /// * `0` OK.
    /// * `-6023` Material not found.
    pub fn get_material_property(
        &self,
        material_name: &str,
    ) -> Result<(i32, f64, f64, f64, f64, f64), anyErr> {
        unsafe {
            let elasticity_ptr = &mut 0.0f64 as *mut f64;
            let poisson_ptr = &mut 0.0f64 as *mut f64;
            let density_ptr = &mut 0.0f64 as *mut f64;
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let damp_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(damp_ptr),
                variant_from_raw_pointer::<f64>(alpha_ptr),
                variant_from_raw_pointer::<f64>(density_ptr),
                variant_from_raw_pointer::<f64>(poisson_ptr),
                variant_from_raw_pointer::<f64>(elasticity_ptr),
                VARIANT::from(material_name),
            ];
            let result_variant = invoke_method(&self.dispatch, "GetMaterialProperty", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((
                        result_code,
                        *elasticity_ptr,
                        *poisson_ptr,
                        *density_ptr,
                        *alpha_ptr,
                        *damp_ptr,
                    ))
                }
                Err(e) => bail!("Error::Property::get_material_property: {}", e),
            }
        }
    }

    /// Get the properties for the specified isotropic material Name.
    /// # Parameters
    /// * `[in] varstrMaterial` Name material name (type: string).
    /// * Extended material properties as output parameters.
    /// # Return values
    /// * `TRUE` Get Material Property Successful.
    /// * `FALSE` Get Material Property Failed.
    pub fn get_material_property_ex(
        &self,
        material_name: &str,
    ) -> Result<(bool, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64), anyErr> {
        unsafe {
            let elasticity_ptr = &mut 0.0f64 as *mut f64;
            let poisson_ptr = &mut 0.0f64 as *mut f64;
            let density_ptr = &mut 0.0f64 as *mut f64;
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let damp_ptr = &mut 0.0f64 as *mut f64;
            let fy_ptr = &mut 0.0f64 as *mut f64;
            let fu_ptr = &mut 0.0f64 as *mut f64;
            let ry_ptr = &mut 0.0f64 as *mut f64;
            let rt_ptr = &mut 0.0f64 as *mut f64;
            let fcu_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(fcu_ptr),
                variant_from_raw_pointer::<f64>(rt_ptr),
                variant_from_raw_pointer::<f64>(ry_ptr),
                variant_from_raw_pointer::<f64>(fu_ptr),
                variant_from_raw_pointer::<f64>(fy_ptr),
                variant_from_raw_pointer::<f64>(damp_ptr),
                variant_from_raw_pointer::<f64>(alpha_ptr),
                variant_from_raw_pointer::<f64>(density_ptr),
                variant_from_raw_pointer::<f64>(poisson_ptr),
                variant_from_raw_pointer::<f64>(elasticity_ptr),
                VARIANT::from(material_name),
            ];
            let result_variant =
                invoke_method(&self.dispatch, "GetMaterialPropertyEx", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((
                        result_code == 1,
                        *elasticity_ptr,
                        *poisson_ptr,
                        *density_ptr,
                        *alpha_ptr,
                        *damp_ptr,
                        *fy_ptr,
                        *fu_ptr,
                        *ry_ptr,
                        *rt_ptr,
                        *fcu_ptr,
                    ))
                }
                Err(e) => bail!("Error::Property::get_material_property_ex: {}", e),
            }
        }
    }

    /// Return the number of 2D orthotropic material present in the current structure.
    /// # Return values
    /// * `<Val>` The number of 2D orthotropic material.
    pub fn get_orthotropic_2d_material_count(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "GetOrthotropic2DMaterialCount", &mut []) };
        match result_variant {
            Ok(var) => {
                let count = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(count)
            }
            Err(e) => bail!("Error::Property::get_orthotropic_2d_material_count: {}", e),
        }
    }

    /// Get the properties for the specified 2D orthotropic material.
    /// # Parameters
    /// * `[in] varMatNo` Material number ID.
    /// * Array properties as output parameters.
    /// # Return values
    /// * `VARIANT` Material string name.
    /// * `NULL` Fail to get the properties.
    pub fn get_orthotropic_2d_material_properties(
        &self,
        mat_no: i32,
    ) -> Result<
        (
            String,
            Vec<f64>,
            Vec<f64>,
            Vec<f64>,
            Vec<f64>,
            Vec<f64>,
            Vec<f64>,
        ),
        anyErr,
    > {
        unsafe {
            let mut e_array = vec![0.0f64; 2];
            let mut poisson_array = vec![0.0f64; 2];
            let mut g_array = vec![0.0f64; 3];
            let mut density_array = vec![0.0f64; 2];
            let mut alpha_array = vec![0.0f64; 2];
            let mut cr_damp_array = vec![0.0f64; 2];

            let sa_e = SafeArrayCreateVector(VT_R8, 0, 2);
            let sa_poisson = SafeArrayCreateVector(VT_R8, 0, 2);
            let sa_g = SafeArrayCreateVector(VT_R8, 0, 3);
            let sa_density = SafeArrayCreateVector(VT_R8, 0, 2);
            let sa_alpha = SafeArrayCreateVector(VT_R8, 0, 2);
            let sa_cr_damp = SafeArrayCreateVector(VT_R8, 0, 2);

            let mut params = [
                variant_from_raw_pointer::<SafeArray<f64>>(sa_cr_damp),
                variant_from_raw_pointer::<SafeArray<f64>>(sa_alpha),
                variant_from_raw_pointer::<SafeArray<f64>>(sa_density),
                variant_from_raw_pointer::<SafeArray<f64>>(sa_g),
                variant_from_raw_pointer::<SafeArray<f64>>(sa_poisson),
                variant_from_raw_pointer::<SafeArray<f64>>(sa_e),
                VARIANT::from(mat_no),
            ];

            let result_variant = invoke_method(
                &self.dispatch,
                "GetOrthotropic2DMaterialProperties",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let material_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;

                    // Extract arrays from SafeArrays
                    for i in 0..2 {
                        let mut index = i as i32;
                        let mut value = 0.0f64;
                        let _ = SafeArrayGetElement(
                            sa_e,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        e_array[i] = value;
                        let _ = SafeArrayGetElement(
                            sa_poisson,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        poisson_array[i] = value;
                        let _ = SafeArrayGetElement(
                            sa_density,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        density_array[i] = value;
                        let _ = SafeArrayGetElement(
                            sa_alpha,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        alpha_array[i] = value;
                        let _ = SafeArrayGetElement(
                            sa_cr_damp,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        cr_damp_array[i] = value;
                    }

                    for i in 0..3 {
                        let mut index = i as i32;
                        let mut value = 0.0f64;
                        let _ = SafeArrayGetElement(
                            sa_g,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        g_array[i] = value;
                    }

                    anyOk((
                        material_name,
                        e_array,
                        poisson_array,
                        g_array,
                        density_array,
                        alpha_array,
                        cr_damp_array,
                    ))
                }
                Err(e) => bail!(
                    "Error::Property::get_orthotropic_2d_material_properties: {}",
                    e
                ),
            }
        }
    }
    /// Assign material to member.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of material.
    /// * `[in] varMemberNo` Single or an array of integers containing list of member nos.
    /// # Return values
    /// * `1` TRUE.
    /// * `0` FALSE.
    pub fn assign_material_to_member(
        &self,
        material_name: &str,
        member_nos: Vec<i32>,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_members = safe_array_from_vec1d::<i32>(member_nos)?;
            let variant_members = variant_from_raw_pointer::<SafeArray<i32>>(sa_members);

            let mut params = [variant_members, VARIANT::from(material_name)];
            let result_variant =
                invoke_method(&self.dispatch, "AssignMaterialToMember", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::assign_material_to_member: {}", e),
            }
        }
    }

    /// Assign material to single member.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of material.
    /// * `[in] member_no` Single member number.
    /// # Return values
    /// * `1` TRUE.
    /// * `0` FALSE.
    pub fn assign_material_to_member_single(
        &self,
        material_name: &str,
        member_no: i32,
    ) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(member_no), VARIANT::from(material_name)];
            let result_variant =
                invoke_method(&self.dispatch, "AssignMaterialToMember", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::assign_material_to_member_single: {}", e),
            }
        }
    }

    /// Assign material to plate. API will skip the plate numbers which are not found.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of material. (String)
    /// * `[in] varPlateNo` Single or an array of integers containing list of plate nos. (Long / Integer)
    /// # Return values
    /// * `0` OK.
    /// * `-113` varPlateNos type error (Long or Int Expected)
    /// * `-4009` All the plate numbers are invalid.
    /// * `-4008` Some of the plate numbers are invalid.
    /// * `-6023` Material not found.
    pub fn assign_material_to_plate(
        &self,
        material_name: &str,
        plate_nos: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_plates = safe_array_from_vec1d::<i32>(plate_nos)?;
            let variant_plates = variant_from_raw_pointer::<SafeArray<i32>>(sa_plates);

            let mut params = [variant_plates, VARIANT::from(material_name)];
            let result_variant =
                invoke_method(&self.dispatch, "AssignMaterialToPlate", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::assign_material_to_plate: {}", e),
            }
        }
    }

    /// Assign material to single plate.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of material.
    /// * `[in] plate_no` Single plate number.
    /// # Return values
    /// * `0` OK.
    /// * `-113` plate type error (Long or Int Expected)
    /// * `-6023` Material not found.
    pub fn assign_material_to_plate_single(
        &self,
        material_name: &str,
        plate_no: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(plate_no), VARIANT::from(material_name)];
            let result_variant =
                invoke_method(&self.dispatch, "AssignMaterialToPlate", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::assign_material_to_plate_single: {}", e),
            }
        }
    }

    /// Assign material to solid.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of material.
    /// * `[in] varSolidNo` Single or an array of integers contains list of solid nos.
    /// # Return values
    /// * `1` TRUE.
    /// * `0` FALSE.
    pub fn assign_material_to_solid(
        &self,
        material_name: &str,
        solid_nos: Vec<i32>,
    ) -> Result<bool, anyErr> {
        unsafe {
            let sa_solids = safe_array_from_vec1d::<i32>(solid_nos)?;
            let variant_solids = variant_from_raw_pointer::<SafeArray<i32>>(sa_solids);

            let mut params = [variant_solids, VARIANT::from(material_name)];
            let result_variant =
                invoke_method(&self.dispatch, "AssignMaterialToSolid", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::assign_material_to_solid: {}", e),
            }
        }
    }

    /// Assign material to single solid.
    /// # Parameters
    /// * `[in] strMaterialName` Identification title of material.
    /// * `[in] solid_no` Single solid number.
    /// # Return values
    /// * `1` TRUE.
    /// * `0` FALSE.
    pub fn assign_material_to_solid_single(
        &self,
        material_name: &str,
        solid_no: i32,
    ) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(solid_no), VARIANT::from(material_name)];
            let result_variant =
                invoke_method(&self.dispatch, "AssignMaterialToSolid", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::assign_material_to_solid_single: {}", e),
            }
        }
    }

    /// Set the string name of this material.
    /// # Parameters
    /// * `[in] strMaterialName` Name of the material.
    /// # Return values
    /// * `1` Succeed.
    pub fn set_material_name(&self, material_name: &str) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(material_name)];
            let result_variant = invoke_method(&self.dispatch, "SetMaterialName", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Property::set_material_name: {}", e),
            }
        }
    }
    /// Get material constants by specified beam number ID.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Long)
    /// * `[out] vardElasticity` Modulus of elasticity (E) (Type: Double)
    /// * `[out] vardPoisson` Poisson's ratio (POI) (Type: Double)
    /// * `[out] vardDensity` Weight density (DEN) (Type: Double)
    /// * `[out] vardAlpha` Coefficient of thermal expansion (ALP) (Type: Double)
    /// * `[out] vardDamp` Damping ratio (DAMP) (Type: Double)
    /// # Return values
    /// * `TRUE` Beam Constants found.
    /// * `FALSE` Beam not found/beam constants not found.
    pub fn get_beam_constants(
        &self,
        beam_no: i32,
    ) -> Result<(bool, f64, f64, f64, f64, f64), anyErr> {
        unsafe {
            let elasticity_ptr = &mut 0.0f64 as *mut f64;
            let poisson_ptr = &mut 0.0f64 as *mut f64;
            let density_ptr = &mut 0.0f64 as *mut f64;
            let alpha_ptr = &mut 0.0f64 as *mut f64;
            let damp_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(damp_ptr),
                variant_from_raw_pointer::<f64>(alpha_ptr),
                variant_from_raw_pointer::<f64>(density_ptr),
                variant_from_raw_pointer::<f64>(poisson_ptr),
                variant_from_raw_pointer::<f64>(elasticity_ptr),
                VARIANT::from(beam_no),
            ];
            let result_variant = invoke_method(&self.dispatch, "GetBeamConstants", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((
                        result_code == 1,
                        *elasticity_ptr,
                        *poisson_ptr,
                        *density_ptr,
                        *alpha_ptr,
                        *damp_ptr,
                    ))
                }
                Err(e) => bail!("Error::Property::get_beam_constants: {}", e),
            }
        }
    }

    /// Retrieve short member properties of the specified beam member.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Int)
    /// * `[out] varfWidth` Width of the section (WID) (Type: Double)
    /// * `[out] varfDepth` Depth of the section (DEP) (Type: Double)
    /// * `[out] varfAx` Cross section area (Ax) (Type: Double)
    /// * `[out] varfAy` Shear area in local y-axis (Ay) (Type: Double)
    /// * `[out] varfAz` Shear area in local z-axis (Az) (Type: Double)
    /// * `[out] varfIx` Moment of inertia about local z-axis (Ix) (Type: Double)
    /// * `[out] varfIy` Moment of inertia about local y-axis (Iy) (Type: Double)
    /// * `[out] varfIz` Torsional constant (Iz) (Type: Double)
    /// # Return values
    /// * `1` OK.
    /// * `0` General Error / Cannot find member / No property attached.
    pub fn get_beam_property(
        &self,
        beam_no: i32,
    ) -> Result<(i32, f64, f64, f64, f64, f64, f64, f64, f64), anyErr> {
        unsafe {
            let width_ptr = &mut 0.0f64 as *mut f64;
            let depth_ptr = &mut 0.0f64 as *mut f64;
            let ax_ptr = &mut 0.0f64 as *mut f64;
            let ay_ptr = &mut 0.0f64 as *mut f64;
            let az_ptr = &mut 0.0f64 as *mut f64;
            let ix_ptr = &mut 0.0f64 as *mut f64;
            let iy_ptr = &mut 0.0f64 as *mut f64;
            let iz_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(iz_ptr),
                variant_from_raw_pointer::<f64>(iy_ptr),
                variant_from_raw_pointer::<f64>(ix_ptr),
                variant_from_raw_pointer::<f64>(az_ptr),
                variant_from_raw_pointer::<f64>(ay_ptr),
                variant_from_raw_pointer::<f64>(ax_ptr),
                variant_from_raw_pointer::<f64>(depth_ptr),
                variant_from_raw_pointer::<f64>(width_ptr),
                VARIANT::from(beam_no),
            ];
            let result_variant = invoke_method(&self.dispatch, "GetBeamProperty", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((
                        result_code,
                        *width_ptr,
                        *depth_ptr,
                        *ax_ptr,
                        *ay_ptr,
                        *az_ptr,
                        *ix_ptr,
                        *iy_ptr,
                        *iz_ptr,
                    ))
                }
                Err(e) => bail!("Error::Property::get_beam_property: {}", e),
            }
        }
    }

    /// Retrieve long member properties of the specified beam member.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID (Type: Int)
    /// * `[out] varfWidth` Width of the section (WID) (Type: Double)
    /// * `[out] varfDepth` Depth of the section (DEP) (Type: Double)
    /// * `[out] varfAx` Cross section area (Ax) (Type: Double)
    /// * `[out] varfAy` Shear area in local y-axis (Ay) (Type: Double)
    /// * `[out] varfAz` Shear area in local z-axis (Az) (Type: Double)
    /// * `[out] varfIx` Moment of inertia about local z-axis (Ix) (Type: Double)
    /// * `[out] varfIy` Moment of inertia about local y-axis (Iy) (Type: Double)
    /// * `[out] varfIz` Torsional constant (Iz) (Type: Double)
    /// * `[out] varfTf` Thickness of top flange (Tf) (Type: Double)
    /// * `[out] varfTw` Thickness of web (Tw) (Type: Double)
    /// # Return values
    /// * `1` OK.
    /// * `0` Cannot find member / No property attached.
    pub fn get_beam_property_all(
        &self,
        beam_no: i32,
    ) -> Result<(i32, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64), anyErr> {
        unsafe {
            let width_ptr = &mut 0.0f64 as *mut f64;
            let depth_ptr = &mut 0.0f64 as *mut f64;
            let ax_ptr = &mut 0.0f64 as *mut f64;
            let ay_ptr = &mut 0.0f64 as *mut f64;
            let az_ptr = &mut 0.0f64 as *mut f64;
            let ix_ptr = &mut 0.0f64 as *mut f64;
            let iy_ptr = &mut 0.0f64 as *mut f64;
            let iz_ptr = &mut 0.0f64 as *mut f64;
            let tf_ptr = &mut 0.0f64 as *mut f64;
            let tw_ptr = &mut 0.0f64 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(tw_ptr),
                variant_from_raw_pointer::<f64>(tf_ptr),
                variant_from_raw_pointer::<f64>(iz_ptr),
                variant_from_raw_pointer::<f64>(iy_ptr),
                variant_from_raw_pointer::<f64>(ix_ptr),
                variant_from_raw_pointer::<f64>(az_ptr),
                variant_from_raw_pointer::<f64>(ay_ptr),
                variant_from_raw_pointer::<f64>(ax_ptr),
                variant_from_raw_pointer::<f64>(depth_ptr),
                variant_from_raw_pointer::<f64>(width_ptr),
                VARIANT::from(beam_no),
            ];
            let result_variant = invoke_method(&self.dispatch, "GetBeamPropertyAll", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((
                        result_code,
                        *width_ptr,
                        *depth_ptr,
                        *ax_ptr,
                        *ay_ptr,
                        *az_ptr,
                        *ix_ptr,
                        *iy_ptr,
                        *iz_ptr,
                        *tf_ptr,
                        *tw_ptr,
                    ))
                }
                Err(e) => bail!("Error::Property::get_beam_property_all: {}", e),
            }
        }
    }

    /// Get releases for the specified member at the specified end.
    /// # Parameters
    /// * `[in] varnBeamNo` The beam number ID.
    /// * `[in] varnEnd` Member Start end (= 0); member End end (= 1).
    /// * `[out] varnReleaseArray` Translational release VARIANT array with 6 elements for 6 DOFs.
    /// * `[out] varfSpringConstArray` Rotational releases VARIANT array with 6 elements for 6 DOFs.
    /// # Return values
    /// * `1` OK.
    /// * `0` General error.
    pub fn get_member_release_spec(
        &self,
        beam_no: i32,
        end: i32,
    ) -> Result<(i32, Vec<i32>, Vec<f64>), anyErr> {
        unsafe {
            let mut release_array = vec![0i32; 6];
            let mut spring_const_array = vec![0.0f64; 6];

            let sa_release = SafeArrayCreateVector(VT_I4, 0, 6);
            let sa_spring = SafeArrayCreateVector(VT_R8, 0, 6);

            let mut params = [
                variant_from_raw_pointer::<SafeArray<f64>>(sa_spring),
                variant_from_raw_pointer::<SafeArray<i32>>(sa_release),
                VARIANT::from(end),
                VARIANT::from(beam_no),
            ];

            let result_variant = invoke_method(&self.dispatch, "GetMemberReleaseSpec", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();

                    // Extract release array
                    for i in 0..6 {
                        let mut index = i as i32;
                        let mut value = 0;
                        let _ = SafeArrayGetElement(
                            sa_release,
                            &mut index as *mut i32,
                            &mut value as *mut i32 as *mut c_void,
                        )?;
                        release_array[i] = value;
                    }

                    // Extract spring constant array
                    for i in 0..6 {
                        let mut index = i as i32;
                        let mut value = 0.0f64;
                        let _ = SafeArrayGetElement(
                            sa_spring,
                            &mut index as *mut i32,
                            &mut value as *mut f64 as *mut c_void,
                        )?;
                        spring_const_array[i] = value;
                    }

                    anyOk((result_code, release_array, spring_const_array))
                }
                Err(e) => bail!("Error::Property::get_member_release_spec: {}", e),
            }
        }
    }

    /// Get the assigned section property ID of specified plate.
    /// # Parameters
    /// * `[in] nPlateNo` The plate number ID.
    /// # Return values
    /// * `<Val>` the assigned section property ID.
    /// * `-4001` Cannot find plate nPlateNo.
    /// * `-6022` No property is attached to the plate.
    pub fn get_plate_section_property_ref_no(&self, plate_no: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(plate_no)];
            let result_variant =
                invoke_method(&self.dispatch, "GetPlateSectionPropertyRefNo", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::get_plate_section_property_ref_no: {}", e),
            }
        }
    }
    /// Gets standard profile default database folder path.
    /// # Returns
    /// * The standard profile default database folder path.
    pub fn get_default_standard_profile_db_folder(&self) -> Result<String, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "GetDefaultStandardProfileDBFolder", &mut []) };
        match result_variant {
            Ok(var) => {
                let folder_path = unsafe {
                    VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?
                };
                anyOk(folder_path)
            }
            Err(e) => bail!(
                "Error::Property::get_default_standard_profile_db_folder: {}",
                e
            ),
        }
    }

    /// Gets standard profile database folder path.
    /// # Returns
    /// * The standard profile database folder path.
    pub fn get_standard_profile_db_folder(&self) -> Result<String, anyErr> {
        let result_variant =
            unsafe { invoke_method(&self.dispatch, "GetStandardProfileDBFolder", &mut []) };
        match result_variant {
            Ok(var) => {
                let folder_path = unsafe {
                    VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?
                };
                anyOk(folder_path)
            }
            Err(e) => bail!("Error::Property::get_standard_profile_db_folder: {}", e),
        }
    }

    /// Gets standard section database name for the specified section property reference number.
    /// # Parameters
    /// * `[in] varSecRefNo` The section property reference ID (Type: Long/Integer).
    /// # Return values
    /// * `<Non-Empty-String>` The standard section database name.
    /// * `<Empty-String>` Specified section property reference does not belong to Standard section database.
    pub fn get_standard_section_database_name(&self, sec_ref_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(sec_ref_no)];
            let result_variant = invoke_method(
                &self.dispatch,
                "GetStandardSectionDatabaseName",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let database_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(database_name)
                }
                Err(e) => bail!("Error::Property::get_standard_section_database_name: {}", e),
            }
        }
    }

    /// Get the section name from the standard section database and table for the specified standard section property reference number.
    /// # Parameters
    /// * `[in] varSecRefNo` The section property reference ID (Type: Long/Integer).
    /// # Return values
    /// * `<Non-Empty-String>` The standard section name.
    /// * `<Empty-String>` Specified section property does not belong to any table of Standard section database.
    pub fn get_standard_section_name(&self, sec_ref_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(sec_ref_no)];
            let result_variant =
                invoke_method(&self.dispatch, "GetStandardSectionName", &mut params);
            match result_variant {
                Ok(var) => {
                    let section_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(section_name)
                }
                Err(e) => bail!("Error::Property::get_standard_section_name: {}", e),
            }
        }
    }

    /// Get the table name from the standard section database for the specified standard section property reference number.
    /// # Parameters
    /// * `[in] varSecRefNo` The section property reference ID (Type: Long/Integer).
    /// # Return values
    /// * `<Non-Empty-String>` The standard section table name.
    /// * `<Empty-String>` Specified section property does not belong to any table of Standard section database.
    pub fn get_standard_section_table_name(&self, sec_ref_no: i32) -> Result<String, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(sec_ref_no)];
            let result_variant =
                invoke_method(&self.dispatch, "GetStandardSectionTableName", &mut params);
            match result_variant {
                Ok(var) => {
                    let table_name = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting err")?
                        .to_string()?;
                    anyOk(table_name)
                }
                Err(e) => bail!("Error::Property::get_standard_section_table_name: {}", e),
            }
        }
    }

    /// Checks if the specified section property reference number is from standard section database source or not.
    /// # Parameters
    /// * `[in] nSecRefNo` The section property reference ID (Type: Long/Integer).
    /// # Return values
    /// * `TRUE` Section source is standard database.
    /// * `FALSE` Section source is other than standard database.
    pub fn is_standard_database_section(&self, sec_ref_no: i32) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(sec_ref_no)];
            let result_variant =
                invoke_method(&self.dispatch, "IsStandardDatabaseSection", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code == 1)
                }
                Err(e) => bail!("Error::Property::is_standard_database_section: {}", e),
            }
        }
    }

    /// Sets standard profile database path.
    /// # Parameters
    /// * `[in] strFolderName` Path of the folder.
    /// # Return values
    /// * `0` Succeed.
    /// * `-1` Error, If path is empty or does not exist.
    pub fn set_standard_profile_db_folder(&self, folder_name: &str) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(folder_name)];
            let result_variant =
                invoke_method(&self.dispatch, "SetStandardProfileDBFolder", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Property::set_standard_profile_db_folder: {}", e),
            }
        }
    }
}

// ********** Section **********
// :: Create Profile
// CreateAnglePropertyFromTable
// CreateBeamPropertyFromTable
// CreateBeamPropertyFromTableEx
// CreateChannelPropertyFromTable
// CreatePlateThicknessProperty
// CreatePrismaticTeeProperty
// CreatePrismaticTrapezoidalProperty
// CreateTeePropertyFromTable
// CreateWideFlangePropertyFromTable
// :: Create Profile from UPT
// AddUPTPropertyCHANNEL
// AddUPTPropertyDOUBLEANGLE
// AddUPTPropertyGENERAL
// AddUPTPropertyISECTION
// AddUPTPropertyTEE
// AddUPTPropertyWIDEFLANGE
// AddUPTPropertyWIDEFLANGECOMPOSITE
// AddUPTPropertyWIDEFLANGEUNEQUAL
// CreatePropertyFromUPTTable
// CreateUPTTable
// GetUptGeneralProfileBoundaryPoints
// GetUptGeneralProfilePointsCount
// GetUptGeneralStressLocationPoints
// :: Beta Angle Operation
// AssignBetaAngle
// GetBetaAngle
// :: Get and Remove General Section Profile
// GetBeamSectionName
// GetBeamSectionPropertyRefNo
// GetBeamSectionPropertyTypeNo
// GetBeamSectionPropertyValuesEx
// GetCountofSectionPropertyValuesEx
// GetCountryTableNo
// GetSectionPropertyAssignedBeamCount
// GetSectionPropertyAssignedBeamList
// GetSectionPropertyCount
// GetSectionPropertyCountry
// GetSectionPropertyList
// GetSectionPropertyName
// GetSectionPropertyType
// GetSectionPropertyValues
// GetSectionPropertyValuesEx
// GetSectionTableNo
// GetShapeCode
// :: Get and Remove UPT Profile
// CreatePropertyFromUserTable
// CreateUPTTableEx
// FindUPTTable
// GetUserProvidedTableCount
// GetUserProvidedTableList
// GetUserProvidedTableNo
// GetUserProvidedTableSectionCount
// GetUserProvidedTableSectionList
// GetUserProvidedTableSectionProperties
// GetUserProvidedTableSectionPropertyCount
// GetUserProvidedTableSectionType
// RemovePropertyFromUPTTable
// RemoveUPTTable
// :: Assign Section to Element
// AssignBeamProperty
// AssignElementSpecToPlate
// AssignMemberSpecToBeam
// AssignPlateThickness
// CreateAssignProfileProperty
// UpdatePropertiesToDesignSection

// ********** Specification **********
// :: Create Specification
// AddControlDependentRelation
// CreateElementIgnoreInplaneRotnSpec
// CreateElementNodeReleaseSpec
// CreateMemberCompressionSpec
// CreateMemberIgnoreStiffSpec
// CreateMemberInactiveSpec
// CreateMemberPartialReleaseSpec
// CreateMemberReleaseSpec
// CreateMemberTensionSpec
// CreateMemberTrussSpec
// :: Get and Remove Spectification
// DeleteAllControlDependentRelations
// DeleteMemberReleaseSpec
// DeleteMemberSpec
// DeleteProperty
// GetAlphaAngleForSection
// GetCentroidLocationForSection
// GetInactiveMemberCount
// GetInactiveMemberList
// GetMemberReleaseSpecEx
// GetMemberSpecCode
// GetPropertyUniqueID
// RemoveAllElementNodeReleaseSpec
// RemoveBeamPropertyHelper
// RemoveElementIgnoreInplaneRotnSpecFromPlate
// RemoveMemberCableSpecFromBeam
// RemoveMemberCompressionSpecFromBeam
// RemoveMemberIgnoreStiffSpecFromBeam
// RemoveMemberInactiveSpecFromBeam
// RemoveMemberReleaseSpecFromBeam
// RemoveMemberTensionSpecFromBeam
// RemoveMemberTrussSpecFromBeam
// RemovePropertyFromBeam
// SetPropertyUniqueID

// ********** Material **********
// :: Create Material Information
// CreateIsotropicMaterialAluminum
// CreateIsotropicMaterialConcrete
// CreateIsotropicMaterialProperties
// CreateIsotropicMaterialPropertiesEx
// CreateIsotropicMaterialSteel
// CreateIsotropicMaterialTimber
// :: Get and Remove Material
// DeleteMaterial
// GetBeamMaterialName
// GetElementMaterialName
// GetIsotropicMaterialAssignedBeamCount
// GetIsotropicMaterialAssignedBeamList
// GetIsotropicMaterialAssignedPlateCount
// GetIsotropicMaterialAssignedSolidCount
// GetIsotropicMaterialAssignedSolidList
// GetIsotropicMaterialCount
// GetIsotropicMaterialProperties
// GetIsotropicMaterialPropertiesAssigned
// GetIsotropicMaterialPropertiesEx
// GetMaterialProperty
// GetMaterialPropertyEx
// GetOrthotropic2DMaterialCount
// GetOrthotropic2DMaterialProperties
// GetOrthotropic3DMaterialCount
// GetOrthotropic3DMaterialProperties
// GetPlateMaterialName
// GetSolidMaterialName
// GetTypeForIsotropicMaterial
// RemoveBeamMaterialHelper
// RemoveMaterialFromBeam
// SetTypeToIsotropicMaterial
// :: Assign Material to Section and Element
// AssignMaterialToMember
// AssignMaterialToPlate
// AssignMaterialToSolid
// SetMaterialID
// SetMaterialName

// ********** Element Property **********
// GetBeamConstants
// GetBeamProperty
// GetBeamPropertyAll
// GetMemberReleaseSpec
// GetPlateSectionPropertyRefNo

// ********** Standard Section **********
// GetDefaultStandardProfileDBFolder
// GetStandardProfileDBFolder
// GetStandardSectionDatabaseName
// GetStandardSectionName
// GetStandardSectionTableName
// IsStandardDatabaseSection
// SetStandardProfileDBFolder
