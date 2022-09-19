use std::io::{self, Write};

use crate::models::{
    experience::{Experience, ExperienceBase},
    grammar::{query::Query, statement::Statement},
    repl::repl_instruction::ReplInstruction,
};

const HELP_MSG: &str = "This is the repl for the Non Axiomatic Logic Engine. The following commands are available:
    /help   | /h: print this help message
    /exit   | /e: exit the repl
    /insert | /i: insert a statement into the experience base
    /remove | /r: remove a statement from the experience base
    /list   | /l: list all statements in the experience base
    /query  | /q: query the experience base
    /clear  | /c: clear the experience base\n";

#[derive(Debug, PartialEq)]
pub enum Action {
    Print(String),
    Nothing(),
    Exit()
}

pub struct Repl {
    counter: usize,
    experience_current_id: usize,
    experience_base: ExperienceBase,
}


impl Repl {
    pub fn new() -> Self {
        Self { 
            counter: 0,
            experience_current_id: 1,
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
                Ok(Action::Print(HELP_MSG.to_string()))
            }
            Ok(ReplInstruction::Exit()) => {
                Ok(Action::Exit())
            }
            Ok(ReplInstruction::Insert(stmt)) => {
                let stmt: Statement = Statement::new(&stmt)?;
                let experience: Experience = Experience::new(stmt, self.experience_current_id);
                self.experience_current_id += 1;

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
                // self.experience_base.query(query);
                Ok(Action::Print(self.experience_base.query(query).to_string()))
                
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
        assert_eq!(repl.experience_base.experiences.len(), 0);
    }

    #[test]
    fn test_execute_help() {
        let mut repl = Repl::new();
        let action = repl.execute("/help".to_string()).unwrap();
        let expected_output = HELP_MSG;
        assert_eq!(action, Action::Print(expected_output.to_string()));
    }

    #[test]
    fn test_execute_exit() {
        let mut repl = Repl::new();
        let action = repl.execute("/exit".to_string()).unwrap();
        assert_eq!(action, Action::Exit());
    }

    #[test]
    fn test_execute_insert() {
        let mut repl = Repl::new();
        let action = repl.execute("/insert a is b".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 1);
        assert_eq!(repl.experience_base.experiences[0].stmt.to_string(), "a -> b");
    }

    #[test]
    fn test_execute_remove() {
        let mut repl = Repl::new();
        let action = repl.execute("/insert a is b".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 1);
        assert_eq!(repl.experience_base.experiences[0].stmt.to_string(), "a -> b");

        let action = repl.execute("/remove 1".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 0);
    }

    #[test]
    fn test_execute_list() {
        let mut repl = Repl::new();
        let action = repl.execute("/insert a is b".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        let action = repl.execute("/insert b is c".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 2);
        assert_eq!(repl.experience_base.experiences[0].stmt.to_string(), "a -> b");
        assert_eq!(repl.experience_base.experiences[1].stmt.to_string(), "b -> c");

        let action = repl.execute("/list".to_string()).unwrap();
        let expected_output = "1: a -> b\n2: b -> c";
        assert_eq!(action, Action::Print(expected_output.to_string()));
    }

    #[test]
    fn test_execute_query() {
        let mut repl = Repl::new();
        let action = repl.execute("/insert a is b".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        let action = repl.execute("/insert b is c".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 2);
        assert_eq!(repl.experience_base.experiences[0].stmt.to_string(), "a -> b");
        assert_eq!(repl.experience_base.experiences[1].stmt.to_string(), "b -> c");

        let action = repl.execute("/query a is ?".to_string()).unwrap();
        let expected_output = "  1: a -> b";
        assert_eq!(action, Action::Print(expected_output.to_string()));

        let action = repl.execute("/query ? is c".to_string()).unwrap();
        let expected_output = "  2: b -> c";
        assert_eq!(action, Action::Print(expected_output.to_string()));

        let action = repl.execute("/query a is b".to_string()).unwrap();
        let expected_output = "  1: a -> b";
        assert_eq!(action, Action::Print(expected_output.to_string()));

        let action = repl.execute("/query a is c".to_string()).unwrap();
        let expected_output = "  No matches found.";
        assert_eq!(action, Action::Print(expected_output.to_string()));

        let action = repl.execute("/query ? is ?".to_string());
        assert_eq!(action.is_err(), true);
    }

    #[test]
    fn test_execute_clear() {
        let mut repl = Repl::new();
        let action = repl.execute("/insert a is b".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        let action = repl.execute("/insert b is c".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 2);
        assert_eq!(repl.experience_base.experiences[0].stmt.to_string(), "a -> b");
        assert_eq!(repl.experience_base.experiences[1].stmt.to_string(), "b -> c");

        let action = repl.execute("/clear".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 0);
    }
}