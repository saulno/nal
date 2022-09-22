use std::fmt;

use super::{copula::Copula, term::Term};

#[derive(Debug, PartialEq, Eq)]
pub struct Statement {
    pub left: Term,
    pub copula: Copula,
    pub right: Term,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.copula, self.right)
    }
}

impl Statement {
    // Create new statement
    pub fn new(s: &str) -> Result<Statement, &str> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.len() != 3 {
            return Err("Invalid statement: Expected <term> <copula> <term>");
        }

        let left = Term::new(tokens[0])?;
        let copula = Copula::new(tokens[1])?;
        let right = Term::new(tokens[2])?;

        Ok(Statement {
            left,
            copula,
            right,
        })
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Statement::new("a is b").unwrap(),
            Statement {
                left: Term::new("a").unwrap(),
                copula: Copula::Inheritance(),
                right: Term::new("b").unwrap(),
            }
        );
        assert_eq!(
            Statement::new("a -> b").unwrap(),
            Statement {
                left: Term::new("a").unwrap(),
                copula: Copula::Inheritance(),
                right: Term::new("b").unwrap(),
            }
        );
        assert_eq!(Statement::new("a is b c").is_err(), true);
        assert_eq!(Statement::new("a is not b").is_err(), true);
        assert_eq!(Statement::new("a is").is_err(), true);
        assert_eq!(Statement::new("").is_err(), true);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            Statement {
                left: Term::new("a").unwrap(),
                copula: Copula::Inheritance(),
                right: Term::new("b").unwrap(),
            }
            .to_string(),
            "a -> b".to_string()
        );
    }
}
