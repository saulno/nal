use crate::models::{
    experience::experience_base::ExperienceBase,
    parser::{copula::Copula, statement::Statement, term::Term},
    semantics::truth_value::TruthValue,
};

pub fn union_extension(
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

    if exp1.stmt.copula == Copula::Similarity() || exp2.stmt.copula == Copula::Similarity() {
        return Err("Union_extension not possible.");
    }

    if exp1.stmt.right == exp2.stmt.right {
        let new_term = format!("({}|{})", exp1.stmt.left, exp2.stmt.left);
        Ok((
            Statement {
                left: Term { word: new_term },
                copula: Copula::Inheritance(),
                right: exp2.stmt.right.clone(),
            },
            TruthValue {
                freq: exp1.truth_value.freq * exp2.truth_value.freq,
                conf: exp1.truth_value.conf * exp2.truth_value.conf,
            },
        ))
    } else {
        Err("Union_extension not possible.")
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::experience::experience_element::ExperienceElement;

    #[test]
    fn test_union_extension() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            1,
            TruthValue::new_from_str("<0.5, 0.89>").unwrap(),
        );
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("f is e").unwrap(),
            2,
            TruthValue::new_from_str("<0.8, 0.89>").unwrap(),
        );
        let experience_3 = ExperienceElement::new_with_truth_value(
            Statement::new("a is b").unwrap(),
            3,
            TruthValue::new_from_str("<0.9, 0.99>").unwrap(),
        );

        experience_base.add(experience_1);
        experience_base.add(experience_2);
        experience_base.add(experience_3);

        assert_eq!(experience_base.experiences.len(), 3);

        let result = union_extension(&experience_base, 1, 2).unwrap();
        assert_eq!(result.0, Statement::new("(d|f) is e").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.40, 0.79>")
                .unwrap()
                .to_string()
        );

        let result = union_extension(&experience_base, 1, 3);
        assert_eq!(result, Err("Union_extension not possible."));
    }
}
