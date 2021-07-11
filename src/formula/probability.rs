use {
    anyhow::Result,
    statrs::{
        distribution::{
            Binomial, ChiSquared, Continuous, Discrete, FisherSnedecor, Geometric, InverseCDF,
            Normal, Poisson, StudentsT, Univariate,
        },
        function::factorial::factorial,
    },
};

pub fn get_factorial(n: u64) -> Result<String> {
    Ok(format!("{}", factorial(n)))
}

pub fn get_permutation(n: u64, k: u64) -> Result<String> {
    Ok(format!("{}", factorial(n) / factorial((n - k))))
}

pub fn get_combination(n: u64, k: u64) -> Result<String> {
    Ok(format!(
        "{}",
        factorial(n) / (factorial(k) * factorial((n - k)))
    ))
}

pub fn get_normal_pdf(x: f64, mean: f64, std_dev: f64) -> Result<String> {
    let mut normal = Normal::new(mean, std_dev);

    if normal.is_err() {
        return Ok(String::from(
            "Ensure mean and std dev are numbers, and that std dev is greater than 0",
        ));
    };

    Ok(format!("{}", normal?.pdf(x)))
}

pub fn get_normal_cdf(
    lower_bound: f64,
    upper_bound: f64,
    mean: f64,
    std_dev: f64,
) -> Result<String> {
    let mut normal = Normal::new(mean, std_dev);

    if normal.is_err() {
        return Ok(String::from(
            "Ensure mean and std dev are numbers, and that std dev is greater than 0",
        ));
    };

    let normal = normal?;

    Ok(format!(
        "{}",
        normal.cdf(upper_bound) - normal.cdf(lower_bound)
    ))
}

pub fn get_inv_normal(area: f64, mean: f64, std_dev: f64) -> Result<String> {
    let mut normal = Normal::new(mean, std_dev);

    if normal.is_err() {
        return Ok(String::from(
            "Ensure mean and std dev are numbers, and that std dev is greater than 0",
        ));
    };

    Ok(format!("{}", normal?.inverse_cdf(area)))
}

pub fn get_t_pdf(x: f64, df: f64) -> Result<String> {
    let mut t = StudentsT::new(0.0, 1.0, df);

    if t.is_err() {
        return Ok(String::from(
            "Ensure x and df are numbers, and that df is greater than 0",
        ));
    };

    Ok(format!("{}", t?.pdf(x)))
}

pub fn get_t_cdf(lower_bound: f64, upper_bound: f64, df: f64) -> Result<String> {
    let mut t = StudentsT::new(0.0, 1.0, df);

    if t.is_err() {
        return Ok(String::from(
            "Ensure both bounds and df are numbers, and that df is greater than 0",
        ));
    };

    let t = t?;

    Ok(format!("{}", t.cdf(upper_bound) - t.cdf(lower_bound)))
}

pub fn get_chi_square_pdf(x: f64, df: f64) -> Result<String> {
    let mut chi_square = ChiSquared::new(df);

    if chi_square.is_err() {
        return Ok(String::from(
            "Ensure x and df are numbers, and that df is greater than 0",
        ));
    };

    Ok(format!("{}", chi_square?.pdf(x)))
}

pub fn get_chi_square_cdf(lower_bound: f64, upper_bound: f64, df: f64) -> Result<String> {
    let mut chi_square = ChiSquared::new(df);

    if chi_square.is_err() {
        return Ok(String::from(
            "Ensure bounds and df are numbers, and that df is greater than 0",
        ));
    };

    let chi_square = chi_square?;

    Ok(format!(
        "{}",
        chi_square.cdf(upper_bound) - chi_square.cdf(lower_bound)
    ))
}
pub fn get_binom_pdf(n: u64, p: f64, x: u64) -> Result<String> {
    let mut binom = Binomial::new(p, n);

    if binom.is_err() {
        return Ok(String::from(
            "Ensure x, n, and p are numbers, and that df is greater than 0",
        ));
    };

    Ok(format!("{}", binom?.pmf(x)))
}

pub fn get_binom_cdf(n: u64, p: f64, lower_bound: f64, upper_bound: f64) -> Result<String> {
    let mut binom = Binomial::new(p, n);

    if binom.is_err() {
        return Ok(String::from("Ensure bounds, n, and p are numbers"));
    };

    let binom = binom?;

    Ok(format!(
        "{}",
        binom.cdf(upper_bound) - binom.cdf(lower_bound)
    ))
}

pub fn get_f_pdf(df_1: f64, df_2: f64, x: f64) -> Result<String> {
    let f = FisherSnedecor::new(df_1, df_2);

    if f.is_err() {
        return Ok(String::from("Ensure df 1, df 2, and x are numbers"));
    }

    Ok(format!("{}", f?.pdf(x)))
}

pub fn get_f_cdf(lower_bound: f64, upper_bound: f64, df_1: f64, df_2: f64) -> Result<String> {
    let f = FisherSnedecor::new(df_1, df_2);

    if f.is_err() {
        return Ok(String::from("Ensure df 1, df 2, and bounds are numbers"));
    }

    let f = f?;

    Ok(format!("{}", f.cdf(upper_bound) - f.cdf(lower_bound)))
}

pub fn get_geo_pdf(p: f64, x: u64) -> Result<String> {
    let geo = Geometric::new(p);

    if geo.is_err() {
        return Ok(String::from("Ensure p and x are numbers"));
    }

    Ok(format!("{}", geo?.pmf(x)))
}

pub fn get_geo_cdf(p: f64, lower_bound: f64, upper_bound: f64) -> Result<String> {
    let geo = Geometric::new(p);

    if geo.is_err() {
        return Ok(String::from("Ensure p and bounds are numbers"));
    }

    let geo = geo?;

    Ok(format!(
        "{}",
        geo.cdf(upper_bound) - geo.cdf(lower_bound) + p
    ))
}

pub fn get_poisson_pdf(lambda: f64, x: u64) -> Result<String> {
    let poisson = Poisson::new(lambda);

    if poisson.is_err() {
        return Ok(String::from("Ensure lambda and x are numbers"));
    }

    Ok(format!("{}", poisson?.pmf(x)))
}

pub fn get_poisson_cdf(lambda: f64, lower_bound: f64, upper_bound: f64) -> Result<String> {
    let poisson = Poisson::new(lambda);

    if poisson.is_err() {
        return Ok(String::from("Ensure lambda and bounds are numbers"));
    }

    let poisson = poisson?;

    Ok(format!(
        "{}",
        poisson.cdf(upper_bound) - poisson.cdf(lower_bound - 1.0)
    ))
}
