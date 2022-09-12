use super::{copula::Copula, term::Term};

#[derive(Debug, PartialEq)]
pub enum OptionalTerm {
    Term(Term),
    Question,
}

#[derive(Debug, PartialEq)]
pub struct Query {
    pub left: OptionalTerm,
    pub copula: Copula,
    pub right: OptionalTerm,
}

impl Query {
    // Create new statement
    pub fn new(tokens: &[String]) -> Result<Query, &str> {
        if tokens.len() != 3 {
            return Err("Invalid query: Expected <term / ?> <copula> <term / ?>");
        }

        if &tokens[0] == "?" {
            let left = OptionalTerm::Question;
            let copula = Copula::new(&tokens[1])?;
            let right = OptionalTerm::Term(Term::new(&tokens[2])?);

            Ok(Query {
                left,
                copula,
                right,
            })
        } else if &tokens[2] == "?" {
            let left = OptionalTerm::Term(Term::new(&tokens[0])?);
            let copula = Copula::new(&tokens[1])?;
            let right = OptionalTerm::Question;

            Ok(Query {
                left,
                copula,
                right,
            })
        } else {
            let left = OptionalTerm::Term(Term::new(&tokens[0])?);
            let copula = Copula::new(&tokens[1])?;
            let right = OptionalTerm::Term(Term::new(&tokens[2])?);

            Ok(Query {
                left,
                copula,
                right,
            })
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Query {
                left: OptionalTerm::Term(left),
                copula,
                right: OptionalTerm::Term(right),
            } => format!(
                "{} {} {}",
                left.to_string(),
                copula.to_string(),
                right.to_string()
            ),
            Query {
                left: OptionalTerm::Question,
                copula,
                right: OptionalTerm::Term(right),
            } => format!("? {} {}", copula.to_string(), right.to_string()),
            Query {
                left: OptionalTerm::Term(left),
                copula,
                right: OptionalTerm::Question,
            } => format!("{} {} ?", left.to_string(), copula.to_string()),
            _ => panic!("Invalid query"),
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Query::new(&vec!["?".to_string(), "is".to_string(), "b".to_string()]).unwrap(),
            Query {
                left: OptionalTerm::Question,
                copula: Copula::Inheritance(),
                right: OptionalTerm::Term(Term::new("b").unwrap()),
            }
        );
        assert_eq!(
            Query::new(&vec!["a".to_string(), "->".to_string(), "?".to_string()]).unwrap(),
            Query {
                left: OptionalTerm::Term(Term::new("a").unwrap()),
                copula: Copula::Inheritance(),
                right: OptionalTerm::Question,
            }
        );
        assert_eq!(
            Query::new(&vec!["a".to_string(), "->".to_string(), "b".to_string()]).unwrap(),
            Query {
                left: OptionalTerm::Term(Term::new("a").unwrap()),
                copula: Copula::Inheritance(),
                right: OptionalTerm::Term(Term::new("b").unwrap()),
            }
        );

        assert_eq!(
            Query::new(&vec!["?".to_string(), "is".to_string(), "?".to_string()]).is_err(),
            true
        );
        assert_eq!(
            Query::new(&vec!["a".to_string(), "is".to_string()]).is_err(),
            true
        );
        assert_eq!(Query::new(&vec![]).is_err(), true);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            Query {
                left: OptionalTerm::Question,
                copula: Copula::Inheritance(),
                right: OptionalTerm::Term(Term::new("b").unwrap()),
            }
            .to_string(),
            "? -> b"
        );
        assert_eq!(
            Query {
                left: OptionalTerm::Term(Term::new("a").unwrap()),
                copula: Copula::Inheritance(),
                right: OptionalTerm::Question,
            }
            .to_string(),
            "a -> ?"
        );
        assert_eq!(
            Query {
                left: OptionalTerm::Term(Term::new("a").unwrap()),
                copula: Copula::Inheritance(),
                right: OptionalTerm::Term(Term::new("b").unwrap()),
            }
            .to_string(),
            "a -> b"
        );
    }
}
