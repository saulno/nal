use std::fmt;

use crate::models::grammar::query::{OptionalTerm, Query};

use super::experience_element::Experience;

pub struct ExperienceBase {
    pub experiences: Vec<Experience>,
}

impl fmt::Display for ExperienceBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = self
            .experiences
            .iter()
            .map(|experience| format!("{}: {}", experience.id, experience.stmt))
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
        }
    }

    pub fn add(&mut self, experience: Experience) {
        self.experiences.push(experience);
    }

    pub fn remove(&mut self, id: usize) {
        self.experiences.retain(|experience| experience.id != id);
    }

    // pub fn to_string(&self) -> String {
    //     self.experiences
    //         .iter()
    //         .map(|experience| format!("{}: {}", experience.id, experience.stmt))
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // }

    pub fn query(&self, q: Query) -> String {
        for experience in &self.experiences {
            match &q.left {
                OptionalTerm::Question => match &q.right {
                    OptionalTerm::Question => {
                        return "  Invalid Query".to_string();
                    }
                    OptionalTerm::Term(right) => {
                        if experience.stmt.right.word == right.word {
                            return format!(
                                "  {}: {} {} {}",
                                experience.id,
                                experience.stmt.left.word,
                                &experience.stmt.copula.to_string(),
                                experience.stmt.right.word
                            );
                        }
                    }
                },
                OptionalTerm::Term(left) => match &q.right {
                    OptionalTerm::Question => {
                        if experience.stmt.left.word == left.word {
                            return format!(
                                "  {}: {} {} {}",
                                experience.id,
                                experience.stmt.left.word,
                                &experience.stmt.copula.to_string(),
                                experience.stmt.right.word
                            );
                        }
                    }
                    OptionalTerm::Term(right) => {
                        if experience.stmt.left.word == left.word
                            && experience.stmt.right.word == right.word
                        {
                            return format!(
                                "  {}: {} {} {}",
                                experience.id,
                                experience.stmt.left.word,
                                &experience.stmt.copula.to_string(),
                                experience.stmt.right.word
                            );
                        }
                    }
                },
            }
        }
        "  No matches found.".to_string()
    }

    pub fn clear(&mut self) {
        self.experiences.clear();
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::grammar::{
        copula::Copula, query::OptionalTerm, statement::Statement, term::Term,
    };

    #[test]
    fn test_experience_base_new() {
        let experience_base = ExperienceBase::new();

        assert_eq!(experience_base.experiences.len(), 0);
    }

    #[test]
    fn test_experience_base_add() {
        let mut experience_base = ExperienceBase::new();
        let experience = Experience::new(
            Statement {
                left: Term::new("a").unwrap(),
                copula: Copula::new("is").unwrap(),
                right: Term::new("b").unwrap(),
            },
            1,
        );

        experience_base.add(experience);

        assert_eq!(experience_base.experiences.len(), 1);
    }

    #[test]
    fn test_experience_base_remove() {
        let mut experience_base = ExperienceBase::new();
        let experience = Experience::new(
            Statement {
                left: Term::new("a").unwrap(),
                copula: Copula::new("is").unwrap(),
                right: Term::new("b").unwrap(),
            },
            1,
        );

        experience_base.add(experience);
        assert_eq!(experience_base.experiences.len(), 1);

        experience_base.remove(1);
        assert_eq!(experience_base.experiences.len(), 0);
    }

    #[test]
    fn test_experience_base_to_string() {
        let mut experience_base = ExperienceBase::new();
        let experience = Experience::new(
            Statement {
                left: Term::new("a").unwrap(),
                copula: Copula::new("is").unwrap(),
                right: Term::new("b").unwrap(),
            },
            1,
        );

        experience_base.add(experience);

        assert_eq!(experience_base.to_string(), "1: a -> b".to_string());
    }

    #[test]
    fn test_experience_base_query() {
        let mut experience_base = ExperienceBase::new();
        let experience = Experience::new(
            Statement {
                left: Term::new("a").unwrap(),
                copula: Copula::new("is").unwrap(),
                right: Term::new("b").unwrap(),
            },
            1,
        );

        experience_base.add(experience);

        assert_eq!(
            experience_base.query(Query {
                left: OptionalTerm::Question,
                copula: Copula::new("is").unwrap(),
                right: OptionalTerm::Term(Term::new("b").unwrap()),
            }),
            "  1: a -> b".to_string()
        );
    }

    #[test]
    fn test_experience_base_clear() {
        let mut experience_base = ExperienceBase::new();
        let experience = Experience::new(
            Statement {
                left: Term::new("a").unwrap(),
                copula: Copula::new("is").unwrap(),
                right: Term::new("b").unwrap(),
            },
            1,
        );

        experience_base.add(experience);
        assert_eq!(experience_base.experiences.len(), 1);

        experience_base.clear();
        assert_eq!(experience_base.experiences.len(), 0);
    }
}
