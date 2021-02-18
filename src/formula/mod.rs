use crate::app::App;
use anyhow::Result;
use probability::factorial;

pub mod distributions;
pub mod intervals;
pub mod probability;
pub mod regressions;

pub fn retrieve_formula(formula_name: &str) -> Vec<String> {
    match_formula_inputs(formula_name)
}

pub fn attempt_formula(formula_name: &str, input: Vec<String>, app: &mut App) -> Result<String> {
    if input.is_empty() {
        Ok(String::from("All inputs are not filled yet."))
    } else {
        let formula = match_formula_equations(formula_name, input);
        //app.current_input().drain(..);
        //app.current_entered_input().drain(..);
        formula
    }
}

fn match_formula_equations(formula_name: &str, input: Vec<String>) -> Result<String> {
    match formula_name {
        // Probability Formulas
        "Factorial (!)" => Ok(format!("{}", "okay")),
        "Permutations" => Ok(String::from("thats cool")),
        "Combinations" => Ok(String::from("ig idk")),
        "Normal Pdf" => Ok(String::from("empty")),

        // Intervals Formulas

        // Tests Formulas

        // Regressions Formulas
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
        "Inverse t" => vec![String::from("Area"), String::from("Deg of Freedom, df")],
        "χ2 Pdf" => vec![String::from("x"), String::from("Deg of Freedom, df")],
        "χ2 Cdf" => vec![
            String::from("Lower Bound"),
            String::from("Upper Bound"),
            String::from("Deg of Freedom, df"),
        ],
        "Inverse χ2" => vec![String::from("Area"), String::from("Deg of Freedom, df")],
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
        "Inverse Binomial" => vec![
            String::from("Cumulative Prob"),
            String::from("Num Trials, n"),
            String::from("Prob Success, p"),
        ],
        "Inverse Binomial N" => vec![
            String::from("Cumulative Prob"),
            String::from("Prob Success, p"),
            String::from("Successes, x"),
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
        "Inverse F" => vec![
            String::from("Area"),
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
            String::from("Sx"),
            String::from("n"),
            String::from("C Level"),
        ],
        "2-Sample z Interval" => vec![
            String::from("σ1"),
            String::from("σ2"),
            String::from("x̄1"),
            String::from("n1"),
            String::from("x̄2"),
            String::from("n2"),
            String::from("C Level"),
        ],
        "2-Sample t Interval" => vec![
            String::from("x̄1"),
            String::from("Sx1"),
            String::from("n1"),
            String::from("x̄2"),
            String::from("Sx2"),
            String::from("n2"),
            String::from("C Level"),
        ],
        "1-Prop z Interval" => vec![
            String::from("Successes, x"),
            String::from("n"),
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
            String::from("Sx"),
            String::from("n"),
            String::from("Alternate Hyp"),
        ],
        "2-Sample z Test" => vec![
            String::from("µ0"),
            String::from("x̄"),
            String::from("Sx"),
            String::from("n"),
            String::from("Alternate Hyp"),
        ],
        "2-Sample t Test" => vec![
            String::from("x̄1"),
            String::from("Sx1"),
            String::from("n1"),
            String::from("x̄2"),
            String::from("Sx2"),
            String::from("n2"),
            String::from("Alternate Hyp"),
            String::from("Pooled"),
        ],
        "1-Prop z Test" => vec![
            String::from("P0"),
            String::from("Successes, x"),
            String::from("n"),
            String::from("Alternate Hyp"),
        ],
        "2-Prop z Test" => vec![
            String::from("Successes, x1"),
            String::from("n1"),
            String::from("Successes, x2"),
            String::from("n2"),
            String::from("Alternate Hyp"),
        ],
        "χ2 GOF" => vec![
            String::from("Observed List"),
            String::from("Expected List"),
            String::from("Deg of Freedom, df"),
        ],
        "χ2 2-way Test" => vec![String::from("Observed Matrix")],
        "2-Sample F Test" => vec![
            String::from("Sx1"),
            String::from("n1"),
            String::from("Sx2"),
            String::from("n2"),
            String::from("Alternate Hyp"),
        ],
        "ANOVA" => vec![
            String::from("Number of Groups"),
            String::from("Group - {n}"),
            String::from("Group - {x̄}"),
            String::from("Group - Sx"),
        ],

        // Regressions Formulas
        "Linear Regression (mx+b)" => vec![String::from("X List"), String::from("Y List")],
        "Linear Regression (a+bx)" => vec![String::from("X List"), String::from("Y List")],
        "Median-Median Line" => vec![String::from("X List"), String::from("Y List")],
        "Quadratic Regression" => vec![String::from("X List"), String::from("Y List")],
        "Cubic Regression" => vec![String::from("X List"), String::from("Y List")],
        "Quartic Regression" => vec![String::from("X List"), String::from("Y List")],

        _ => vec![String::from("No formula found with that name!")],
    }
}
