use super::{term::Term, copula::Copula};

pub struct Statement {
    pub left: Term,
    pub copula: Copula,
    pub right: Term,
}

impl Statement {
    // Create new statement
    pub fn new(tokens: &[String]) -> Result<Statement, &str> {

        if tokens.len() != 3 {
            return Err("Invalid statement <term> <copula> <term>");
        }
        
        if let Err(e) = Term::new(&tokens[0]) {
            return Err(e);
        } 
        if let Err(e) = Copula::new(&tokens[1]) {
            return Err(e);
        } 
        if let Err(e) = Term::new(&tokens[2]) {
            return Err(e);
        } 
        let left: Term = Term::new(&tokens[0]).unwrap();
        let copula = Copula::new(&tokens[1]).unwrap();
        let right = Term::new(&tokens[2]).unwrap();
        

        Ok(Statement {
            left,
            copula,
            right,
        })
    }
}