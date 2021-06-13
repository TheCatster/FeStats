use crate::app::App;
use probability::{
    get_binom_cdf, get_binom_pdf, get_chi_square_cdf, get_chi_square_pdf, get_combination,
    get_f_cdf, get_f_pdf, get_factorial, get_geo_cdf, get_geo_pdf, get_inv_normal, get_normal_cdf,
    get_normal_pdf, get_permutation, get_poisson_cdf, get_poisson_pdf, get_t_cdf, get_t_pdf,
};

use anyhow::Result;

pub mod intervals;
pub mod probability;
pub mod tests;

const C_LEVELS: [&str; 3] = ["90", "95", "99"];

pub fn retrieve_formula(formula_name: &str) -> Vec<String> {
    match_formula_inputs(formula_name)
}

pub fn attempt_formula(app: &mut App, formula_name: &str, inputs: &Vec<String>) -> Result<String> {
    if inputs.is_empty() || inputs.len() - 1 < retrieve_formula(formula_name).len() {
        if formula_name.contains("Interval") {
            Ok(format!(
                "All inputs are not filled yet.\nPlease select a C-Level of{}.",
                C_LEVELS
                    .iter()
                    .map(|x| format!(" {},", *x))
                    .collect::<String>()
            ))
        } else {
            Ok(String::from("All inputs are not filled yet."))
        }
    } else {
        let inputs: &Vec<String> = &inputs.iter().skip(1).map(|x| String::from(x)).collect();

        for input in inputs {
            let input = input.trim().parse::<f64>();

            match input {
                Ok(_) => {}
                Err(_) => {
                    return Ok(String::from(
                        "Not all inputs are numbers. Please enter them again.",
                    ))
                }
            }
        }
        match_formula_equations(app, formula_name, inputs)

    }
}

fn match_formula_equations(
    app: &mut App,
    formula_name: &str,
    input: &Vec<String>,
) -> Result<String> {
    match formula_name {
        // Probability Formulas
        "Factorial (!)" => {
            let input = input[0].parse::<u64>();

            match input {
                Ok(input) => Ok(get_factorial(input)?),
                Err(_) => Ok(String::from(
                    "The number must be an integer without a decimal. Please try again.",
                )),
            }
        }
        "Permutations" => {
            let input_n = input[0].parse::<u64>();
            let input_k = input[1].parse::<u64>();

            match input_n {
                Ok(_) => match input_k {
                    Ok(_) => Ok(format!("{}", get_permutation(input_n?, input_k?)?)),
                    Err(_) => Ok(String::from(
                        "Both numbers must be an integer without a decimal. Please try again.",
                    )),
                },
                Err(_) => Ok(String::from(
                    "Both numbers must be an integer without a decimal. Please try again.",
                )),
            }
        }
        "Combinations" => {
            let input_n = input[0].parse::<u64>();
            let input_k = input[1].parse::<u64>();

            match input_n {
                Ok(_) => match input_k {
                    Ok(_) => Ok(format!("{}", get_combination(input_n?, input_k?)?)),
                    Err(_) => Ok(String::from(
                        "Both numbers must be an integer without a decimal. Please try again.",
                    )),
                },
                Err(_) => Ok(String::from(
                    "Both numbers must be an integer without a decimal. Please try again.",
                )),
            }
        }
        "Normal Pdf" => get_normal_pdf(
            input[0].parse::<f64>()?,
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()?,
        ),
        "Normal Cdf" => get_normal_cdf(
            input[0].parse::<f64>()?,
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()?,
            input[3].parse::<f64>()?,
        ),
        "Inverse Normal" => get_inv_normal(
            input[0].parse::<f64>()?,
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()?,
        ),
        "t Pdf" => get_t_pdf(input[0].parse::<f64>()?, input[1].parse::<f64>()?),
        "t Cdf" => get_t_cdf(
            input[0].parse::<f64>()?,
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()?,
        ),
        "χ2 Pdf" => get_chi_square_pdf(input[0].parse::<f64>()?, input[1].parse::<f64>()?),
        "χ2 Cdf" => get_chi_square_cdf(
            input[0].parse::<f64>()?,
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()?,
        ),
        "Binomial Pdf" => {
            let n = input[0].parse::<u64>();
            let x = input[2].parse::<u64>();

            match n {
                Ok(n) => match x {
                    Ok(x) => get_binom_pdf(n, input[1].parse::<f64>()?, x),
                    Err(_) => Ok(String::from(
                        "The number must be an integer without a decimal. Please try again.",
                    )),
                },
                Err(_) => Ok(String::from(
                    "The number must be an integer without a decimal. Please try again.",
                )),
            }
        }
        "Binomial Cdf" => {
            let n = input[0].parse::<u64>();

            match n {
                Ok(n) => get_binom_cdf(
                    n,
                    input[1].parse::<f64>()?,
                    input[2].parse::<f64>()?,
                    input[3].parse::<f64>()?,
                ),
                Err(_) => Ok(String::from(
                    "The number must be an integer without a decimal. Please try again.",
                )),
            }
        }
        "F Pdf" => get_f_pdf(
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()?,
            input[0].parse::<f64>()?,
        ),
        "F Cdf" => get_f_cdf(
            input[0].parse::<f64>()?,
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()?,
            input[3].parse::<f64>()?,
        ),
        "Geometric Pdf" => {
            let x = input[1].parse::<u64>();

            match x {
                Ok(x) => get_geo_pdf(input[0].parse::<f64>()?, x),
                Err(_) => Ok(String::from(
                    "The number must be an integer without a decimal. Please try again.",
                )),
            }
        }
        "Geometric Cdf" => get_geo_cdf(
            input[0].parse::<f64>()?,
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()?,
        ),
        "Poisson Pdf" => {
            let x = input[1].parse::<u64>();

            match x {
                Ok(x) => get_poisson_pdf(input[0].parse::<f64>()?, x),
                Err(_) => Ok(String::from(
                    "The number must be an integer without a decimal. Please try again.",
                )),
            }
        }
        "Poisson Cdf" => get_poisson_cdf(
            input[0].parse::<f64>()?,
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()?,
        ),

        // Intervals Formulas
        "z Interval" => app.get_z_interval(
            input[0].parse::<f64>()?,
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()?,
            input[3].parse::<f64>()? / 100.0,
        ),
        "t Interval" => app.get_t_interval(
            input[0].parse::<f64>()?,
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()? - 1.0,
            input[3].parse::<f64>()? / 100.0,
        ),
        "2-Sample t Interval" => app.get_2_sample_t_interval(
            input[0].parse::<f64>()?,
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()?,
            input[3].parse::<f64>()?,
            input[4].parse::<f64>()?,
            input[5].parse::<f64>()?,
            input[6].parse::<f64>()? / 100.0,
        ),
        "2-Prop z Interval" => app.get_2_proportion_z_interval(
            input[0].parse::<f64>()?,
            input[1].parse::<f64>()?,
            input[2].parse::<f64>()?,
            input[3].parse::<f64>()?,
            input[4].parse::<f64>()? / 100.0,
        ),

        // Tests Formulas
        // "z Test" => app.get_2_proportion_z_interval(x_1, n_1, x_2, n_2, c_level),

        _ => Ok(String::from("No formula found with that name!")),
    }
}

fn match_formula_inputs(formula_name: &str) -> Vec<String> {
    match formula_name {
        // Probability Formulas
        "Factorial (!)" => vec![String::from("n")],
        "Permutations" => vec![String::from("n"), String::from("r")],
        "Combinations" => vec![String::from("n"), String::from("r")],
        "Normal Pdf" => vec![String::from("x"), String::from("µ"), String::from("σ")],
        "Normal Cdf" => vec![
            String::from("Lower Bound"),
            String::from("Upper Bound"),
            String::from("µ"),
            String::from("σ"),
        ],
        "Inverse Normal" => vec![String::from("Area"), String::from("µ"), String::from("σ")],
        "t Pdf" => vec![String::from("x"), String::from("Deg of Freedom, df")],
        "t Cdf" => vec![
            String::from("Lower Bound"),
            String::from("Upper Bound"),
            String::from("Deg of Freedom, df"),
        ],
        "χ2 Pdf" => vec![String::from("x"), String::from("Deg of Freedom, df")],
        "χ2 Cdf" => vec![
            String::from("Lower Bound"),
            String::from("Upper Bound"),
            String::from("Deg of Freedom, df"),
        ],
        "Binomial Pdf" => vec![
            String::from("Num Trials, n"),
            String::from("Prob Success, p"),
            String::from("x"),
        ],
        "Binomial Cdf" => vec![
            String::from("Num Trials, n"),
            String::from("Prob Success, p"),
            String::from("Lower Bound"),
            String::from("Upper Bound"),
        ],
        "F Pdf" => vec![
            String::from("x"),
            String::from("Numerator df"),
            String::from("Denominator df"),
        ],
        "F Cdf" => vec![
            String::from("Lower Bound"),
            String::from("Upper Bound"),
            String::from("Numerator df"),
            String::from("Denominator df"),
        ],
        "Geometric Pdf" => vec![String::from("Prob Success, p"), String::from("x")],
        "Geometric Cdf" => vec![
            String::from("Prob Success, p"),
            String::from("Lower Bound"),
            String::from("Upper Bound"),
        ],
        "Poisson Pdf" => vec![String::from("λ"), String::from("x")],
        "Poisson Cdf" => vec![
            String::from("λ"),
            String::from("Lower Bound"),
            String::from("Upper Bound"),
        ],

        // Intervals Formulas
        "z Interval" => vec![
            String::from("σ"),
            String::from("x̄"),
            String::from("n"),
            String::from("C Level"),
        ],
        "t Interval" => vec![
            String::from("x̄"),
            String::from("σ"),
            String::from("n"),
            String::from("C Level"),
        ],
        "2-Sample t Interval" => vec![
            String::from("x̄1"),
            String::from("σ1"),
            String::from("n1"),
            String::from("x̄2"),
            String::from("σ2"),
            String::from("n2"),
            String::from("C Level"),
        ],
        "2-Prop z Interval" => vec![
            String::from("Successes, x1"),
            String::from("n1"),
            String::from("Successes, x2"),
            String::from("n2"),
            String::from("C Level"),
        ],

        // Tests Formulas
        "z Test" => vec![
            String::from("µ0"),
            String::from("σ"),
            String::from("x̄"),
            String::from("n"),
            String::from("Alternate Hyp"),
        ],
        "t Test" => vec![
            String::from("µ0"),
            String::from("x̄"),
            String::from("σ"),
            String::from("n"),
            String::from("Alternate Hyp"),
        ],
        "2-Sample t Test" => vec![
            String::from("x̄1"),
            String::from("σ1"),
            String::from("n1"),
            String::from("x̄2"),
            String::from("σ2"),
            String::from("n2"),
            String::from("Alternate Hyp"),
            String::from("Pooled"),
        ],
        "2-Prop z Test" => vec![
            String::from("Successes, x1"),
            String::from("n1"),
            String::from("Successes, x2"),
            String::from("n2"),
            String::from("Alternate Hyp"),
        ],
        _ => vec![String::from("No formula found with that name!")],
    }
}
