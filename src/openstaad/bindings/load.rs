use serde::Serialize;
use serde_json::{Value, json};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum PrimiryLoadType {
    Error = -1,
    Dead = 0,
    Live = 1,
    RoofLive = 2,
    Wind = 3,
    SeismicH = 4,
    SeismicV = 5,
    Snow = 6,
    Fluids = 7,
    Soil = 8,
    Rain = 9,
    Ponding = 10,
    Dust = 11,
    Traffic = 12,
    Temp = 13,
    Imperfection = 14,
    Accidental = 15,
    Flood = 16,
    Ice = 17,
    WindIce = 18,
    CraneHook = 19,
    Mass = 20,
    Gravity = 21,
    Push = 22,
    None = 23,
}
impl PrimiryLoadType {
    pub fn as_code(&self) -> Value {
        json!(*self as i8)
    }
    pub fn as_name(&self) -> Value {
        match self {
            Self::Dead => json!("Dead"),
            Self::Live => json!("Live"),
            Self::RoofLive => json!("RoofLive"),
            Self::Wind => json!("Wind"),
            Self::SeismicH => json!("Seismic-H"),
            Self::SeismicV => json!("Seismic-V"),
            Self::Snow => json!("Snow"),
            Self::Fluids => json!("Fluids"),
            Self::Soil => json!("Soil"),
            Self::Rain => json!("Rain"),
            Self::Ponding => json!("Ponding"),
            Self::Dust => json!("Dust"),
            Self::Traffic => json!("Traffic"),
            Self::Temp => json!("Temp"),
            Self::Imperfection => json!("Imperfection"),
            Self::Accidental => json!("Accidental"),
            Self::Flood => json!("Flood"),
            Self::Ice => json!("Ice"),
            Self::WindIce => json!("Wind Ice"),
            Self::CraneHook => json!("Crane Hook"),
            Self::Mass => json!("Mass"),
            Self::Gravity => json!("Gravity"),
            Self::Push => json!("Push"),
            Self::None => json!("None"),
            Self::Error => json!("Error"),
        }
    }
    pub fn from(value: Value) -> Self {
        let code = value.as_i64().unwrap() as i8;
        match code {
            0 => Self::Dead,
            1 => Self::Live,
            2 => Self::RoofLive,
            3 => Self::Wind,
            4 => Self::SeismicH,
            5 => Self::SeismicV,
            6 => Self::Snow,
            7 => Self::Fluids,
            8 => Self::Soil,
            9 => Self::Rain,
            10 => Self::Ponding,
            11 => Self::Dust,
            12 => Self::Traffic,
            13 => Self::Temp,
            14 => Self::Imperfection,
            15 => Self::Accidental,
            16 => Self::Flood,
            17 => Self::Ice,
            18 => Self::WindIce,
            19 => Self::CraneHook,
            20 => Self::Mass,
            21 => Self::Gravity,
            22 => Self::Push,
            23 => Self::None,
            _ => Self::Error,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum LoadItemType {
    Error = -1,
    SelfWeight = 4000,
    NodalLoad = 3110,
    InclinedNodalLoad = 3120,
    SupportDisplacementNodalLoad = 3910,
    RegionNodalLoad = 3312,
    UniformForce = 3210,
    UniformMoment = 3220,
    ConcentratedForce = 3230,
    ConcentratedMoment = 3240,
    LinearVarying = 3250,
    Trapezoidal = 3260,
    Hydrostatic = 3261,
    PrePostStress = 3620,
    FixedEnd = 3810,
    PhysicalUniformForce = 3275,
    PhysicalUniformMoment = 3280,
    PhysicalConcentratedForce = 3285,
    PhysicalConcentratedMoment = 3290,
    PhysicalTrapezoidal = 3295,
    Area = 3410,
    FloorLoadYrange = 3510,
    FloorLoadXrange = 3511,
    TurkishLoad = 4570,
    GB50011Load = 4575,
    Colombian2010Load = 4576,
    TimeHistoryLoad = 4820,
    SnowLoadData = 4651,
    RepeatLoadData = 4201,
    NotionalLoadData = 4223,
    ReferenceLoad = 4220,
    ReferenceLoadData = 4221,
    SpectrumData = 4101,
    CalulateRayleighFrequency = 4701,
    RepeatLoad = 4200,
    FloorLoadZrange = 3520,
    FloorLoadGroup = 3530,
    OneWayFloorLoadXrange = 3551,
    OneWayFloorLoadYrange = 3552,
    OneWayFloorLoadZrange = 3553,
    OneWayFloorLoadGroup = 3554,
    PressureOnFullplate = 3310,
    PlateConcentratedLoad = 3311,
    // PartialPlatePressureLoad = 3312, // Duplicated!!
    PlateTrapezoidal = 3320,
    Solid = 3322,
    Temperature = 3710,
    Strain = 3720,
    StrainRate = 3721,
    UBCLoad = 4400,
    WindLoad = 4600,
    WindLoadDynamic = 4610,
    IbcLoad = 4405,
    Load1893 = 4410,
    AijLoad = 4500,
    ColombianLoad = 4510,
    CFELoad = 4520,
    RPALoad = 4530,
    NTCLoad = 4540,
    NRCLoad = 4550,
    NRCLoad2005 = 4560,
    NRCLoad2010 = 4561,
    SpectrumLoad = 4100,
    CalulateNaturalFrequency = 4700,
    ModalCalculationRequested = 4710,
    NotionalLoad = 4222,
    SnowLoad = 4650,
}
impl LoadItemType {
    pub fn as_code(&self) -> Value {
        json!(*self as i16)
    }
    pub fn from(value: &Value) -> Self {
        let code = value.as_i64().unwrap() as i16;
        match code {
            4000 => Self::SelfWeight,
            3110 => Self::NodalLoad,
            3120 => Self::InclinedNodalLoad,
            3910 => Self::SupportDisplacementNodalLoad,
            3312 => Self::RegionNodalLoad,
            3210 => Self::UniformForce,
            3220 => Self::UniformMoment,
            3230 => Self::ConcentratedForce,
            3240 => Self::ConcentratedMoment,
            3250 => Self::LinearVarying,
            3260 => Self::Trapezoidal,
            3261 => Self::Hydrostatic,
            3620 => Self::PrePostStress,
            3810 => Self::FixedEnd,
            3275 => Self::PhysicalUniformForce,
            3280 => Self::PhysicalUniformMoment,
            3285 => Self::PhysicalConcentratedForce,
            3290 => Self::PhysicalConcentratedMoment,
            3295 => Self::PhysicalTrapezoidal,
            3410 => Self::Area,
            3510 => Self::FloorLoadYrange,
            3511 => Self::FloorLoadXrange,
            4570 => Self::TurkishLoad,
            4575 => Self::GB50011Load,
            4576 => Self::Colombian2010Load,
            4820 => Self::TimeHistoryLoad,
            4651 => Self::SnowLoadData,
            4201 => Self::RepeatLoadData,
            4223 => Self::NotionalLoadData,
            4220 => Self::ReferenceLoad,
            4221 => Self::ReferenceLoadData,
            4101 => Self::SpectrumData,
            4701 => Self::CalulateRayleighFrequency,
            4200 => Self::RepeatLoad,
            3520 => Self::FloorLoadZrange,
            3530 => Self::FloorLoadGroup,
            3551 => Self::OneWayFloorLoadXrange,
            3552 => Self::OneWayFloorLoadYrange,
            3553 => Self::OneWayFloorLoadZrange,
            3554 => Self::OneWayFloorLoadGroup,
            3310 => Self::PressureOnFullplate,
            3311 => Self::PlateConcentratedLoad,
            3320 => Self::PlateTrapezoidal,
            3322 => Self::Solid,
            3710 => Self::Temperature,
            3720 => Self::Strain,
            3721 => Self::StrainRate,
            4400 => Self::UBCLoad,
            4600 => Self::WindLoad,
            4610 => Self::WindLoadDynamic,
            4405 => Self::IbcLoad,
            4410 => Self::Load1893,
            4500 => Self::AijLoad,
            4510 => Self::ColombianLoad,
            4520 => Self::CFELoad,
            4530 => Self::RPALoad,
            4540 => Self::NTCLoad,
            4550 => Self::NRCLoad,
            4560 => Self::NRCLoad2005,
            4561 => Self::NRCLoad2010,
            4100 => Self::SpectrumLoad,
            4700 => Self::CalulateNaturalFrequency,
            4710 => Self::ModalCalculationRequested,
            4222 => Self::NotionalLoad,
            4650 => Self::SnowLoad,
            _ => Self::Error,
        }
    }
    pub fn has_assigned(&self) -> bool {
        !matches! {self,
            Self::RepeatLoad
            | Self::RepeatLoadData
            | Self::ReferenceLoad
            | Self::ReferenceLoadData
            | Self::NotionalLoad
            | Self::NotionalLoadData
            | Self::FloorLoadGroup
            | Self::CalulateRayleighFrequency
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LoadItemAttribute {
    NodalLoad(NodalLoad),
    ConcentratedForce(ConcentratedForce),
    ConcentratedMoment(ConcentratedMoment),
    UniformForce(UniformForce),
    UniformMoment(UniformMoment),
    SelfWeight(SelfWeight),
    FloorLoadGroup(FloorLoadGroup),
    RepeatLoadData(RepeatLoadData),
    ReferenceLoadData(ReferenceLoadData),
    NotionalLoadData(NotionalLoadData),
    CalulateRayleighFrequency(CalulateRayleighFrequency),
    Error(ErrorLoadItem),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NodalLoad {
    pub load: Vec<Value>, // Vec<f64>
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConcentratedForce {
    pub direction: i64,
    pub load: f64,
    pub d1: f64,
    pub d2: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConcentratedMoment {
    pub direction: i64,
    pub load: f64,
    pub d1: f64,
    pub d2: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UniformForce {
    pub direction: i64,
    pub load: f64,
    pub d1: f64,
    pub d2: f64,
    pub d3: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UniformMoment {
    pub direction: i64,
    pub load: f64,
    pub d1: f64,
    pub d2: f64,
    pub d3: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SelfWeight {
    // SelfWeight는 보통 별도의 데이터가 필요하지 않음
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FloorLoadGroup {
    // FloorLoadGroup도 보통 별도의 데이터가 필요하지 않음
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RepeatLoadData {
    pub cases: Vec<Value>,
    pub factors: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReferenceLoadData {
    pub cases: Vec<Value>,
    pub factors: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NotionalLoadData {
    pub cases: Vec<Value>,
    pub factors: Vec<Value>,
    pub directions: Vec<Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CalulateRayleighFrequency {}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ErrorLoadItem {}
