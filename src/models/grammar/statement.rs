use super::{copula::Copula, term::Term};

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub left: Term,
    pub copula: Copula,
    pub right: Term,
}

impl Statement {
    // Create new statement
    pub fn new(tokens: &[String]) -> Result<Statement, &str> {
        if tokens.len() != 3 {
            return Err("Invalid statement: Expected <term> <copula> <term>");
        }

        let left = Term::new(&tokens[0])?;
        let copula = Copula::new(&tokens[1])?;
        let right = Term::new(&tokens[2])?;

        Ok(Statement {
            left,
            copula,
            right,
        })
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.left.to_string(),
            self.copula.to_string(),
            self.right.to_string()
        )
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Statement::new(&vec!["a".to_string(), "is".to_string(), "b".to_string()]).unwrap(),
            Statement {
                left: Term::new("a").unwrap(),
                copula: Copula::Inheritance(),
                right: Term::new("b").unwrap(),
            }
        );
        assert_eq!(
            Statement::new(&vec!["a".to_string(), "->".to_string(), "b".to_string()]).unwrap(),
            Statement {
                left: Term::new("a").unwrap(),
                copula: Copula::Inheritance(),
                right: Term::new("b").unwrap(),
            }
        );
        assert_eq!(
            Statement::new(&vec![
                "a".to_string(),
                "is".to_string(),
                "b".to_string(),
                "c".to_string()
            ])
            .is_err(),
            true
        );
        assert_eq!(
            Statement::new(&vec![
                "a".to_string(),
                "is not".to_string(),
                "b".to_string()
            ])
            .is_err(),
            true
        );
        assert_eq!(
            Statement::new(&vec!["a".to_string(), "is".to_string()]).is_err(),
            true
        );
        assert_eq!(Statement::new(&vec![]).is_err(), true);
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
