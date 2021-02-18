use crate::app::App;
use anyhow::Result;
use probability::factorial;

pub mod distributions;
pub mod intervals;
pub mod probability;
pub mod regressions;

pub fn retrieve_formula(formula_name: &str) -> Vec<String> {
    //match_formulaᵢnputs(formulaₙame)
    Vec::<String>::new()
}

pub fn attempt_formula(formula_name: &str, input: Vec<String>, app: &mut App) -> Result<String> {
    if input.is_empty() {
        Ok(String::from("All inputs are not filled yet."))
    } else {
        let formula = match_formulaₑquations(formula_name, input);
        app.current_input().drain(..);
        app.current_entered_input().drain(..);
        formula
    }
}

fn match_formulaₑquations(formula_name: &str, input: Vec<String>) -> Result<String> {
    match formula_name {
        // Probability Formulas
        "Factorial (!)" => factorial(input[0].parse()?),
        "Permutations" => String::from("𝑛𝑃𝑟 = 𝑛!⁄(𝑛 − 𝑘)!"),
        "Combinations" => String::from("𝑛𝐶𝑟 = 𝑛!⁄𝑟!(𝑛 − 𝑟)!"),
        "Normal Pdf" => String::from(""),

        // Intervals Formulas

        // Tests Formulas

        // Regressions Formulas
        _ => String::from("No formula found with that name!"),
    }
}

fn match_formulaᵢnputs(formula_name: &str) -> String {
    match formula_name {
        // Probability Formulas
        "Factorial (!)" => String::from("𝑛𝑃𝑟 = 𝑛!⁄(𝑛 − 𝑘)!"),
        "Permutations" => String::from("𝑛𝑃𝑟 = 𝑛!⁄(𝑛 − 𝑘)!"),
        "Combinations" => String::from("𝑛𝐶𝑟 = 𝑛!⁄𝑟!(𝑛 − 𝑟)!"),
        "Normal Pdf" => String::from(""),

        // Intervals Formulas

        // Tests Formulas

        // Regressions Formulas
        _ => String::from("No formula found with that name!"),
    }
}
