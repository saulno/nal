use std::io::{self, Write};

use crate::models::{grammar::{statement::Statement, query::Query}};

use super::experience::{ExperienceBase, Experience};

pub enum ReplInstruction {
    Help(),
    Exit(),
    Insert(Vec<String>),
    Remove(usize),
    List(),
    Query(Vec<String>),
    Clear(),
    Unknown()
}

impl ReplInstruction {
    // Create new repl
    pub fn new(instruction: &[String]) -> Result<ReplInstruction, &str> {
        match instruction[0].as_str() {
            "/help" | "/h"  => Ok(ReplInstruction::Help()),
            "/exit" | "/e"  => Ok(ReplInstruction::Exit()),
            "/list" | "/l"  => Ok(ReplInstruction::List()),
            "/clear" | "/c" => Ok(ReplInstruction::Clear()),
            "/insert" | "/i" => Ok(ReplInstruction::Insert(instruction[1..].to_vec())),
            "/remove" | "/r" => Ok(ReplInstruction::Remove(instruction[1].parse::<usize>().unwrap())),
            "/query" | "/q"  => Ok(ReplInstruction::Query(instruction[1..].to_vec())),
            _ => Ok(ReplInstruction::Unknown())
        }
    }
}

pub struct Repl {
    counter: usize
}

impl Repl {
    pub fn new() -> Self {
        Self {
            counter: 0
        }
    }

    pub fn run(&mut self) {
        let mut experience_base: ExperienceBase = ExperienceBase::new();

        loop {
            self.counter += 1;

            let mut input = String::new();
            let mut instruction: Vec<String> = Vec::new();

            print!("{}> ", self.counter);
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut input).unwrap();

            for word in input.split_whitespace() {
                instruction.push(word.to_string());
            }

            match ReplInstruction::new(&instruction) {
                Ok(ReplInstruction::Help()) => {
                    println!("Help");
                },
                Ok(ReplInstruction::Exit()) => {
                    println!("Exit");
                    break;
                },
                Ok(ReplInstruction::Insert(stmt)) => {
                    if let Err(e) = Statement::new(&stmt) {
                        println!("Error: {}", e);
                    } else {
                        let stmt: Statement = Statement::new(&stmt).unwrap();
                        let experience: Experience = Experience::new(stmt);
                        experience_base.add(experience);
                    }
                },
                Ok(ReplInstruction::Remove(id)) => {
                    experience_base.remove(id);
                },
                Ok(ReplInstruction::List()) => {
                    experience_base.list();
                },
                Ok(ReplInstruction::Query(q)) => {
                    if let Err(e) = Query::new(&q) {
                        println!("Error: {}", e);
                    } else {
                        let q: Query = Query::new(&q).unwrap();
                        experience_base.query(q);
                    }
                },
                Ok(ReplInstruction::Clear()) => {
                    experience_base.clear();
                },
                Ok(ReplInstruction::Unknown()) => {
                    println!("Unknown instruction");
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}

