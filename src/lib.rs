pub mod models;

use std::error::Error;

use models::{config::Config, repl::repl_console::ReplConsole};

// run the program
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::Help() => {
            println!("Help");
        }
        Config::Repl() => {
            // println!("Repl");
            let mut repl = ReplConsole::new();
            repl.run();
        }
    }

    Ok(())
}
