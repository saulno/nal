#[derive(Clone, Debug, PartialEq)]
pub struct Term {
    pub word: String,
}

impl Term {
    // Create new term
    pub fn new(word: &str) -> Result<Term, &str> {
        if word.contains(" ") {
            return Err("Term can't contain whitespaces");
        }

        if word.eq("") {
            return Err("Term can't be empty");
        }

        if word.eq("?") {
            return Err("Term can't be a question mark (?)");
        }

        Ok(Term {
            word: String::from(word),
        })
    }

    pub fn to_string(&self) -> String {
        self.word.clone()
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Term::new("").is_err(), true);
        assert_eq!(Term::new("is not a term").is_err(), true);
        assert_eq!(Term::new("?").is_err(), true);
        assert_eq!(Term::new("bird").unwrap().word, "bird".to_string());
        assert_eq!(Term::new("water").unwrap().to_string(), "water".to_string());
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Term::new("bird").unwrap().to_string(), "bird".to_string());
        assert_eq!(Term::new("water").unwrap().to_string(), "water".to_string());
    }
}
