use anyhow::Result;
use std::collections::HashMap;

fn get_p_value(z_score: u64) -> f64 {
    match z_score {
        90 => 1.6448536270,
        95 => 1.9599639845,
        99 => 2.5758293035,
        _ => 0.0,
    }
}

pub fn get_z_interval(sigma: u64, xbar: u64, n: u64, c_level: u64) -> Result<String> {
    let z_score = get_p_value(c_level);

    Ok(format!(
        "Upper: {}\nLower: {}",
        (xbar as f64 + (z_score * sigma as f64 / (n as f64).sqrt())),
        (xbar as f64 - (z_score * (sigma as f64 / (n as f64).sqrt())))
    ))
}

pub fn get_t_interval() {}
