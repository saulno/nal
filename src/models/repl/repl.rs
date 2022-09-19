use rustyline::{Editor, error::ReadlineError};

use crate::models::{
    experience::{experience::Experience, experience_base::ExperienceBase},
    grammar::{query::Query, statement::Statement},
    inference::{inference::transitivity, inference_instruction::InferenceInstruction},
    repl::repl_instruction::ReplInstruction,
};

const HELP_MSG: &str =
    "This is the repl for the Non Axiomatic Logic Engine. The following commands are available:
    /help   | /h: print this help message
    /exit   | /e: exit the repl
    /assert | /a: insert a statement into the experience base
    /remove | /r: remove a statement from the experience base
    /list   | /l: list all statements in the experience base
    /query  | /q: query the experience base
    /infer  | /i: infer a statement from the experience base
    /clear  | /c: clear the experience base\n";

#[derive(Debug, PartialEq)]
pub enum Action {
    Print(String),
    Nothing(),
    Exit(),
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
            experience_base: ExperienceBase::new(),
        }
    }

    pub fn run(&mut self) {

        let mut rl = Editor::<()>::new().unwrap();
        println!("Welcome to the Non-Axiomatic Logic Engine Repl.\n Type /help for a list of commands.");
        loop {
            self.counter += 1; 
            let readline = rl.readline(format!("{}>> ", self.counter).as_str());
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    let action = self.execute(line);
                    match action {
                        Ok(action) => match action {
                            Action::Print(msg) => println!("{}", msg),
                            Action::Nothing() => (),
                            Action::Exit() => break,
                        },
                        Err(msg) => println!("{}", msg),
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    }

    fn execute(&mut self, input: String) -> Result<Action, String> {
        let instruction: Vec<String> = input
            .split_whitespace()
            .map(|elem| elem.to_string())
            .collect();

        match ReplInstruction::new(&instruction) {
            Ok(ReplInstruction::Help()) => Ok(Action::Print(HELP_MSG.to_string())),
            Ok(ReplInstruction::Exit()) => Ok(Action::Exit()),
            Ok(ReplInstruction::Assert(stmt)) => {
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
            Ok(ReplInstruction::List()) => Ok(Action::Print(self.experience_base.to_string())),
            Ok(ReplInstruction::Query(q)) => {
                let query: Query = Query::new(&q)?;
                Ok(Action::Print(self.experience_base.query(query).to_string()))
            }
            Ok(ReplInstruction::Infer(args)) => {
                let inference = InferenceInstruction::new(&args)?;
                match inference {
                    InferenceInstruction::Transitivity(id1, id2) => {
                        let result = transitivity(&self.experience_base, id1, id2)?;
                        let exp1 = self
                            .experience_base
                            .experiences
                            .iter()
                            .find(|experience| experience.id == id1)
                            .ok_or("First id not found in experience base.")?;
                        let exp2 = self
                            .experience_base
                            .experiences
                            .iter()
                            .find(|experience| experience.id == id2)
                            .ok_or("Second id not found in experience base.")?;
                        Ok(Action::Print(format!(
                            "  {}: {}\n  {}: {}\n  RESULT: {}",
                            id1,
                            exp1.stmt.to_string(),
                            id2,
                            exp2.stmt.to_string(),
                            result.to_string()
                        )))
                    }
                }
            }
            Ok(ReplInstruction::Clear()) => {
                self.experience_base.clear();
                Ok(Action::Print("Ok.".to_string()))
            }
            Ok(ReplInstruction::Unknown()) => Ok(Action::Print("Unknown command.".to_string())),
            Err(e) => Ok(Action::Print(e.to_string())),
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
    fn test_execute_assert() {
        let mut repl = Repl::new();
        let action = repl.execute("/assert a is b".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 1);
        assert_eq!(
            repl.experience_base.experiences[0].stmt.to_string(),
            "a -> b"
        );
    }

    #[test]
    fn test_execute_remove() {
        let mut repl = Repl::new();
        let action = repl.execute("/assert a is b".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 1);
        assert_eq!(
            repl.experience_base.experiences[0].stmt.to_string(),
            "a -> b"
        );

        let action = repl.execute("/remove 1".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 0);
    }

    #[test]
    fn test_execute_list() {
        let mut repl = Repl::new();
        let action = repl.execute("/assert a is b".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        let action = repl.execute("/assert b is c".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 2);
        assert_eq!(
            repl.experience_base.experiences[0].stmt.to_string(),
            "a -> b"
        );
        assert_eq!(
            repl.experience_base.experiences[1].stmt.to_string(),
            "b -> c"
        );

        let action = repl.execute("/list".to_string()).unwrap();
        let expected_output = "1: a -> b\n2: b -> c";
        assert_eq!(action, Action::Print(expected_output.to_string()));
    }

    #[test]
    fn test_execute_query() {
        let mut repl = Repl::new();
        let action = repl.execute("/assert a is b".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        let action = repl.execute("/assert b is c".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 2);
        assert_eq!(
            repl.experience_base.experiences[0].stmt.to_string(),
            "a -> b"
        );
        assert_eq!(
            repl.experience_base.experiences[1].stmt.to_string(),
            "b -> c"
        );

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
        let action = repl.execute("/assert a is b".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        let action = repl.execute("/assert b is c".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 2);
        assert_eq!(
            repl.experience_base.experiences[0].stmt.to_string(),
            "a -> b"
        );
        assert_eq!(
            repl.experience_base.experiences[1].stmt.to_string(),
            "b -> c"
        );

        let action = repl.execute("/clear".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 0);
    }

    #[test]
    fn test_execute_infer_transitivity() {
        let mut repl = Repl::new();
        repl.execute("/assert a is b".to_string()).unwrap();
        repl.execute("/assert b is c".to_string()).unwrap();
        repl.execute("/assert c is d".to_string()).unwrap();

        let action = repl.execute("/infer transitivity 1 2".to_string()).unwrap();
        let expected_output = "  1: a -> b\n  2: b -> c\n  RESULT: a -> c";
        assert_eq!(action, Action::Print(expected_output.to_string()));
    }
}
