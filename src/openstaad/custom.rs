use crate::{
    openstaad::{app::OpenStaad, bindings::*, execute::execute_method},
    tools::{
        SafeArrayP, invoke_method,
        notify::watch_file_background,
        sa_to_vec1d,
        unit::{round_with_factor, unit_factor},
        variant_with_ptr_from,
    },
};
use anyhow::{Context, Result, anyhow, bail};
use chrono::format;
use log::{error, info, warn};
use serde_json::{Value, json};
use std::{collections::HashMap, path::Path, sync::Arc};
use tauri::{AppHandle, Emitter};
use windows::Win32::System::{
    Com::{IDispatch, SAFEARRAY},
    Ole::SafeArrayCreateVector,
    Variant::{
        VARIANT, VT_I4, VariantToDouble, VariantToInt32, VariantToStringAlloc, VariantToUInt32,
    },
};
use windows_core::BSTR;

fn get_units(app: &IDispatch) -> Result<(String, String)> {
    unsafe {
        let lu_ptr = &mut BSTR::default() as *mut BSTR;
        let _ = invoke_method(
            app,
            "GetInputUnitForLength",
            &mut [variant_with_ptr_from::<BSTR>(lu_ptr)],
        )?;
        let lu_bstr = &*lu_ptr;
        let length_unit = lu_bstr.to_string();

        let fu_ptr = &mut BSTR::default() as *mut BSTR;
        let _ = invoke_method(
            app,
            "GetInputUnitForForce",
            &mut [variant_with_ptr_from::<BSTR>(fu_ptr)],
        )?;
        let fu_bstr = &*fu_ptr;
        let force_unit = fu_bstr.to_string();

        Ok((length_unit, force_unit))
    }
}

fn get_unit_factors(app: &IDispatch, lunit: &str, funit: &str) -> Result<(f64, f64)> {
    unsafe {
        let bu_var = invoke_method(app, "GetBaseUnit", &mut [])?;
        let base_unit = VariantToInt32(&bu_var as *const VARIANT)?;
        let lf = unit_factor(base_unit, lunit)?;
        let ff = unit_factor(base_unit, funit)?;
        Ok((lf, ff))
    }
}

pub fn get_node_table(app: &mut Staad) -> Result<Vec<NodeTableRow>> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };
    let geo = openstaad.get_geometry()?;

    let (lunit, funit) = get_units(&openstaad.dispatch)?;
    let (lf, ff) = get_unit_factors(&openstaad.dispatch, lunit.as_str(), funit.as_str())?;

    unsafe {
        let node_count_var = invoke_method(&geo.dispatch, "GetNodeCount", &mut [])?;
        let node_count = VariantToUInt32(&node_count_var)?;

        let mut node_list_psa = SafeArrayCreateVector(VT_I4, 0, node_count);
        let node_list_ppsa = &mut node_list_psa as *mut *mut SAFEARRAY;
        let node_list_var = variant_with_ptr_from::<SafeArrayP<i32>>(node_list_ppsa);
        let _ = invoke_method(&geo.dispatch, "GetNodeList", &mut [node_list_var])?;
        let node_list = sa_to_vec1d::<i32>(node_list_psa)?;

        let mut coords_table: Vec<NodeTableRow> = Vec::new();
        for n in node_list {
            // let coord_values = execute_method(&geometry, "GetNodeCoordinates", &[_n.into()])?;
            let x_ptr: *mut f64 = &mut 0. as *mut f64;
            let y_ptr: *mut f64 = &mut 0. as *mut f64;
            let z_ptr: *mut f64 = &mut 0. as *mut f64;
            let _ = invoke_method(
                &geo.dispatch,
                "GetNodeCoordinates",
                &mut [
                    variant_with_ptr_from::<f64>(z_ptr),
                    variant_with_ptr_from::<f64>(y_ptr),
                    variant_with_ptr_from::<f64>(x_ptr),
                    n.into(),
                ],
            )?;
            let row = NodeTableRow {
                id: n,
                x: *x_ptr * lf,
                y: *y_ptr * lf,
                z: *z_ptr * lf,
            };
            coords_table.push(row);
        }
        Ok(coords_table)
    }
}

pub fn get_beam_table(app: &mut Staad) -> Result<Vec<BeamTableRow>> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    let geo = openstaad.get_geometry()?;
    let prop = openstaad.get_property()?;

    let (lunit, funit) = get_units(&openstaad.dispatch)?;
    let (lf, ff) = get_unit_factors(&openstaad.dispatch, lunit.as_str(), funit.as_str())?;

    unsafe {
        let beam_count_var = invoke_method(&geo.dispatch, "GetMemberCount", &mut [])?;
        let beam_count = VariantToUInt32(&beam_count_var)?;

        let mut beam_list_psa = SafeArrayCreateVector(VT_I4, 0, beam_count);
        let beam_list_ppsa = &mut beam_list_psa as *mut *mut SAFEARRAY;
        let beam_list_var = variant_with_ptr_from::<SafeArrayP<i32>>(beam_list_ppsa);
        let _ = invoke_method(&geo.dispatch, "GetBeamList", &mut [beam_list_var])?;
        let beam_list = sa_to_vec1d::<i32>(beam_list_psa)?;

        let mut beams_table: Vec<BeamTableRow> = Vec::new();
        for n in beam_list {
            let node_a_ptr: *mut i32 = &mut 0 as *mut i32;
            let node_b_ptr: *mut i32 = &mut 0 as *mut i32;
            let _ = invoke_method(
                &geo.dispatch,
                "GetMemberIncidence",
                &mut [
                    variant_with_ptr_from::<i32>(node_a_ptr),
                    variant_with_ptr_from::<i32>(node_b_ptr),
                    n.into(),
                ],
            )?;

            let prop_ref_var = invoke_method(
                &prop.dispatch,
                "GetBeamSectionPropertyRefNo",
                &mut [n.into()],
            )?;
            let prop_ref = VariantToInt32(&prop_ref_var as *const VARIANT)?;

            let mat_var = invoke_method(&prop.dispatch, "GetBeamMaterialName", &mut [n.into()])?;
            let mat_pwstr = VariantToStringAlloc(&mat_var as *const VARIANT)?;
            let mat = mat_pwstr.to_string()?;

            let beta_var = invoke_method(&prop.dispatch, "GetBetaAngle", &mut [n.into()])?;
            let beta = VariantToDouble(&beta_var as *const VARIANT)?;

            let length_var = invoke_method(&geo.dispatch, "GetBeamLength", &mut [n.into()])?;
            let length = VariantToDouble(&length_var as *const VARIANT)?;

            let row = BeamTableRow {
                id: n,
                i: *node_a_ptr,
                j: *node_b_ptr,
                property: prop_ref,
                material: mat,
                beta,
                length: length * lf,
            };
            beams_table.push(row);
        }
        Ok(beams_table)
    }
}

pub fn get_section_list(app: &mut Staad) -> Result<Vec<SectionObj>> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    // let geo = openstaad.get_geometry()?;
    let prop = openstaad.get_property()?;
    let property = Staad::Property(Arc::clone(&prop));

    // let (lf, ff) = get_unit_factors(&openstaad.dispatch)?;

    let section_list_value = execute_method(&property, "GetSectionPropertyList", &[])?;
    let section_list = section_list_value[1]
        .as_array()
        .context("Context err: section_list")?;

    let mut tb: Vec<SectionObj> = Vec::new();
    for ref_val in section_list {
        let id = ref_val.clone();
        let _id = id.as_i64().context("Context err: sec_ref")? as i32;

        let section_type = execute_method(&property, "GetSectionPropertyType", &[_id.into()])?;

        let name_result = execute_method(&property, "GetSectionPropertyName", &[_id.into()])?;
        let name = name_result[1].clone();

        let assigned_result = execute_method(
            &property,
            "GetSectionPropertyAssignedBeamList",
            &[_id.into()],
        )?;
        let assigned = assigned_result[1].clone();

        tb.push(SectionObj {
            id,
            section_type,
            name,
            assigned,
        });
    }
    Ok(tb)
}

pub fn get_section_property_tables(app: &mut Staad) -> Result<Vec<(String, Value)>> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    // let geo = openstaad.get_geometry()?;
    let prop = openstaad.get_property()?;
    let property = Staad::Property(Arc::clone(&prop));

    let (lf, ff) = get_units(&openstaad.dispatch)?;

    let section_list_value = execute_method(&property, "GetSectionPropertyList", &[])?;
    let section_list = section_list_value[1]
        .as_array()
        .context("Context err: section_list")?;

    let mut idx_count_map: HashMap<Value, (usize, usize)> = HashMap::new();
    let mut tables: Vec<(String, Value)> = Vec::new();
    for ref_val in section_list {
        let id = ref_val.clone();
        let _id = id.as_i64().context("Context err: sec_ref")? as i32;

        let prop_values_ex =
            execute_method(&property, "GetSectionPropertyValuesEx", &[_id.into()])?;
        let sec_type_val = prop_values_ex[1].clone();

        let wrapped_idx = idx_count_map.get_mut(&sec_type_val);
        let mut tidx = 0 as usize;
        let mut tcount = 0 as usize;
        match wrapped_idx {
            Some((idx, count)) => {
                tidx = *idx;
                tcount = *count;
            }
            None => {
                let sec_type_num = sec_type_val.as_u64().context("Context err: sec_type")? as u16;
                let sec_type = SectionType::from(sec_type_num).context("Context err: sec_type")?;
                let sec_type_str = sec_type.as_str();
                let mut props = sec_type.properties();
                props.splice(0..0, ["Id".to_string(), "Name".to_string()]);
                tcount = props.len();

                tables.push((sec_type_str.to_string(), json!([props])));

                tidx = (tables.len() - 1) as usize;
                let sec_type_val = sec_type_val.clone();
                idx_count_map.insert(sec_type_val, (tidx, tcount));
            }
        }
        let name_result = execute_method(&property, "GetSectionPropertyName", &[_id.into()])?;
        let name = name_result[1].clone();

        let mut prop_values = prop_values_ex[2].clone();
        let mut_values = prop_values
            .as_array_mut()
            .context("Context err: mut_values")?;
        mut_values.splice(0..0, [id, name]);
        mut_values.truncate(tcount);

        let mut_row = tables[tidx].1.as_array_mut().context("Context err: tb")?;

        mut_row.push(prop_values);
    }
    Ok(tables)
}

pub fn get_beta_list(app: &mut Staad) -> Result<Vec<BetaObj>> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    let geo = openstaad.get_geometry()?;
    let geometry = Staad::Geometry(Arc::clone(&geo));
    let prop = openstaad.get_property()?;
    let property = Staad::Property(Arc::clone(&prop));

    // let (lf, ff) = get_unit_factors(&openstaad.dispatch)?;

    let beam_list_value = execute_method(&geometry, "GetBeamList", &[])?;
    let beam_list = beam_list_value
        .as_array()
        .context("Context err: beam_list")?;

    let mut beta_map: HashMap<Value, Value> = HashMap::new();
    for n in beam_list {
        let id = n.clone();
        let beam_id = id.as_i64().context("Context err: sec_ref")? as i32;

        let angle = execute_method(&property, "GetBetaAngle", &[beam_id.into()])?;
        let searched = beta_map.get_mut(&angle);
        match searched {
            Some(json_arr) => {
                let vec = json_arr
                    .as_array_mut()
                    .context("Context err: beta angle json array")?;
                vec.push(id);
            }
            None => {
                beta_map.insert(angle, json!([id]));
            }
        }
    }
    let mut list: Vec<BetaObj> = Vec::new();
    beta_map.iter().for_each(|(k, v)| {
        list.push(BetaObj {
            angle: k.clone(),
            assigned: v.clone(),
        });
    });
    Ok(list)
}

pub fn get_isotropic_material_list(app: &mut Staad) -> Result<Vec<IsotropicMaterialObj>> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    let prop = openstaad.get_property()?;
    let property = Staad::Property(Arc::clone(&prop));

    // let (lf, ff) = get_unit_factors(&openstaad.dispatch)?;

    let material_count_val = execute_method(&property, "GetIsotropicMaterialCount", &[])?;
    let mat_count = material_count_val
        .as_u64()
        .context("Context err: mat_count")? as usize;

    let mut list: Vec<IsotropicMaterialObj> = Vec::new();
    for n in 1..mat_count + 1 {
        let no = n as i32;
        let name_val = execute_method(&property, "GetIsotropicMaterialProperties", &[no.into()])?;
        let name = name_val[0].clone();
        let name_str = name.as_str().context("Context err: mat_name")?.to_string();

        let type_val =
            execute_method(&property, "GetTypeForIsotropicMaterial", &[name_str.into()])?;
        let type_num = type_val[1].as_u64().context("Context err: mat_type")? as u8;
        let mat_type = IsotropicMaterialType::from(type_num);
        let type_name = mat_type.as_string();

        list.push(IsotropicMaterialObj {
            name,
            r#type: json!(type_name),
        })
    }
    Ok(list)
}

pub fn get_orthotropic2d_material_list(app: &mut Staad) -> Result<Vec<IsotropicMaterialObj>> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    let prop = openstaad.get_property()?;
    let property = Staad::Property(Arc::clone(&prop));

    // let (lf, ff) = get_unit_factors(&openstaad.dispatch)?;

    let material_count_val = execute_method(&property, "GetOrthotropic2DMaterialCount", &[])?;
    let mat_count = material_count_val
        .as_u64()
        .context("Context err: mat_count")? as usize;

    let mut list: Vec<IsotropicMaterialObj> = Vec::new();
    // for n in 0..mat_count + 1 {
    for n in 0..(mat_count + 1) {
        let no = n as i32;
        let name_val = execute_method(
            &property,
            "GetOrthotropic2DMaterialProperties",
            &[no.into()],
        )?;
        let name = name_val[0].clone();
        let name_str = name.as_str().context("Context err: mat_name")?.to_string();

        let type_val =
            execute_method(&property, "GetTypeForIsotropicMaterial", &[name_str.into()])?;
        let type_num = type_val[1].as_u64().context("Context err: mat_type")? as u8;
        let mat_type = IsotropicMaterialType::from(type_num);
        let type_name = mat_type.as_string();

        list.push(IsotropicMaterialObj {
            name,
            r#type: json!(type_name),
        })
    }
    Ok(list)
}

pub fn get_specification_list(app: &mut Staad) -> Result<Vec<SpecificationObj>> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    let geo = openstaad.get_geometry()?;
    let geometry = Staad::Geometry(Arc::clone(&geo));
    let prop = openstaad.get_property()?;
    let property = Staad::Property(Arc::clone(&prop));

    // let (lf, ff) = get_unit_factors(&openstaad.dispatch)?;

    let beam_list_value = execute_method(&geometry, "GetBeamList", &[])?;
    let beam_list = beam_list_value
        .as_array()
        .context("Context err: beam_list")?;

    let to_round_decimal = 4;
    let mut spec_map: HashMap<Value, (Value, Value)> = HashMap::new();
    for n in beam_list {
        let id = n.clone();
        let beam_id = id.as_i64().context("Context err: sec_ref")? as i32;

        let spec_code_result = execute_method(&property, "GetMemberSpecCode", &[beam_id.into()])?;
        let spec_code = spec_code_result[1].clone();
        // Release
        if SpecificationType::Other.as_value() == spec_code {
            for loca in 0..2 {
                let release_result = execute_method(
                    &property,
                    "GetMemberReleaseSpecEx",
                    &[beam_id.into(), loca.into()],
                )?;
                let release_true = json!(1);
                let is_release = release_result[0].clone();
                if is_release == release_true {
                    let release_arr_val = &release_result[1];
                    let spring_arr_val = &release_result[2];
                    let mp_val = &release_result[3];
                    let mp_arr_val = &release_result[4];
                    let loca_str = match loca {
                        0 => "START".to_string(),
                        1 => "END".to_string(),
                        _ => bail!(""),
                    };

                    let mut name_details: Vec<String> = Vec::new();
                    if let Value::Array(arr) = &release_arr_val {
                        if arr.contains(&ReleaseType::MP.as_value()) {
                            let mp_f64 = mp_val.as_f64().context("MP context err")?;
                            let multiplier = 10_f64.powi(to_round_decimal);
                            let rounded = (mp_f64 * multiplier).round() / multiplier;
                            name_details.push(format!("MP {}", rounded))
                        } else if arr.contains(&ReleaseType::MPs.as_value()) {
                            for idx in 3..6 as usize {
                                if ReleaseType::MPs.as_value() == arr[idx].clone() {
                                    let mp_f64 = mp_arr_val[idx - 3]
                                        .as_f64()
                                        .context("MP arr context err")?;
                                    let multiplier = 10_f64.powi(to_round_decimal);
                                    let rounded = (mp_f64 * multiplier).round() / multiplier;
                                    match idx {
                                        3 => name_details.push(format!("MPX {}", rounded)),
                                        4 => name_details.push(format!("MPY {}", rounded)),
                                        5 => name_details.push(format!("MPZ {}", rounded)),
                                        _ => {}
                                    }
                                }
                            }
                        } else {
                            for idx in 0..6 as usize {
                                let release_val = arr[idx].clone();
                                if ReleaseType::Spring.as_value() == release_val {
                                    let k_f64 = spring_arr_val[idx]
                                        .as_f64()
                                        .context("Spring arr context err")?;
                                    let multiplier = 10_f64.powi(to_round_decimal);
                                    let rounded = (k_f64 * multiplier).round() / multiplier;
                                    let spring_str = match idx {
                                        0 => format!("KFX {}", rounded),
                                        1 => format!("KFY {}", rounded),
                                        2 => format!("KFZ {}", rounded),
                                        3 => format!("KMX {}", rounded),
                                        4 => format!("KMY {}", rounded),
                                        5 => format!("KMZ {}", rounded),
                                        _ => bail!("Invalid spring index"),
                                    };
                                    name_details.push(spring_str);
                                }
                                if ReleaseType::Release.as_value() == release_val {
                                    let release_str = match idx {
                                        0 => "FX".to_string(),
                                        1 => "FY".to_string(),
                                        2 => "FZ".to_string(),
                                        3 => "MX".to_string(),
                                        4 => "MY".to_string(),
                                        5 => "MZ".to_string(),
                                        _ => bail!("Invalid release index"),
                                    };
                                    name_details.push(release_str);
                                }
                            }
                        }
                    }
                    name_details.insert(0, loca_str);
                    let name = json!(name_details.join(" "));
                    let spec = spec_map.get_mut(&name);
                    match spec {
                        Some((type_code, json_arr)) => {
                            let vec = json_arr
                                .as_array_mut()
                                .context("Context err: beta angle json array")?;
                            vec.push(id.clone());
                        }
                        None => {
                            spec_map.insert(name, (spec_code.clone(), json!([beam_id])));
                        }
                    }
                }
            }
        }
        // Truss
        if SpecificationType::Truss.as_value() == spec_code {
            let name = json!("MEMBER TRUSS");
            let spec = spec_map.get_mut(&name);
            match spec {
                Some((type_code, json_arr)) => {
                    let vec = json_arr
                        .as_array_mut()
                        .context("Context err: beta angle json array")?;
                    vec.push(id.clone());
                }
                None => {
                    spec_map.insert(name, (spec_code.clone(), json!([beam_id])));
                }
            }
        }
    }

    let mut list: Vec<SpecificationObj> = Vec::new();
    spec_map.iter().for_each(|(k, (type_code, assigned))| {
        list.push(SpecificationObj {
            name: k.clone(),
            r#type: type_code.clone(),
            assigned: assigned.clone(),
        });
    });
    Ok(list)
}

pub fn get_support_list(app: &mut Staad) -> Result<Vec<SupportObj>> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    // let geo = openstaad.get_geometry()?;
    // let geometry = Staad::Geometry(Arc::clone(&geo));
    let spt = openstaad.get_support()?;
    let support = Staad::Support(Arc::clone(&spt));

    // let (lf, ff) = get_unit_factors(&openstaad.dispatch)?;
    // let sf = ff / lf;
    let supported_val = execute_method(&support, "GetSupportNodes", &[])?;
    let node_list = supported_val[1]
        .as_array()
        .context("Context err: beam_list")?;

    let mut sppt_map: HashMap<Value, (Value, Value, Value)> = HashMap::new();
    for n in node_list {
        let node_id_val = n.clone();
        let node_id = node_id_val.as_i64().context("Context err: sec_ref")? as i32;

        let support_info_val =
            execute_method(&support, "GetSupportInformationEx", &[node_id.into()])?;
        let sppt_id_val = support_info_val[1].clone();
        let sppt_type_val = support_info_val[2].clone();
        // let release_arr_val = support_info_val[3].clone();
        // let spring_arr_val = support_info_val[4].clone();

        let sppt_type = SupportType::from(sppt_type_val);
        let sppt = sppt_map.get_mut(&sppt_id_val);
        match sppt {
            Some((name, _type, json_arr)) => {
                let vec = json_arr
                    .as_array_mut()
                    .context("Context err: beta angle json array")?;
                vec.push(node_id_val);
            }
            None => {
                let sppt_name = sppt_type.as_name();
                sppt_map.insert(
                    sppt_id_val,
                    (sppt_name, sppt_type.as_code(), json!([node_id_val])),
                );
            }
        }
    }
    let mut list: Vec<SupportObj> = Vec::new();
    sppt_map.iter().for_each(|(k, (name, code, assigned))| {
        list.push(SupportObj {
            id: k.clone(),
            name: name.clone(),
            r#type: code.clone(),
            assigned: assigned.clone(),
        });
    });
    Ok(list)
}

pub fn get_reference_load_list(app: &mut Staad) -> Result<Vec<PrimiryLoadObj>> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    let ld = openstaad.get_load()?;
    let load = Staad::Load(Arc::clone(&ld));

    let mut list: Vec<PrimiryLoadObj> = Vec::new();
    let rload_val = execute_method(&load, "GetReferenceLoadCaseNumbers", &[])?;
    let rload_ids = rload_val[1].as_array().context("Context err: beam_list")?;
    for n in rload_ids {
        let rload_id_val = n.clone();
        let rload_id = rload_id_val.as_i64().context("Context err: sec_ref")? as i32;

        let title_val = execute_method(&load, "GetReferenceLoadCaseTitle", &[rload_id.into()])?;
        let type_val = execute_method(&load, "GetReferenceLoadType", &[rload_id.into()])?;

        list.push(PrimiryLoadObj {
            id: rload_id_val,
            r#type: PrimiryLoadType::from(type_val).as_name(),
            title: title_val,
        })
    }
    Ok(list)
}

pub fn get_load_case_list(app: &mut Staad) -> Result<Vec<PrimiryLoadObj>> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    let ld = openstaad.get_load()?;
    let load = Staad::Load(Arc::clone(&ld));

    let mut list: Vec<PrimiryLoadObj> = Vec::new();
    let rload_val = execute_method(&load, "GetPrimaryLoadCaseNumbers", &[])?;
    let rload_ids = rload_val[1].as_array().context("Context err: beam_list")?;
    for n in rload_ids {
        let rload_id_val = n.clone();
        let rload_id = rload_id_val.as_i64().context("Context err: sec_ref")? as i32;

        let title_val = execute_method(&load, "GetLoadCaseTitle", &[rload_id.into()])?;
        let type_val = execute_method(&load, "GetLoadType", &[rload_id.into()])?;

        list.push(PrimiryLoadObj {
            id: rload_id_val,
            r#type: PrimiryLoadType::from(type_val).as_name(),
            title: title_val,
        })
    }
    Ok(list)
}

pub fn get_load_item_list(app: &mut Staad, loadcase: Value) -> Result<Vec<LoadItemObj>> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    let ld = openstaad.get_load()?;
    let load = Staad::Load(Arc::clone(&ld));

    let num_decimal = 3;
    let (lunit, funit) = get_units(&openstaad.dispatch)?;
    let (lf, ff) = get_unit_factors(&openstaad.dispatch, lunit.as_str(), funit.as_str())?;

    let lc_id = loadcase.as_u64().context("Context err: lc_id")? as i32;
    let _ = execute_method(&load, "SetLoadActive", &[lc_id.into()]);

    let count_val = execute_method(&load, "GetLoadItemsCount", &[lc_id.into()])?;
    let count = count_val.as_u64().context("Context err: count")? as usize;

    let mut li_idx_map_by_type: HashMap<i32, usize> = HashMap::new();
    let mut list: Vec<LoadItemObj> = Vec::new();
    for idx in 0..count {
        let li_idx = idx as i32;
        let type_code_val =
            execute_method(&load, "GetLoadItemType", &[lc_id.into(), li_idx.into()])?;
        let type_code = type_code_val.as_i64().context("Context err: type_code")? as i32;
        let _type = LoadItemType::from(&type_code_val);

        let li_idx_by_type = li_idx_map_by_type.get_mut(&type_code);
        let mut cur_idx = 0;
        match li_idx_by_type {
            Some(last_idx) => {
                *last_idx += 1;
                cur_idx = *last_idx;
            }
            None => {
                li_idx_map_by_type.insert(type_code, cur_idx);
            }
        }
        let mut tindex = cur_idx.clone();

        let mut assigned: Value = Value::Null;
        if _type.has_assigned() {
            assigned = execute_method(
                &load,
                "GetAssignmentListForLoadType",
                &[type_code.into(), (cur_idx as i32).into()],
            )?;
        }

        let mut is_pass = false;
        let mut name = String::new();
        match _type {
            LoadItemType::NodalLoad => {
                let data = execute_method(&load, "GetNodalLoadInfo", &[(cur_idx as i32).into()])?;
                let loads = data[1].as_array().context("Context err: loads")?;
                let mut details = Vec::new();
                for i in 0..6 as usize {
                    if loads[i] != json!(0.0) {
                        let mut factor = 1.;
                        if i < 3 {
                            factor = ff;
                        } else {
                            factor = ff * lf;
                        }
                        let name = match i {
                            0 => "FX",
                            1 => "FY",
                            2 => "FZ",
                            3 => "MX",
                            4 => "MY",
                            5 => "MZ",
                            _ => bail!("Invalid nodal load index"),
                        };
                        let val = round_with_factor(&loads[i], factor, num_decimal)?;
                        details.push(format!("{} {} {},{}", name, val, funit, lunit));
                    }
                }
                name = details.join(" ");
            }
            LoadItemType::ConcentratedForce => {
                tindex = idx.clone();
                let data = execute_method(&load, "GetMemberLoadInfo", &[(idx as i32).into()])?;
                let direction = &data[1].as_i64().context("Context err: dir")?;
                let forces = data[2].as_array().context("Context err: loads")?;
                let dists = data[3].as_array().context("Context err: dists")?;

                let dir = match direction {
                    1 => "X",
                    2 => "Y",
                    3 => "Z",
                    4 => "GX",
                    5 => "GY",
                    6 => "GZ",
                    _ => "Invalid Direction",
                }
                .to_string();
                let p = round_with_factor(&forces[0], ff, num_decimal)?;
                let d1 = round_with_factor(&dists[0], lf, num_decimal)?;
                let d2 = round_with_factor(&dists[1], lf, num_decimal)?;
                name = format!("CONC {} {} {} {} {},{}", dir, p, d1, d2, funit, lunit);
            }
            LoadItemType::ConcentratedMoment => {
                tindex = idx.clone();
                let data = execute_method(&load, "GetMemberLoadInfo", &[(idx as i32).into()])?;
                let direction = &data[1].as_i64().context("Context err: dir")?;
                let moments = data[2].as_array().context("Context err: loads")?;
                let dists = data[3].as_array().context("Context err: dists")?;

                let dir = match direction {
                    1 => "X",
                    2 => "Y",
                    3 => "Z",
                    4 => "GX",
                    5 => "GY",
                    6 => "GZ",
                    _ => "Invalid Direction",
                }
                .to_string();
                let p = round_with_factor(&moments[0], ff * lf, num_decimal)?;
                let d1 = round_with_factor(&dists[0], lf, num_decimal)?;
                let d2 = round_with_factor(&dists[1], lf, num_decimal)?;
                name = format!("CMOM {} {} {} {} {}/{}", dir, p, d1, d2, funit, lunit);
            }
            LoadItemType::UniformForce => {
                tindex = idx.clone();
                let data = execute_method(&load, "GetMemberLoadInfo", &[(idx as i32).into()])?;
                let direction = &data[1].as_i64().context("Context err: dir")?;
                let forces = data[2].as_array().context("Context err: loads")?;
                let dists = data[3].as_array().context("Context err: dists")?;

                let dir = match direction {
                    1 => "X",
                    2 => "Y",
                    3 => "Z",
                    4 => "GX",
                    5 => "GY",
                    6 => "GZ",
                    7 => "PX",
                    8 => "PY",
                    9 => "PZ",
                    _ => "Invalid Direction",
                }
                .to_string();
                let w = round_with_factor(&forces[0], ff / lf, num_decimal)?;
                let d1 = round_with_factor(&dists[0], lf, num_decimal)?;
                let d2 = round_with_factor(&dists[1], lf, num_decimal)?;
                let d3 = round_with_factor(&dists[2], lf, num_decimal)?;

                name = format!("UNI {} {} {} {} {} {}/{}", dir, w, d1, d2, d3, funit, lunit);
            }
            LoadItemType::UniformMoment => {
                tindex = idx.clone();
                let data = execute_method(&load, "GetElementLoadInfo", &[(idx as i32).into()])?;
                let direction = &data[1].as_i64().context("Context err: dir")?;
                let forces = data[2].as_array().context("Context err: loads")?;
                let dists = data[3].as_array().context("Context err: dists")?;

                let dir = match direction {
                    1 => "X",
                    2 => "Y",
                    3 => "Z",
                    4 => "GX",
                    5 => "GY",
                    6 => "GZ",
                    7 => "PX",
                    8 => "PY",
                    9 => "PZ",
                    _ => "Invalid Direction",
                }
                .to_string();
                let w = round_with_factor(&forces[0], ff, num_decimal)?;
                let d1 = round_with_factor(&dists[0], lf, num_decimal)?;
                let d2 = round_with_factor(&dists[1], lf, num_decimal)?;
                let d3 = round_with_factor(&dists[2], lf, num_decimal)?;

                name = format!(
                    "UMOM {} {} {} {} {} {}-{}/{}",
                    dir, w, d1, d2, d3, funit, lunit, lunit
                );
            }
            LoadItemType::SelfWeight => {
                tindex = idx.clone();
                name = "SELFWEIGHT".to_string()
            }
            LoadItemType::FloorLoadGroup => {
                tindex = idx.clone();
                name = "GROUP FLOAD".to_string()
            }
            LoadItemType::RepeatLoadData => {
                tindex = cur_idx + 1;
                let count_val = execute_method(
                    &load,
                    "GetNoLoadFactorInRepeatLoad",
                    &[(tindex as i32).into()],
                )?;
                let count = count_val.as_u64().context("Context err: count")? as usize;
                let data =
                    execute_method(&load, "GetRepeatLoadByIndex", &[(tindex as i32).into()])?;
                let ids = &data[1];
                let factors = &data[2];
                let mut details = vec!["REPEAT LOAD".to_string()];
                for i in 0..count {
                    details.push(format!(
                        "R{} {}",
                        ids[i],
                        round_with_factor(&factors[i], 1., 2)?
                    ));
                }
                name = details.join(" ");
            }
            LoadItemType::ReferenceLoadData => {
                let count_val = execute_method(
                    &load,
                    "GetNoOfSetsInReferenceLoad",
                    &[(tindex as i32).into()],
                )?;
                let count = count_val.as_u64().context("Context err: count")? as usize;
                let data =
                    execute_method(&load, "GetReferenceLoadByIndex", &[(cur_idx as i32).into()])?;
                let ids = &data[1];
                let factors = &data[2];
                let mut details = vec!["REFERENCE LOAD".to_string()];
                for i in 0..count {
                    details.push(format!(
                        "R{} {}",
                        ids[i],
                        round_with_factor(&factors[i], 1., 2)?
                    ));
                }
                name = details.join(" ");
            }
            LoadItemType::NotionalLoadData => {
                tindex = cur_idx + 1;
                let count_val = execute_method(
                    &load,
                    "GetNoLoadFactorDirectionInNotionalLoad",
                    &[(tindex as i32).into()],
                )?;
                let count = count_val.as_u64().context("Context err: count")? as usize;
                let data =
                    execute_method(&load, "GetNotionalLoadByIndex", &[(tindex as i32).into()])?;
                let ids = &data[1];
                let factors = &data[2];
                let dirs = &data[3];
                let mut details = vec!["NOTIONAL LOAD".to_string()];
                for i in 0..count {
                    let ref_id = ids[i].as_i64().context("Context err: ids")?;
                    let ref_name = match ref_id.signum() {
                        1 => format!("{}", ref_id),
                        -1 => format!("R{}", ref_id.abs()),
                        _ => "id err".to_string(),
                    };
                    let dir = Axis::from(dirs[i].clone());
                    let fac = round_with_factor(&factors[i], 1., 3)?;
                    details.push(format!("{} {} {}", ref_name, dir.as_str(), fac));
                }
                name = details.join(" ");
            }

            _ => {
                is_pass = true;
                warn!(
                    "'{}': Invalid load item type: {}:{:#?}",
                    idx, type_code_val, _type
                );
            }
        }
        if !is_pass {
            list.push(LoadItemObj {
                id: json!(idx),
                index: json!(tindex),
                r#type: type_code_val.clone(),
                name: json!(name),
                assigned: assigned.clone(),
            })
        }
    }
    Ok(list)
}

pub fn analyze(app: &mut Staad, handle: AppHandle) -> Result<Value> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    handle
        .emit("staad_analysis_start", "Start analysis")
        .map_err(|e| anyhow!(e))?;

    let std_file_path_val = execute_method(app, "GetSTAADFile", &[true.into()])?;
    let std_file_path = std_file_path_val
        .as_str()
        .context("Context err: std_file_path")?;
    let std_path = Path::new(std_file_path);
    let log_path = std_path.with_extension("log");
    let _watcher_handle = watch_file_background(&log_path, handle.clone());

    let code_result = execute_method(app, "AnalyzeEx", &[1.into(), 0.into(), 1.into()]);
    match code_result {
        Ok(code) => {
            let _ = handle
                .emit("staad_analysis_complete", &code)
                .map_err(|e| anyhow!(e));
            Ok(code)
        }
        Err(err) => {
            let _ = handle
                .emit("staad_analysis_error", err.to_string())
                .map_err(|e| anyhow!(e));
            bail!(err)
        }
    }
}
