use {anyhow::Result, std::collections::HashMap};

fn get_z_crit(c_level: u64) -> f64 {
    match c_level {
        90 => 1.6448536270,
        95 => 1.9599639845,
        99 => 2.5758293035,
        _ => 0.0,
    }
}

fn get_t_crit(c_level: u64, df: u64) -> f64 {
    0.0
}

pub fn get_z_interval(sigma: u64, xbar: u64, n: u64, c_level: u64) -> Result<String> {
    let z_score = get_z_crit(c_level);

    Ok(format!(
        "Upper: {}\nLower: {}\nME: {}",
        (xbar as f64 + (z_score * sigma as f64 / (n as f64).sqrt())),
        (xbar as f64 - (z_score * (sigma as f64 / (n as f64).sqrt()))),
        (z_score * (sigma as f64 / (n as f64).sqrt()))
    ))
}

pub fn get_t_interval() {}
