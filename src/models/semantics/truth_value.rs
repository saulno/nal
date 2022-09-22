use std::fmt;

use crate::models::{experience::experience_base::ExperienceBase, parser::statement::Statement};

use super::meaning::intension_from_term;

#[derive(Debug, PartialEq, Eq)]
pub enum TruthValue {
    True,
    False,
    Unknown,
}

impl fmt::Display for TruthValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TruthValue::True => write!(f, "true"),
            TruthValue::False => write!(f, "false"),
            TruthValue::Unknown => write!(f, "unknown"),
        }
    }
}

impl TruthValue {
    // Create new truth value
    pub fn new(symbols: &str) -> Result<TruthValue, &str> {
        match symbols {
            "true" => Ok(TruthValue::True),
            "false" => Ok(TruthValue::False),
            "unknown" => Ok(TruthValue::Unknown),
            _ => Err("Invalid truth value"),
        }
    }

    pub fn from_statement<'a>(
        stmt: &Statement,
        exp_base: &ExperienceBase,
    ) -> Result<TruthValue, &'a str> {
        let left = stmt.left.clone();
        let right = stmt.right.clone();
        if intension_from_term(&left, exp_base).contains(&right) {
            Ok(TruthValue::True)
        } else {
            Ok(TruthValue::False)
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use crate::models::experience::experience_element::ExperienceElement;

    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(TruthValue::new("true").unwrap(), TruthValue::True);
        assert_eq!(TruthValue::new("false").unwrap(), TruthValue::False);
        assert_eq!(TruthValue::new("unknown").unwrap(), TruthValue::Unknown);
        assert_eq!(TruthValue::new("something else").is_err(), true);
    }

    #[test]
    fn test_stmt_in_knowledge() {
        let mut exp_base = ExperienceBase::new();
        exp_base.add(ExperienceElement::new(Statement::new("a is b").unwrap(), 1));
        exp_base.add(ExperienceElement::new(Statement::new("b is c").unwrap(), 1));
        assert_eq!(exp_base.experiences.len(), 2);

        assert_eq!(
            TruthValue::from_statement(&Statement::new("a is b").unwrap(), &exp_base).unwrap(),
            TruthValue::True
        );
        assert_eq!(
            TruthValue::from_statement(&Statement::new("b is c").unwrap(), &exp_base).unwrap(),
            TruthValue::True
        );
        assert_eq!(
            TruthValue::from_statement(&Statement::new("a is c").unwrap(), &exp_base).unwrap(),
            TruthValue::True
        );

        assert_eq!(
            TruthValue::from_statement(&Statement::new("a is d").unwrap(), &exp_base).unwrap(),
            TruthValue::False
        );
        assert_eq!(
            TruthValue::from_statement(&Statement::new("b is a").unwrap(), &exp_base).unwrap(),
            TruthValue::False
        );
    }
}
