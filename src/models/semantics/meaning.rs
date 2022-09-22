use std::collections::HashSet;

use crate::models::{experience::experience_base::ExperienceBase, parser::term::Term};

pub fn extension_from_term(term: &Term, exp_base: &ExperienceBase) -> HashSet<Term> {
    if !exp_base.terms.contains(term) {
        return HashSet::new();
    }

    let mut extension = HashSet::new();
    extension.insert(term.clone());

    loop {
        let new_elems: Vec<Term> = exp_base
            .experiences
            .iter()
            .filter(|exp| {
                extension.contains(&exp.stmt.right) && !extension.contains(&exp.stmt.left)
            })
            .map(|exp| exp.stmt.left.clone())
            .collect();

        if new_elems.is_empty() {
            break;
        }
        for elem in new_elems {
            extension.insert(elem);
        }
    }

    extension
}

pub fn intension_from_term(term: &Term, exp_base: &ExperienceBase) -> HashSet<Term> {
    if !exp_base.terms.contains(term) {
        return HashSet::new();
    }

    let mut intension = HashSet::new();
    intension.insert(term.clone());

    loop {
        let new_elems: Vec<Term> = exp_base
            .experiences
            .iter()
            .filter(|exp| {
                intension.contains(&exp.stmt.left) && !intension.contains(&exp.stmt.right)
            })
            .map(|exp| exp.stmt.right.clone())
            .collect();
        if new_elems.is_empty() {
            break;
        }
        for elem in new_elems {
            intension.insert(elem);
        }
    }
    intension
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        experience::experience_element::ExperienceElement,
        parser::{statement::Statement, term::Term},
    };

    #[test]
    fn test_extension_from_term() {
        let mut experience_base = ExperienceBase::new();
        experience_base.add(ExperienceElement::new(
            Statement::new("robin is bird").unwrap(),
            1,
        ));
        experience_base.add(ExperienceElement::new(
            Statement::new("bird is animal").unwrap(),
            4,
        ));
        experience_base.add(ExperienceElement::new(
            Statement::new("penguin is bird").unwrap(),
            2,
        ));
        experience_base.add(ExperienceElement::new(
            Statement::new("chicken is bird").unwrap(),
            3,
        ));
        experience_base.add(ExperienceElement::new(
            Statement::new("human is animal").unwrap(),
            5,
        ));
        experience_base.add(ExperienceElement::new(
            Statement::new("saul is human").unwrap(),
            6,
        ));
        assert_eq!(experience_base.experiences.len(), 6);

        let robin = Term::new("robin").unwrap();
        let penguin = Term::new("penguin").unwrap();
        let chicken = Term::new("chicken").unwrap();
        let bird = Term::new("bird").unwrap();
        let human = Term::new("human").unwrap();
        let animal = Term::new("animal").unwrap();
        let saul = Term::new("saul").unwrap();

        let extension = extension_from_term(&animal, &experience_base);
        assert_eq!(extension.len(), 7);
        assert!(extension.contains(&animal));
        assert!(extension.contains(&bird));
        assert!(extension.contains(&human));
        assert!(extension.contains(&robin));
        assert!(extension.contains(&penguin));
        assert!(extension.contains(&chicken));
        assert!(extension.contains(&saul));

        let extension = extension_from_term(&bird, &experience_base);
        assert_eq!(extension.len(), 4);
        assert!(extension.contains(&bird));
        assert!(extension.contains(&robin));
        assert!(extension.contains(&penguin));
        assert!(extension.contains(&chicken));

        let extension = extension_from_term(&human, &experience_base);
        assert_eq!(extension.len(), 2);
        assert!(extension.contains(&human));
        assert!(extension.contains(&saul));

        let extension = extension_from_term(&saul, &experience_base);
        assert_eq!(extension.len(), 1);
        assert!(extension.contains(&saul));

        let liquid = Term::new("liquid").unwrap();
        let extension = extension_from_term(&liquid, &experience_base);
        assert_eq!(extension.len(), 0);
    }

    #[test]
    fn test_intension_from_term() {
        let mut experience_base = ExperienceBase::new();
        experience_base.add(ExperienceElement::new(
            Statement::new("robin is bird").unwrap(),
            1,
        ));
        experience_base.add(ExperienceElement::new(
            Statement::new("penguin is bird").unwrap(),
            2,
        ));
        experience_base.add(ExperienceElement::new(
            Statement::new("chicken is bird").unwrap(),
            3,
        ));
        experience_base.add(ExperienceElement::new(
            Statement::new("bird is animal").unwrap(),
            4,
        ));
        experience_base.add(ExperienceElement::new(
            Statement::new("human is animal").unwrap(),
            5,
        ));
        experience_base.add(ExperienceElement::new(
            Statement::new("saul is human").unwrap(),
            6,
        ));
        assert_eq!(experience_base.experiences.len(), 6);

        let penguin = Term::new("penguin").unwrap();
        let bird = Term::new("bird").unwrap();
        let animal = Term::new("animal").unwrap();
        let human = Term::new("human").unwrap();
        let saul = Term::new("saul").unwrap();

        let intension = intension_from_term(&penguin, &experience_base);
        assert_eq!(intension.len(), 3);
        assert!(intension.contains(&penguin));
        assert!(intension.contains(&bird));
        assert!(intension.contains(&animal));

        let intension = intension_from_term(&saul, &experience_base);
        println!("{:?}", intension);
        assert_eq!(intension.len(), 3);
        assert!(intension.contains(&saul));
        assert!(intension.contains(&human));
        assert!(intension.contains(&animal));

        let intension = intension_from_term(&animal, &experience_base);
        assert_eq!(intension.len(), 1);
        assert!(intension.contains(&animal));

        let liquid = Term::new("liquid").unwrap();
        let intension = intension_from_term(&liquid, &experience_base);
        assert_eq!(intension.len(), 0);
    }
}
