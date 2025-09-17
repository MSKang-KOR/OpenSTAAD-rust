use serde::Serialize;
use serde_json::{Value, json};
use std::sync::Arc;

use crate::openstaad::{
    app::OpenStaad, command::Command, design::Design, geometry::Geometry, load::Load,
    output::Output, property::Property, support::Support,
};

#[derive(Debug)]
pub enum Staad {
    OpenStaad(OpenStaad),
    Geometry(Arc<Geometry>),
    Command(Arc<Command>),
    Design(Arc<Design>),
    Load(Arc<Load>),
    Output(Arc<Output>),
    Property(Arc<Property>),
    Support(Arc<Support>),
}

unsafe impl Send for Staad {}
unsafe impl Sync for Staad {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Axis {
    Error = -1,
    X = 1,
    Y = 2,
    Z = 3,
    GX = 4,
    GY = 5,
    GZ = 6,
    PX = 7,
    PY = 8,
    PZ = 9,
}

impl Axis {
    pub fn as_code(&self) -> Value {
        json!(*self as i8)
    }
    pub fn as_str(&self) -> &str {
        match self {
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::GX => "GX",
            Self::GY => "GY",
            Self::GZ => "GZ",
            Self::PX => "PX",
            Self::PY => "PY",
            Self::PZ => "PZ",
            Self::Error => "ERROR",
        }
    }
    pub fn from(value: Value) -> Self {
        let n = value.as_i64().unwrap() as i8;
        match n {
            1 => Self::X,
            2 => Self::Y,
            3 => Self::Z,
            4 => Self::GX,
            5 => Self::GY,
            6 => Self::GZ,
            7 => Self::PX,
            8 => Self::PY,
            9 => Self::PZ,
            _ => Self::Error,
        }
    }
}
