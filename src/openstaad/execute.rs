use anyhow::{Result, bail};
use log::warn;
use serde_json::Value::Null;
use serde_json::{Value, to_value as to_json};
use windows::Win32::System::Com::{
    CLSCTX_LOCAL_SERVER, CLSIDFromProgID, CoCreateInstance, IDispatch,
};
use windows::Win32::System::Variant::{VariantClear, VariantToInt32};
use windows::Win32::{
    Foundation::VARIANT_BOOL,
    System::{
        Com::SAFEARRAY,
        Ole::SafeArrayCreateVector,
        Variant::{VARIANT, VT_BSTR, VT_I4, VT_R8},
    },
};
use windows_core::{BSTR, HSTRING, PCWSTR};

// use crate::tools::value_types::count_general_input;
use crate::{
    openstaad::bindings::Staad,
    tools::{
        invoke::invoke_method,
        value_types::{InType, Input, OutType},
        variant::SafeArrayP,
        variant_with_ptr_from,
    },
};

pub fn execute_method(instance: &Staad, method_name: &str, params: &[Input]) -> Result<Value> {
    let (app, methods) = match instance {
        Staad::OpenStaad(v) => (&v.dispatch, &v.methods),
        Staad::Geometry(v) => (&v.dispatch, &v.methods),
        Staad::Command(v) => (&v.dispatch, &v.methods),
        Staad::Design(v) => (&v.dispatch, &v.methods),
        Staad::Load(v) => (&v.dispatch, &v.methods),
        Staad::Output(v) => (&v.dispatch, &v.methods),
        Staad::Property(v) => (&v.dispatch, &v.methods),
        Staad::Support(v) => (&v.dispatch, &v.methods),
        _ => bail!("[Execute] Unsupported Staad instance".to_string()),
    };
    let (_inputs, _outputs) = match methods.get(method_name) {
        Some(sig) => (&sig.inputs, &sig.outputs),
        None => bail!(format!("[Execute] Unsupported method: {}", method_name)),
    };

    let params_count = params.len();
    let required_count = InType::count_general_type(_inputs);
    if params_count != required_count {
        bail!(
            "[Execute] '{}' method takes {} arguments but {} arguments were supplied",
            method_name,
            required_count,
            params_count
        );
    }

    // Create separate storage for mutable pointers to ensure each has unique memory location
    let mut mut_storages: Vec<Box<dyn std::any::Any>> = Vec::new();
    let mut __variants: Vec<VARIANT> = _inputs
        .iter()
        .enumerate()
        .map(|(i, _type)| {
            if _type.is_general_type() {
                return params[i].to_variant();
            }
            // Create individual storage for each mutable parameter
            match _type {
                InType::MutInt => {
                    let mut mut_value = Box::new(0i32);
                    let ptr = mut_value.as_mut() as *mut i32;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<i32>(ptr)
                }
                InType::MutDouble => {
                    let mut mut_value = Box::new(0f64);
                    let ptr = mut_value.as_mut() as *mut f64;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<f64>(ptr)
                }
                InType::MutStr => {
                    let mut mut_value = Box::new(BSTR::default());
                    let ptr = mut_value.as_mut() as *mut BSTR;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<BSTR>(ptr)
                }
                InType::MutBool => {
                    let mut mut_value = Box::new(VARIANT_BOOL(-1));
                    let ptr = mut_value.as_mut() as *mut VARIANT_BOOL;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<bool>(ptr)
                }
                InType::MutVecInt => unsafe {
                    let count = get_array_count(app, method_name, params, i);
                    let psa = SafeArrayCreateVector(VT_I4, 0, count as u32);
                    let mut mut_value = Box::new(psa);
                    let ptr = mut_value.as_mut() as *mut *mut SAFEARRAY;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<SafeArrayP<i32>>(ptr)
                },
                InType::MutVecDouble => unsafe {
                    let count = get_array_count(app, method_name, params, i);
                    let psa = SafeArrayCreateVector(VT_R8, 0, count as u32);
                    let mut mut_value = Box::new(psa);
                    let ptr = mut_value.as_mut() as *mut *mut SAFEARRAY;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<SafeArrayP<f64>>(ptr)
                },
                InType::MutVecStr => unsafe {
                    let count = get_array_count(app, method_name, params, i);
                    let psa = SafeArrayCreateVector(VT_BSTR, 0, count as u32);
                    let mut mut_value = Box::new(psa);
                    let ptr = mut_value.as_mut() as *mut *mut SAFEARRAY;
                    mut_storages.push(mut_value);
                    variant_with_ptr_from::<SafeArrayP<BSTR>>(ptr)
                },
                InType::MemberSteelDgnParams => unsafe {
                    let clsid_str =
                        PCWSTR::from_raw(HSTRING::from("StaadPro.MembSteelDgnParams").as_ptr());
                    let clsid = CLSIDFromProgID(clsid_str).unwrap();
                    let _instance = CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER).unwrap();
                    let instace_ptr: *mut Option<IDispatch> = &mut Some(_instance);
                    variant_with_ptr_from::<Option<IDispatch>>(instace_ptr)
                },
                _ => VARIANT::default(),
            }
        })
        .collect();
    let mut __params: &mut [VARIANT] = &mut __variants[..];
    __params.reverse();
    unsafe {
        let args_len = _inputs.len();
        let invoke_result = invoke_method(app, method_name, __params);
        match invoke_result {
            Ok(result_variant) => {
                let mut result_values: Vec<Value> = Vec::new();
                for (oi, ot) in _outputs.iter().enumerate() {
                    let serde_v = match ot {
                        &OutType::Index(i) => {
                            let in_type = _inputs.get(i as usize).unwrap();
                            let _var = &__params[args_len - ((i + 1) as usize)];
                            let _v = in_type.to_output_as(_var);
                            let _ = VariantClear(_var as *const VARIANT as *mut VARIANT);
                            _v
                        }
                        _ => {
                            let _var = &result_variant;
                            let _v = ot.to_value(_var);
                            let _ = VariantClear(_var as *const VARIANT as *mut VARIANT);
                            _v
                        }
                    };
                    match serde_v {
                        Ok(_v) => {
                            result_values.push(_v);
                        }
                        Err(e) => bail!("{}", e),
                    }
                }
                if result_values.len() > 0 {
                    if result_values.len() == 1 {
                        let v = to_json(&result_values[0])?;
                        return Ok(v);
                    }
                    let v = to_json(&result_values)?;
                    return Ok(v);
                } else {
                    return Ok(Null);
                }
            }
            Err(e) => bail!("Fail to invoke method: {}", e),
        }
    }
}

fn get_array_count(dispatch: &IDispatch, method: &str, params: &[Input], index: usize) -> u32 {
    let mut count = 0 as u32;
    unsafe {
        match method {
            // Geometry::Node
            "GetNodeList" => {
                let variant = invoke_method(dispatch, "GetNodeCount", &mut []).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            // Geometry::Beam
            "BreakBeamsAtSpecificNodes" => {
                let var = params[0].clone().to_variant();
                let variant = invoke_method(
                    dispatch,
                    "GetCountOfBreakableBeamsAtSpecificNodes",
                    &mut [var],
                )
                .unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetBeamList" => {
                let variant = invoke_method(dispatch, "GetMemberCount", &mut []).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetBeamsConnectedAtNode" => {
                let var = params[0].clone().to_variant();
                let variant =
                    invoke_method(dispatch, "GetNoOfBeamsConnectedAtNode", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "InterSectBeams" => {
                let variant = invoke_method(
                    dispatch,
                    "GetIntersectBeamsCount",
                    &mut [
                        params[2].clone().to_variant(),
                        params[1].clone().to_variant(),
                    ],
                )
                .unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            // Geometry::Group
            "GetGroupEntities" => {
                let var = params[0].clone().to_variant();
                let variant = invoke_method(dispatch, "GetGroupEntityCount", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetGroupNames" => {
                let var = params[0].clone().to_variant();
                let variant = invoke_method(dispatch, "GetGroupCount", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            // Property::UPT
            "GetUptGeneralProfileBoundaryPoints" => {
                let var1 = params[0].clone().to_variant();
                let var2 = params[1].clone().to_variant();
                let variant = invoke_method(
                    dispatch,
                    "GetUptGeneralProfilePointsCount",
                    &mut [var2, var1],
                )
                .unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetUptGeneralStressLocationPoints" => {
                count = 4; // Fixed size array according to documentation
            }
            // Support methods
            "GetElasticFootingAssignmentList" => {
                let var1 = params[0].clone().to_variant();
                let count_ptr = &mut 0i32 as *mut i32;
                let _ = invoke_method(
                    dispatch,
                    "GetElasticFootingDetail",
                    &mut [
                        variant_with_ptr_from::<i32>(count_ptr),
                        variant_with_ptr_from::<f64>(&mut 0f64 as *mut f64),
                        variant_with_ptr_from::<i32>(&mut 0i32 as *mut i32),
                        variant_with_ptr_from::<f64>(&mut 0f64 as *mut f64),
                        variant_with_ptr_from::<f64>(&mut 0f64 as *mut f64),
                        var1,
                    ],
                )
                .unwrap();
                count = *count_ptr as u32;
            }
            "GetElasticMatAssignmentList" => {
                let var1 = params[0].clone().to_variant();
                let count_ptr = &mut 0i32 as *mut i32;
                let _ = invoke_method(
                    dispatch,
                    "GetElasticMatDetail",
                    &mut [
                        variant_with_ptr_from::<i32>(count_ptr),
                        variant_with_ptr_from::<i32>(&mut 0i32 as *mut i32),
                        variant_with_ptr_from::<i32>(&mut 0i32 as *mut i32),
                        variant_with_ptr_from::<f64>(&mut 0f64 as *mut f64),
                        variant_with_ptr_from::<i32>(&mut 0i32 as *mut i32),
                        var1,
                    ],
                )
                .unwrap();
                count = *count_ptr as u32;
            }
            "GetPlateMatAssignmentList" => {
                let var1 = params[0].clone().to_variant();
                let count_ptr = &mut 0i32 as *mut i32;
                let _ = invoke_method(
                    dispatch,
                    "GetPlateMatDetail",
                    &mut [
                        variant_with_ptr_from::<i32>(count_ptr),
                        variant_with_ptr_from::<i32>(&mut 0i32 as *mut i32),
                        variant_with_ptr_from::<i32>(&mut 0i32 as *mut i32),
                        variant_with_ptr_from::<f64>(&mut 0f64 as *mut f64),
                        variant_with_ptr_from::<f64>(&mut 0f64 as *mut f64),
                        variant_with_ptr_from::<f64>(&mut 0f64 as *mut f64),
                        variant_with_ptr_from::<i32>(&mut 0i32 as *mut i32),
                        var1,
                    ],
                )
                .unwrap();
                count = *count_ptr as u32;
            }
            "GetSupportNodes" => {
                let variant = invoke_method(dispatch, "GetSupportCount", &mut []).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetSupportInformation" => {
                count = 6; // Fixed size array for release and spring specs (FX, FY, FZ, MX, MY, MZ)
            }
            "GetSupportInformationEx" => {
                count = 6; // Fixed size array for release and spring specs (FX, FY, FZ, MX, MY, MZ)
            }
            // Reference Load methods
            "GetReferenceLoadCaseNumbers" => {
                let variant =
                    invoke_method(dispatch, "GetReferenceLoadCaseCount", &mut []).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            // Nodal Load methods
            "GetNodalLoadInfo" => {
                count = 6; // Fixed size array for forces (FX, FY, FZ, MX, MY, MZ)
            }
            "GetNodalLoads" => {
                let var = params[0].clone().to_variant();
                let variant = invoke_method(dispatch, "GetNodalLoadCount", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            // Member Load methods
            "GetConcForces" => {
                let var = params[0].clone().to_variant();
                let variant = invoke_method(dispatch, "GetConcForceCount", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetConcMoments" => {
                let var = params[0].clone().to_variant();
                let variant = invoke_method(dispatch, "GetConcMomentCount", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetLinearVaryingLoads" => {
                let var = params[0].clone().to_variant();
                let variant =
                    invoke_method(dispatch, "GetLinearVaryingLoadCount", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetMemberLoadInfo" => {
                count = 3; // Fixed size arrays for force and distance parameters
            }
            "GetTrapLoads" => {
                let var = params[0].clone().to_variant();
                let variant = invoke_method(dispatch, "GetTrapLoadCount", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetUDLLoads" => {
                let var = params[0].clone().to_variant();
                let variant = invoke_method(dispatch, "GetUDLLoadCount", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetUNIMoments" => {
                let var = params[0].clone().to_variant();
                let variant = invoke_method(dispatch, "GetUNIMomentCount", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            // Element Load methods
            "GetElementConcLoads" => {
                let var = params[0].clone().to_variant();
                let variant =
                    invoke_method(dispatch, "GetElementConcLoadCount", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetElementLoadInfo" => {
                count = 4; // Fixed size arrays for force and distance parameters (W1-W4, X1-X2, Y1-Y2)
            }
            "GetElementPressureLoads" => {
                let var = params[0].clone().to_variant();
                let variant =
                    invoke_method(dispatch, "GetElementPressureLoadCount", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            // Floor Load methods
            "GetInfluenceArea" => {
                let variant = invoke_method(
                    dispatch,
                    "GetBeamCountAtFloor",
                    &mut [
                        params[6].clone().to_variant(),
                        params[5].clone().to_variant(),
                        params[4].clone().to_variant(),
                        params[3].clone().to_variant(),
                        params[2].clone().to_variant(),
                        params[1].clone().to_variant(),
                        params[0].clone().to_variant(),
                    ],
                )
                .unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            // Repeat Load methods
            "GetNotionalLoadByIndex" => {
                let var = params[0].clone().to_variant();
                let variant = invoke_method(
                    dispatch,
                    "GetNoLoadFactorDirectionInNotionalLoad",
                    &mut [var],
                )
                .unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetReferenceLoadByIndex" => {
                let var = params[0].clone().to_variant();
                let variant =
                    invoke_method(dispatch, "GetNoOfSetsInReferenceLoad", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetRepeatLoadByIndex" => {
                let var = params[0].clone().to_variant();
                let variant =
                    invoke_method(dispatch, "GetNoLoadFactorInRepeatLoad", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            // Load Combination methods
            "GetLoadAndFactorForCombination" => {
                let var = params[0].clone().to_variant();
                let variant = invoke_method(
                    dispatch,
                    "GetNoOfLoadAndFactorPairsForCombination",
                    &mut [var],
                )
                .unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetLoadCombinationCaseNumbers" => {
                let variant =
                    invoke_method(dispatch, "GetLoadCombinationCaseCount", &mut []).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            // Load Case Operation methods
            "GetAssignmentListForLoadType" => {
                let var1 = params[0].clone().to_variant();
                let var2 = params[1].clone().to_variant();
                let variant =
                    invoke_method(dispatch, "GetListSizeForLoadType", &mut [var2, var1]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetLoadsInLoadList" => {
                let var = params[0].clone().to_variant();
                let variant =
                    invoke_method(dispatch, "GetLoadCountInLoadList", &mut [var]).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetPrimaryLoadCaseNumbers" => {
                let variant = invoke_method(dispatch, "GetPrimaryLoadCaseCount", &mut []).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            // Load Envelopes methods
            "GetEnvelopeIDs" => {
                let variant = invoke_method(dispatch, "GetEnvelopeCount", &mut []).unwrap();
                count = VariantToInt32(&variant as *const VARIANT).unwrap() as u32;
            }
            "GetLoadListfromLoadEnvelope" => {
                let var = params[0].clone().to_variant();
                let count_ptr = &mut 0i32 as *mut i32;
                let _ = invoke_method(
                    dispatch,
                    "GetLoadEnvelopeDetails",
                    &mut [
                        variant_with_ptr_from::<i32>(count_ptr),
                        variant_with_ptr_from::<i32>(&mut 0i32 as *mut i32),
                        var,
                    ],
                )
                .unwrap();
                count = *count_ptr as u32;
            }
            // Output::Nodes/Joints/Supports
            "GetBasePressures" => {
                if let Input::VecInt(_vec) = &params[1] {
                    count = _vec.len() as u32;
                }
            }
            "GetMatInfluenceAreas" => {
                if let Input::VecInt(_vec) = &params[0] {
                    count = _vec.len() as u32;
                }
            }
            "GetNodeDisplacements" => {
                count = 6;
            }
            "GetSupportReactions" => {
                count = 6;
            }
            // Output::Members
            "GetIntermediateMemberAbsTransDisplacements" => {
                count = 6; // X, Y, Z displacements and rotations
            }
            "GetIntermediateMemberForcesAtDistance" => {
                count = 6; // Fx, Fy, Fz, Mx, My, Mz
            }
            "GetIntermediateMemberTransDisplacements" => {
                count = 6; // X, Y, Z displacements and rotations
            }
            "GetMemberEndDisplacements" => {
                count = 3; // X, Y, Z displacements only (per VBA example)
            }
            "GetMemberEndForces" => {
                count = 6; // FX, FY, FZ, MX, MY, MZ
            }
            "GetPMemberEndForces" => {
                count = 6; // FX, FY, FZ, MX, MY, MZ
            }
            "GetPMemberIntermediateForcesAtDistance" => {
                count = 6; // Fx, Fy, Fz, Mx, My, Mz
            }
            // Output::Plate
            "GetAllPlateCenterForces" => {
                count = 5; // SQX, SQY, SX, SY, SXY
            }
            "GetAllPlateCenterMoments" => {
                count = 3; // MX, MY, MXY
            }
            "GetAllPlateCenterPrincipalStressesAndAngles" => {
                count = 8; // 6 stress values + 2 angles
            }
            "GetAllPlateCenterPrincipalStressesAndAnglesEx" => {
                if index == 2 {
                    count = 6; // stress array: Top/Bottom Max/Min/TauMax (6 values)
                } else if index == 3 {
                    count = 2; // angles array: Top/Bottom angles (2 values)
                } else {
                    count = 0; // Should not happen for this method
                    warn!(
                        "Unexpected index {} for GetAllPlateCenterPrincipalStressesAndAnglesEx",
                        index
                    );
                }
            }
            "GetAllPlateCenterStressesAndMoments" => {
                count = 8; // SQX, SQY, MX, MY, MXY, SX, SY, SXY
            }
            "GetPlateCornerForces" => {
                count = 6; // Force components at corner
            }
            "GetPlateStressAtPoint" => {
                count = 32; // Fixed size stress array per documentation
            }
            "GetResultantForceAlongLineForParametricSurface" => {
                count = 6; // Fx, Fy, Fz, Mx, My, Mz
            }
            "GetResultantForceAlongLineForPlateList" => {
                count = 6; // Fx, Fy, Fz, Mx, My, Mz
            }
            // Output::Solids
            "GetAllSolidNormalStresses" => {
                count = 3; // SXX, SYY, SZZ
            }
            "GetAllSolidPrincipalStresses" => {
                count = 3; // S_1, S_2, S_3
            }
            "GetAllSolidShearStresses" => {
                count = 3; // SXY, SYZ, SZX
            }
            // Output::Dynamic
            "GetModalDisplacementAtNode" => {
                count = 6; // X, Y, Z, rX, rY, rZ displacements
            }
            "GetNLNodeDisplacements" => {
                count = 6; // X, Y, Z, rX, rY, rZ displacements
            }
            "GetTimeHistoryResponse" => {
                // Array size depends on number of integration steps
                // This needs special handling - get from GetTimeHistoryIntegrationStepInfo
                let variant = invoke_method(
                    dispatch,
                    "GetTimeHistoryIntegrationStepInfo",
                    &mut [variant_with_ptr_from::<f64>(&mut 0f64 as *mut f64)],
                )
                .unwrap();
                count = (VariantToInt32(&variant as *const VARIANT).unwrap() + 1) as u32;
            }
            // Output::Static
            "GetStaticCheckResult" => {
                count = 6; // FX, FY, FZ, MX, MY, MZ (both varForces and varReactions have same size)
            }
            // Output::Buckling
            "GetBucklingModeDisplacementAtNode" => {
                count = 6; // X, Y, Z, rX, rY, rZ displacements
            }
            // Output::Design
            "GetMemberSteelDesignResults" => {
                count = 3; // Design forces array: FX, MY, MZ
            }
            _ => {
                warn!("Must be submitted safe array count: {}", method)
            }
        };
    }
    count
}
