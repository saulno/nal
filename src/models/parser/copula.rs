use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Copula {
    Inheritance(),
    Similarity(),
}

impl fmt::Display for Copula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Copula::Inheritance() => write!(f, "->"),
            Copula::Similarity() => write!(f, "<->"),
        }
    }
}

impl Copula {
    // Create new copula
    pub fn new(symbols: &str) -> Result<Copula, &str> {
        match symbols {
            "is" | "->" => Ok(Copula::Inheritance()),
            "similar" | "<->" => Ok(Copula::Similarity()),
            _ => Err("Invalid copula"),
        }
    }
}

// write tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Copula::new("is").unwrap(), Copula::Inheritance());
        assert_eq!(Copula::new("->").unwrap(), Copula::Inheritance());
        assert_eq!(Copula::new("similar").unwrap(), Copula::Similarity());
        assert_eq!(Copula::new("<->").unwrap(), Copula::Similarity());
        assert_eq!(Copula::new("is not").is_err(), true);
        assert_eq!(Copula::new("is not a").is_err(), true);
        assert_eq!(Copula::new("is not a copula").is_err(), true);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Copula::Inheritance().to_string(), "->".to_string());
    }
}
