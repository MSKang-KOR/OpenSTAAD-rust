use crate::openstaad::tools::{
    com::{get_dispatch, invoke_method},
    parameters::DesignParameters,
    safe_array::safe_array_from_vec1d,
    variant::{SafeArray, variant_from_raw_pointer},
};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use serde::{Deserialize, Serialize};
use windows::Win32::System::{
    Com::{CLSCTX_LOCAL_SERVER, CLSIDFromProgID, CoCreateInstance, IDispatch},
    Variant::{VARIANT, VariantToInt32},
};
use windows_core::{HSTRING, PCWSTR};

#[derive(Debug, Serialize, Deserialize)]
pub struct Design {
    #[serde(skip)]
    pub dispatch: Option<IDispatch>,
    pub id: String,
}

impl Design {
    pub fn new(staad: Option<IDispatch>) -> Self {
        let _design = unsafe { get_dispatch(staad.as_ref().unwrap(), "Design", &mut []).unwrap() };
        Self {
            dispatch: Some(_design),
            id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// Design Command.
    /// # Parameters
    /// * `[in] nBriefRef` The Design Brief reference ID(Type:Long).
    /// * `[in] strCommandName` Design command name(Type:String).
    /// * `[in] strCommandValue` Parameter for design command(Type:String).
    /// * `[in] varMembers` VARIANT array number ID of Member(s) to be assigned to(Type:Long Array).
    /// # Return values
    /// * `0` Assign Design Command Successful.
    /// * `-1` Assign Design Command Generate error.
    pub fn assign_design_command(
        &self,
        brief_ref: i32,
        command_name: &str,
        command_value: &str,
        members: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_members = safe_array_from_vec1d::<i32>(members)?;
            let variant_members = variant_from_raw_pointer::<SafeArray<i32>>(sa_members);

            let mut params = [
                variant_members,
                VARIANT::from(command_value),
                VARIANT::from(command_name),
                VARIANT::from(brief_ref),
            ];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AssignDesignCommand",
                &mut params,
            );
            match result_variant {
                Ok(v) => {
                    let result_code = VariantToInt32(&v as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Design::assign_design_command: {}", e),
            }
        }
    }

    /// Assign physical member(s) to Design Group using Design Command.
    /// # Parameters
    /// * `[in] nBriefRef` Design Brief reference ID(Type:Long).
    /// * `[in] strCommandName` Name of the command: scSteelGroup (= 9987)(Type:String).
    /// * `[in] strCommandValue` Property Specification: "Ax", "Ay", "Az"(Type:String).
    /// * `[in] sameAsMember` Reference ID of the beam to represent members in this group for design(Type:Long).
    /// * `[in] varMembers` Member number ID(s) VARIANT array(Type:Long Array).
    /// # Return values
    /// * `0` OK.
    /// * `-1` General error.
    pub fn assign_design_group(
        &self,
        brief_ref: i32,
        command_name: &str,
        command_value: &str,
        same_as_member: i32,
        members: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_members = safe_array_from_vec1d::<i32>(members)?;
            let variant_members = variant_from_raw_pointer::<SafeArray<i32>>(sa_members);

            let mut params = [
                variant_members,
                VARIANT::from(same_as_member),
                VARIANT::from(command_value),
                VARIANT::from(command_name),
                VARIANT::from(brief_ref),
            ];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AssignDesignGroup",
                &mut params,
            );
            match result_variant {
                Ok(v) => {
                    let result_code = VariantToInt32(&v as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Design::assign_design_group: {}", e),
            }
        }
    }

    /// Assigns Design Parameters to specified Design Brief using Design Command.
    /// # Parameters
    /// * `[in] nBriefRef` The Design Brief reference ID(Type:Long).
    /// * `[in] strParamName` Design command name(Type:String).
    /// * `[in] strParamValue` Parameter for design command(Type:String).
    /// * `[in] varMembers` Member number ID(s) VARIANT array(Type:Long Array).
    /// # Return values
    /// * `0` Assign Design Parameters Successful.
    /// * `-1` Assign Design Parameters General error.
    pub fn assign_design_parameter(
        &self,
        brief_ref: i32,
        param_name: &str,
        param_value: &str,
        members: Vec<i32>,
    ) -> Result<i32, anyErr> {
        unsafe {
            let sa_members = safe_array_from_vec1d::<i32>(members)?;
            let variant_members = variant_from_raw_pointer::<SafeArray<i32>>(sa_members);

            let mut params = [
                variant_members,
                VARIANT::from(param_value),
                VARIANT::from(param_name),
                VARIANT::from(brief_ref),
            ];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "AssignDesignParameter",
                &mut params,
            );
            match result_variant {
                Ok(v) => {
                    let result_code = VariantToInt32(&v as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Design::assign_design_parameter: {}", e),
            }
        }
    }

    /// Creates a new design brief with specified design code. This should then be populated with the necessary design parameters (see the function OSDesignUI::AssignDesignParameter ) and completed with a design command (see function OSDesignUI::AssignDesignCommand).
    /// # Parameters
    /// * `[in] nDesignCode` Design Code Index.
    /// # Return values
    /// * `<Val>` The reference ID of the design brief created.
    /// * `-1` General error.
    pub fn create_design_brief(&self, design_code: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(design_code)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateDesignBrief",
                &mut params,
            );
            match result_variant {
                Ok(v) => {
                    let brief_ref = VariantToInt32(&v as *const VARIANT).unwrap();
                    anyOk(brief_ref)
                }
                Err(e) => bail!("Error::Design::create_design_brief: {}", e),
            }
        }
    }

    /// Returns the Design Brief Code for specified Design brief.
    /// # Parameters
    /// * `[in] nBriefRef` Design Brief reference ID.
    /// # Return values
    /// * `<Val>` The design brief code in long integer.
    /// * `0` STAAD Design Brief Not Found.
    pub fn get_design_brief_code(&self, brief_ref: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(brief_ref)];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetDesignBriefCode",
                &mut params,
            );
            match result_variant {
                Ok(v) => {
                    let design_code = VariantToInt32(&v as *const VARIANT).unwrap();
                    anyOk(design_code)
                }
                Err(e) => bail!("Error::Design::get_design_brief_code: {}", e),
            }
        }
    }

    pub fn get_member_design_parameters(
        &self,
        brief_ref: i32,
        member_no: i32,
    ) -> Result<(i32, DesignParameters), anyErr> {
        unsafe {
            let clsid_str = PCWSTR::from_raw(HSTRING::from("StaadPro.MembSteelDgnParams").as_ptr());
            let clsid = CLSIDFromProgID(clsid_str).context("CLSID 생성 실패")?;
            let _instance: IDispatch = CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER)
                .context("MembSteelDgnParams 인스턴스 생성 실패")?;
            let instace_ptr: *mut Option<IDispatch> = &mut Some(_instance);

            let mut params = [
                variant_from_raw_pointer::<Option<IDispatch>>(instace_ptr),
                VARIANT::from(member_no),
                VARIANT::from(brief_ref),
            ];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetMemberDesignParameters",
                &mut params,
            );

            let params_dispatch = (*instace_ptr).as_ref().unwrap();
            let design_params = DesignParameters::new(params_dispatch);

            match result_variant {
                Ok(v) => {
                    let status = VariantToInt32(&v as *const VARIANT).unwrap();
                    anyOk((status, design_params))
                }
                Err(e) => bail!("Error::Design::get_member_design_parameters: {}", e),
            }
        }
    }
}


unsafe impl Send for Design{}
unsafe impl Sync for Design{}
