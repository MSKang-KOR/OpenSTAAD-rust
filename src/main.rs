mod staad;
use crate::staad::geometry::Geometry;
use staad::process::StaadProcess;

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

                if let Some(geo_dispatch) = _staad.geometry.as_ref() {
                    let geometry = Geometry::new(geo_dispatch);
                    println!("geometry: {:?}", geometry);

                    let node_no = geometry.get_last_node_no();
                    println!("{:#?}", node_no);

                    // let _ = geometry.get_node_list();
                    println!("\n=== Testing different approaches to get node list ===");

                    // 2. Property 방식 시도
                    println!("\n2. Trying property approach:");
                    match geometry.get_node_list_as_property() {
                        Ok(nodes) => {
                            if !nodes.is_empty() {
                                println!(
                                    "✓ Property approach succeeded with {} nodes",
                                    nodes.len()
                                );
                                return; // 성공하면 여기서 종료
                            } else {
                                println!("△ Property approach returned empty list");
                            }
                        }
                        Err(e) => {
                            println!("✗ Property approach failed: {}", e);
                        }
                    }

                    // 3. 대안적 VARIANT 구성 방식
                    println!("\n3. Trying alternative VARIANT construction:");
                    match geometry.get_node_list_alternative() {
                        Ok(nodes) => {
                            if !nodes.is_empty() {
                                println!(
                                    "✓ Alternative approach succeeded with {} nodes",
                                    nodes.len()
                                );
                                return;
                            } else {
                                println!("△ Alternative approach returned empty list");
                            }
                        }
                        Err(e) => {
                            println!("✗ Alternative approach failed: {}", e);
                        }
                    }

                    // 4. Raw dispatch 방식
                    println!("\n4. Trying raw dispatch approach:");
                    match geometry.get_node_list_raw_dispatch() {
                        Ok(nodes) => {
                            if !nodes.is_empty() {
                                println!("✓ Raw dispatch succeeded with {} nodes", nodes.len());
                                return;
                            } else {
                                println!("△ Raw dispatch returned empty list");
                            }
                        }
                        Err(e) => {
                            println!("✗ Raw dispatch failed: {}", e);
                        }
                    }

                    // 5. 원래 방식도 다시 시도
                    println!("\n5. Trying original approach:");
                    match geometry.get_node_list() {
                        Ok(nodes) => {
                            if !nodes.is_empty() {
                                println!(
                                    "✓ Original approach succeeded with {} nodes",
                                    nodes.len()
                                );
                            } else {
                                println!("△ Original approach returned empty list");
                            }
                        }
                        Err(e) => {
                            println!("✗ Original approach failed: {}", e);
                        }
                    }

                    println!("\n=== All approaches completed ===");
                }
            }
        }
    };
}
