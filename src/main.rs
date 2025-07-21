mod staad;
use std::mem::ManuallyDrop;

use staad::process::StaadProcess;
use windows::Win32::System::{
    Ole::{SafeArrayCreate, SafeArrayCreateVector},
    Variant::{
        VARIANT, VARIANT_0, VARIANT_0_0, VARIANT_0_0_0, VT_ARRAY, VT_EMPTY, VT_I4, VariantInit,
    },
};

use crate::staad::geometry::{self, Geometry};

#[tokio::main]
async fn main() {
    // StaadBackgroundAnalyzer
    println!("Hello, world!");
    let mut _staad = StaadProcess::new("");
    match _staad.start().await {
        Err(e) => {
            println!("{:#?}", e)
        }
        _ => {
            // let _ = _staad.test_code();
            let root = _staad.root.as_ref();
            if let Some(root_dispatch) = root {
                println!("root id: {:?}", root_dispatch);

                let geometry = Geometry::new(&root_dispatch);
                println!("geometry: {:?}", geometry);

                let __ = geometry.get_last_node_no();

                // let vars = VARIANT::default();
                // // vars.Anonymous.Anonymous.vt = VT_EMPTY;

                // unsafe {
                //     let psa = SafeArrayCreateVector(VT_I4, 0, 0); // low bound = 0, length = 0
                //     if psa.is_null() {
                //         panic!("Failed to create SAFEARRAY");
                //     }

                //     // VARIANT 구조체를 생성하여 SAFEARRAY를 포함시킵니다.
                //     let variant = VARIANT {
                //         Anonymous: VARIANT_0 {
                //             Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                //                 vt: (VT_ARRAY | VT_I4),
                //                 wReserved1: 0,
                //                 wReserved2: 0,
                //                 wReserved3: 0,
                //                 Anonymous: VARIANT_0_0_0 { parray: psa },
                //             }),
                //         },
                //     };
                //     let _ = geometry.get_node_list(variant);
                // }
            }
        }
    };
}
