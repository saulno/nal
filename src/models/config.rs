// Enum with the different options to run

#[derive(Debug, PartialEq, Eq)]
pub enum Config {
    Help(),
    Repl(),
}

impl Config {
    // Create new config
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        match args[1].as_str() {
            "help" | "-h" | "--help" => Ok(Config::Help()),
            "repl" => Ok(Config::Repl()),
            _ => Err("Invalid argument"),
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
            Config::new(&vec!["".to_string(), "help".to_string()]).unwrap(),
            Config::Help()
        );
        assert_eq!(
            Config::new(&vec!["".to_string(), "-h".to_string()]).unwrap(),
            Config::Help()
        );
        assert_eq!(
            Config::new(&vec!["".to_string(), "--help".to_string()]).unwrap(),
            Config::Help()
        );
        assert_eq!(
            Config::new(&vec!["".to_string(), "repl".to_string()]).unwrap(),
            Config::Repl()
        );

        // assert_eq!(Config::new(&vec!["".to_string(), "help".to_string(), "repl".to_string()]).is_err(), true);
        assert_eq!(Config::new(&vec!["".to_string()]).is_err(), true);
    }
}
