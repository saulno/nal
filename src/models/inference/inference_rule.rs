use crate::models::{
    experience::experience_base::ExperienceBase,
    grammar::{copula::Copula, statement::Statement},
};

fn transitivity(
    experience_base: &ExperienceBase,
    id_exp_1: usize,
    id_exp_2: usize,
) -> Result<Statement, &str> {
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

    if exp1.stmt.right == exp2.stmt.left {
        Ok(Statement {
            left: exp1.stmt.left.clone(),
            copula: Copula::Inheritance(),
            right: exp2.stmt.right.clone(),
        })
    } else if exp2.stmt.right == exp1.stmt.left {
        Ok(Statement {
            left: exp2.stmt.left.clone(),
            copula: Copula::Inheritance(),
            right: exp1.stmt.right.clone(),
        })
    } else {
        Err("Transitivity not possible.")
    }
}

pub fn print_transitivity(
    experience_base: &ExperienceBase,
    id_exp_1: usize,
    id_exp_2: usize,
) -> Result<String, &str> {
    let result = transitivity(experience_base, id_exp_1, id_exp_2)?;
    let exp1 = experience_base
        .experiences
        .iter()
        .find(|experience| experience.id == id_exp_1)
        .ok_or("First id not found in experience base.")?;
    let exp2 = experience_base
        .experiences
        .iter()
        .find(|experience| experience.id == id_exp_2)
        .ok_or("Second id not found in experience base.")?;
    Ok(format!(
        "  {}: {}\n  {}: {}\n  RESULT: {}",
        id_exp_1, exp1.stmt, id_exp_2, exp2.stmt, result
    ))
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        experience::experience_element::Experience,
        grammar::{copula::Copula, statement::Statement, term::Term},
    };

    #[test]
    fn test_experience_base_new() {
        let experience_base = ExperienceBase::new();
        assert_eq!(experience_base.experiences.len(), 0);
    }

    #[test]
    fn test_transitivity() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = Experience::new(
            Statement {
                left: Term::new("a").unwrap(),
                copula: Copula::new("is").unwrap(),
                right: Term::new("b").unwrap(),
            },
            1,
        );
        let experience_2 = Experience::new(
            Statement {
                left: Term::new("b").unwrap(),
                copula: Copula::new("is").unwrap(),
                right: Term::new("c").unwrap(),
            },
            2,
        );

        let experience_3 = Experience::new(
            Statement {
                left: Term::new("d").unwrap(),
                copula: Copula::new("is").unwrap(),
                right: Term::new("e").unwrap(),
            },
            3,
        );

        experience_base.add(experience_1);
        experience_base.add(experience_2);
        experience_base.add(experience_3);

        assert_eq!(experience_base.experiences.len(), 3);

        let result = transitivity(&experience_base, 1, 2).unwrap();
        assert_eq!(result.to_string(), "a -> c".to_string());

        let result = transitivity(&experience_base, 2, 1).unwrap();
        assert_eq!(result.to_string(), "a -> c".to_string());

        let result = transitivity(&experience_base, 1, 3);
        assert_eq!(result, Err("Transitivity not possible."));
    }
}
