use crate::models::{
    experience::experience_base::ExperienceBase,
    parser::{copula::Copula, statement::Statement},
    semantics::truth_value::TruthValue,
};

pub fn analogy(
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

    if exp1.stmt.copula == Copula::Inheritance() && exp2.stmt.copula == Copula::Similarity() {
        let f1 = exp1.truth_value.freq;
        let f2 = exp2.truth_value.freq;
        let c1 = exp1.truth_value.conf;
        let c2 = exp2.truth_value.conf;

        if exp1.stmt.left == exp2.stmt.left {
            Ok((
                Statement {
                    left: exp2.stmt.right.clone(),
                    copula: Copula::Inheritance(),
                    right: exp1.stmt.right.clone(),
                },
                TruthValue {
                    freq: f1 * f2,
                    conf: f2 * c1 * c2,
                },
            ))
        } else if exp1.stmt.left == exp2.stmt.right {
            Ok((
                Statement {
                    left: exp2.stmt.left.clone(),
                    copula: Copula::Similarity(),
                    right: exp1.stmt.right.clone(),
                },
                TruthValue {
                    freq: f1 * f2,
                    conf: f2 * c1 * c2,
                },
            ))
        } else {
            Err("Analogy not possible.")
        }
    } else if exp1.stmt.copula == Copula::Similarity() && exp2.stmt.copula == Copula::Inheritance()
    {
        let f1 = exp2.truth_value.freq;
        let f2 = exp1.truth_value.freq;
        let c1 = exp2.truth_value.conf;
        let c2 = exp1.truth_value.conf;

        if exp2.stmt.left == exp1.stmt.left {
            Ok((
                Statement {
                    left: exp1.stmt.right.clone(),
                    copula: Copula::Similarity(),
                    right: exp2.stmt.right.clone(),
                },
                TruthValue {
                    freq: f1 * f2,
                    conf: f2 * c1 * c2,
                },
            ))
        } else if exp2.stmt.left == exp1.stmt.right {
            Ok((
                Statement {
                    left: exp1.stmt.left.clone(),
                    copula: Copula::Similarity(),
                    right: exp2.stmt.right.clone(),
                },
                TruthValue {
                    freq: f1 * f2,
                    conf: f2 * c1 * c2,
                },
            ))
        } else {
            Err("Analogy not possible.")
        }
    } else {
        Err("Analogy not possible.")
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
    fn test_analogy() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            1,
            TruthValue::new_from_str("<0.5, 0.89>").unwrap(),
        );
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("d <-> f").unwrap(),
            2,
            TruthValue::new_from_str("<0.8, 0.89>").unwrap(),
        );

        experience_base.add(experience_1);
        experience_base.add(experience_2);

        assert_eq!(experience_base.experiences.len(), 2);

        let result = super::analogy(&experience_base, 1, 2);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0, Statement::new("f -> e").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.40, 0.63>")
                .unwrap()
                .to_string()
        );
    }
}
