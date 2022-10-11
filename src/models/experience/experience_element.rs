use std::fmt;

use chrono::NaiveDateTime;

use crate::models::{parser::statement::Statement, semantics::truth_value::TruthValue};

pub struct ExperienceElement {
    pub id: usize,
    pub stmt: Statement,
    pub created_at: NaiveDateTime,
    pub truth_value: TruthValue, // pub updated_at: NaiveDateTime,
}

impl fmt::Display for ExperienceElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} {} {} {}",
            self.id, self.stmt.left.word, &self.stmt.copula, self.stmt.right.word, self.truth_value
        )
    }
}

impl ExperienceElement {
    pub fn new(stmt: Statement, id: usize) -> Self {
        let now: NaiveDateTime = chrono::Local::now().naive_local();

        Self {
            id,
            stmt,
            created_at: now,
            truth_value: TruthValue::new().unwrap(), // updated_at: now,
        }
    }

    pub fn new_with_truth_value(stmt: Statement, id: usize, truth_value: TruthValue) -> Self {
        let now: NaiveDateTime = chrono::Local::now().naive_local();

        Self {
            id,
            stmt,
            created_at: now,
            truth_value, // updated_at: now,
        }
    }
}
