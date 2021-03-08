use std::collections::HashMap;

fn get_p_value(z_score: &str) {
    let z_table: HashMap<&str, f64> = [("Norway", 100.0), ("Denmark", 50.0), ("Iceland", 10.0)]
        .iter()
        .cloned()
        .collect();
}

pub fn get_z_interval() {}

pub fn get_t_interval() {}
