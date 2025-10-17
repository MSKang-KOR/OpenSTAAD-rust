use anyhow::{Result, anyhow};
use log::{info, warn};
use regex::Regex;
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::{
    bindings::{
        Definitions, LoadItemObj, LoadItemType, Loading, PrimiryLoad, PrimiryLoadType,
        WindDefinition,
    },
    parser::{regex::*, section::*},
};

const JOB: &str = "START JOB INFORMATION";
const JOINT: &str = "JOINT COORDINATES";
const MEMBER: &str = "MEMBER INCIDENCES";
const GROUP: &str = "START GROUP DEFINITION";
const RELEASE: &str = "MEMBER RELEASE";
const TRUSS: &str = "MEMBER TRUSS";
const USERSECTION: &str = "MEMBER PROPERTY";
const SECTION: &str = "MEMBER PROPERTY EUROPEAN";
const CONSTANTS: &str = "CONSTANTS";
const MATERIAL: &str = "DEFINE MATERIAL START";
const SUPPORT: &str = "SUPPORTS";
const REFERENCELOAD: &str = "DEFINE REFERENCE LOADS";
const WINDLOAD: &str = "DEFINE WIND LOAD";

pub fn parsing_loadings(text: String) -> Result<Loading> {
    // Parsing spec

    let mut lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();

    let mut def_map: HashMap<&str, bool> = vec![
        JOB,
        JOINT,
        MEMBER,
        GROUP,
        RELEASE,
        TRUSS,
        USERSECTION,
        SECTION,
        CONSTANTS,
        MATERIAL,
        SUPPORT,
        REFERENCELOAD,
        WINDLOAD,
    ]
    .into_iter()
    .map(|e| (e, false))
    .collect();

    let mut group_type: Option<String> = None;
    let mut material_item_type: Option<String> = None;
    let mut load_item_type: Option<String> = None;
    let mut material_entries: Vec<(String, String)> = Vec::new();

    let mut reference_loads: Vec<PrimiryLoad> = vec![];
    let mut ref_load_index: usize = 0;

    let mut load_case_details: Vec<PrimiryLoad> = vec![];
    let mut load_case_index: usize = 0;

    let mut load_item_index: usize = 0;

    let mut wind_defs: Vec<WindDefinition> = vec![];
    let mut wind_def_index: usize = 0;

    fn load_item_parser(load_item_type: &str, line: &str) -> Result<LoadItemObj> {
        if !load_item_type.to_string().contains(" LOAD") {
            return Err(anyhow!("Not load item"));
        }
        match load_item_type {
            "JOINT LOAD" => parse_nodal_load(&line),
            "MEMBER LOAD" => parse_member_load(&line),
            "FLOOR LOAD" => parse_floor_load(&line),
            "TEMPERATURE LOAD" => parse_temp_load(&line),
            "REPEAT LOAD" => parse_repeat_load(&line),
            "REFERENCE LOAD" => parse_reference_load(&line),
            "NOTIONAL LOAD" => parse_notional_load(&line),
            _ => {
                warn!("Invalid load item type: {}", load_item_type);
                return Err(anyhow!("Invalid load item type: {}", &line));
            }
        }
    }

    for (i, _) in text.lines().enumerate() {
        let line = &lines[i];
        if line.starts_with('*') {
            continue;
        }
        // Handle line continuation
        if REGEX_CONNECT_ST.is_match(&line) {
            if i + 1 < lines.len() {
                let continued =
                    REGEX_CONNECT_ST.replace(&lines[i], " ").to_string() + &lines[i + 1];
                lines[i + 1] = continued;
                continue;
            }
        }

        let is_upper = REGEX_UPPER_ST.is_match(&line);
        let is_command = REGEX_COMMAND_ST.is_match(&line);
        let is_end = REGEX_END_ST.is_match(&line);
        let is_next = is_upper || is_command || is_end;

        // Current work
        if is_next {
            let keys_to_update: Vec<&str> = def_map
                .iter()
                .filter(|(_, v)| **v)
                .map(|(&k, _)| k)
                .collect();

            for k in keys_to_update {
                if [REFERENCELOAD, WINDLOAD, GROUP, MATERIAL].contains(&k) {
                    if REGEX_END_ST.is_match(&line) {
                        if k == MATERIAL {
                            let _mat: HashMap<String, String> =
                                material_entries.iter().cloned().collect();
                        }
                        def_map.insert(k, false);
                    }
                } else {
                    def_map.insert(k, false);
                }
            }

            if let Some(&key) = def_map.keys().find(|&&k| k == line) {
                def_map.insert(key, true);
            }
        }

        // Job
        if *def_map.get(JOB).unwrap_or(&false) {
            if let Some(captures) = REGEX_JOB_ST.captures(&line) {
                let _job_ref = &captures[1];
            }
        }

        // Joints
        if *def_map.get(JOINT).unwrap_or(&false) {
            let line_split: Vec<&str> = line.trim_end_matches(';').split(';').collect();
            for e in line_split {
                if REGEX_COORD_ST.is_match(e) {
                    let parts: Vec<&str> = e.trim().split_whitespace().collect();
                    if parts.len() >= 4 {
                        let key = parts[0];
                        let x: f64 = parts[1].parse().unwrap_or(0.0);
                        let y: f64 = parts[2].parse().unwrap_or(0.0);
                        let z: f64 = parts[3].parse().unwrap_or(0.0);
                    }
                }
            }
        }

        // Members
        if *def_map.get(MEMBER).unwrap_or(&false) {
            let line_split: Vec<&str> = line.trim_end_matches(';').split(';').collect();
            for e in line_split {
                if REGEX_MEMBER_ST.is_match(e) {
                    let parts: Vec<&str> = e.trim().split_whitespace().collect();
                    if parts.len() >= 3 {
                        let key = parts[0];
                        let i_key: i32 = parts[1].parse().unwrap_or(0);
                        let j_key: i32 = parts[2].parse().unwrap_or(0);
                    }
                }
            }
        }

        // Member Release
        if *def_map.get(RELEASE).unwrap_or(&false) {
            // let parsed = parse_member_release(&line);
        }

        // Member Truss
        if *def_map.get(TRUSS).unwrap_or(&false) {
            // let parsed = parse_keys(&line);
        }

        // Member Property (Section)
        if *def_map.get(SECTION).unwrap_or(&false) || *def_map.get(USERSECTION).unwrap_or(&false) {
            // let parsed = parse_section(&line);
        }

        // Material
        if *def_map.get(MATERIAL).unwrap_or(&false) {
            if let Some(captures) = REGEX_MATERIAL_ITEM_ST.captures(&line) {
                if !material_entries.is_empty() {
                    let _mat: HashMap<String, String> = material_entries.iter().cloned().collect();
                }
                let mat_type = captures[1].to_string();
                let name = captures[2].to_string();
                material_item_type = Some(mat_type);
                material_entries = vec![("name".to_string(), name)];
            } else if !material_entries.is_empty() {
                // let parsed = parse_material(&line);
            }
        }

        // Constants
        if *def_map.get(CONSTANTS).unwrap_or(&false) {
            if let Some(captures) = REGEX_CONSTANT_ITEM_ST.captures(&line) {
                // let parsed = parse_constants(&line);
            }
        }

        // Support
        if *def_map.get(SUPPORT).unwrap_or(&false) {
            // let parsed = parse_support(&line);
        }

        // Load
        if *def_map.get(REFERENCELOAD).unwrap_or(&false) {
            if let Some(_captures) = REGEX_PRIMIRY_LOAD_ST.captures(&line) {
                let key = _captures.name("key").map(|m| m.as_str()).unwrap_or("");
                if key == "LIST" {
                    continue;
                }
                let type_ = _captures.name("type").map(|m| m.as_str()).unwrap_or("");
                let title = _captures.name("title").map(|m| m.as_str()).unwrap_or("");
                let ref_load = PrimiryLoad {
                    id: json!(key.chars().skip(1).collect::<String>()),
                    r#type: PrimiryLoadType::from_str(json!(type_)).as_code(),
                    title: json!(title),
                    children: vec![],
                };
                reference_loads.push(ref_load);
                ref_load_index = reference_loads.len() - 1;

                load_item_index = 0;
                load_item_type = None;
            }
            if is_upper {
                if let Some(captures) = REGEX_UPPER_ST.captures(&line) {
                    load_item_type = Some(captures[1].to_string());
                }
            }
            if let Some(ref li_type) = load_item_type {
                let mut parsed = load_item_parser(&li_type, &line);
                if let Ok(ref mut load_item) = parsed {
                    load_item.id = json!(load_item_index);
                    reference_loads[ref_load_index]
                        .children
                        .push(load_item.clone());
                    load_item_index += 1;
                };
            };
            // Parse load based on load_item_type
        } else {
            if let Some(_captures) = REGEX_PRIMIRY_LOAD_ST.captures(&line) {
                let key = _captures.name("key").map(|m| m.as_str()).unwrap_or("");
                if key == "LIST" {
                    continue;
                }
                let type_ = _captures.name("type").map(|m| m.as_str()).unwrap_or("");
                let title = _captures.name("title").map(|m| m.as_str()).unwrap_or("");
                let load_case = PrimiryLoad {
                    id: json!(key),
                    r#type: PrimiryLoadType::from_str(json!(type_)).as_code(),
                    title: json!(title),
                    children: vec![],
                };
                load_case_details.push(load_case);
                load_case_index = load_case_details.len() - 1;

                load_item_index = 0;
                load_item_type = None;
            }
            if is_upper {
                if let Some(captures) = REGEX_UPPER_ST.captures(&line) {
                    load_item_type = Some(captures[1].to_string());
                }
            }
            if let Some(ref li_type) = load_item_type {
                let mut parsed = load_item_parser(&li_type, &line);
                if let Ok(ref mut load_item) = parsed {
                    load_item.id = json!(load_item_index);
                    load_case_details[load_case_index]
                        .children
                        .push(load_item.clone());
                    load_item_index += 1;
                };
            };
            // Parse load based on load_item_type
        }
        if *def_map.get(WINDLOAD).unwrap_or(&false) {
            if let Some(_captures) = REGEX_WIND_LOAD_ST.captures(&line) {
                let id = _captures[1].parse::<u64>()?;
                let name = _captures[2].to_string();
                wind_defs.push(WindDefinition {
                    id,
                    name,
                    children: vec![],
                });
                wind_def_index = wind_defs.len() - 1;
            } else {
                // if wind_def_index >= wind_defs.len() {
                //     continue;
                // }
                let mut parsed = parse_wind_load(&line);
                if let Ok(ref mut wind_item) = parsed {
                    let item_id = wind_defs[wind_def_index].children.len();
                    wind_item.id = json!(item_id);
                    wind_defs[wind_def_index].children.push(wind_item.clone());
                }
            }
        }
    }

    Ok(Loading {
        definitions: Definitions {
            reference_load: reference_loads,
            wind: wind_defs,
        },
        load_case_details,
        load_envelopes: vec![],
    })
}

pub fn parsing_specifications(text: String) -> Result<Vec<Value>> {
    let mut lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();

    let mut def_map: HashMap<&str, bool> = vec![
        JOB,
        JOINT,
        MEMBER,
        GROUP,
        RELEASE,
        TRUSS,
        USERSECTION,
        SECTION,
        CONSTANTS,
        MATERIAL,
        SUPPORT,
        REFERENCELOAD,
        WINDLOAD,
    ]
    .into_iter()
    .map(|e| (e, false))
    .collect();

    let mut material_entries: Vec<(String, String)> = Vec::new();

    let mut spec_index: usize = 0;
    let mut specifications: Vec<Value> = vec![];
    for (i, _) in text.lines().enumerate() {
        let line = &lines[i];
        if line.starts_with('*') {
            continue;
        }
        // Handle line continuation
        if REGEX_CONNECT_ST.is_match(&line) {
            if i + 1 < lines.len() {
                let continued =
                    REGEX_CONNECT_ST.replace(&lines[i], " ").to_string() + &lines[i + 1];
                lines[i + 1] = continued;
                continue;
            }
        }

        let is_upper = REGEX_UPPER_ST.is_match(&line);
        let is_command = REGEX_COMMAND_ST.is_match(&line);
        let is_end = REGEX_END_ST.is_match(&line);
        let is_next = is_upper || is_command || is_end;

        // Current work
        if is_next {
            let keys_to_update: Vec<&str> = def_map
                .iter()
                .filter(|(_, v)| **v)
                .map(|(&k, _)| k)
                .collect();

            for k in keys_to_update {
                if [REFERENCELOAD, WINDLOAD, GROUP, MATERIAL].contains(&k) {
                    if REGEX_END_ST.is_match(&line) {
                        if k == MATERIAL {
                            let _mat: HashMap<String, String> =
                                material_entries.iter().cloned().collect();
                        }
                        def_map.insert(k, false);
                    }
                } else {
                    def_map.insert(k, false);
                }
            }
            if let Some(&key) = def_map.keys().find(|&&k| line.contains(k)) {
                def_map.insert(key, true);
            }
        }

        // Member Release
        if *def_map.get(RELEASE).unwrap_or(&false) {
            let mut parsed = parse_member_release(&line);
            if let Ok(ref mut release) = parsed {
                specifications.push(json!(release));
                spec_index = specifications.len()
            }
        }

        // Member Truss
        if *def_map.get(TRUSS).unwrap_or(&false) {
            let mut parsed = parse_member_truss(&line);
            if let Ok(ref mut truss) = parsed {
                if !line.contains(TRUSS) {
                    truss.id = (spec_index as u32) + 1;
                    specifications.push(json!(truss));
                    spec_index = specifications.len()
                }
            }
        }
    }

    Ok(specifications)
}
