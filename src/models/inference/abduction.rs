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

    if exp1.stmt.copula == Copula::Similarity()
        || exp2.stmt.copula == Copula::Similarity()
        || exp1.stmt.right != exp2.stmt.right
    {
        return Err("Abduction not possible.");
    }

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
}

// tests
#[cfg(test)]
mod tests {

    use super::*;
    use crate::models::{
        experience::experience_element::ExperienceElement, parser::statement::Statement,
        semantics::truth_value::TruthValue,
    };

    #[test]
    fn test_abduction() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("a is b").unwrap(),
            1,
            TruthValue::new_from_str("<0.5, 0.89>").unwrap(),
        );
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("c is b").unwrap(),
            2,
            TruthValue::new_from_str("<0.8, 0.89>").unwrap(),
        );
        let experience_3 = ExperienceElement::new_with_truth_value(
            Statement::new("d is c").unwrap(),
            3,
            TruthValue::new_from_str("<0.9, 0.99>").unwrap(),
        );

        experience_base.add(experience_1);
        experience_base.add(experience_2);
        experience_base.add(experience_3);

        assert_eq!(experience_base.experiences.len(), 3);

        let result = abduction(&experience_base, 1, 2).unwrap();
        assert_eq!(result.0, Statement::new("c is a").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.80, 0.28>")
                .unwrap()
                .to_string()
        );

        let result = abduction(&experience_base, 1, 3);
        assert_eq!(result, Err("Abduction not possible."));
    }
}
