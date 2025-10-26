use anyhow::{Context, Result, anyhow, bail};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{Value, json};
use std::collections::HashSet;

use crate::bindings::{
    Axis, ConcentratedForce, FloorLoadGroup, LoadItemAttribute, LoadItemObj, LoadItemType,
    NodalLoad, NotionalLoadData, ReferenceLoadData, Release, RepeatLoadData, Section, Temperature,
    Truss, UniformForce,
};

// IDs
static REGEX_RANGE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s*(\d+)\s+TO\s+(\d+)\s*").unwrap());

// Member Release
static REGEX_RELEASE_LOCATION: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(START|END|BOTH)\s*").unwrap());
static REGEX_RELEASE_RESTRAINT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(FX|FY|FZ|MX|MY|MZ)\s*").unwrap());
static REGEX_RELEASE_SPRING: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(KFX|KFY|KFZ|KMX|KMY|KMZ)\s+([-+]?\d*\.?\d+)\s*").unwrap());
static REGEX_RELEASE_PARTIAL_MOMENT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(MP|MPX|MPY|MPZ)\s+([-+]?\d*\.?\d+)\s*").unwrap());

// Propert::Section
static REGEX_SECTION: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\s+(?<source>TABLE|UPTABLE)\s+(?<type>[^\s]+)\s+(?<name>[^\s]+)")
        .expect("Invalid regex")
});

// LoadItem::Nodal Load
static REGEX_INDIVIDUAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b(\d+)\b").unwrap());
static REGEX_NODAL_LOAD_VALIDATION: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s+(INCLINED|FX|FY|FZ|MX|MY|MZ)\s+").unwrap());
static REGEX_NODAL_LOAD_INCLINED: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"INCLINED\s+(?:(?:(?<x>[-\d.]+)\s+(?<y>[-\d.]+)\s+(?<z>[-\d.]+))|(REF)\s+(?<xref>[-\d.]+)\s+(?<yref>[-\d.]+)\s+(?<zref>[-\d.]+)|(REFJT)\s+(?<joint>[-\d.]+))"
    ).unwrap()
});
static REGEX_NODAL_LOAD_PAIR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(FX|FY|FZ|MX|MY|MZ)\s+([-\d.]+)").unwrap());
// Member Load
static REGEX_MEMBER_LOAD_TYPE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(CON|UNI)\s+").unwrap());
static REGEX_MEMBER_LOAD_VALUE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"([A-Z]+)\s+([-+]?\d*\.?\d+)(?:\s+([-+]?\d*\.?\d+)(?:\s+([-+]?\d*\.?\d+)(?:\s+([-+]?\d*\.?\d+))?)?)?"
    ).unwrap()
});
// Floor Load
static REGEX_FLOAD_VALIDATION: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+(FLOAD)\s+").unwrap());
static REGEX_FLOAD_GROUP_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"(_[^\s]+)\s+").unwrap());
static REGEX_FLOAD_PRESSURE: Lazy<Regex> = Lazy::new(|| Regex::new(r"FLOAD\s+([-\d.]+)").unwrap());
static REGEX_FLOAD_DIRECTION: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b(GX|GY|GZ)").unwrap());
// Temperature LOAD
static REGEX_TEMP_VARS: Lazy<Regex> = Lazy::new(|| {
    {
    Regex::new(
        r"TEMP\s+([-+]?\d+(?:\.\d*)?)\s*(?:([-+]?\d+(?:\.\d*)?)\s*)?(?:([-+]?\d+(?:\.\d*)?)\s*)?$",
    )
}
.unwrap()
});
// Repeat Load & Reference Load
static REGEX_REF_LOAD_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\S+)\s+([-+]?\d+(?:\.\d+)?)").unwrap());
// Notional Load
static REGEX_NOTIONAL_LOAD_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\S+)\s+([XYZ])\s+([-+]?\d+(?:\.\d+)?)").unwrap());
// Wind Load
static REGEX_WIND_LOAD_ITEM_TYPE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b(INT)\s+").unwrap());
static REGEX_WIND_LOAD_INTENSITY_TABLE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s+([-+]?\d+(?:\.\d*)?)").unwrap());

pub fn parse_keys(s: &str) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut keys: Vec<i32> = Vec::new();

    // TO 범위 처리
    for cap in REGEX_RANGE.captures_iter(s) {
        let start: i32 = cap[1].parse().unwrap();
        let end: i32 = cap[2].parse().unwrap();
        for i in start..=end {
            if seen.insert(i) {
                keys.push(i);
            }
        }
    }

    // TO 범위 제거 후 개별 숫자 처리
    let without_ranges = REGEX_RANGE.replace_all(s, " ");

    for cap in REGEX_INDIVIDUAL.captures_iter(&without_ranges) {
        let key: i32 = cap[1].parse().unwrap();
        if seen.insert(key) {
            keys.push(key);
        }
    }

    // 정렬
    keys.sort_unstable();

    keys
}

pub fn parse_member_release(s: &str) -> Result<Release> {
    // Location 매칭
    let location_match = REGEX_RELEASE_LOCATION.find(s);
    if location_match.is_none() {
        bail!("Not member release: {}", s);
    }

    fn release_index_and_type(force: &str) -> Result<(usize, i64)> {
        let r: (usize, i64) = match force {
            "FX" => (0, 1),
            "FY" => (1, 1),
            "FZ" => (2, 1),
            "MX" => (3, 1),
            "MY" => (4, 1),
            "MZ" => (5, 1),
            "KFX" => (0, -1),
            "KFY" => (1, -1),
            "KFZ" => (2, -1),
            "KMX" => (3, -1),
            "KMY" => (4, -1),
            "KMZ" => (5, -1),
            "MPX" => (3, -2),
            "MPY" => (4, -2),
            "MPZ" => (5, -2),
            "MP" => (6, -3),
            _ => bail!("Invalid releas force type: {}", force),
        };
        Ok(r)
    }

    let location_cap = location_match.unwrap();
    let split_index = location_cap.start();
    let key_part = &s[..split_index];
    let assigned = json!(parse_keys(key_part));
    let location = location_match.unwrap().as_str().trim().to_string();
    let location_code = match location.as_str() {
        "START" => 0,
        "END" => 1,
        _ => bail!("Invalid location type: {}", location),
    };
    let mut name = format!("{}", location);
    let mut release_array: [i64; 6] = [0, 0, 0, 0, 0, 0];
    let mut spring_array: [f64; 6] = [0., 0., 0., 0., 0., 0.];
    let mut mp_array: [f64; 3] = [0., 0., 0.];
    let mut mp: f64 = 0.;
    let mut is_partial_moment = false;

    for cap in REGEX_RELEASE_RESTRAINT.captures_iter(s) {
        let (i, type_code) = release_index_and_type(&cap[1])?;
        release_array[i] = type_code;

        name += format!(" {}", &cap[1]).as_str();
    }
    for cap in REGEX_RELEASE_SPRING.captures_iter(s) {
        let (i, type_code) = release_index_and_type(&cap[1])?;
        let val = cap[2].parse::<f64>()?;
        release_array[i] = type_code;
        spring_array[i] = val;

        name += format!(" {} {}", &cap[1], &val).as_str();
    }
    for cap in REGEX_RELEASE_PARTIAL_MOMENT.captures_iter(s) {
        let (i, type_code) = release_index_and_type(&cap[1])?;
        let val = cap[2].parse::<f64>()?;
        is_partial_moment = true;
        if i == 6 {
            release_array = [0, 0, 0, -3, -3, -3];
            mp = val;
        } else {
            release_array[i - 3] = 0;
            release_array[i] = type_code;
            mp_array[i] = val;
        }
        name += format!(" {} {}", &cap[1], &val).as_str();
    }

    Ok(Release {
        id: 0,
        name: json!(name),
        r#type: json!(-1),
        assigned: json!(assigned),
        location: location_code,
        is_partial_moment,
        release_array: json!(release_array),
        spring_array: json!(spring_array),
        mp_array: json!(mp_array),
        mp: json!(mp),
    })
}

pub fn parse_member_truss(s: &str) -> Result<Truss> {
    let assigned = parse_keys(s);
    Ok(Truss {
        id: 0,
        name: json!("MEMBER TRUSS"),
        r#type: json!(0),
        assigned: json!(assigned),
    })
}

pub fn parse_section(str_input: &str) -> Result<Section> {
    let caps = REGEX_SECTION
        .captures(str_input)
        .ok_or(anyhow!("Invalid SECTION: {}", str_input))?;

    let assigned = parse_keys(str_input);
    let source = caps
        .name("source")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("Invalid SECTION source: {}", str_input))?;
    let r#type = caps
        .name("type")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("Invalid SECTION type: {}", str_input))?;
    let name = caps
        .name("name")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("Invalid SECTION name: {}", str_input))?;

    // 5. 결과 반환
    Ok(Section {
        id: 0,
        source,
        name,
        r#type,
        assigned,
    })
}

pub fn parse_nodal_load(s: &str) -> Result<LoadItemObj> {
    let validation_match = REGEX_NODAL_LOAD_VALIDATION.find(s);

    match validation_match {
        Some(matches) => {
            // Joint keys
            let split_index = matches.start();
            let key_part = &s[..split_index];
            let assigned = json!(parse_keys(key_part));

            // Load params
            let mut load = vec![0., 0., 0., 0., 0., 0.];
            let mut name = "".to_string();
            for cap in REGEX_NODAL_LOAD_PAIR.captures_iter(s) {
                let key = &cap[1];
                let idx = match key {
                    "FX" => 0,
                    "FY" => 1,
                    "FZ" => 2,
                    "MX" => 3,
                    "MY" => 4,
                    "MZ" => 5,
                    _ => bail!("Invalid load of JOINT LOAD '{}'", key),
                };
                let value: f64 = cap[2].parse()?;
                name += format!("{} {} ", key, value).as_str();
                load[idx] = value;
            }
            let attribute = json!(NodalLoad { load });
            Ok(LoadItemObj {
                id: json!(0),
                index: json!(0),
                r#type: LoadItemType::NodalLoad.as_code(),
                name: json!(name),
                assigned,
                attribute,
            })
        }
        None => bail!("Invalid JOINT LOAD: {}", s),
    }
}
pub fn parse_member_load(s: &str) -> Result<LoadItemObj> {
    let type_match = REGEX_MEMBER_LOAD_TYPE
        .find(s)
        .ok_or(anyhow!("Invalid MEMBER LOAD type: {}", s))?;
    let type_cap = REGEX_MEMBER_LOAD_TYPE
        .captures(s)
        .ok_or(anyhow!("Invalid MEMBER LOAD type: {}", s))?;

    let type_index = type_match.start();
    let load_type = &type_cap[1];
    let key_part = s[..type_index].trim();
    let load_part = s[type_index..].trim();

    // Member keys
    let assigned = json!(parse_keys(key_part));

    // Load params
    let load_matches: Vec<_> = REGEX_MEMBER_LOAD_VALUE.captures_iter(load_part).collect();
    let cap = &load_matches[0];
    let direction = Axis::str_to_code(&cap[1]);
    let load: f64 = cap
        .get(2)
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0.0);
    let d1: f64 = cap
        .get(3)
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0.0);
    let d2: f64 = cap
        .get(4)
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0.0);
    let d3: f64 = cap
        .get(5)
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0.0);
    let mut name = format!("{} {} {} {}", load_type, direction, d1, d2);

    let attribute = match load_type {
        "CON" => json!(ConcentratedForce {
            direction,
            load,
            d1,
            d2,
        }),
        "UNI" => {
            if d3.abs() > 1e-3 {
                name += format!(" {}", d3).as_str();
            }
            json!(UniformForce {
                direction,
                load,
                d1,
                d2,
                d3,
            })
        }
        _ => bail!("Invalid MEMBER LOAD attribute: {}", s),
    };
    let r#type = match load_type {
        "CON" => LoadItemType::ConcentratedForce.as_code(),
        "UNI" => LoadItemType::UniformForce.as_code(),
        _ => bail!("Invalid MEMBER LOAD type: {}", load_type),
    };

    Ok(LoadItemObj {
        id: json!(0),
        index: json!(0),
        r#type,
        name: json!(name),
        assigned,
        attribute,
    })
}
pub fn parse_floor_load(s: &str) -> Result<LoadItemObj> {
    let validation_match = REGEX_FLOAD_VALIDATION.find(s);
    if validation_match.is_none() {
        return Err(anyhow!("Invalid FLOOR LOAD: {}", s));
    }

    let group_name: String = REGEX_FLOAD_GROUP_NAME
        .captures(s)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or("".to_string());
    let pressure: f64 = REGEX_FLOAD_PRESSURE
        .captures(s)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0.);
    let dir: String = REGEX_FLOAD_DIRECTION
        .captures(s)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or("".to_string());
    let name = format!("{} FLOAD {} {}", group_name, pressure, dir);
    let direction = Axis::str_to_code(&dir);

    Ok(LoadItemObj {
        id: json!(0),
        index: json!(0),
        r#type: LoadItemType::FloorLoadGroup.as_code(),
        name: json!(name),
        assigned: serde_json::Value::Null,
        attribute: json!(FloorLoadGroup {
            group: group_name,
            pressure,
            direction,
        }),
    })
}
pub fn parse_temp_load(s: &str) -> Result<LoadItemObj> {
    if let Some(captures) = REGEX_TEMP_VARS.captures(s) {
        let axial_elongation = captures
            .get(1)
            .and_then(|m| m.as_str().parse::<f64>().ok())
            .unwrap_or(0.);
        let top_to_bottom = captures
            .get(2)
            .and_then(|m| m.as_str().parse::<f64>().ok())
            .unwrap_or(0.);
        let side_to_side = captures
            .get(3)
            .and_then(|m| m.as_str().parse::<f64>().ok())
            .unwrap_or(0.);
        let mut name = format!("TEMP {}", axial_elongation);
        if top_to_bottom.abs() > 1e-3 {
            name += &(top_to_bottom.to_string());
        }
        if side_to_side.abs() > 1e-3 {
            name += &(side_to_side.to_string());
        }

        Ok(LoadItemObj {
            id: json!(0),
            index: json!(0),
            r#type: LoadItemType::Temperature.as_code(),
            name: json!(name),
            assigned: serde_json::Value::Null,
            attribute: json!(Temperature {
                axial_elongation,
                top_to_bottom,
                side_to_side,
            }),
        })
    } else {
        bail!("Not TEMPERATURE LOAD: {}", s);
    }
}
pub fn parse_repeat_load(s: &str) -> Result<LoadItemObj> {
    let mut repeat_load = RepeatLoadData {
        cases: vec![],
        factors: vec![],
    };

    let _iter: Vec<_> = REGEX_REF_LOAD_PATTERN.captures_iter(s).collect();
    if _iter.len() == 0 {
        return Err(anyhow!("Not REF LOAD pairs: {}", s));
    }
    let mut name = "REPEAT LOAD:".to_string();
    for caps in _iter {
        let id = caps[1].parse::<i64>()?;
        let fac = caps[2].parse::<f64>()?;
        repeat_load.cases.push(json!(id));
        repeat_load.factors.push(json!(fac));

        name += format!(" {} {}", id, fac).as_str();
    }
    Ok(LoadItemObj {
        id: json!(0),
        index: json!(0),
        r#type: LoadItemType::RepeatLoadData.as_code(),
        name: json!(name),
        assigned: serde_json::Value::Null,
        attribute: json!(repeat_load),
    })
}
pub fn parse_reference_load(s: &str) -> Result<LoadItemObj> {
    let mut ref_load = ReferenceLoadData {
        cases: vec![],
        factors: vec![],
    };

    let _iter: Vec<_> = REGEX_REF_LOAD_PATTERN.captures_iter(s).collect();
    if _iter.len() == 0 {
        return Err(anyhow!("Not REF LOAD pairs: {}", s));
    }
    let mut name = "REFERENCE LOAD:".to_string();
    for caps in _iter {
        let id = caps[1].chars().skip(1).collect::<String>().parse::<f64>()?;
        let fac = caps[2].parse::<f64>()?;
        ref_load.cases.push(json!(id));
        ref_load.factors.push(json!(fac));

        name += format!(" R{} {}", id, fac).as_str();
    }
    Ok(LoadItemObj {
        id: json!(0),
        index: json!(0),
        r#type: LoadItemType::ReferenceLoadData.as_code(),
        name: json!(name),
        assigned: serde_json::Value::Null,
        attribute: json!(ref_load),
    })
}
pub fn parse_notional_load(s: &str) -> Result<LoadItemObj> {
    let mut not_load = NotionalLoadData {
        cases: vec![],
        factors: vec![],
        directions: vec![],
    };

    let _iter: Vec<_> = REGEX_NOTIONAL_LOAD_PATTERN.captures_iter(s).collect();
    if _iter.len() == 0 {
        return Err(anyhow!("Not NOTIONAL LOAD pairs: {}", s));
    }
    let mut name = "NOTIONAL LOAD:".to_string();
    for caps in _iter {
        let key = &caps[1];
        let id = match key.contains("R") {
            true => key.chars().skip(1).collect::<String>().parse::<f64>()?,
            false => key.parse::<f64>()?,
        };
        let direction = &caps[2];
        let dir_code = Axis::str_to_code(direction);
        let fac = caps[3].parse::<f64>()?;
        not_load.cases.push(json!(id));
        not_load.factors.push(json!(fac));
        not_load.directions.push(json!(dir_code));

        name += format!(" {} {}", key, fac).as_str();
    }
    Ok(LoadItemObj {
        id: json!(0),
        index: json!(0),
        r#type: LoadItemType::NotionalLoadData.as_code(),
        name: json!(name),
        assigned: serde_json::Value::Null,
        attribute: json!(not_load),
    })
}
pub fn parse_wind_load(s: &str) -> Result<LoadItemObj> {
    let type_cap = REGEX_WIND_LOAD_ITEM_TYPE.captures(s);

    let item_type = match type_cap {
        Some(cap) => cap[1].to_string(),
        None => bail!("Invalid WIND LOAD ITEM type: {}", s),
    };

    let _captures: Vec<_> = REGEX_WIND_LOAD_INTENSITY_TABLE.captures_iter(s).collect();
    let table_length = _captures.len() / 2;
    if table_length < 1 {
        return Err(anyhow!("Not WIND LOAD table data: {}", s));
    };

    let mut intensity: Vec<f64> = vec![];
    let mut height: Vec<f64> = vec![];
    for i in 0.._captures.len() {
        let caps = &_captures[i];
        let val = caps[1].parse::<f64>()?;
        if i < table_length {
            intensity.push(val)
        } else {
            height.push(val)
        }
    }
    Ok(LoadItemObj {
        id: json!(0),
        index: json!(0),
        r#type: json!(item_type),
        name: json!("INTENSITY"),
        assigned: serde_json::Value::Null,
        attribute: json!({
            "intensity":intensity,
            "height":height
        }),
    })
}
