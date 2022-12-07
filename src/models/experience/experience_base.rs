use std::{collections::HashSet, fmt};

use crate::models::{
    parser::{
        query::{OptionalTerm, Query},
        statement::Statement,
        term::Term,
    },
    semantics::truth_value::TruthValue,
};

use super::experience_element::ExperienceElement;

#[derive(Clone)]
pub struct ExperienceBase {
    pub experiences: Vec<ExperienceElement>,
    pub terms: HashSet<Term>,
    pub last_id: usize,
}

impl fmt::Display for ExperienceBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = self
            .experiences
            .iter()
            .map(|experience| format!("{}", experience))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", result)
    }
}

impl Default for ExperienceBase {
    fn default() -> Self {
        Self::new()
    }
}

impl ExperienceBase {
    pub fn new() -> Self {
        Self {
            experiences: Vec::new(),
            terms: HashSet::new(),
            last_id: 0,
        }
    }

    pub fn get_next_id(&self) -> usize {
        self.last_id + 1
    }

    pub fn add(&mut self, experience: ExperienceElement) {
        self.terms.insert(experience.stmt.left.clone());
        self.terms.insert(experience.stmt.right.clone());
        self.experiences.push(experience);
        self.last_id += 1;
    }

    pub fn remove(&mut self, id: usize) -> Result<(), &str> {
        // self.experiences.retain(|experience| experience.id != id);
        let index = match self
            .experiences
            .iter()
            .position(|experience| experience.id == id)
        {
            Some(index) => index,
            None => return Err("Experience id not found."),
        };
        let left = self.experiences[index].stmt.left.clone();
        let right = self.experiences[index].stmt.right.clone();
        self.terms.remove(&left);
        self.terms.remove(&right);
        self.experiences.remove(index);
        Ok(())
    }

    pub fn query(&self, q: Query) -> String {
        let mut stmt_left: Term = Term::new("a").unwrap();
        let mut stmt_right: Term = Term::new("b").unwrap();

        for experience in &self.experiences {
            match &q.left {
                OptionalTerm::Question => match &q.right {
                    OptionalTerm::Question => {
                        return "  Invalid Query".to_string();
                    }
                    OptionalTerm::Term(right) => {
                        if experience.stmt.right.word == right.word {
                            return format!("  {}", experience);
                        }
                    }
                },
                OptionalTerm::Term(left) => match &q.right {
                    OptionalTerm::Question => {
                        if experience.stmt.left.word == left.word {
                            return format!("  {}", experience);
                        }
                    }
                    OptionalTerm::Term(right) => {
                        stmt_right = right.clone();
                        stmt_left = left.clone();

                        if experience.stmt.left.word == left.word
                            && experience.stmt.right.word == right.word
                        {
                            return format!("  {}", experience);
                        }
                    }
                },
            }
        }

        let stmt = Statement {
            left: stmt_left,
            copula: q.copula,
            right: stmt_right,
        };
        format!(
            "  {} {}",
            stmt,
            TruthValue::from_statement(&stmt, self).unwrap()
        )
    }

    pub fn clear(&mut self) {
        self.experiences.clear();
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::parser::statement::Statement;

    #[test]
    fn test_experience_base_new() {
        let experience_base = ExperienceBase::new();

        assert_eq!(experience_base.experiences.len(), 0);
    }

    #[test]
    fn test_experience_base_add() {
        let mut experience_base = ExperienceBase::new();
        let experience = ExperienceElement::new(Statement::new("a is b").unwrap(), 1);

        experience_base.add(experience);

        assert_eq!(experience_base.experiences.len(), 1);
    }

    #[test]
    fn test_experience_base_remove() {
        let mut experience_base = ExperienceBase::new();
        let experience = ExperienceElement::new(Statement::new("a is b").unwrap(), 1);

        experience_base.add(experience);
        assert_eq!(experience_base.experiences.len(), 1);

        experience_base.remove(1).unwrap();
        assert_eq!(experience_base.experiences.len(), 0);
    }

    #[test]
    fn test_experience_base_to_string() {
        let mut experience_base = ExperienceBase::new();
        let experience = ExperienceElement::new(Statement::new("a is b").unwrap(), 1);

        experience_base.add(experience);

        assert_eq!(
            experience_base.to_string(),
            "1: a -> b <1.00, 0.99>".to_string()
        );
    }

    #[test]
    fn test_experience_base_query() {
        let mut experience_base = ExperienceBase::new();
        let experience = ExperienceElement::new(Statement::new("a is b").unwrap(), 1);

        experience_base.add(experience);

        assert_eq!(
            experience_base.query(Query::new("? is b").unwrap()),
            "  1: a -> b <1.00, 0.99>".to_string()
        );
    }

    #[test]
    fn test_experience_base_clear() {
        let mut experience_base = ExperienceBase::new();
        let experience = ExperienceElement::new(Statement::new("a is b").unwrap(), 1);

        experience_base.add(experience);
        assert_eq!(experience_base.experiences.len(), 1);

        experience_base.clear();
        assert_eq!(experience_base.experiences.len(), 0);
    }
}
