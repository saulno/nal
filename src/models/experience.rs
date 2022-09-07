use std::sync::atomic::{AtomicUsize, Ordering};
use chrono::NaiveDateTime;

use crate::models::grammar::query::OptionalTerm;

use super::grammar::{statement::Statement, query::Query};

static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub struct Experience {
    pub id: usize,
    pub stmt: Statement,
    pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
}

impl Experience {
    pub fn new(stmt: Statement) -> Self {
        let now: NaiveDateTime = chrono::Local::now().naive_local();
        let id: usize = OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst);

        Self {
            id,
            stmt,
            created_at: now,
            // updated_at: now,
        }
    }
}
pub struct ExperienceBase {
    pub experiences: Vec<Experience>,
}

impl ExperienceBase {
    pub fn new() -> Self {
        Self {
            experiences: Vec::new(),
        }
    }

    pub fn add(&mut self, experience: Experience) {
        self.experiences.push(experience);
    }

    pub fn remove(&mut self, id: usize) {
        self.experiences.retain(|experience| experience.id != id);
    }

    pub fn list(&self) {
        for experience in &self.experiences {
            println!("  {}: {} {} {}", experience.id, experience.stmt.left.word, &experience.stmt.copula.to_string(), experience.stmt.right.word);
        }
    }

    pub fn query(&self, q: Query) {
        for experience in &self.experiences {
            match &q.left {
                OptionalTerm::Question => {
                    match &q.right {
                        OptionalTerm::Question => {
                            println!("  Invalid Query");
                            return;
                        },
                        OptionalTerm::Term(right) => {
                            if experience.stmt.right.word == right.word {
                                println!("  {}: {} {} {}", experience.id, experience.stmt.left.word, &experience.stmt.copula.to_string(), experience.stmt.right.word);
                                return;
                            }
                        }
                    }
                },
                OptionalTerm::Term(left) => {
                    match &q.right {
                        OptionalTerm::Question => {
                            println!("  {}: {} {} {}", experience.id, experience.stmt.left.word, &experience.stmt.copula.to_string(), experience.stmt.right.word);
                            return;
                        },
                        OptionalTerm::Term(right) => {
                            if experience.stmt.left.word == left.word && experience.stmt.right.word == right.word {
                                println!("  {}: {} {} {}", experience.id, experience.stmt.left.word, &experience.stmt.copula.to_string(), experience.stmt.right.word);
                                return;
                            }
                        }
                    }
                },
            }
        }
        println!("  No matching experience found");
    }

    pub fn clear(&mut self) {
        self.experiences.clear();
    }
}
