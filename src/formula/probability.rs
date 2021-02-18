use {anyhow::Result, factorial::Factorial, std::fmt::Display};

pub fn factorial<T>(n: T) -> Result<String>
where
    T: Display + Factorial,
{
    Ok(format!("{}", n.factorial()))
}
