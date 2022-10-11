use crate::models::{
    experience::experience_base::ExperienceBase, parser::statement::Statement,
    semantics::truth_value::TruthValue,
};

pub fn revision(
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
        let f1 = exp1.truth_value.freq;
        let f2 = exp2.truth_value.freq;
        let c1 = exp1.truth_value.conf;
        let c2 = exp2.truth_value.conf;

        Ok((
            exp1.stmt.clone(),
            TruthValue {
                freq: (f1 * c1 + (1.0 - c2) + f2 * c2 * (1.0 - c1))
                    / (c1 * (1.0 - c2) + f2 * c2 * (1.0 - c1)),
                conf: (c1 * (1.0 - c2) + c2 * (1.0 - c1))
                    / (c1 * (1.0 - c2) + c2 * (1.0 - c1) + (1.0 - c1) * (1.0 - c2)),
            },
        ))
    } else {
        Err("Revision not possible.")
    }
}
