use rustyline::{error::ReadlineError, Editor};

use crate::models::{
    experience::{experience_base::ExperienceBase, experience_element::ExperienceElement},
    inference::{inference_instruction::InferenceInstruction, inference_rule::print_transitivity},
    parser::{query::Query, statement::Statement},
    repl::repl_instruction::ReplInstruction,
    semantics::truth_value::TruthValue,
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
        transitivity | trans | t <id1> <id2>: infer a statement from the experience base using transitivity
    /clear  | /c: clear the experience base\n";

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Print(String),
    Nothing(),
    Exit(),
}

pub struct ReplConsole {
    counter: usize,
    experience_current_id: usize,
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
            experience_current_id: 1,
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
                let experience: ExperienceElement =
                    if TruthValue::new_from_str(&s[3..].join(" ")).is_ok() {
                        let truth_value = TruthValue::new_from_str(&s[3..].join(" ")).unwrap();
                        ExperienceElement::new_with_truth_value(
                            stmt,
                            self.experience_current_id,
                            truth_value,
                        )
                    } else {
                        ExperienceElement::new(stmt, self.experience_current_id)
                    };
                self.experience_current_id += 1;

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
                match inference {
                    InferenceInstruction::Transitivity(id_exp_1, id_exp_2) => {
                        let result = print_transitivity(&self.experience_base, id_exp_1, id_exp_2)?;
                        Ok(Action::Print(result))
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
    fn test_execute_infer_transitivity() {
        let mut repl = ReplConsole::new();
        repl.execute("/assert a is b".to_string()).unwrap();
        repl.execute("/assert b is c".to_string()).unwrap();
        repl.execute("/assert c is d".to_string()).unwrap();

        let action = repl.execute("/infer transitivity 1 2".to_string()).unwrap();
        let expected_output = "  1: a -> b\n  2: b -> c\n  RESULT: a -> c";
        assert_eq!(action, Action::Print(expected_output.to_string()));
    }
}
