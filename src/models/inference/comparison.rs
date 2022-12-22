use crate::models::{
    experience::experience_base::ExperienceBase,
    parser::{copula::Copula, statement::Statement},
    semantics::truth_value::TruthValue,
};

pub fn comparison(
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

    if exp1.stmt.copula != Copula::Inheritance() || exp2.stmt.copula != Copula::Inheritance() {
        return Err("Comparison only possible with inheritance copula.");
    }

    let f1 = exp1.truth_value.freq;
    let f2 = exp2.truth_value.freq;
    let c1 = exp1.truth_value.conf;
    let c2 = exp2.truth_value.conf;
    let k = 1.0;

    if exp1.stmt.left == exp2.stmt.left {
        Ok((
            Statement {
                left: exp2.stmt.right.clone(),
                copula: Copula::Similarity(),
                right: exp1.stmt.right.clone(),
            },
            TruthValue {
                freq: (f1 * f2) / (f1 + f2 - f1 * f2),
                conf: (c1 * c2 * (f1 + f2 - f1 * f2)) / (c1 * c2 * (f1 + f2 - f1 * f2) + k),
            },
        ))
    } else {
        Err("Comparison not possible.")
    }
}

// test
#[cfg(test)]
mod tests {
    use crate::models::{
        experience::{experience_base::ExperienceBase, experience_element::ExperienceElement},
        parser::statement::Statement,
        semantics::truth_value::TruthValue,
    };

    #[test]
    fn test_comparison() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            1,
            TruthValue::new_from_str("<0.5, 0.89>").unwrap(),
        );
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("d is f").unwrap(),
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

        let result = super::comparison(&experience_base, 1, 2);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0, Statement::new("f <-> e").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.44, 0.42>")
                .unwrap()
                .to_string()
        );
    }
}
