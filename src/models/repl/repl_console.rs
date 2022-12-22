use std::fs;

use rustyline::{error::ReadlineError, Editor};

use crate::models::{
    experience::{experience_base::ExperienceBase, experience_element::ExperienceElement},
    inference::{
        inference_instruction::InferenceInstruction,
        inference_rule::{infer_and_update, print_inference_result},
    },
    parser::{query::Query, statement::Statement},
    repl::repl_instruction::ReplInstruction,
    semantics::truth_value::TruthValue,
};

const HELP_MSG: &str =
    "This is the repl for the Non Axiomatic Logic Engine. The following commands are available:
    /help   | /h: print this help message
    /exit   | /e: exit the repl
    /file   | /f: load and execute a file
    /assert | /a: insert a statement into the experience base
    /remove | /r: remove a statement from the experience base
    /list   | /l: list all statements in the experience base
    /query  | /q: query the experience base
    /infer  | /i: infer a statement from the experience base
        revision                | rev | r  <id1> <id2>: infer a statement from the experience base using revision
        choice                  | cho | ch <id1> <id2>: infer a statement from the experience base using choice
        deduction               | ded | d  <id1> <id2>: infer a statement from the experience base using deduction
        induction               | ind | i  <id1> <id2>: infer a statement from the experience base using induction
        exemplification         | exe | e  <id1> <id2>: infer a statement from the experience base using exemplification
        abduction               | abd | a  <id1> <id2>: infer a statement from the experience base using abduction
        conversion              | cnv | c  <id>: infer a statement from the experience base using conversion
        comparison              | com      <id1> <id2>: infer a statement from the experience base using comparison
        analogy                 | ana      <id1> <id2>: infer a statement from the experience base using analogy
        resemblance             | res      <id1> <id2>: infer a statement from the experience base using resemblance
        union_extension         | ue       <id1> <id2>: infer a statement from the experience base using union_extension
        union_intension         | ui       <id1> <id2>: infer a statement from the experience base using union_intension
        intersection_extension  | ie       <id1> <id2>: infer a statement from the experience base using intersection_extension
        intersection_intension  | ii       <id1> <id2>: infer a statement from the experience base using intersection_intension
        difference_extension    | de       <id1> <id2>: infer a statement from the experience base using difference_extension
        difference_intension    | di       <id1> <id2>: infer a statement from the experience base using difference_intension
    /clear  | /c: clear the experience base\n";

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Print(String),
    Nothing(),
    Exit(),
}

pub struct ReplConsole {
    counter: usize,
    experience_base: ExperienceBase,
}

impl Default for ReplConsole {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplConsole {
    pub fn new() -> Self {
        Self {
            counter: 0,
            experience_base: ExperienceBase::new(),
        }
    }

    pub fn run(&mut self) {
        let mut rl = Editor::<()>::new().unwrap();
        println!(
            "Welcome to the Non-Axiomatic Logic Engine Repl.\nType /help for a list of commands."
        );
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
            Ok(ReplInstruction::Assert(s)) => {
                let stmt: Statement = Statement::new(&s[..3].join(" "))?;
                let next_id = self.experience_base.get_next_id();
                let experience: ExperienceElement =
                    if TruthValue::new_from_str(&s[3..].join(" ")).is_ok() {
                        let truth_value = TruthValue::new_from_str(&s[3..].join(" ")).unwrap();
                        ExperienceElement::new_with_truth_value(stmt, next_id, truth_value)
                    } else {
                        ExperienceElement::new(stmt, next_id)
                    };

                self.experience_base.add(experience);
                Ok(Action::Print("Ok.".to_string()))
            }
            Ok(ReplInstruction::Remove(id)) => {
                self.experience_base.remove(id)?;
                Ok(Action::Print("Ok.".to_string()))
            }
            Ok(ReplInstruction::List()) => Ok(Action::Print(self.experience_base.to_string())),
            Ok(ReplInstruction::Query(q)) => {
                let query: Query = Query::new(&q.join(" "))?;
                Ok(Action::Print(self.experience_base.query(query)))
            }
            Ok(ReplInstruction::Infer(args)) => {
                let inference = InferenceInstruction::new(&args)?;
                let result = print_inference_result(&self.experience_base, inference)?;
                Ok(Action::Print(result))
            }
            Ok(ReplInstruction::InferUpdate(args)) => {
                let inference = InferenceInstruction::new(&args)?;
                let result = infer_and_update(&mut self.experience_base, inference)?;
                Ok(Action::Print(result))
            }
            Ok(ReplInstruction::Clear()) => {
                self.experience_base.clear();
                Ok(Action::Print("Ok.".to_string()))
            }
            Ok(ReplInstruction::File(path)) => {
                // open file and read it
                let file = if fs::read_to_string(&path).is_ok() {
                    fs::read_to_string(path).unwrap()
                } else {
                    return Err("File not found.".to_string());
                };
                // remove comment lines that start with #
                let lines = file
                    .lines()
                    .filter(|line| !line.starts_with('#'))
                    .collect::<Vec<&str>>();
                // remove empty lines
                let lines: Vec<&str> = lines
                    .iter()
                    .filter(|line| !line.is_empty())
                    .copied()
                    .collect();
                // execute each line
                for line in lines {
                    println!("Executing: {}", line);
                    let res = self.execute(line.to_string())?;
                    match res {
                        Action::Print(msg) => println!("{}", msg),
                        Action::Nothing() => (),
                        Action::Exit() => break,
                    }
                }

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
        let repl = ReplConsole::new();
        assert_eq!(repl.counter, 0);
        assert_eq!(repl.experience_base.experiences.len(), 0);
    }

    #[test]
    fn test_execute_help() {
        let mut repl = ReplConsole::new();
        let action = repl.execute("/help".to_string()).unwrap();
        let expected_output = HELP_MSG;
        assert_eq!(action, Action::Print(expected_output.to_string()));
    }

    #[test]
    fn test_execute_exit() {
        let mut repl = ReplConsole::new();
        let action = repl.execute("/exit".to_string()).unwrap();
        assert_eq!(action, Action::Exit());
    }

    #[test]
    fn test_execute_assert() {
        let mut repl = ReplConsole::new();
        let action = repl.execute("/assert a is b".to_string()).unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 1);
        assert_eq!(
            repl.experience_base.experiences[0].to_string(),
            "1: a -> b <1.00, 0.99>"
        );

        let action = repl
            .execute("/assert c is d <0.787, 0.5678>".to_string())
            .unwrap();
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 2);
        assert_eq!(
            repl.experience_base.experiences[1].to_string(),
            "2: c -> d <0.79, 0.57>"
        );
    }

    #[test]
    fn test_execute_remove() {
        let mut repl = ReplConsole::new();
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
        let mut repl = ReplConsole::new();
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
        let expected_output = "1: a -> b <1.00, 0.99>\n2: b -> c <1.00, 0.99>";
        assert_eq!(action, Action::Print(expected_output.to_string()));
    }

    #[test]
    fn test_execute_query() {
        let mut repl = ReplConsole::new();
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
        let expected_output = "  1: a -> b <1.00, 0.99>";
        assert_eq!(action, Action::Print(expected_output.to_string()));

        let action = repl.execute("/query ? is c".to_string()).unwrap();
        let expected_output = "  2: b -> c <1.00, 0.99>";
        assert_eq!(action, Action::Print(expected_output.to_string()));

        let action = repl.execute("/query a is b".to_string()).unwrap();
        let expected_output = "  1: a -> b <1.00, 0.99>";
        assert_eq!(action, Action::Print(expected_output.to_string()));

        let action = repl.execute("/query a is c".to_string()).unwrap();
        let expected_output = "  a -> c <0.50, 0.80>";
        assert_eq!(action, Action::Print(expected_output.to_string()));

        let action = repl.execute("/query ? is ?".to_string());
        assert_eq!(action.is_err(), true);
    }

    #[test]
    fn test_execute_clear() {
        let mut repl = ReplConsole::new();
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
    fn test_execute_infer_deduction() {
        let mut repl = ReplConsole::new();
        repl.execute("/assert a is b".to_string()).unwrap();
        repl.execute("/assert b is c".to_string()).unwrap();
        repl.execute("/assert c is d".to_string()).unwrap();

        let action = repl.execute("/infer deduction 1 2".to_string()).unwrap();
        let expected_output =
            "  1: a -> b <1.00, 0.99>\n  2: b -> c <1.00, 0.99>\n  RESULT: a -> c <1.00, 0.98>";
        assert_eq!(action, Action::Print(expected_output.to_string()));
    }

    #[test]
    fn test_execute_file() {
        let mut repl = ReplConsole::new();
        let action = match repl.execute("/file script.nal".to_string()) {
            Ok(action) => action,
            Err(err) => Action::Print(format!("Error: {}", err)),
        };
        assert_eq!(action, Action::Print("Ok.".to_string()));
        assert_eq!(repl.experience_base.experiences.len(), 7);
        assert_eq!(
            repl.experience_base.experiences[0].stmt.to_string(),
            "robin -> bird"
        );
        assert_eq!(
            repl.experience_base.experiences[1].stmt.to_string(),
            "bird -> animal"
        );
        assert_eq!(
            repl.experience_base.experiences[2].stmt.to_string(),
            "saul -> human"
        );
        assert_eq!(
            repl.experience_base.experiences[3].stmt.to_string(),
            "human -> animal"
        );
    }
}
