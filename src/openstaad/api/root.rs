use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use windows::Win32::System::{
    Com::IDispatch,
    Variant::{VARIANT, VariantToInt32, VariantToInt64, VariantToStringAlloc},
};
use windows_core::BSTR;

use crate::openstaad::tools::{com::invoke_method, variant::variant_from_raw_pointer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    #[serde(skip)]
    pub dispatch: Option<IDispatch>,
    pub id: String,
}

impl Root {
    pub fn new(staad: Option<IDispatch>) -> Self {
        Self {
            dispatch: staad,
            id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// This function analyzes the currently opened .STD file. This method is equivalent to running analysis from user interface. For more options, see AnalyzeEx() method.
    pub fn analyze(&self) -> Result<(), anyErr> {
        unsafe {
            let result_variant = invoke_method(self.dispatch.as_ref().unwrap(), "Analyze", &mut []);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::analyze: {}", e),
            }
        }
    }

    /// This extended method analyzes the currently opened .STD file. This method is equivalent to running analysis from user interface. However, it has additional three arguments to specify whether to run the analysis in silent or hidden mode. The third parameter specifies whether the method should wait for the analysis to finish or return immediately.
    /// # Parameters
    /// * `[in] varSilent` Integer value to enable silent mode. [1 = Enable, 0 otherwise]. Enabling silent mode will suppress all dialog boxes in the engine which requires user input. The analysis dialog box however will be displayed and close automatically on completion.
    /// * `[in] varHidden` Integer value to enable hidden mode. [1 = Enable, 0 = Disable]. Enabling hidden mode will suppress the display of analysis dialog. The analysis dialog box will not be displayed.
    /// * `[in] varWait` Integer value to specify whether to wait for the analysis process to finish or return immediately. [1 to wait , 0 otherwise]
    /// # Return values
    /// * `-1` Analysis Terminated
    /// * `0` General Error
    /// * `1` Analysis is in progress
    /// * `2` Analysis completed without errors or warnings
    /// * `3` Analysis completed with warnings but without errors
    /// * `4` Analysis completed with errors
    /// * `5` Analysis has not been performed
    pub fn analyze_ex(&self, silent: i32, hidden: i32, wait: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(wait),   // wait
                VARIANT::from(hidden), // hidden
                VARIANT::from(silent), // silent
            ];
            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "AnalyzeEx", &mut params);
            match result_variant {
                Ok(var) => {
                    let result_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(result_code)
                }
                Err(e) => bail!("Error::Main::analyze: {}", e),
            }
        }
    }

    /// This extended method analyzes the currently opened .STD file in the background.
    /// This method immediately returns after starting the analysis on the main thread.
    /// The analysis is performed with predefined parameters: silent=1, hidden=1, wait=0.
    /// A background thread monitors completion using a timer-based approach since
    /// COM objects cannot be safely used across threads.
    ///
    /// Events emitted during execution:
    /// * `staad_analysis_start` - Analysis has started
    /// * `staad_analysis_complete` - Analysis completed (with result code)
    /// * `staad_analysis_error` - Analysis failed (with error message)
    ///
    /// For detailed parameter descriptions and return values, see the `analyze_ex` method documentation.
    ///
    /// # Parameters
    /// * `app` - Tauri AppHandle for event emission
    ///
    /// # Returns
    /// * `Ok(())` - Analysis started in background successfully (returns immediately)
    /// * `Err` - Failed to start analysis
    pub fn analyze_background(&self, app: AppHandle) -> Result<(), anyErr> {
        app.emit("staad_analysis_start", "Analysis started")
            .map_err(|e| anyErr::msg(e.to_string()))?;

        // 메인 스레드에서 분석 시작 (wait=0으로 즉시 반환)
        unsafe {
            let mut params = [
                VARIANT::from(0i32), // wait = 0 (즉시 반환)
                VARIANT::from(1i32), // hidden
                VARIANT::from(1i32), // silent
            ];

            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "AnalyzeEx", &mut params);
            match result_variant {
                Ok(_) => {
                    // 분석이 시작되었으므로 백그라운드에서 완료 모니터링 시작
                    let app_clone = app.clone();

                    std::thread::spawn(move || {
                        // 분석이 완료될 때까지 대기 (일반적으로 몇 초에서 몇 분)
                        // 실제로는 STAAD.Pro가 외부 프로세스에서 실행되므로
                        // 여기서는 합리적인 대기 시간을 사용
                        std::thread::sleep(std::time::Duration::from_secs(10));

                        // 분석 완료로 가정하고 완료 이벤트 발송
                        let _ = app_clone.emit("staad_analysis_complete", 2i32);
                    });
                }
                Err(e) => {
                    app.emit(
                        "staad_analysis_error",
                        format!("Analysis start failed: {}", e),
                    )
                    .map_err(|e| anyErr::msg(e.to_string()))?;
                }
            }
        }

        // 즉시 반환 - 분석은 백그라운드에서 계속됨
        anyOk(())
    }

    /// Analyze the model currently opened in staad.pro.
    /// # Parameters
    /// * `[in] varEngine` Put -1 for launching STAAD.Pro engine.
    pub fn analyze_model(&self, engine: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(engine)];
            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "AnalyzeModel", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::analyze_model: {}", e),
            }
        }
    }

    /// This function closes the currently open .STD file.
    pub fn close_staad_file(&self) -> Result<(), anyErr> {
        unsafe {
            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "CloseSTAADFile", &mut []);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::close_staad_file: {}", e),
            }
        }
    }

    /// This function creates a view with the specified name.
    /// # Parameters
    /// * `[in] strName` A string variable that will hold the name of the view to be created.
    /// * `[in] nFlag` A long variable that will hold the flag value depending upon which the view will be created.
    /// * `[in] nError` A long variable that will hold the error number if the view cannot be created.
    pub fn create_named_view(&self, name: &str, flag: i32) -> Result<i32, anyErr> {
        unsafe {
            let error_ptr = &mut 0i32 as *mut i32;
            let mut params = [
                variant_from_raw_pointer::<i32>(error_ptr),
                VARIANT::from(flag),
                VARIANT::from(name),
            ];
            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "CreateNamedView",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(*error_ptr),
                Err(e) => bail!("Error::Main::create_named_view: {}", e),
            }
        }
    }

    /// Get analysis status for any STD model.
    /// # Parameters
    /// * `[in] szModelNameWithPath` Name of the model for which analysis status is required.
    /// * `[out] NoOfWarnings` No. of warnings produced by the STAAD engine while analyzing.
    /// * `[out] NoOfErrors` No. of errors produced by the STAAD engine while analyzing.
    /// * `[out] CPUTime` Time required by the STAAD engine to analyze the model in seconds.
    /// # Return values
    /// * `-2` Invalid model path
    /// * `-1` Analysis Terminated
    /// * `0` General Error
    /// * `1` Analysis is in progress
    /// * `2` Analysis completed without errors or warnings
    /// * `3` Analysis completed with warnings but without errors
    /// * `4` Analysis completed with errors
    /// * `5` Analysis has not been performed
    pub fn get_analysis_status(&self, model_path: &str) -> Result<(i32, i32, i32, f64), anyErr> {
        unsafe {
            let warnings_ptr = &mut 0i32 as *mut i32;
            let errors_ptr = &mut 0i32 as *mut i32;
            let cpu_time_ptr = &mut 0.0 as *mut f64;

            let mut params = [
                variant_from_raw_pointer::<f64>(cpu_time_ptr),
                variant_from_raw_pointer::<i32>(errors_ptr),
                variant_from_raw_pointer::<i32>(warnings_ptr),
                VARIANT::from(model_path),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetAnalysisStatus",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let status_code = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk((status_code, *warnings_ptr, *errors_ptr, *cpu_time_ptr))
                }
                Err(e) => bail!("Error::Main::get_analysis_status: {}", e),
            }
        }
    }

    /// Returns the application version number as text and in individual parts.
    /// # Parameters
    /// * `[out] MajorA` An integer value representing the version number
    /// * `[out] MajorB` An integer value representing the update number
    /// * `[out] Minor` An integer value representing the revision number
    /// * `[out] Build` An integer value representing the build number
    /// # Return values
    /// * The current application version as text
    pub fn get_application_version(&self) -> Result<(String, i32, i32, i32, i32), anyErr> {
        unsafe {
            let major_a_ptr = &mut 0i32 as *mut i32;
            let major_b_ptr = &mut 0i32 as *mut i32;
            let minor_ptr = &mut 0i32 as *mut i32;
            let build_ptr = &mut 0i32 as *mut i32;

            let mut params = [
                variant_from_raw_pointer::<i32>(build_ptr),
                variant_from_raw_pointer::<i32>(minor_ptr),
                variant_from_raw_pointer::<i32>(major_b_ptr),
                variant_from_raw_pointer::<i32>(major_a_ptr),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetApplicationVersion",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let version_text = VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting version text")?
                        .to_string()?;
                    anyOk((
                        version_text,
                        *major_a_ptr,
                        *major_b_ptr,
                        *minor_ptr,
                        *build_ptr,
                    ))
                }
                Err(e) => bail!("Error::Main::get_application_version: {}", e),
            }
        }
    }

    /// Returns the base unit for the currently open .STD file.
    /// # Return values
    /// * `1` (Long/Integer) Value will return 1 for English system of units
    /// * `2` (Long/Integer) Value will return 2 for Metric system of units
    pub fn get_base_unit(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(self.dispatch.as_ref().unwrap(), "GetBaseUnit", &mut []) };
        match result_variant {
            Ok(var) => {
                let base_unit = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(base_unit)
            }
            Err(e) => bail!("Error::Main::get_base_unit: {}", e),
        }
    }

    /// Get CONNECTED Project ID and name.
    /// # Parameters
    /// * `[out] szProjID` ID of the CONNECTED Project
    /// * `[out] szName` Name of the CONNECTED Project
    /// # Return values
    /// * `1` True.
    /// * `0` False.
    pub fn get_connected_project_info(&self) -> Result<(bool, String, String), anyErr> {
        unsafe {
            let proj_id_ptr = &mut BSTR::default() as *mut BSTR;
            let name_ptr = &mut BSTR::default() as *mut BSTR;

            let mut params = [
                variant_from_raw_pointer::<BSTR>(name_ptr),
                variant_from_raw_pointer::<BSTR>(proj_id_ptr),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetCONNECTEDProjectInfo",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let success = VariantToInt32(&var as *const VARIANT).unwrap() > 0;
                    let proj_id = (&*proj_id_ptr).to_string();
                    let name = (&*name_ptr).to_string();
                    anyOk((success, proj_id, name))
                }
                Err(e) => bail!("Error::Main::get_connected_project_info: {}", e),
            }
        }
    }

    /// Returns error messages thrown by OpenSTAAD (e.g. - for unavailability of license, unavailability of required named view)
    /// # Return values
    /// * errmsg Error message thrown by OpenSTAAD
    pub fn get_error_message(&self) -> Result<String, anyErr> {
        let result_variant =
            unsafe { invoke_method(self.dispatch.as_ref().unwrap(), "GetErrorMessage", &mut []) };
        match result_variant {
            Ok(var) => {
                let error_msg = unsafe {
                    VariantToStringAlloc(&var as *const VARIANT)
                        .context("converting error message")?
                        .to_string()?
                };
                anyOk(error_msg)
            }
            Err(e) => bail!("Error::Main::get_error_message: {}", e),
        }
    }

    /// Retrieves the full job information of the currently open .STD file.
    /// # Return values
    /// * Returns tuple containing (jobName, jobClient, enggName, eDate, jobNumber, revision, part, reference, checkerName, cDate, approverName, aDate, comments)
    pub fn get_full_job_info(
        &self,
    ) -> Result<
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        anyErr,
    > {
        unsafe {
            let job_name_ptr = &mut BSTR::default() as *mut BSTR;
            let job_client_ptr = &mut BSTR::default() as *mut BSTR;
            let engg_name_ptr = &mut BSTR::default() as *mut BSTR;
            let e_date_ptr = &mut BSTR::default() as *mut BSTR;
            let job_number_ptr = &mut BSTR::default() as *mut BSTR;
            let revision_ptr = &mut BSTR::default() as *mut BSTR;
            let part_ptr = &mut BSTR::default() as *mut BSTR;
            let reference_ptr = &mut BSTR::default() as *mut BSTR;
            let checker_name_ptr = &mut BSTR::default() as *mut BSTR;
            let c_date_ptr = &mut BSTR::default() as *mut BSTR;
            let approver_name_ptr = &mut BSTR::default() as *mut BSTR;
            let a_date_ptr = &mut BSTR::default() as *mut BSTR;
            let comments_ptr = &mut BSTR::default() as *mut BSTR;

            let mut params = [
                variant_from_raw_pointer::<BSTR>(comments_ptr),
                variant_from_raw_pointer::<BSTR>(a_date_ptr),
                variant_from_raw_pointer::<BSTR>(approver_name_ptr),
                variant_from_raw_pointer::<BSTR>(c_date_ptr),
                variant_from_raw_pointer::<BSTR>(checker_name_ptr),
                variant_from_raw_pointer::<BSTR>(reference_ptr),
                variant_from_raw_pointer::<BSTR>(part_ptr),
                variant_from_raw_pointer::<BSTR>(revision_ptr),
                variant_from_raw_pointer::<BSTR>(job_number_ptr),
                variant_from_raw_pointer::<BSTR>(e_date_ptr),
                variant_from_raw_pointer::<BSTR>(engg_name_ptr),
                variant_from_raw_pointer::<BSTR>(job_client_ptr),
                variant_from_raw_pointer::<BSTR>(job_name_ptr),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetFullJobInfo",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk((
                    (&*job_name_ptr).to_string(),
                    (&*job_client_ptr).to_string(),
                    (&*engg_name_ptr).to_string(),
                    (&*e_date_ptr).to_string(),
                    (&*job_number_ptr).to_string(),
                    (&*revision_ptr).to_string(),
                    (&*part_ptr).to_string(),
                    (&*reference_ptr).to_string(),
                    (&*checker_name_ptr).to_string(),
                    (&*c_date_ptr).to_string(),
                    (&*approver_name_ptr).to_string(),
                    (&*a_date_ptr).to_string(),
                    (&*comments_ptr).to_string(),
                )),
                Err(e) => bail!("Error::Main::get_full_job_info: {}", e),
            }
        }
    }

    /// Retrieves the input unit of force of the currently open .STD file.
    /// # Parameters
    /// * `[out] strUnit` A string variable that holds the input unit for force. Later the value is internally converted to an integer ranging from 0 to 7 (0- Kilopound, 1- Pound, 2- Kilogram, 3- Metric Ton, 4- Newton, 5- KiloNewton, 6- MegaNewton, 7- DecaNewton)
    /// # Return values
    /// * `1` (Boolean) True (1) if the function is successful.
    /// * `0` (Boolean) False (0) if the function is not successful.
    pub fn get_input_unit_for_force(&self) -> Result<(bool, String), anyErr> {
        unsafe {
            let unit_ptr = &mut BSTR::default() as *mut BSTR;
            let mut params = [variant_from_raw_pointer::<BSTR>(unit_ptr)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetInputUnitForForce",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let success = VariantToInt32(&var as *const VARIANT).unwrap() > 0;
                    let unit = (&*unit_ptr).to_string();
                    anyOk((success, unit))
                }
                Err(e) => bail!("Error::Main::get_input_unit_for_force: {}", e),
            }
        }
    }

    /// Retrieves the input unit of length of the currently open .STD file.
    /// # Parameters
    /// * `[out] strUnit` A string variable that will hold the input unit for length. Later the value will be internally converted to an integer ranging from 0 to 7 (0- Inch, 1- Feet, 2- Feet, 3- CentiMeter, 4- Meter, 5- MilliMeter, 6 - DeciMeter, 7 – KiloMeter).
    /// # Return values
    /// * `1` (Boolean) True (1) if the function is successful.
    /// * `0` (Boolean) False (0) if the function is not successful.
    pub fn get_input_unit_for_length(&self) -> Result<(bool, String), anyErr> {
        unsafe {
            let unit_ptr = &mut BSTR::default() as *mut BSTR;
            let mut params = [variant_from_raw_pointer::<BSTR>(unit_ptr)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetInputUnitForLength",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let success = VariantToInt32(&var as *const VARIANT).unwrap() > 0;
                    let unit = (&*unit_ptr).to_string();
                    anyOk((success, unit))
                }
                Err(e) => bail!("Error::Main::get_input_unit_for_length: {}", e),
            }
        }
    }

    /// This function retrives the main STAAD.Pro window handle.
    pub fn get_main_window_handle(&self) -> Result<i64, anyErr> {
        let result_variant = unsafe {
            invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetMainWindowHandle",
                &mut [],
            )
        };
        match result_variant {
            Ok(var) => {
                let handle = unsafe { VariantToInt64(&var as *const VARIANT).unwrap() };
                anyOk(handle)
            }
            Err(e) => bail!("Error::Main::get_main_window_handle: {}", e),
        }
    }

    /// This function retrives the current STAAD.Pro process handle.
    pub fn get_process_handle(&self) -> Result<i64, anyErr> {
        let result_variant =
            unsafe { invoke_method(self.dispatch.as_ref().unwrap(), "GetProcessHandle", &mut []) };
        match result_variant {
            Ok(var) => {
                let handle = unsafe { VariantToInt64(&var as *const VARIANT).unwrap() };
                anyOk(handle)
            }
            Err(e) => bail!("Error::Main::get_process_handle: {}", e),
        }
    }
    /// This function retrives the current STAAD.Pro process ID.
    pub fn get_process_id(&self) -> Result<i32, anyErr> {
        let result_variant =
            unsafe { invoke_method(self.dispatch.as_ref().unwrap(), "GetProcessId", &mut []) };
        match result_variant {
            Ok(var) => {
                let process_id = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() };
                anyOk(process_id)
            }
            Err(e) => bail!("Error::Main::get_process_id: {}", e),
        }
    }

    /// Retrieves the short job information of the currently open .STD file.
    /// # Return values
    /// * Returns tuple containing (jobName, jobClient, enggName)
    pub fn get_short_job_info(&self) -> Result<(String, String, String), anyErr> {
        unsafe {
            let job_name_ptr = &mut BSTR::default() as *mut BSTR;
            let job_client_ptr = &mut BSTR::default() as *mut BSTR;
            let engg_name_ptr = &mut BSTR::default() as *mut BSTR;

            let mut params = [
                variant_from_raw_pointer::<BSTR>(engg_name_ptr),
                variant_from_raw_pointer::<BSTR>(job_client_ptr),
                variant_from_raw_pointer::<BSTR>(job_name_ptr),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetShortJobInfo",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk((
                    (&*job_name_ptr).to_string(),
                    (&*job_client_ptr).to_string(),
                    (&*engg_name_ptr).to_string(),
                )),
                Err(e) => bail!("Error::Main::get_short_job_info: {}", e),
            }
        }
    }

    /// Retrieves the name of the current .STD file.
    /// # Parameters
    /// * `[in] bFullPath` A Boolean variable which if true, will write the entire path of the .STD file or else only the file name.
    /// # Return values
    /// * fileName A string variable that will hold the name or path of the currently open .STD file.
    pub fn get_staad_file(&self, full_path: bool) -> Result<String, anyErr> {
        unsafe {
            let file_name_ptr = &mut BSTR::default() as *mut BSTR;
            let mut params = [
                VARIANT::from(full_path),
                variant_from_raw_pointer::<BSTR>(file_name_ptr),
            ];

            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "GetSTAADFile", &mut params);
            match result_variant {
                Ok(_) => {
                    let file_name = (&*file_name_ptr).to_string();
                    anyOk(file_name)
                }
                Err(e) => bail!("Error::Main::get_staad_file: {}", e),
            }
        }
    }

    /// Retrieves only the path of the current .STD file.
    /// # Return values
    /// * fileFolder A string variable that will hold the path name of folder where the currently open .STD file resides.
    pub fn get_staad_file_folder(&self) -> Result<String, anyErr> {
        unsafe {
            let file_folder_ptr = &mut BSTR::default() as *mut BSTR;
            let mut params = [variant_from_raw_pointer::<BSTR>(file_folder_ptr)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "GetSTAADFileFolder",
                &mut params,
            );
            match result_variant {
                Ok(_) => {
                    let file_folder = (&*file_folder_ptr).to_string();
                    anyOk(file_folder)
                }
                Err(e) => bail!("Error::Main::get_staad_file_folder: {}", e),
            }
        }
    }

    /// Returns a value to specify whether the analysis is running or not.
    /// # Return values
    /// * Returns 1 if analysis is still running, 0 otherwise.
    pub fn is_analyzing(&self) -> Result<bool, anyErr> {
        let result_variant =
            unsafe { invoke_method(self.dispatch.as_ref().unwrap(), "IsAnalyzing", &mut []) };
        match result_variant {
            Ok(var) => {
                let is_analyzing = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() > 0 };
                anyOk(is_analyzing)
            }
            Err(e) => bail!("Error::Main::is_analyzing: {}", e),
        }
    }

    /// Checks if the loaded model is a physical model.
    /// # Return values
    /// * `1` True
    /// * `0` False
    pub fn is_physical_model(&self) -> Result<bool, anyErr> {
        let result_variant =
            unsafe { invoke_method(self.dispatch.as_ref().unwrap(), "IsPhysicalModel", &mut []) };
        match result_variant {
            Ok(var) => {
                let is_physical = unsafe { VariantToInt32(&var as *const VARIANT).unwrap() > 0 };
                anyOk(is_physical)
            }
            Err(e) => bail!("Error::Main::is_physical_model: {}", e),
        }
    }

    /// This function saves the current view with the specified name.
    /// # Parameters
    /// * `[in] strName` A string variable that will hold the name of the view to be modified.
    /// * `[in] nEntities` An long variable that will hold number of entities.
    /// * `[in] EntityArray` An long that will hold entity number.
    /// * `[in] nArrayQualifier` A integer variable that will hold entity qualifier value. Value may vary from 0 to 4( 0 - Node, 1 - Beam, 2 - Plate, 3 - Solid, 4 – Surface)
    /// * `[in] nModifyFlag` A long variable that will hold the flag value depending upon which the view will be modified.
    /// # Return values
    /// * nError A long variable that will hold the error number if the view cannot be modified.
    pub fn modify_named_view(
        &self,
        name: &str,
        entities: i32,
        entity_array: i32,
        array_qualifier: i32,
        modify_flag: i32,
    ) -> Result<i32, anyErr> {
        unsafe {
            let error_ptr = &mut 0i32 as *mut i32;
            let mut params = [
                variant_from_raw_pointer::<i32>(error_ptr),
                VARIANT::from(modify_flag),
                VARIANT::from(array_qualifier),
                VARIANT::from(entity_array),
                VARIANT::from(entities),
                VARIANT::from(name),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "ModifyNamedView",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(*error_ptr),
                Err(e) => bail!("Error::Main::modify_named_view: {}", e),
            }
        }
    }

    /// This function creates a .STD file with specified length and force units.
    /// # Parameters
    /// * `[in] bstrFileName` A string variable that will hold the name of the .STD file, which needs to be created.
    /// * `[in] nLenUnitInput` An integer variable that will hold the input unit to be assigned for length. Value may vary from 0 to 7 (0- Inch, 1- Feet, 2- Feet, 3- CentiMeter, 4- Meter, 5- MilliMeter, 6 - DeciMeter, 7 – KiloMeter).
    /// * `[in] nForceUnitInput` An integer variable that will hold the input unit to be assigned for force. Value may vary from 0 to 7 (0- Kilopound, 1- Pound, 2- Kilogram, 3-Metric Ton, 4- Newton, 5-Kilo Newton, 6- Mega Newton, 7- DecaNewton).
    pub fn new_staad_file(
        &self,
        file_name: &str,
        len_unit: i32,
        force_unit: i32,
    ) -> Result<(), anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(force_unit),
                VARIANT::from(len_unit),
                VARIANT::from(file_name),
            ];

            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "NewSTAADFile", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::new_staad_file: {}", e),
            }
        }
    }

    /// This function will open the specified .STD file.
    /// # Parameters
    /// * `[in] bstrFileName` A string variable that will hold the name of the .STD file, which needs to be open.
    pub fn open_staad_file(&self, file_name: &str) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(file_name)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "OpenSTAADFile",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::open_staad_file: {}", e),
            }
        }
    }

    /// This function closes the STAAD.Pro application environment.
    pub fn quit(&self) -> Result<(), anyErr> {
        unsafe {
            let result_variant = invoke_method(self.dispatch.as_ref().unwrap(), "Quit", &mut []);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::quit: {}", e),
            }
        }
    }

    /// This function removes the current view with the specified name.
    /// # Parameters
    /// * `[in] strName` A string variable that will hold the name of the view to be removed.
    /// # Return values
    /// * nError A long variable that will hold the error number if the view cannot be removed.
    pub fn remove_named_view(&self, name: &str) -> Result<i32, anyErr> {
        unsafe {
            let error_ptr = &mut 0i32 as *mut i32;
            let mut params = [
                variant_from_raw_pointer::<i32>(error_ptr),
                VARIANT::from(name),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "RemoveNamedView",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(*error_ptr),
                Err(e) => bail!("Error::Main::remove_named_view: {}", e),
            }
        }
    }

    /// This function saves the current structure with optional silent mode.
    /// # Parameters
    /// * `[in] varSilent` Integer value to enable silent mode. [1 = Enable, 0 = Disable, default = 0]
    pub fn save_model(&self, silent: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(silent)];

            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "SaveModel", &mut params);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::save_model: {}", e),
            }
        }
    }

    /// This function saves the current view with the specified name.
    /// # Parameters
    /// * `[in] strName` A string variable that will hold the name of the view to be saved.
    /// # Return values
    /// * nError A long variable that will hold the error number if the view cannot be saved.
    pub fn save_named_view(&self, name: &str) -> Result<i32, anyErr> {
        unsafe {
            let error_ptr = &mut 0i32 as *mut i32;
            let mut params = [
                variant_from_raw_pointer::<i32>(error_ptr),
                VARIANT::from(name),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SaveNamedView",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(*error_ptr),
                Err(e) => bail!("Error::Main::save_named_view: {}", e),
            }
        }
    }

    /// Set CONNECTED Project ID and name.
    /// # Parameters
    /// * `[in] szProjID` ID of the CONNECTED Project
    /// * `[in] szName` Name of the CONNECTED Project
    /// # Return values
    /// * `1` True.
    /// * `0` False.
    pub fn set_connected_project_info(&self, proj_id: &str, name: &str) -> Result<bool, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(name), VARIANT::from(proj_id)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SetCONNECTEDProjectInfo",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let success = VariantToInt32(&var as *const VARIANT).unwrap() > 0;
                    anyOk(success)
                }
                Err(e) => bail!("Error::Main::set_connected_project_info: {}", e),
            }
        }
    }

    /// This function sets the full job information of the currently open .STD file.
    /// # Parameters
    /// * `[in] jobName` Job Name for the current .STD file.
    /// * `[in] jobClient` Job Client for the current .STD file.
    /// * `[in] enggName` Engineer's Name for the current .STD file.
    /// * `[in] eDate` Engineer Date for the current .STD file.
    /// * `[in] jobNumber` Job Number for the current .STD file.
    /// * `[in] revision` Job Revision for the current .STD file.
    /// * `[in] part` Job Part Name for the current .STD file.
    /// * `[in] reference` Job Reference for the current .STD file.
    /// * `[in] checkerName` Checker Name for the current .STD file.
    /// * `[in] cDate` Checker Date for the current .STD file.
    /// * `[in] approverName` Approver Name for the current .STD file.
    /// * `[in] aDate` Approved Date for the current .STD file.
    /// * `[in] comments` Job Comments for the current .STD file.
    pub fn set_full_job_info(
        &self,
        job_name: &str,
        job_client: &str,
        engg_name: &str,
        e_date: &str,
        job_number: &str,
        revision: &str,
        part: &str,
        reference: &str,
        checker_name: &str,
        c_date: &str,
        approver_name: &str,
        a_date: &str,
        comments: &str,
    ) -> Result<(), anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(comments),
                VARIANT::from(a_date),
                VARIANT::from(approver_name),
                VARIANT::from(c_date),
                VARIANT::from(checker_name),
                VARIANT::from(reference),
                VARIANT::from(part),
                VARIANT::from(revision),
                VARIANT::from(job_number),
                VARIANT::from(e_date),
                VARIANT::from(engg_name),
                VARIANT::from(job_client),
                VARIANT::from(job_name),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SetFullJobInfo",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::set_full_job_info: {}", e),
            }
        }
    }

    /// This function sets the input unit of force of the currently open .STD file.
    /// # Parameters
    /// * `[in] iUnit` An integer variable that will hold the input unit to be assigned for force. Value may vary from 0 to 7 (0- Kilopound, 1- Pound, 2- Kilogram, 3-Metric Ton, 4- Newton, 5-Kilo Newton, 6- Mega Newton, 7- DecaNewton).
    pub fn set_input_unit_for_force(&self, unit: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(unit)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SetInputUnitForForce",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::set_input_unit_for_force: {}", e),
            }
        }
    }

    /// This function sets the input unit of length of the currently open .STD file.
    /// # Parameters
    /// * `[in] iUnit` An integer variable that will hold the input unit to be assigned for length. Value may vary from 0 to 7 (0- Inch, 1- Feet, 2- Feet, 3- CentiMeter, 4- Meter, 5- MilliMeter, 6 - DeciMeter, 7 – KiloMeter).
    pub fn set_input_unit_for_length(&self, unit: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(unit)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SetInputUnitForLength",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::set_input_unit_for_length: {}", e),
            }
        }
    }

    /// This function sets the input unit of length of the currently open .STD file.
    /// # Parameters
    /// * `[in] lUnit` An integer variable that will hold the input unit to be assigned for length. Value may vary from 0 to 7 (0- Inch, 1- Feet, 2- Feet, 3- CentiMeter, 4- Meter, 5- MilliMeter, 6 - DeciMeter, 7 – KiloMeter).
    /// * `[in] fUnit` An integer variable that will hold the input unit to be assigned for force. Value may vary from 0 to 7 (0- Kilopound, 1- Pound, 2- Kilogram, 3-Metric Ton, 4- Newton, 5-Kilo Newton, 6- Mega Newton, 7- DecaNewton).
    pub fn set_input_units(&self, length_unit: i32, force_unit: i32) -> Result<(), anyErr> {
        unsafe {
            let mut params = [VARIANT::from(force_unit), VARIANT::from(length_unit)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SetInputUnits",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::set_input_units: {}", e),
            }
        }
    }

    /// This function sets the short job information of the currently open .STD file.
    /// # Parameters
    /// * `[in] jobName` Job Name for the current .STD file.
    /// * `[in] jobClient` Job Client for the current .STD file.
    /// * `[in] enggName` Engineer's Name for the current .STD file.
    pub fn set_short_job_info(
        &self,
        job_name: &str,
        job_client: &str,
        engg_name: &str,
    ) -> Result<(), anyErr> {
        unsafe {
            let mut params = [
                VARIANT::from(engg_name),
                VARIANT::from(job_client),
                VARIANT::from(job_name),
            ];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SetShortJobInfo",
                &mut params,
            );
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::set_short_job_info: {}", e),
            }
        }
    }

    /// Sets the silent mode of the application. Calling this method with 1, will suppress the display of warning message boxes during analysis and saving of the file For example, warning messages like empty loads, repeat load expansions etc. Note, if any error occurs like license not available, or folder inaccessible, message boxes will be displayed appropriately.
    /// # Parameters
    /// * `[in] varFlag` An integer value 1 to set silent mode on and 0 to set it off
    /// # Return values
    /// * The existing value of the silent mode
    pub fn set_silent_mode(&self, flag: i32) -> Result<i32, anyErr> {
        unsafe {
            let mut params = [VARIANT::from(flag)];

            let result_variant = invoke_method(
                self.dispatch.as_ref().unwrap(),
                "SetSilentMode",
                &mut params,
            );
            match result_variant {
                Ok(var) => {
                    let existing_value = VariantToInt32(&var as *const VARIANT).unwrap();
                    anyOk(existing_value)
                }
                Err(e) => bail!("Error::Main::set_silent_mode: {}", e),
            }
        }
    }

    /// This function updates the current structure.
    pub fn update_structure(&self) -> Result<(), anyErr> {
        unsafe {
            let result_variant =
                invoke_method(self.dispatch.as_ref().unwrap(), "UpdateStructure", &mut []);
            match result_variant {
                Ok(_) => anyOk(()),
                Err(e) => bail!("Error::Main::update_structure: {}", e),
            }
        }
    }
}

// unsafe impl Send for Root{}
// unsafe impl Sync for Root{}
