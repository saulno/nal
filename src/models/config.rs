// Enum with the different options to run
pub enum Config {
    Help(),
    Repl()
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
            _ => Err("Invalid argument")
        }
    }
}