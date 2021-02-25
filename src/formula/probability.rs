use {anyhow::Result, statrs::function};

pub fn factorial(n: u64) -> Result<String> {
    Ok(format!("{}", function::factorial::factorial(n)))
}
