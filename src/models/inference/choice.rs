use ordered_float::OrderedFloat;

use crate::models::{
    experience::experience_base::ExperienceBase, parser::statement::Statement,
    semantics::truth_value::TruthValue,
};

pub fn choice(
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
        match OrderedFloat(exp1.truth_value.conf).cmp(&OrderedFloat(exp2.truth_value.conf)) {
            std::cmp::Ordering::Greater => Ok((exp1.stmt.clone(), exp1.truth_value.clone())),
            std::cmp::Ordering::Less => Ok((exp2.stmt.clone(), exp2.truth_value.clone())),
            std::cmp::Ordering::Equal => {
                match OrderedFloat(exp1.truth_value.freq).cmp(&OrderedFloat(exp2.truth_value.freq))
                {
                    std::cmp::Ordering::Greater => {
                        Ok((exp1.stmt.clone(), exp1.truth_value.clone()))
                    }
                    std::cmp::Ordering::Less => Ok((exp2.stmt.clone(), exp2.truth_value.clone())),
                    std::cmp::Ordering::Equal => Err("Equal experiences."),
                }
            }
        }
    } else {
        let expectation_1 = exp1.truth_value.conf * (exp1.truth_value.freq - 0.5) + 0.5;
        let expectation_2 = exp2.truth_value.conf * (exp2.truth_value.freq - 0.5) + 0.5;
        match OrderedFloat(expectation_1).cmp(&OrderedFloat(expectation_2)) {
            std::cmp::Ordering::Greater => Ok((exp1.stmt.clone(), exp1.truth_value.clone())),
            std::cmp::Ordering::Less => Ok((exp2.stmt.clone(), exp2.truth_value.clone())),
            std::cmp::Ordering::Equal => Err("Equal expectations."),
        }
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        experience::{experience_base::ExperienceBase, experience_element::ExperienceElement},
        parser::statement::Statement,
        semantics::truth_value::TruthValue,
    };

    #[test]
    fn test_choice() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            1,
            TruthValue::new_from_str("<0.5, 0.89>").unwrap(),
        );
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            2,
            TruthValue::new_from_str("<0.8, 0.89>").unwrap(),
        );
        let experience_3 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            3,
            TruthValue::new_from_str("<0.3, 0.95>").unwrap(),
        );
        let experience_4 = ExperienceElement::new_with_truth_value(
            Statement::new("a is b").unwrap(),
            4,
            TruthValue::new_from_str("<0.9, 0.99>").unwrap(),
        );

        experience_base.add(experience_1);
        experience_base.add(experience_2);
        experience_base.add(experience_3);
        experience_base.add(experience_4);

        assert_eq!(experience_base.experiences.len(), 4);

        let result = choice(&experience_base, 1, 2).unwrap();
        assert_eq!(result.0, Statement::new("d is e").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.8, 0.89>").unwrap().to_string()
        );

        let result = choice(&experience_base, 1, 3).unwrap();
        assert_eq!(result.0, Statement::new("d is e").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.3, 0.95>").unwrap().to_string()
        );

        let result = choice(&experience_base, 1, 4).unwrap();
        assert_eq!(result.0, Statement::new("a is b").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.9, 0.99>").unwrap().to_string()
        );
    }
}
