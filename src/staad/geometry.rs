use anyhow::{Context, Error, Ok as anyOk, Result, bail};
use windows::Win32::System::{
    Com::IDispatch,
    Variant::{VARIANT, VariantToInt16ArrayAlloc, VariantToStringAlloc},
};
use windows_core::PWSTR;

use crate::staad::process::{get_dispatch, invoke_method_on_object};

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

    pub fn get_node_list(&self, array_variant: VARIANT) -> () {
        // let params = [variant.clone()];
        let params = [array_variant];
        // let _variant = unsafe {
        //     invoke_method_on_object(&self.dispatch.as_ref().unwrap(), "GetNodeList", &params)
        // }?;
        let _variant = unsafe {
            invoke_method_on_object(&self.dispatch.as_ref().unwrap(), "GetLastNodeNo", &params)
        };
        // let result_array =
        //     unsafe { VariantToInt16ArrayAlloc(&_variant as *const VARIANT, pprgn, pcelem) };
    }

    pub fn get_last_node_no(&self) -> Result<()> {
        let params = [];
        let result_variant = unsafe {
            invoke_method_on_object(self.dispatch.as_ref().unwrap(), "GetLastNodeNo", &params)
        };
        let check_dipatch =
            unsafe { get_dispatch(self.dispatch.as_ref().unwrap(), "GetLastNodeNo", &params) };
        println!("{:#?}", check_dipatch);
        
        if let Ok(var) = result_variant {
            let node_no = unsafe {
                VariantToStringAlloc(&var as *const VARIANT)
                    .unwrap()
                    .to_string()
            };
            println!("{:#?}", node_no);
        }
        Ok(())
    }
}

// // 노드 좌표 가져오기
// pub fn get_node_coordinates(&self, node_no: i32) -> Result<(f64, f64, f64)> {
//     // 매개변수 설정 (노드 번호)
//     let mut node_variant = VARIANT::default();
//     // node_variant에 node_no 값 설정하는 코드 필요
//     let params = [node_variant];

//     let variant =
//         unsafe { invoke_method_on_object(&self.dispatch, "GetNodeCoordinates", &params) }?;
//     // 결과 파싱 로직 (x, y, z 좌표)
//     // 실제 구현에서는 반환되는 VARIANT 배열을 파싱해야 함

//     Ok((0.0, 0.0, 0.0)) // 임시 반환값
// }

// // 빔 연결 정보 가져오기
// pub fn get_beam_incidences(&self, beam_no: i32) -> Result<(i32, i32)> {
//     let mut beam_variant = VARIANT::default();
//     // beam_variant에 beam_no 값 설정하는 코드 필요
//     let params = [beam_variant];

//     let variant =
//         unsafe { invoke_method_on_object(&self.dispatch, "GetBeamIncidences", &params) }?;
//     // 결과 파싱 로직 (시작노드, 끝노드)

//     Ok((0, 0)) // 임시 반환값
// }
// }
