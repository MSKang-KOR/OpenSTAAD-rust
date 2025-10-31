use anyhow::{Result, anyhow};
use log::{info, warn};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::{
    bindings::{
        Beam, Definitions, LoadItemObj, LoadItemType, Loading, Node, PrimiryLoad, PrimiryLoadType,
        Section, WindDefinition,
    },
    parser::section::*,
};

// STD separater
static REGEX_CONNECT_ST: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s*-\s*$").unwrap());
static REGEX_UPPER_ST: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([A-Z\s]+)$").unwrap());
static REGEX_COMMAND_ST: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(DEFINE|START|PERFORM)\b").unwrap());
static REGEX_END_ST: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(END)\b").unwrap());
static REGEX_JOB_ST: Lazy<Regex> = Lazy::new(|| Regex::new(r"^JOB REF (\S+)").unwrap());
static REGEX_COORD_ST: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
    r"^\s*([-+]?\d+(?:\.\d+)?)\s+([-+]?\d+(?:\.\d+)?)\s+([-+]?\d+(?:\.\d+)?)\s+([-+]?\d+(?:\.\d+)?)\s*$",
).unwrap()
});
static REGEX_MEMBER_ST: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*([-+]?\d+(?:\.\d+)?)\s+([-+]?\d+(?:\.\d+)?)\s+([-+]?\d+(?:\.\d+)?)\s*$")
        .unwrap()
});
static REGEX_GROUP_TYPE_ST: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*(GEOMETRY|JOINT|MEMBER|ELEMENT|SOLID|FLOOR)\s*").unwrap());
static REGEX_PRIMIRY_LOAD_ST: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
    r"^LOAD\s+(?P<key>\S+)(?:\s+(?:LOADTYPE\s+(?P<type>\S+))?(?:\s*TITLE\s+)?(?:(?P<title>.+)))?$",
).unwrap()
});
static REGEX_MATERIAL_ITEM_ST: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(ISOTROPIC|2DORTHOTROPIC)\s+(.+)").unwrap());
static REGEX_CONSTANT_ITEM_ST: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(BETA|MATERIAL)\s+(.+)").unwrap());
static REGEX_WIND_LOAD_ST: Lazy<Regex> = Lazy::new(|| Regex::new(r"TYPE\s+(\d+)\s+(.+)").unwrap());

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

pub fn parsing_std(text: String) -> Result<Value> {
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

    let mut nodes: HashMap<usize, Node> = HashMap::new();
    let mut beams: HashMap<usize, Beam> = HashMap::new();

    let mut group_type: Option<String> = None;
    let mut material_item_type: Option<String> = None;
    let mut material_entries: Vec<(String, String)> = Vec::new();

    let mut specifications: HashMap<usize, Value> = HashMap::new();
    let mut sections: HashMap<usize, Section> = HashMap::new();

    let mut reference_loads: HashMap<usize, PrimiryLoad> = HashMap::new();
    let mut ref_load_id: usize = 0;

    let mut load_case_details: HashMap<usize, PrimiryLoad> = HashMap::new();
    let mut load_case_id: usize = 0;

    let mut load_item_type: Option<String> = None;

    let mut wind_defs: HashMap<usize, WindDefinition> = HashMap::new();
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
                        let id = parts[0].parse::<u32>()?;
                        let x: f64 = parts[1].parse().unwrap_or(0.0);
                        let y: f64 = parts[2].parse().unwrap_or(0.0);
                        let z: f64 = parts[3].parse().unwrap_or(0.0);
                        nodes.insert(id as usize, Node { id, x, y, z });
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
                        let id = parts[0].parse::<u32>()?;
                        let i_key = parts[1].parse::<u32>()?;
                        let j_key = parts[2].parse::<u32>()?;
                        beams.insert(
                            id as usize,
                            Beam {
                                id,
                                i: i_key,
                                j: j_key,
                            },
                        );
                    }
                }
            }
        }

        // Member Release
        if *def_map.get(RELEASE).unwrap_or(&false) {
            let mut parsed = parse_member_release(&line);
            if let Ok(ref mut release) = parsed {
                let id = specifications.keys().len() + 1;
                release.id = id as u32;
                specifications.insert(id, json!(release));
            }
        }

        // Member Truss
        if *def_map.get(TRUSS).unwrap_or(&false) {
            let mut parsed = parse_member_truss(&line);
            if let Ok(ref mut truss) = parsed {
                if !line.contains(TRUSS) {
                    let id = specifications.keys().len() + 1;
                    truss.id = id as u32;
                    specifications.insert(id, json!(truss));
                }
            }
        }

        // Member Property (Section)
        if *def_map.get(SECTION).unwrap_or(&false) || *def_map.get(USERSECTION).unwrap_or(&false) {
            let mut parsed = parse_section(&line);
            if let Ok(ref mut _sec) = parsed {
                let id = sections.keys().len() + 1;
                _sec.id = id as u32;
                sections.insert(id, _sec.clone());
            }
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
                let id_str = key
                    .strip_prefix("R") // "R" 접두사를 제거합니다.
                    .unwrap_or(key); // "R"이 없으면 원래 key를 사용합니다.

                let id = id_str
                    .parse::<usize>()
                    .map_err(|e| anyhow!("id parsing err1 {:#?}: {:#?}", key, e))?;
                let ref_load = PrimiryLoad {
                    id: json!(id),
                    r#type: PrimiryLoadType::from_str(json!(type_)).as_code(),
                    title: json!(title),
                    children: vec![],
                };
                reference_loads.insert(id, ref_load);
                ref_load_id = id;

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
                    // load_item.id = json!(load_item_index);
                    let parent_opt = reference_loads.get_mut(&ref_load_id);
                    if let Some(parent) = parent_opt {
                        let item_id = parent.children.len();
                        load_item.id = json!(item_id);
                        parent.children.push(load_item.clone());
                    };
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
                let id = key
                    .parse::<usize>()
                    .map_err(|e| anyhow!("id parsing err2 {:#?}: {:#?}", key, e))?;
                let load_case = PrimiryLoad {
                    id: json!(id),
                    r#type: PrimiryLoadType::from_str(json!(type_)).as_code(),
                    title: json!(title),
                    children: vec![],
                };
                load_case_details.insert(id, load_case);
                load_case_id = id;

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
                    let parent_opt = load_case_details.get_mut(&load_case_id);
                    if let Some(parent) = parent_opt {
                        let item_id = parent.children.len();
                        load_item.id = json!(item_id);
                        parent.children.push(load_item.clone());
                    };
                };
            };
            // Parse load based on load_item_type
        }
        if *def_map.get(WINDLOAD).unwrap_or(&false) {
            if let Some(_captures) = REGEX_WIND_LOAD_ST.captures(&line) {
                let id = _captures[1].parse::<u64>()?;
                let name = _captures[2].to_string();
                wind_defs.insert(
                    wind_def_index + 1,
                    WindDefinition {
                        id,
                        name,
                        children: vec![],
                    },
                );
                wind_def_index = wind_defs.len();
            } else {
                let mut parsed = parse_wind_load(&line);
                if let Ok(ref mut wind_item) = parsed {
                    let parent_opt = wind_defs.get_mut(&wind_def_index);
                    if let Some(parent) = parent_opt {
                        let item_id = parent.children.len();
                        wind_item.id = json!(item_id);
                        parent.children.push(wind_item.clone());
                    };
                }
            }
        }
    }

    let loadings = Loading {
        definitions: Definitions {
            reference_load: reference_loads,
            wind: wind_defs,
        },
        load_case_details,
        load_envelopes: HashMap::new(),
    };

    Ok(json!({
        "nodes":nodes,
        "beams":beams,
        "loadings":loadings,
        "specifications":specifications,
        "properties": {
            "sections":sections
        }
    }))
}
