use ordered_float::OrderedFloat;

use crate::models::{
    experience::experience_base::ExperienceBase, parser::statement::Statement,
    semantics::truth_value::TruthValue,
};

pub fn selection(
    experience_base: &ExperienceBase,
    id_exp_1: usize,
    id_exp_2: usize,
) -> Result<(Statement, TruthValue), &str> {
    let exp1 = experience_base
        .experiences
        .iter()
        .find(|exp| exp.id == id_exp_1)
        .ok_or("Experience 1 not found.")?;

    let exp2 = experience_base
        .experiences
        .iter()
        .find(|exp| exp.id == id_exp_2)
        .ok_or("Experience 2 not found.")?;

    if exp1.stmt.left == exp2.stmt.left && exp1.stmt.right == exp2.stmt.right {
        match OrderedFloat(exp1.truth_value.conf).cmp(&OrderedFloat(exp2.truth_value.conf)) {
            std::cmp::Ordering::Greater => Ok((exp1.stmt.clone(), exp1.truth_value.clone())),
            std::cmp::Ordering::Less => Ok((exp2.stmt.clone(), exp2.truth_value.clone())),
            std::cmp::Ordering::Equal => {
                match OrderedFloat(exp1.truth_value.freq).cmp(&OrderedFloat(exp2.truth_value.freq))
                {
                    std::cmp::Ordering::Greater => {
                        Ok((exp1.stmt.clone(), exp1.truth_value.clone()))
                    }
                    std::cmp::Ordering::Less => Ok((exp2.stmt.clone(), exp2.truth_value.clone())),
                    std::cmp::Ordering::Equal => Err("Equal experiences."),
                }
            }
        }
    } else {
        let expectation_1 = exp1.truth_value.conf * (exp1.truth_value.freq - 0.5) + 0.5;
        let expectation_2 = exp2.truth_value.conf * (exp2.truth_value.freq - 0.5) + 0.5;
        match OrderedFloat(expectation_1).cmp(&OrderedFloat(expectation_2)) {
            std::cmp::Ordering::Greater => Ok((exp1.stmt.clone(), exp1.truth_value.clone())),
            std::cmp::Ordering::Less => Ok((exp2.stmt.clone(), exp2.truth_value.clone())),
            std::cmp::Ordering::Equal => Err("Equal expectations."),
        }
    }
}
