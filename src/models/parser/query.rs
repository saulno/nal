use std::fmt;

use super::{copula::Copula, term::Term};

#[derive(Debug, PartialEq, Eq)]
pub enum OptionalTerm {
    Term(Term),
    Question,
}

impl fmt::Display for OptionalTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OptionalTerm::Term(term) => write!(f, "{}", term),
            OptionalTerm::Question => write!(f, "?"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Query {
    pub left: OptionalTerm,
    pub copula: Copula,
    pub right: OptionalTerm,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.copula, self.right)
    }
}

impl Query {
    // Create new statement
    pub fn new(s: &str) -> Result<Query, &str> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.len() != 3 {
            return Err("Invalid query: Expected <term / ?> <copula> <term / ?>");
        }

        if tokens[0] == "?" {
            let left = OptionalTerm::Question;
            let copula = Copula::new(tokens[1])?;
            let right = OptionalTerm::Term(Term::new(tokens[2])?);

            Ok(Query {
                left,
                copula,
                right,
            })
        } else if tokens[2] == "?" {
            let left = OptionalTerm::Term(Term::new(tokens[0])?);
            let copula = Copula::new(tokens[1])?;
            let right = OptionalTerm::Question;

            Ok(Query {
                left,
                copula,
                right,
            })
        } else {
            let left = OptionalTerm::Term(Term::new(tokens[0])?);
            let copula = Copula::new(tokens[1])?;
            let right = OptionalTerm::Term(Term::new(tokens[2])?);

            Ok(Query {
                left,
                copula,
                right,
            })
        }
    }

    pub fn from_vec(tokens: &[String]) -> Result<Query, &str> {
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
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Query::new("? is b").unwrap(),
            Query {
                left: OptionalTerm::Question,
                copula: Copula::Inheritance(),
                right: OptionalTerm::Term(Term::new("b").unwrap()),
            }
        );
        assert_eq!(
            Query::new("a -> ?").unwrap(),
            Query {
                left: OptionalTerm::Term(Term::new("a").unwrap()),
                copula: Copula::Inheritance(),
                right: OptionalTerm::Question,
            }
        );
        assert_eq!(
            Query::new("a -> b").unwrap(),
            Query {
                left: OptionalTerm::Term(Term::new("a").unwrap()),
                copula: Copula::Inheritance(),
                right: OptionalTerm::Term(Term::new("b").unwrap()),
            }
        );

        assert_eq!(Query::new("? is ?").is_err(), true);
        assert_eq!(Query::new("a is").is_err(), true);
        assert_eq!(Query::new("").is_err(), true);
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
