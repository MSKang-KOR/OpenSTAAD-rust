use anyhow::{Result, bail};

pub fn get_unit_factor(base_unit: i32, unit: &str) -> Result<f64> {
    let factor = match base_unit {
        // inch, kip
        1 => match unit {
            "inch" => 1.0,
            "ft" => 0.0833333,
            "mm" => 25.40,
            "cm" => 2.540,
            "dm" => 0.2540,
            "m" => 0.02540,

            "kg" => 4.535600e+002,
            "kN" => 4.4482,
            "kip" => 1.,
            "lb" => 1000.,
            "MTon" => 4.535600E-001,
            "N" => 4.448220E+003,
            _ => bail!("Unknown unit: {}", unit),
        },
        // m, kN
        0 => match unit {
            "inch" => 39.37008,
            "ft" => 3.2808,
            "mm" => 1000.,
            "cm" => 100.,
            "dm" => 10.,
            "m" => 1.,

            "kg" => 1.019644E+002,
            "kN" => 1.,
            "kip" => 2.248090E-001,
            "lb" => 2.248090E+002,
            "MTon" => 1.019644E-001,
            "N" => 1000.,
            _ => bail!("Unknown unit: {}", unit),
        },
        _ => bail!("Invalid Base unit: {}", base_unit),
    };
    Ok(factor)
}
