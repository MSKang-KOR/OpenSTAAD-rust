use anyhow::{Context, Result, anyhow, bail};
use log::{error, info, warn};
use serde_json::Value;
use std::sync::Arc;

use crate::{
    openstaad::{
        app::OpenStaad,
        bindings::{BeamTableRow, NodeTableRow, Staad},
        execute::execute_method,
        geometry,
    },
    tools::unit::get_unit_factor,
};

fn get_unit_factors(app: &Staad) -> Result<(f64, f64)> {
    let base_unit_value = execute_method(&app, "GetBaseUnit", &[])?;
    let base_unit = base_unit_value.as_i64().context("Base unit context err")? as i32;

    let length_unit_value = execute_method(&app, "GetInputUnitForLength", &[])?;
    let length_unit = length_unit_value[1]
        .as_str()
        .context("Length unit context err,{}")?;

    let force_unit_length = execute_method(&app, "GetInputUnitForForce", &[])?;
    let force_unit = force_unit_length[1]
        .as_str()
        .context("Force unit context err")?;

    let lf = get_unit_factor(base_unit, length_unit)?;
    let ff = get_unit_factor(base_unit, force_unit)?;
    Ok((lf, ff))
}

pub fn get_nodes_table(app: &mut Staad) -> Result<Value> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    let mut _geo = openstaad.get_geometry()?;
    let geometry = Staad::Geometry(Arc::clone(&_geo));

    let (lf, ff) = get_unit_factors(app)?;
    let node_list = execute_method(&geometry, "GetNodeList", &[])?;
    let mut coords_table: Vec<NodeTableRow> = Vec::new();
    for node in node_list.as_array().unwrap() {
        let _n = node.as_i64().context("Context err")? as i32;
        let coord_values = execute_method(&geometry, "GetNodeCoordinates", &[_n.into()])?;

        let mut coords: Vec<f64> = Vec::new();
        let _ = coord_values
            .as_array()
            .unwrap()
            .iter()
            .for_each(|v| coords.push(v.as_f64().unwrap() * lf));

        let row = NodeTableRow {
            id: _n,
            x: coords[0],
            y: coords[1],
            z: coords[2],
        };
        coords_table.push(row);
    }

    serde_json::to_value(&coords_table).map_err(|e| anyhow!(e))
}

pub fn get_beams_table(app: &mut Staad) -> Result<Value> {
    let openstaad: &mut OpenStaad = match app {
        Staad::OpenStaad(v) => v,
        _ => bail!("Not OpenStaad instance"),
    };

    let mut _geo = openstaad.get_geometry()?;
    let mut _prop = openstaad.get_property()?;
    let geometry = Staad::Geometry(Arc::clone(&_geo));
    let property = Staad::Property(Arc::clone(&_prop));

    let (lf, ff) = get_unit_factors(app)?;
    let beam_list = execute_method(&geometry, "GetBeamList", &[])?;
    let mut beams_table: Vec<BeamTableRow> = Vec::new();
    for beam in beam_list.as_array().unwrap() {
        let _n = beam.as_i64().context("Context err")? as i32;

        let node_values = execute_method(&geometry, "GetMemberIncidence", &[_n.into()])?;
        let mut nodes: Vec<i32> = node_values
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_i64().unwrap() as i32)
            .collect();

        let prop_values = execute_method(&property, "GetBeamSectionPropertyRefNo", &[_n.into()])?;
        let prop = prop_values.as_i64().unwrap() as i32;

        let mat_values = execute_method(&property, "GetBeamMaterialName", &[_n.into()])?;
        let mat = mat_values.as_str().unwrap();

        let beta_values = execute_method(&property, "GetBetaAngle", &[_n.into()])?;
        let beta = beta_values.as_f64().unwrap();

        let length_values = execute_method(&geometry, "GetBeamLength", &[_n.into()])?;
        let length = length_values.as_f64().unwrap();

        let row = BeamTableRow {
            id: _n,
            i: nodes[0],
            j: nodes[1],
            property: prop,
            material: mat.to_string(),
            beta,
            length: length * lf,
        };
        beams_table.push(row);
    }

    serde_json::to_value(&beams_table).map_err(|e| anyhow!(e))
}
