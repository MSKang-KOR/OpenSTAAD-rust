use std::{mem::ManuallyDrop, ptr::null_mut};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use windows::Win32::System::{
    Com::{IDispatch, SAFEARRAY},
    Ole::{
        SafeArrayAccessData, SafeArrayGetDim, SafeArrayGetLBound, SafeArrayGetUBound,
        SafeArrayUnaccessData,
    },
    Variant::{
        VARIANT, VARIANT_0_0, VARIANT_0_0_0, VT_ARRAY, VT_BYREF, VT_EMPTY, VT_I4, VT_NULL,
        VT_VARIANT, VariantClear, VariantToStringAlloc,
    },
};

use crate::staad::process::{invoke_method_with_result, invoke_method_without_result};

#[derive(Debug)]
pub struct Geometry {
    pub dispatch: Option<IDispatch>,
    pub node_list: Vec<i32>,
}

impl Geometry {
    pub fn new(dispatch: &IDispatch) -> Self {
        Geometry {
            dispatch: Some(dispatch.clone()),
            node_list: vec![],
        }
    }

    pub fn get_last_node_no(&self) -> Result<String, anyErr> {
        let params: &mut [VARIANT; 0] = &mut [];
        let result_variant = unsafe {
            invoke_method_with_result(self.dispatch.as_ref().unwrap(), "GetLastNodeNo", params)
        };
        match result_variant {
            Ok(var) => {
                let result_node_no = unsafe {
                    VariantToStringAlloc(&var as *const VARIANT)
                        .unwrap()
                        .to_string()
                };
                anyOk(result_node_no.unwrap())
            }
            Err(e) => bail!("Error::Geometry::get_last_node_no: {}", e),
        }
    }

    pub fn get_node_list(&self) -> Result<Vec<i32>, anyErr> {
        unsafe {
            // println!("1. Creating SAFEARRAY for output");
            // // Create an empty SAFEARRAY for LONG (i32) values
            // let psa = SafeArrayCreateVector(VT_I4, 0, 0);
            // if psa.is_null() {
            //     bail!("Failed to create SAFEARRAY");
            // }

            println!("1. Preparing for GetNodeList call");
            // Create VARIANT that holds the SAFEARRAY
            let mut result_variant = VARIANT::default();
            // result_variant.Anonymous.Anonymous = ManuallyDrop::new(VARIANT_0_0 {
            //     vt: (VT_BYREF | VT_ARRAY | VT_I4),
            //     wReserved1: 0,
            //     wReserved2: 0,
            //     wReserved3: 0,
            //     Anonymous: VARIANT_0_0_0 { parray: psa },
            // });
            // Create the out parameter - this should be passed by reference
            let mut out_param = VARIANT::default();
            out_param.Anonymous.Anonymous = ManuallyDrop::new(VARIANT_0_0 {
                vt: VT_BYREF | VT_VARIANT,
                wReserved1: 0,
                wReserved2: 0,
                wReserved3: 0,
                Anonymous: VARIANT_0_0_0 {
                    pvarVal: &mut result_variant as *mut VARIANT,
                },
            });
            let mut params: [VARIANT; 1] = [out_param];

            println!("2. Invoking COM method");
            let _ = invoke_method_without_result(
                self.dispatch.as_ref().unwrap(),
                "GetNodeList",
                &mut params,
            )?;

            println!("3. Processing result");

            // Check what we got back
            let vt = result_variant.Anonymous.Anonymous.vt;
            println!("Result VARIANT type: 0x{:04X}", vt.0);

            // Handle empty results
            if vt == VT_EMPTY || vt == VT_NULL {
                println!("Empty or null result");
                return Ok(Vec::new());
            }

            // Check if it's an array
            if (vt & VT_ARRAY).0 == 0 {
                println!("Result is not an array type, cleaning up");
                let _ = VariantClear(&mut result_variant);
                return Ok(Vec::new());
            }

            // Extract SAFEARRAY
            let psa = result_variant.Anonymous.Anonymous.Anonymous.parray;
            if psa.is_null() {
                println!("SAFEARRAY is null");
                let _ = VariantClear(&mut result_variant);
                return Ok(Vec::new());
            }

            // Validate array
            let dims = SafeArrayGetDim(psa);
            if dims != 1 {
                println!("Expected 1D array, got {}D", dims);
                let _ = VariantClear(&mut result_variant);
                bail!("Invalid array dimensions: {}", dims);
            }

            // Get bounds - handle potential errors
            let lbound_result = SafeArrayGetLBound(psa, 1);
            let ubound_result = SafeArrayGetUBound(psa, 1);

            let (lbound, ubound) = match (lbound_result, ubound_result) {
                (Ok(lb), Ok(ub)) => (lb, ub),
                _ => {
                    println!("Failed to get array bounds");
                    let _ = VariantClear(&mut result_variant);
                    bail!("Failed to get SAFEARRAY bounds");
                }
            };

            let count = (ubound - lbound + 1) as usize;
            println!(
                "Array info - Lower: {}, Upper: {}, Count: {}",
                lbound, ubound, count
            );

            if count == 0 {
                let _ = VariantClear(&mut result_variant);
                return Ok(Vec::new());
            }

            // Access data safely
            let mut data_ptr: *mut std::ffi::c_void = null_mut();
            let access_result = SafeArrayAccessData(psa, &mut data_ptr);

            if access_result.is_err() {
                println!("Failed to access SAFEARRAY data");
                let _ = VariantClear(&mut result_variant);
                bail!("SAFEARRAY data access failed");
            }

            if data_ptr.is_null() {
                SafeArrayUnaccessData(psa).ok(); // Try to unaccess, but don't fail if it errors
                let _ = VariantClear(&mut result_variant);
                bail!("SAFEARRAY data pointer is null");
            }

            // Convert to Vec
            let result_vec = {
                let int_array = std::slice::from_raw_parts(data_ptr as *const i32, count);
                int_array.to_vec()
            };

            // Unaccess data
            if let Err(e) = SafeArrayUnaccessData(psa) {
                println!("Warning: Failed to unaccess SAFEARRAY: {:?}", e);
            }

            // Clean up
            if let Err(e) = VariantClear(&mut result_variant) {
                println!("Warning: VariantClear failed: {:?}", e);
            }

            println!("Successfully retrieved {} nodes", result_vec.len());
            if result_vec.len() <= 10 {
                println!("Node IDs: {:?}", result_vec);
            } else {
                println!("First 10 nodes: {:?}", &result_vec[..10]);
            }

            Ok(result_vec)
        }
    }
    // 방법 1: Property Get 방식으로 시도
    pub fn get_node_list_as_property(&self) -> Result<Vec<i32>, anyErr> {
        unsafe {
            println!("=== Trying GetNodeList as property ===");

            let params: &mut [VARIANT; 0] = &mut [];
            let result_variant =
                invoke_method_with_result(self.dispatch.as_ref().unwrap(), "GetNodeList", params)?;

            let vt = result_variant.Anonymous.Anonymous.vt;
            println!("Property result VARIANT type: 0x{:04X}", vt.0);

            if vt == VT_EMPTY || vt == VT_NULL {
                return Ok(Vec::new());
            }

            if (vt & VT_ARRAY).0 != 0 {
                let psa = result_variant.Anonymous.Anonymous.Anonymous.parray;
                if !psa.is_null() {
                    return self.extract_nodes_from_safearray(psa);
                }
            }

            Ok(Vec::new())
        }
    }

    // 방법 2: 다른 VARIANT 구성 방식
    pub fn get_node_list_alternative(&self) -> Result<Vec<i32>, anyErr> {
        unsafe {
            use windows::Win32::System::Ole::SafeArrayCreateVector;

            println!("=== Alternative VARIANT construction ===");

            // 미리 할당된 SAFEARRAY 생성
            let psa = SafeArrayCreateVector(VT_I4, 0, 0);
            if psa.is_null() {
                bail!("Failed to create SAFEARRAY");
            }

            // VARIANT를 SAFEARRAY로 초기화
            let mut result_variant = VARIANT::default();
            result_variant.Anonymous.Anonymous = ManuallyDrop::new(VARIANT_0_0 {
                vt: VT_ARRAY | VT_I4,
                wReserved1: 0,
                wReserved2: 0,
                wReserved3: 0,
                Anonymous: VARIANT_0_0_0 { parray: psa },
            });
            // result_variant.Anonymous.Anonymous.vt = VT_ARRAY | VT_I4;
            // result_variant.Anonymous.Anonymous.Anonymous.parray = psa;

            // 직접 포인터로 전달
            let mut out_param = VARIANT::default();
            out_param.Anonymous.Anonymous = ManuallyDrop::new(VARIANT_0_0 {
                vt: VT_BYREF | VT_ARRAY | VT_I4,
                wReserved1: 0,
                wReserved2: 0,
                wReserved3: 0,
                Anonymous: VARIANT_0_0_0 {
                    parray: result_variant.Anonymous.Anonymous.Anonymous.parray,
                },
            });
            // out_param.Anonymous.Anonymous.vt = VT_BYREF | VT_ARRAY | VT_I4;
            // out_param.Anonymous.Anonymous.Anonymous.pparray =
            //     &mut result_variant.Anonymous.Anonymous.Anonymous.parray;

            let mut params: [VARIANT; 1] = [out_param];

            println!("Calling with pre-allocated SAFEARRAY");
            match invoke_method_without_result(
                self.dispatch.as_ref().unwrap(),
                "GetNodeList",
                &mut params,
            ) {
                Ok(_) => {
                    println!("COM call succeeded");

                    // 결과 SAFEARRAY 확인
                    let final_psa = result_variant.Anonymous.Anonymous.Anonymous.parray;
                    if !final_psa.is_null() {
                        let result = self.extract_nodes_from_safearray(final_psa);
                        let _ = VariantClear(&mut result_variant);
                        return result;
                    }
                }
                Err(e) => {
                    println!("COM call failed: {}", e);
                    let _ = VariantClear(&mut result_variant);
                    return Err(e);
                }
            }

            let _ = VariantClear(&mut result_variant);
            Ok(Vec::new())
        }
    }

    // 방법 3: DISPPARAMS 직접 구성
    pub fn get_node_list_raw_dispatch(&self) -> Result<Vec<i32>, anyErr> {
        unsafe {
            use windows::Win32::System::Com::{DISPATCH_METHOD, DISPPARAMS};
            use windows::core::{HSTRING, PCWSTR};

            println!("=== Raw dispatch approach ===");

            let dispatch = self.dispatch.as_ref().unwrap();

            // Method ID 가져오기
            let method_name = "GetNodeList";
            let name = PCWSTR::from_raw(HSTRING::from(method_name).as_ptr());
            let mut dispid = 0;
            let iid = &windows::core::GUID::default();

            dispatch
                .GetIDsOfNames(iid, &name, 1, 0, &mut dispid)
                .context("Failed to get method ID")?;

            println!("Method ID for GetNodeList: {}", dispid);

            // 결과를 받을 VARIANT 준비
            let mut result_variant = VARIANT::default();
            let mut out_param = VARIANT::default();
            out_param.Anonymous.Anonymous = ManuallyDrop::new(VARIANT_0_0 {
                vt: VT_BYREF | VT_VARIANT,
                wReserved1: 0,
                wReserved2: 0,
                wReserved3: 0,
                Anonymous: VARIANT_0_0_0 {
                    pvarVal: &mut result_variant,
                },
            });
            // out_param.Anonymous.Anonymous.vt = VT_BYREF | VT_VARIANT;
            // out_param.Anonymous.Anonymous.Anonymous.pvarVal = &mut result_variant;

            let mut params = [out_param];

            // DISPPARAMS 구성 (parameters are in reverse order)
            let dispparams = DISPPARAMS {
                rgvarg: params.as_mut_ptr(),
                cArgs: 1,
                cNamedArgs: 0,
                rgdispidNamedArgs: std::ptr::null_mut(),
            };

            let mut excepinfo = windows::Win32::System::Com::EXCEPINFO::default();

            println!("Calling Invoke directly");
            let hresult = dispatch.Invoke(
                dispid,
                iid,
                0,
                DISPATCH_METHOD,
                &dispparams,
                None,
                Some(&mut excepinfo),
                None,
            );

            if let Err(e) = hresult {
                println!("Invoke failed: {:?}", e);
                return Err(anyErr::msg("Direct invoke failed"));
            }

            println!("Direct invoke succeeded");
            let vt = result_variant.Anonymous.Anonymous.vt;
            println!("Result type: 0x{:04X}", vt.0);

            if (vt & VT_ARRAY).0 != 0 {
                let psa = result_variant.Anonymous.Anonymous.Anonymous.parray;
                if !psa.is_null() {
                    let result = self.extract_nodes_from_safearray(psa);
                    let _ = VariantClear(&mut result_variant);
                    return result;
                }
            }

            let _ = VariantClear(&mut result_variant);
            Ok(Vec::new())
        }
    }

    // SAFEARRAY에서 노드 추출하는 공통 함수
    fn extract_nodes_from_safearray(&self, psa: *mut SAFEARRAY) -> Result<Vec<i32>, anyErr> {
        unsafe {
            use std::ptr::null_mut;
            use windows::Win32::System::Ole::{
                SafeArrayAccessData, SafeArrayGetDim, SafeArrayGetLBound, SafeArrayGetUBound,
                SafeArrayUnaccessData,
            };

            let dims = SafeArrayGetDim(psa);
            if dims != 1 {
                bail!("Expected 1D array, got {}D", dims);
            }

            let lbound = SafeArrayGetLBound(psa, 1)?;
            let ubound = SafeArrayGetUBound(psa, 1)?;
            let count = (ubound - lbound + 1) as usize;

            println!(
                "SAFEARRAY info - dims: {}, bounds: {}..{}, count: {}",
                dims, lbound, ubound, count
            );

            if count == 0 {
                return Ok(Vec::new());
            }

            let mut data_ptr: *mut std::ffi::c_void = null_mut();
            SafeArrayAccessData(psa, &mut data_ptr)?;

            if data_ptr.is_null() {
                SafeArrayUnaccessData(psa).ok();
                bail!("Data pointer is null");
            }

            let int_array = std::slice::from_raw_parts(data_ptr as *const i32, count);
            let result = int_array.to_vec();

            SafeArrayUnaccessData(psa)?;

            println!(
                "Extracted {} nodes: {:?}",
                result.len(),
                if result.len() <= 10 {
                    &result[..]
                } else {
                    &result[..10]
                }
            );
            Ok(result)
        }
    }
}
