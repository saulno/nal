#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Copula {
    Inheritance(),
}

impl Copula {
    // Create new copula
    pub fn new(symbols: &str) -> Result<Copula, &str> {
        match symbols {
            "is" | "->" => Ok(Copula::Inheritance()),
            _ => Err("Invalid copula"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Copula::Inheritance() => "->".to_string(),
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
        assert_eq!(Copula::new("is not").is_err(), true);
        assert_eq!(Copula::new("is not a").is_err(), true);
        assert_eq!(Copula::new("is not a copula").is_err(), true);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Copula::Inheritance().to_string(), "->".to_string());
    }
}
