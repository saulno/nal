use chrono::NaiveDateTime;

use crate::models::parser::statement::Statement;

pub struct ExperienceElement {
    pub id: usize,
    pub stmt: Statement,
    pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
}

impl ExperienceElement {
    pub fn new(stmt: Statement, id: usize) -> Self {
        let now: NaiveDateTime = chrono::Local::now().naive_local();

        Self {
            id,
            stmt,
            created_at: now,
            // updated_at: now,
        }
    }
}
