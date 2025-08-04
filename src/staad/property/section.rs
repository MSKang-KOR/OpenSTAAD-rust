use crate::staad::{
    property::root::Property,
    safe_array::safe_array_from_vec1d,
    utils::invoke_method_with_result,
    variant::{SafeArray, SafeArrayP, variant_from_raw_pointer},
};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use std::ffi::c_void;
use windows::Win32::System::{
    Com::SAFEARRAY,
    Ole::{SafeArrayCreateVector, SafeArrayGetElement, SafeArrayPutElement},
    Variant::{
        VARIANT, VT_BSTR, VT_I4, VT_R8, VariantToDouble, VariantToInt32, VariantToStringAlloc,
    },
};
use windows_core::BSTR;

// ::Create Profile
// CreateAnglePropertyFromTable
// CreateBeamPropertyFromTable
// CreateBeamPropertyFromTableEx
// CreateChannelPropertyFromTable
// CreatePlateThicknessProperty
// CreatePrismaticTeeProperty
// CreatePrismaticTrapezoidalProperty
// CreateTeePropertyFromTable
// CreateWideFlangePropertyFromTable
// ::Create Profile from UPT
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
// ::Get and Remove UPT Profile
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

#[derive(Debug)]
pub struct Section<'a> {
    pub property: &'a Property<'a>,
}

impl<'a> Section<'a> {
    pub fn new(property: &'a Property<'a>) -> Self {
        Self { property }
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreateAnglePropertyFromTable",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreateBeamPropertyFromTable",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreateBeamPropertyFromTableEx",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreatePlateThicknessProperty",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreatePrismaticTeeProperty",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreateTeePropertyFromTable",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AddUPTPropertyCHANNEL",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AddUPTPropertyDOUBLEANGLE",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AddUPTPropertyGENERAL",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AddUPTPropertyISECTION",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AddUPTPropertyTEE",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AddUPTPropertyWIDEFLANGE",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreatePropertyFromUPTTable",
                &mut params,
            );
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
            let result_variant =
                invoke_method_with_result(&self.property.dispatch, "CreateUPTTable", &mut params);
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

            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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

            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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

            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
            let result_variant =
                invoke_method_with_result(&self.property.dispatch, "AssignBetaAngle", &mut params);
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
            let result_variant =
                invoke_method_with_result(&self.property.dispatch, "GetBetaAngle", &mut params);
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetBeamSectionName",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetBeamSectionPropertyRefNo",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetBeamSectionPropertyTypeNo",
                &mut params,
            );
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

            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
        let result_variant = unsafe {
            invoke_method_with_result(
                &self.property.dispatch,
                "GetCountofSectionPropertyValuesEx",
                &mut [],
            )
        };
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetCountryTableNo",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
        let result_variant = unsafe {
            invoke_method_with_result(&self.property.dispatch, "GetSectionPropertyCount", &mut [])
        };
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetSectionPropertyCountry",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetSectionPropertyList",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetSectionPropertyName",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetSectionPropertyType",
                &mut params,
            );
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

            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetSectionPropertyValues",
                &mut params,
            );
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

            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetSectionPropertyValuesEx",
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetSectionTableNo",
                &mut params,
            );
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
            let result_variant =
                invoke_method_with_result(&self.property.dispatch, "GetShapeCode", &mut params);
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreatePropertyFromUserTable",
                &mut params,
            );
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
            let result_variant =
                invoke_method_with_result(&self.property.dispatch, "CreateUPTTableEx", &mut params);
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
            let result_variant =
                invoke_method_with_result(&self.property.dispatch, "FindUPTTable", &mut params);
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
        let result_variant = unsafe {
            invoke_method_with_result(
                &self.property.dispatch,
                "GetUserProvidedTableCount",
                &mut [],
            )
        };
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetUserProvidedTableList",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "GetUserProvidedTableNo",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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

            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "RemovePropertyFromUPTTable",
                &mut params,
            );
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
            let result_variant =
                invoke_method_with_result(&self.property.dispatch, "RemoveUPTTable", &mut params);
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AssignBeamProperty",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AssignElementSpecToPlate",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AssignMemberSpecToBeam",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AssignPlateThickness",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "AssignPlateThickness",
                &mut params,
            );
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
            let result_variant = invoke_method_with_result(
                &self.property.dispatch,
                "CreateAssignProfileProperty",
                &mut params,
            );
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
        let result_variant = unsafe {
            invoke_method_with_result(
                &self.property.dispatch,
                "UpdatePropertiesToDesignSection",
                &mut [],
            )
        };
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
}
