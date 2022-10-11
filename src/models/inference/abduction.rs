use crate::models::{
    experience::experience_base::ExperienceBase,
    parser::{copula::Copula, statement::Statement},
    semantics::truth_value::TruthValue,
};

pub fn abduction(
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

    let positive_evidence = exp1.truth_value.freq
        * exp2.truth_value.freq
        * exp1.truth_value.conf
        * exp2.truth_value.conf;
    let negative_evidence = (1.0 - exp2.truth_value.freq)
        * exp1.truth_value.freq
        * exp1.truth_value.conf
        * exp2.truth_value.conf;
    let k_horizon = 1.0;
    if exp1.stmt.right == exp2.stmt.right {
        Ok((
            Statement {
                left: exp2.stmt.left.clone(),
                copula: Copula::Inheritance(),
                right: exp1.stmt.left.clone(),
            },
            TruthValue {
                freq: positive_evidence / (positive_evidence + negative_evidence),
                conf: (positive_evidence + negative_evidence)
                    / (positive_evidence + negative_evidence + k_horizon),
            },
        ))
    } else {
        Err("Abduction not possible.")
    }
}
