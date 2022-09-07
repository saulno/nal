#[derive(Copy)]
#[derive(Clone)]
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

    pub fn to_string(self) -> String {
        match self {
            Copula::Inheritance() => "->".to_string(),
        }
    }
}