use crate::models::{
    experience::experience_base::ExperienceBase,
    parser::{copula::Copula, statement::Statement},
    semantics::truth_value::TruthValue,
};

pub fn deduction(
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

    if exp1.stmt.right == exp2.stmt.left {
        Ok((
            Statement {
                left: exp1.stmt.left.clone(),
                copula: Copula::Inheritance(),
                right: exp2.stmt.right.clone(),
            },
            TruthValue {
                freq: exp1.truth_value.freq * exp2.truth_value.freq,
                conf: exp1.truth_value.conf
                    * exp2.truth_value.conf
                    * exp1.truth_value.freq
                    * exp2.truth_value.freq,
            },
        ))
    } else if exp2.stmt.right == exp1.stmt.left {
        Ok((
            Statement {
                left: exp2.stmt.left.clone(),
                copula: Copula::Inheritance(),
                right: exp1.stmt.right.clone(),
            },
            TruthValue {
                freq: exp1.truth_value.freq * exp2.truth_value.freq,
                conf: exp1.truth_value.conf
                    * exp2.truth_value.conf
                    * exp1.truth_value.freq
                    * exp2.truth_value.freq,
            },
        ))
    } else {
        Err("Deduction not possible.")
    }
}
