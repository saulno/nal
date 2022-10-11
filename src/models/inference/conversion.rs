use crate::models::{
    experience::experience_base::ExperienceBase,
    parser::{copula::Copula, statement::Statement},
    semantics::truth_value::TruthValue,
};

pub fn conversion(
    experience_base: &ExperienceBase,
    id_exp: usize,
) -> Result<(Statement, TruthValue), &str> {
    let exp = experience_base
        .experiences
        .iter()
        .find(|exp| exp.id == id_exp)
        .ok_or("Experience not found.")?;

    let positive_evidence = exp.truth_value.freq * exp.truth_value.conf;
    let negative_evidence = 0.0;
    let k_horizon = 1.0;
    Ok((
        Statement {
            left: exp.stmt.right.clone(),
            copula: Copula::Inheritance(),
            right: exp.stmt.left.clone(),
        },
        TruthValue {
            freq: positive_evidence / (positive_evidence + negative_evidence),
            conf: (positive_evidence + negative_evidence)
                / (positive_evidence + negative_evidence + k_horizon),
        },
    ))
}
