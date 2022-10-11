use std::{collections::HashSet, fmt};

use crate::models::{
    experience::experience_base::ExperienceBase,
    parser::{statement::Statement, term::Term},
};

use super::meaning::{extension_from_term, intension_from_term};

#[derive(Debug, PartialEq)]
pub struct TruthValue {
    freq: f64,
    conf: f64,
}

impl fmt::Display for TruthValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}>", self.freq, self.conf)
    }
}

impl TruthValue {
    // Create new truth value
    pub fn new() -> Result<TruthValue, String> {
        Ok(TruthValue {
            freq: 1.0,
            conf: 0.99,
        })
    }

    pub fn from_statement<'a>(
        stmt: &Statement,
        exp_base: &ExperienceBase,
    ) -> Result<TruthValue, &'a str> {
        let left = stmt.left.clone();
        let right = stmt.right.clone();

        let k_horizon = 1;

        let positive_evidence = TruthValue::get_positive_evidence(&left, &right, exp_base);
        let negative_evidence = TruthValue::get_negative_evidence(&left, &right, exp_base);

        let total_evidence = positive_evidence + negative_evidence;

        let freq = positive_evidence as f64 / total_evidence as f64;
        let conf = total_evidence as f64 / (total_evidence as f64 + k_horizon as f64);

        Ok(TruthValue { freq, conf })
    }

    fn get_positive_evidence(left: &Term, right: &Term, exp_base: &ExperienceBase) -> usize {
        let left_extension = extension_from_term(left, exp_base);
        let right_extension = extension_from_term(right, exp_base);
        let left_intension = intension_from_term(left, exp_base);
        let right_intension = intension_from_term(right, exp_base);

        let temp1: HashSet<Term> = left_extension
            .intersection(&right_extension)
            .cloned()
            .collect();
        let temp2: HashSet<Term> = left_intension
            .intersection(&right_intension)
            .cloned()
            .collect();
        let positive_evidence = temp1.union(&temp2).count();

        positive_evidence
    }

    fn get_negative_evidence(left: &Term, right: &Term, exp_base: &ExperienceBase) -> usize {
        let left_extension = extension_from_term(left, exp_base);
        let right_extension = extension_from_term(right, exp_base);
        let left_intension = intension_from_term(left, exp_base);
        let right_intension = intension_from_term(right, exp_base);

        let temp1: HashSet<Term> = left_extension
            .difference(&right_extension)
            .cloned()
            .collect();
        let temp2: HashSet<Term> = left_intension
            .difference(&right_intension)
            .cloned()
            .collect();
        let negative_evidence = temp1.union(&temp2).count();

        negative_evidence
    }
}

// tests
#[cfg(test)]
mod tests {
    use crate::models::experience::experience_element::ExperienceElement;

    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            TruthValue::new().unwrap(),
            TruthValue {
                freq: 1.0,
                conf: 0.99
            }
        );
    }

    #[test]
    fn test_stmt_in_knowledge() {
        let mut exp_base = ExperienceBase::new();
        exp_base.add(ExperienceElement::new(
            Statement::new("eagle is bird").unwrap(),
            1,
        ));
        exp_base.add(ExperienceElement::new(
            Statement::new("bird is animal").unwrap(),
            2,
        ));
        exp_base.add(ExperienceElement::new(
            Statement::new("water is liquid").unwrap(),
            3,
        ));
        exp_base.add(ExperienceElement::new(
            Statement::new("milk is liquid").unwrap(),
            4,
        ));
        assert_eq!(exp_base.experiences.len(), 4);

        let query_stmt = Statement::new("bird is liquid").unwrap();

        assert_eq!(
            TruthValue::get_positive_evidence(&query_stmt.left, &query_stmt.right, &exp_base),
            0
        );
        assert_eq!(
            TruthValue::get_negative_evidence(&query_stmt.left, &query_stmt.right, &exp_base),
            3
        );
        assert_eq!(
            TruthValue::from_statement(&query_stmt, &exp_base).unwrap(),
            TruthValue {
                freq: 0.0,
                conf: 0.75
            }
        );
    }
}
