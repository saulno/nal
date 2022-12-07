use crate::models::{
    experience::{experience_base::ExperienceBase, experience_element::ExperienceElement},
    parser::statement::Statement,
    semantics::truth_value::TruthValue,
};

use super::{
    abduction::abduction, conversion::conversion, deduction::deduction,
    exemplification::exemplification, induction::induction,
    inference_instruction::InferenceInstruction, revision::revision, selection::selection,
};

fn execute_inference(
    experience_base: &ExperienceBase,
    inference_instruction: InferenceInstruction,
) -> Result<((Statement, TruthValue), usize, usize), &str> {
    let (result, id_exp_1, id_exp_2) = match inference_instruction {
        InferenceInstruction::Revision(id_exp_1, id_exp_2) => (
            revision(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::Selection(id_exp_1, id_exp_2) => (
            selection(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::Deduction(id_exp_1, id_exp_2) => (
            deduction(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::Induction(id_exp_1, id_exp_2) => (
            induction(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::Exemplification(id_exp_1, id_exp_2) => (
            exemplification(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::Abduction(id_exp_1, id_exp_2) => (
            abduction(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::Conversion(id_exp) => {
            (conversion(experience_base, id_exp)?, id_exp, id_exp)
        }
    };

    Ok((result, id_exp_1, id_exp_2))
}

pub fn print_inference_result(
    experience_base: &ExperienceBase,
    inference_instruction: InferenceInstruction,
) -> Result<String, &str> {
    let ((stmt, truth_value), id_exp_1, id_exp_2) =
        execute_inference(experience_base, inference_instruction)?;

    let exp1 = experience_base
        .experiences
        .iter()
        .find(|experience| experience.id == id_exp_1)
        .ok_or("First id not found in experience base.")?;
    let exp2 = experience_base
        .experiences
        .iter()
        .find(|experience| experience.id == id_exp_2)
        .ok_or("Second id not found in experience base.")?;

    if id_exp_1 != id_exp_2 {
        Ok(format!(
            "  {}\n  {}\n  RESULT: {} {}",
            exp1, exp2, stmt, truth_value
        ))
    } else {
        Ok(format!("  {}\n  RESULT: {} {}", exp1, stmt, truth_value))
    }
}

pub fn infer_and_update(
    experience_base: &mut ExperienceBase,
    inference_instruction: InferenceInstruction,
) -> Result<String, &str> {
    let clone_experience_base = experience_base.clone();
    let ((stmt, truth_value), id_exp_1, id_exp_2) =
        match execute_inference(&clone_experience_base, inference_instruction) {
            Ok(result) => result,
            Err(_err) => return Err("Error while executing inference."),
        };

    let new_id = experience_base.get_next_id();
    experience_base.add(ExperienceElement::new_with_truth_value(
        stmt.clone(),
        new_id,
        truth_value.clone(),
    ));

    let exp1 = experience_base
        .experiences
        .iter()
        .find(|experience| experience.id == id_exp_1)
        .ok_or("First id not found in experience base.")?;
    let exp2 = experience_base
        .experiences
        .iter()
        .find(|experience| experience.id == id_exp_2)
        .ok_or("Second id not found in experience base.")?;

    if id_exp_1 != id_exp_2 {
        Ok(format!(
            "  {}\n  {}\n  RESULT: {} {}",
            exp1, exp2, stmt, truth_value
        ))
    } else {
        Ok(format!("  {}\n  RESULT: {} {}", exp1, stmt, truth_value))
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        experience::experience_element::ExperienceElement, parser::statement::Statement,
    };

    use crate::models::{
        experience::experience_base::ExperienceBase, semantics::truth_value::TruthValue,
    };

    #[test]
    fn test_experience_base_new() {
        let experience_base = ExperienceBase::new();
        assert_eq!(experience_base.experiences.len(), 0);
    }

    #[test]
    fn test_revision() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new(Statement::new("d is e").unwrap(), 1);
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            2,
            TruthValue::new_from_str("<0.8, 0.89>").unwrap(),
        );
        let experience_3 = ExperienceElement::new_with_truth_value(
            Statement::new("a is b").unwrap(),
            3,
            TruthValue::new_from_str("<0.9, 0.99>").unwrap(),
        );

        experience_base.add(experience_1);
        experience_base.add(experience_2);
        experience_base.add(experience_3);

        assert_eq!(experience_base.experiences.len(), 3);

        let result = revision(&experience_base, 1, 2).unwrap();
        assert_eq!(result.0, Statement::new("d is e").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<9.54, 0.99>")
                .unwrap()
                .to_string()
        );

        let result = revision(&experience_base, 1, 3);
        assert_eq!(result, Err("Revision not possible."));
    }

    #[test]
    fn test_selection() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            1,
            TruthValue::new_from_str("<0.5, 0.89>").unwrap(),
        );
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            2,
            TruthValue::new_from_str("<0.8, 0.89>").unwrap(),
        );
        let experience_3 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            3,
            TruthValue::new_from_str("<0.3, 0.95>").unwrap(),
        );
        let experience_4 = ExperienceElement::new_with_truth_value(
            Statement::new("a is b").unwrap(),
            4,
            TruthValue::new_from_str("<0.9, 0.99>").unwrap(),
        );

        experience_base.add(experience_1);
        experience_base.add(experience_2);
        experience_base.add(experience_3);
        experience_base.add(experience_4);

        assert_eq!(experience_base.experiences.len(), 4);

        let result = selection(&experience_base, 1, 2).unwrap();
        assert_eq!(result.0, Statement::new("d is e").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.8, 0.89>").unwrap().to_string()
        );

        let result = selection(&experience_base, 1, 3).unwrap();
        assert_eq!(result.0, Statement::new("d is e").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.3, 0.95>").unwrap().to_string()
        );

        let result = selection(&experience_base, 1, 4).unwrap();
        assert_eq!(result.0, Statement::new("a is b").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.9, 0.99>").unwrap().to_string()
        );
    }

    #[test]
    fn test_deduction() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("d is e").unwrap(),
            1,
            TruthValue::new_from_str("<0.5, 0.89>").unwrap(),
        );
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("e is f").unwrap(),
            2,
            TruthValue::new_from_str("<0.8, 0.89>").unwrap(),
        );
        let experience_3 = ExperienceElement::new_with_truth_value(
            Statement::new("a is b").unwrap(),
            3,
            TruthValue::new_from_str("<0.9, 0.99>").unwrap(),
        );

        experience_base.add(experience_1);
        experience_base.add(experience_2);
        experience_base.add(experience_3);

        assert_eq!(experience_base.experiences.len(), 3);

        let result = deduction(&experience_base, 1, 2).unwrap();
        assert_eq!(result.0, Statement::new("d is f").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.40, 0.32>")
                .unwrap()
                .to_string()
        );

        let result = deduction(&experience_base, 1, 3);
        assert_eq!(result, Err("Deduction not possible."));
    }

    #[test]
    fn test_induction() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("a is b").unwrap(),
            1,
            TruthValue::new_from_str("<0.5, 0.89>").unwrap(),
        );
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("a is c").unwrap(),
            2,
            TruthValue::new_from_str("<0.8, 0.89>").unwrap(),
        );
        let experience_3 = ExperienceElement::new_with_truth_value(
            Statement::new("b is c").unwrap(),
            3,
            TruthValue::new_from_str("<0.9, 0.99>").unwrap(),
        );

        experience_base.add(experience_1);
        experience_base.add(experience_2);
        experience_base.add(experience_3);

        assert_eq!(experience_base.experiences.len(), 3);

        let result = induction(&experience_base, 1, 2).unwrap();
        assert_eq!(result.0, Statement::new("c is b").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.50, 0.39>")
                .unwrap()
                .to_string()
        );

        let result = induction(&experience_base, 1, 3);
        assert_eq!(result, Err("Induction not possible."));
    }

    #[test]
    fn test_exemplification() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("a is b").unwrap(),
            1,
            TruthValue::new_from_str("<0.5, 0.89>").unwrap(),
        );
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("a is c").unwrap(),
            2,
            TruthValue::new_from_str("<0.8, 0.89>").unwrap(),
        );
        let experience_3 = ExperienceElement::new_with_truth_value(
            Statement::new("b is c").unwrap(),
            3,
            TruthValue::new_from_str("<0.9, 0.99>").unwrap(),
        );

        experience_base.add(experience_1);
        experience_base.add(experience_2);
        experience_base.add(experience_3);

        assert_eq!(experience_base.experiences.len(), 3);

        let result = exemplification(&experience_base, 1, 3).unwrap();
        assert_eq!(result.0, Statement::new("c is a").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<1.00, 0.28>")
                .unwrap()
                .to_string()
        );

        let result = exemplification(&experience_base, 3, 1).unwrap();
        assert_eq!(result.0, Statement::new("c is a").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<1.00, 0.28>")
                .unwrap()
                .to_string()
        );

        let result = exemplification(&experience_base, 1, 2);
        assert_eq!(result, Err("Exemplification not possible."));
    }

    #[test]
    fn test_abduction() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("a is b").unwrap(),
            1,
            TruthValue::new_from_str("<0.5, 0.89>").unwrap(),
        );
        let experience_2 = ExperienceElement::new_with_truth_value(
            Statement::new("c is b").unwrap(),
            2,
            TruthValue::new_from_str("<0.8, 0.89>").unwrap(),
        );
        let experience_3 = ExperienceElement::new_with_truth_value(
            Statement::new("d is c").unwrap(),
            3,
            TruthValue::new_from_str("<0.9, 0.99>").unwrap(),
        );

        experience_base.add(experience_1);
        experience_base.add(experience_2);
        experience_base.add(experience_3);

        assert_eq!(experience_base.experiences.len(), 3);

        let result = abduction(&experience_base, 1, 2).unwrap();
        assert_eq!(result.0, Statement::new("c is a").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<0.80, 0.28>")
                .unwrap()
                .to_string()
        );

        let result = abduction(&experience_base, 1, 3);
        assert_eq!(result, Err("Abduction not possible."));
    }

    #[test]
    fn test_conversion() {
        let mut experience_base = ExperienceBase::new();
        let experience_1 = ExperienceElement::new_with_truth_value(
            Statement::new("a is b").unwrap(),
            1,
            TruthValue::new_from_str("<0.5, 0.89>").unwrap(),
        );

        experience_base.add(experience_1);

        assert_eq!(experience_base.experiences.len(), 1);

        let result = conversion(&experience_base, 1).unwrap();
        assert_eq!(result.0, Statement::new("b is a").unwrap());
        assert_eq!(
            result.1.to_string(),
            TruthValue::new_from_str("<1.00, 0.31>")
                .unwrap()
                .to_string()
        );
    }
}
