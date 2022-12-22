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

    if exp.stmt.copula == Copula::Similarity() {
        return Err("Conversion not possible.");
    }

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

// tests
#[cfg(test)]
mod tests {

    use super::*;
    use crate::models::{
        experience::experience_element::ExperienceElement, parser::statement::Statement,
        semantics::truth_value::TruthValue,
    };

    #[test]
    fn test_conversion() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("a is b").unwrap(),
            1,
            TruthValue::new_from_str("<0.6, 0.90>").unwrap(),
        );

        experience_base.add(experience_1);

        assert_eq!(experience_base.experiences.len(), 1);

        let result = conversion(&experience_base, 1).unwrap();
        assert_eq!(result.0, Statement::new("b is a").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<1.00, 0.35>")
                .unwrap()
                .to_string()
        );
    }
}
