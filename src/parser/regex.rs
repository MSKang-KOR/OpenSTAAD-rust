use once_cell::sync::Lazy;
use regex::Regex;

// STD separater
pub static REGEX_CONNECT_ST: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s*-\s*$").unwrap());
pub static REGEX_UPPER_ST: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([A-Z\s]+)$").unwrap());
pub static REGEX_COMMAND_ST: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(DEFINE|START|PERFORM)\b").unwrap());
pub static REGEX_END_ST: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(END)\b").unwrap());
pub static REGEX_JOB_ST: Lazy<Regex> = Lazy::new(|| Regex::new(r"^JOB REF (\S+)").unwrap());
pub static REGEX_COORD_ST: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
    r"^\s*([-+]?\d+(?:\.\d+)?)\s+([-+]?\d+(?:\.\d+)?)\s+([-+]?\d+(?:\.\d+)?)\s+([-+]?\d+(?:\.\d+)?)\s*$",
).unwrap()
});
pub static REGEX_MEMBER_ST: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*([-+]?\d+(?:\.\d+)?)\s+([-+]?\d+(?:\.\d+)?)\s+([-+]?\d+(?:\.\d+)?)\s*$")
        .unwrap()
});
pub static REGEX_GROUP_TYPE_ST: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*(GEOMETRY|JOINT|MEMBER|ELEMENT|SOLID|FLOOR)\s*").unwrap());
pub static REGEX_PRIMIRY_LOAD_ST: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
    r"^LOAD\s+(?P<key>\S+)(?:\s+(?:LOADTYPE\s+(?P<type>\S+))?(?:\s*TITLE\s+)?(?:(?P<title>.+)))?$",
).unwrap()
});
pub static REGEX_MATERIAL_ITEM_ST: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(ISOTROPIC|2DORTHOTROPIC)\s+(.+)").unwrap());
pub static REGEX_CONSTANT_ITEM_ST: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(BETA|MATERIAL)\s+(.+)").unwrap());
pub static REGEX_WIND_LOAD_ST: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"TYPE\s+(\d+)\s+(.+)").unwrap());

// IDs
pub static REGEX_RANGE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s*(\d+)\s+TO\s+(\d+)\s*").unwrap());
// Nodal Load
pub static REGEX_INDIVIDUAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b(\d+)\b").unwrap());
pub static REGEX_NODAL_LOAD_VALIDATION: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s+(INCLINED|FX|FY|FZ|MX|MY|MZ)\s+").unwrap());
pub static REGEX_NODAL_LOAD_INCLINED: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"INCLINED\s+(?:(?:(?<x>[-\d.]+)\s+(?<y>[-\d.]+)\s+(?<z>[-\d.]+))|(REF)\s+(?<xref>[-\d.]+)\s+(?<yref>[-\d.]+)\s+(?<zref>[-\d.]+)|(REFJT)\s+(?<joint>[-\d.]+))"
    ).unwrap()
});
pub static REGEX_NODAL_LOAD_PAIR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(FX|FY|FZ|MX|MY|MZ)\s+([-\d.]+)").unwrap());
// Member Load
pub static REGEX_MEMBER_LOAD_TYPE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(CON|UNI)\s+").unwrap());
pub static REGEX_MEMBER_LOAD_VALUE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"([A-Z]+)\s+([-+]?\d*\.?\d+)(?:\s+([-+]?\d*\.?\d+)(?:\s+([-+]?\d*\.?\d+)(?:\s+([-+]?\d*\.?\d+))?)?)?"
    ).unwrap()
});
// Floor Load
pub static REGEX_FLOAD_VALIDATION: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s+(FLOAD)\s+").unwrap());
pub static REGEX_FLOAD_GROUP_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"(_[^\s]+)\s+").unwrap());
pub static REGEX_FLOAD_PRESSURE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"FLOAD\s+([-\d.]+)").unwrap());
pub static REGEX_FLOAD_DIRECTION: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b(GX|GY|GZ)").unwrap());
// Temperature LOAD
pub static REGEX_TEMP_VARS: Lazy<Regex> = Lazy::new(|| {
    {
    Regex::new(
        r"TEMP\s+([-+]?\d+(?:\.\d*)?)\s*(?:([-+]?\d+(?:\.\d*)?)\s*)?(?:([-+]?\d+(?:\.\d*)?)\s*)?$",
    )
}
.unwrap()
});
// Repeat Load & Reference Load
pub static REGEX_REF_LOAD_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\S+)\s+([-+]?\d+(?:\.\d+)?)").unwrap());
// Notional Load
pub static REGEX_NOTIONAL_LOAD_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\S+)\s+([XYZ])\s+([-+]?\d+(?:\.\d+)?)").unwrap());
// Wind Load
pub static REGEX_WIND_LOAD_ITEM_TYPE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b(INT)\s+").unwrap());
pub static REGEX_WIND_LOAD_INTENSITY_TABLE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s+([-+]?\d+(?:\.\d*)?)").unwrap());
