use super::{term::Term, copula::Copula};

pub enum OptionalTerm {
    Term(Term),
    Question,
}
pub struct Query {
    pub left: OptionalTerm,
    pub copula: Copula,
    pub right: OptionalTerm,
}

impl Query {
    // Create new statement
    pub fn new(tokens: &[String]) -> Result<Query, &str> {

        if tokens.len() != 3 {
            return Err("Invalid statement <term> <copula> <term>");
        }

        if &tokens[0] == "?" {
            if let Err(e) = Term::new(&tokens[2]) {
                return Err(e);
            }
            if let Err(e) = Copula::new(&tokens[1]) {
                return Err(e);
            }
            let left = OptionalTerm::Question;
            let copula = Copula::new(&tokens[1]).unwrap();
            let right = OptionalTerm::Term(Term::new(&tokens[2]).unwrap());

            Ok(Query {
                left,
                copula,
                right,
            })
        } else if &tokens[2] == "?" {
            if let Err(e) = Term::new(&tokens[0]) {
                return Err(e);
            }
            if let Err(e) = Copula::new(&tokens[1]) {
                return Err(e);
            }
            let left = OptionalTerm::Term(Term::new(&tokens[0]).unwrap());
            let copula = Copula::new(&tokens[1]).unwrap();
            let right = OptionalTerm::Question;

            Ok(Query {
                left,
                copula,
                right,
            })
        } else {
            Err("Invalid query")
        }
    }
}