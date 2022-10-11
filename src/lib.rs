pub mod models;

use std::error::Error;

use models::{config::Config, repl::repl_console::ReplConsole};

const HELP_STR: &str =
    "This is the console to interact with the NAL. Run the REPL with the command 'repl'.";

// run the program
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::Help() => {
            println!("{}", HELP_STR);
        }
        Config::Repl() => {
            // println!("Repl");
            let mut repl = ReplConsole::new();
            repl.run();
        }
    }

    Ok(())
}
