use crate::models::{
    experience::experience_base::ExperienceBase,
    parser::{copula::Copula, statement::Statement},
    semantics::truth_value::TruthValue,
};

pub fn exemplification(
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
    let negative_evidence = 0.0;
    let k_horizon = 1.0;

    if exp1.stmt.copula == Copula::Similarity() || exp2.stmt.copula == Copula::Similarity() {
        return Err("Exemplification not possible.");
    }

    if exp1.stmt.right == exp2.stmt.left {
        Ok((
            Statement {
                left: exp2.stmt.right.clone(),
                copula: Copula::Inheritance(),
                right: exp1.stmt.left.clone(),
            },
            TruthValue {
                freq: positive_evidence / (positive_evidence + negative_evidence),
                conf: (positive_evidence + negative_evidence)
                    / (positive_evidence + negative_evidence + k_horizon),
            },
        ))
    } else if exp1.stmt.left == exp2.stmt.right {
        Ok((
            Statement {
                left: exp1.stmt.right.clone(),
                copula: Copula::Inheritance(),
                right: exp2.stmt.left.clone(),
            },
            TruthValue {
                freq: positive_evidence / (positive_evidence + negative_evidence),
                conf: (positive_evidence + negative_evidence)
                    / (positive_evidence + negative_evidence + k_horizon),
            },
        ))
    } else {
        Err("Exemplification not possible.")
    }
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
    fn test_exemplification() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("a is b").unwrap(),
            1,
            TruthValue::new_from_str("<0.5, 0.89>").unwrap(),
        );
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("a is c").unwrap(),
            2,
            TruthValue::new_from_str("<0.82, 0.69>").unwrap(),
        );
        let experience_3 = ExperienceElement::new_with_truth_value(
            Statement::new("b is c").unwrap(),
            3,
            TruthValue::new_from_str("<0.8, 0.89>").unwrap(),
        );

        experience_base.add(experience_1);
        experience_base.add(experience_2);
        experience_base.add(experience_3);

        assert_eq!(experience_base.experiences.len(), 3);

        let result = exemplification(&experience_base, 1, 3).unwrap();
        assert_eq!(result.0, Statement::new("c is a").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<1.00, 0.24>")
                .unwrap()
                .to_string()
        );

        let result = exemplification(&experience_base, 3, 1).unwrap();
        assert_eq!(result.0, Statement::new("c is a").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<1.00, 0.24>")
                .unwrap()
                .to_string()
        );

        let result = exemplification(&experience_base, 1, 2);
        assert_eq!(result, Err("Exemplification not possible."));
    }
}
