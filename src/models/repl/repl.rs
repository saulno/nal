use std::io::{self, Write};

use crate::models::{
    experience::{Experience, ExperienceBase},
    grammar::{query::Query, statement::Statement},
    repl::repl_instruction::ReplInstruction,
};
pub enum Action {
    Print(String),
    Nothing(),
    Exit()
}

pub struct Repl {
    counter: usize,
    experience_base: ExperienceBase,
}


impl Repl {
    pub fn new() -> Self {
        Self { 
            counter: 0,
            experience_base: ExperienceBase::new() 
        }
    }

    pub fn run(&mut self) {

        loop {
            self.counter += 1;

            let mut input = String::new();

            print!("{}> ", self.counter);
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut input).unwrap();

            match self.execute(input) {
                Ok(action) => match action {
                    Action::Print(s) => println!("{}", s),
                    Action::Nothing() => (),
                    Action::Exit() => break,
                },
                Err(e) => println!("{}", e),
            }
        }
    }

    fn execute(&mut self, input: String) -> Result<Action, String> {
        let instruction: Vec<String> = input.split_whitespace().map(|elem| elem.to_string()).collect();

        match ReplInstruction::new(&instruction) {
            Ok(ReplInstruction::Help()) => {
                Ok(Action::Print("Help".to_string()))
            }
            Ok(ReplInstruction::Exit()) => {
                Ok(Action::Exit())
            }
            Ok(ReplInstruction::Insert(stmt)) => {
                let stmt: Statement = Statement::new(&stmt)?;
                let experience: Experience = Experience::new(stmt);
                self.experience_base.add(experience);
                Ok(Action::Print("Ok.".to_string()))
            }
            Ok(ReplInstruction::Remove(id)) => {
                self.experience_base.remove(id);
                Ok(Action::Print("Ok.".to_string()))
            }
            Ok(ReplInstruction::List()) => {
                Ok(Action::Print(self.experience_base.to_string()))
            }
            Ok(ReplInstruction::Query(q)) => {
                let query: Query = Query::new(&q)?;
                self.experience_base.query(query);
                Ok(Action::Print("Ok.".to_string()))
                
            }
            Ok(ReplInstruction::Clear()) => {
                self.experience_base.clear();
                Ok(Action::Print("Ok.".to_string()))
            }
            Ok(ReplInstruction::Unknown()) => {
                Ok(Action::Print("Unknown command.".to_string()))
            }
            Err(e) => {
                Ok(Action::Print(e.to_string()))
            }
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repl = Repl::new();
        assert_eq!(repl.counter, 0);
    }

    #[test]
    fn test_run() {
        let mut repl = Repl::new();
        repl.run();
    }
}