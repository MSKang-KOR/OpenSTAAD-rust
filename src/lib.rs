
// // src/lib.rs
// use wasm_bindgen::prelude::*;
// use serde::{Deserialize, Serialize};
// use js_sys::Array;

// // 기존 모듈들을 import
// mod staad;
// use staad::{
//     process::StaadProcess,
//     root::Root,
//     geometry::root::Geometry,
//     geometry::node::Node,
// };

// // JavaScript의 console.log 바인딩
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
    
//     #[wasm_bindgen(js_namespace = console)]
//     fn error(s: &str);
// }

// // 매크로로 console.log 사용을 간편하게
// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

// macro_rules! console_error {
//     ($($t:tt)*) => (error(&format_args!($($t)*).to_string()))
// }

// // JavaScript용 결과 타입
// #[wasm_bindgen]
// #[derive(Serialize, Deserialize)]
// pub struct WasmResult {
//     success: bool,
//     data: String,
//     error: Option<String>,
// }

// #[wasm_bindgen]
// impl WasmResult {
//     #[wasm_bindgen(getter)]
//     pub fn success(&self) -> bool {
//         self.success
//     }
    
//     #[wasm_bindgen(getter)]
//     pub fn data(&self) -> String {
//         self.data.clone()
//     }
    
//     #[wasm_bindgen(getter)]
//     pub fn error(&self) -> Option<String> {
//         self.error.clone()
//     }
// }

// // StaadProcess 래퍼
// #[wasm_bindgen]
// pub struct WasmStaadProcess {
//     inner: StaadProcess,
// }

// #[wasm_bindgen]
// impl WasmStaadProcess {
//     #[wasm_bindgen(constructor)]
//     pub fn new(staad_path: &str) -> WasmStaadProcess {
//         console_log!("Creating new STAAD process with path: {}", staad_path);
//         WasmStaadProcess {
//             inner: StaadProcess::new(staad_path),
//         }
//     }

//     #[wasm_bindgen]
//     pub async fn start(&mut self) -> WasmResult {
//         match self.inner.start().await {
//             Ok(_) => WasmResult {
//                 success: true,
//                 data: "STAAD process started successfully".to_string(),
//                 error: None,
//             },
//             Err(e) => WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some(e.to_string()),
//             },
//         }
//     }

//     #[wasm_bindgen]
//     pub fn get_root(&self) -> Option<WasmRoot> {
//         self.inner.root.as_ref().map(|root| WasmRoot::from_root(root))
//     }

//     #[wasm_bindgen]
//     pub fn get_geometry(&self) -> Option<WasmGeometry> {
//         self.inner.geometry.as_ref().map(|geometry| WasmGeometry::from_geometry(geometry))
//     }
// }

// // Root 래퍼
// #[wasm_bindgen]
// pub struct WasmRoot {
//     // 내부적으로는 참조만 유지하고, 실제 작업시에는 원본 객체 사용
//     has_root: bool,
// }

// #[wasm_bindgen]
// impl WasmRoot {
//     pub fn from_root(_root: &Root) -> Self {
//         WasmRoot { has_root: true }
//     }

//     #[wasm_bindgen]
//     pub async fn analyze_ex(&self, silent: i32, hidden: i32, wait: i32) -> WasmResult {
//         if !self.has_root {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Root object not available".to_string()),
//             };
//         }
        
//         // 실제로는 원본 Root 객체에 접근해야 함
//         // 이 부분은 아키텍처 재설계가 필요함
//         WasmResult {
//             success: true,
//             data: format!("analyze_ex called with params: {}, {}, {}", silent, hidden, wait),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn get_connected_project_info(&self) -> WasmResult {
//         if !self.has_root {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Root object not available".to_string()),
//             };
//         }
        
//         // 실제 구현이 필요함
//         WasmResult {
//             success: true,
//             data: "project_info_placeholder".to_string(),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn get_staad_file(&self, full_path: bool) -> WasmResult {
//         if !self.has_root {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Root object not available".to_string()),
//             };
//         }
        
//         WasmResult {
//             success: true,
//             data: format!("staad_file_path (full_path: {})", full_path),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn get_base_unit(&self) -> WasmResult {
//         if !self.has_root {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Root object not available".to_string()),
//             };
//         }
        
//         WasmResult {
//             success: true,
//             data: "1".to_string(), // 예시 단위
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn set_input_units(&self, l_unit: i32, f_unit: i32) -> WasmResult {
//         if !self.has_root {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Root object not available".to_string()),
//             };
//         }
        
//         WasmResult {
//             success: true,
//             data: format!("Units set: length={}, force={}", l_unit, f_unit),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn update_structure(&self) -> WasmResult {
//         if !self.has_root {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Root object not available".to_string()),
//             };
//         }
        
//         WasmResult {
//             success: true,
//             data: "Structure updated".to_string(),
//             error: None,
//         }
//     }
// }

// // Geometry 래퍼
// #[wasm_bindgen]
// pub struct WasmGeometry {
//     has_geometry: bool,
// }

// #[wasm_bindgen]
// impl WasmGeometry {
//     pub fn from_geometry(_geometry: &Geometry) -> Self {
//         WasmGeometry { has_geometry: true }
//     }

//     #[wasm_bindgen]
//     pub fn get_node(&self) -> Option<WasmNode> {
//         if self.has_geometry {
//             Some(WasmNode::new())
//         } else {
//             None
//         }
//     }
// }

// // Node 래퍼
// #[wasm_bindgen]
// pub struct WasmNode {
//     has_node: bool,
// }

// #[wasm_bindgen]
// impl WasmNode {
//     pub fn new() -> Self {
//         WasmNode { has_node: true }
//     }

//     #[wasm_bindgen]
//     pub fn add_node(&self, coord_x: f64, coord_y: f64, coord_z: f64, base_unit: i32) -> WasmResult {
//         if !self.has_node {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Node object not available".to_string()),
//             };
//         }
        
//         console_log!("Adding node at coordinates: ({}, {}, {})", coord_x, coord_y, coord_z);
        
//         WasmResult {
//             success: true,
//             data: format!("Node added at ({}, {}, {}) with base_unit: {}", coord_x, coord_y, coord_z, base_unit),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn create_node(&self, node_no: i32, coord_x: f64, coord_y: f64, coord_z: f64) -> WasmResult {
//         if !self.has_node {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Node object not available".to_string()),
//             };
//         }
        
//         console_log!("Creating node {} at coordinates: ({}, {}, {})", node_no, coord_x, coord_y, coord_z);
        
//         WasmResult {
//             success: true,
//             data: format!("Node {} created at ({}, {}, {})", node_no, coord_x, coord_y, coord_z),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn delete_node(&self, node_no: i32) -> WasmResult {
//         if !self.has_node {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Node object not available".to_string()),
//             };
//         }
        
//         WasmResult {
//             success: true,
//             data: format!("Node {} deleted", node_no),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn get_node_count(&self) -> WasmResult {
//         if !self.has_node {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Node object not available".to_string()),
//             };
//         }
        
//         WasmResult {
//             success: true,
//             data: "10".to_string(), // 예시 노드 개수
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn get_node_coordinates(&self, node_no: i32, base_unit: i32) -> WasmResult {
//         if !self.has_node {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Node object not available".to_string()),
//             };
//         }
        
//         // 좌표를 JSON 문자열로 반환
//         let coordinates = serde_json::json!({
//             "x": 1.0,
//             "y": 2.0,
//             "z": 3.0,
//             "base_unit": base_unit
//         });
        
//         WasmResult {
//             success: true,
//             data: coordinates.to_string(),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn get_node_distance(&self, node_a: i32, node_b: i32, base_unit: i32) -> WasmResult {
//         if !self.has_node {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Node object not available".to_string()),
//             };
//         }
        
//         WasmResult {
//             success: true,
//             data: format!("Distance between nodes {} and {}: 5.0 (unit: {})", node_a, node_b, base_unit),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn set_node_coordinate(&self, node_no: i32, coord_x: f64, coord_y: f64, coord_z: f64, base_unit: i32) -> WasmResult {
//         if !self.has_node {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Node object not available".to_string()),
//             };
//         }
        
//         WasmResult {
//             success: true,
//             data: format!("Node {} coordinates set to ({}, {}, {}) with unit: {}", node_no, coord_x, coord_y, coord_z, base_unit),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn get_node_unique_id(&self, node_no: i32) -> WasmResult {
//         if !self.has_node {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Node object not available".to_string()),
//             };
//         }
        
//         WasmResult {
//             success: true,
//             data: format!("unique-id-{}", node_no),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn set_node_unique_id(&self, node_no: i32, unique_id: &str) -> WasmResult {
//         if !self.has_node {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Node object not available".to_string()),
//             };
//         }
        
//         WasmResult {
//             success: true,
//             data: format!("Node {} unique ID set to: {}", node_no, unique_id),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn is_orphan_node(&self, node_no: i32) -> WasmResult {
//         if !self.has_node {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Node object not available".to_string()),
//             };
//         }
        
//         WasmResult {
//             success: true,
//             data: format!("Node {} orphan status: false", node_no),
//             error: None,
//         }
//     }

//     // 다중 노드 생성 메서드
//     #[wasm_bindgen]
//     pub fn create_multiple_nodes(&self, node_ids_js: &Array, coordinates_js: &Array) -> WasmResult {
//         if !self.has_node {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Node object not available".to_string()),
//             };
//         }

//         // JavaScript Array를 Rust Vec로 변환
//         let node_ids: Vec<i32> = node_ids_js
//             .iter()
//             .filter_map(|val| val.as_f64().map(|v| v as i32))
//             .collect();

//         let coordinates_count = coordinates_js.length() as usize;
        
//         WasmResult {
//             success: true,
//             data: format!("Created {} nodes with {} coordinates", node_ids.len(), coordinates_count),
//             error: None,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn get_node_list(&self) -> WasmResult {
//         if !self.has_node {
//             return WasmResult {
//                 success: false,
//                 data: String::new(),
//                 error: Some("Node object not available".to_string()),
//             };
//         }
        
//         // 노드 리스트를 JSON 배열로 반환
//         let node_list = serde_json::json!([1, 2, 3, 4, 5]);
        
//         WasmResult {
//             success: true,
//             data: node_list.to_string(),
//             error: None,
//         }
//     }
// }

// // 초기화 함수
// #[wasm_bindgen(start)]
// pub fn main() {
//     console_log!("STAAD WebAssembly module initialized");
// }

// // 전역 유틸리티 함수들
// #[wasm_bindgen]
// pub fn get_version() -> String {
//     "STAAD WASM v1.0.0".to_string()
// }

// #[wasm_bindgen]
// pub fn test_connection() -> WasmResult {
//     WasmResult {
//         success: true,
//         data: "WebAssembly connection test successful".to_string(),
//         error: None,
//     }
// }