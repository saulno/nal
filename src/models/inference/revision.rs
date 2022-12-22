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
                freq: (f1 * c1 * (1.0 - c2) + f2 * c2 * (1.0 - c1))
                    / (c1 * (1.0 - c2) + c2 * (1.0 - c1)),
                conf: (c1 * (1.0 - c2) + c2 * (1.0 - c1))
                    / (c1 * (1.0 - c2) + c2 * (1.0 - c1) + (1.0 - c1) * (1.0 - c2)),
            },
        ))
    } else {
        Err("Revision not possible.")
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
    fn test_revision() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            1,
            TruthValue::new_from_str("<0.89, 0.9>").unwrap(),
        );
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            2,
            TruthValue::new_from_str("<0.80, 0.95>").unwrap(),
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

        let result = super::revision(&experience_base, 1, 2).unwrap();
        assert_eq!(result.0, Statement::new("d is e").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.83, 0.97>")
                .unwrap()
                .to_string()
        );

        let result = super::revision(&experience_base, 1, 3);
        assert_eq!(result, Err("Revision not possible."));
    }
}
