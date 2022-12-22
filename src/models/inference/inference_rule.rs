use crate::models::{
    experience::{experience_base::ExperienceBase, experience_element::ExperienceElement},
    parser::statement::Statement,
    semantics::truth_value::TruthValue,
};

use super::{
    abduction::abduction, analogy::analogy, choice::choice, comparison::comparison,
    conversion::conversion, deduction::deduction, difference_extension::difference_extension,
    difference_intension::difference_intension, exemplification::exemplification,
    induction::induction, inference_instruction::InferenceInstruction,
    intersection_extension::intersection_extension, intersection_intension::intersection_intension,
    resemblance::resemblance, revision::revision, union_extension::union_extension,
    union_intension::union_intension,
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
        InferenceInstruction::Choice(id_exp_1, id_exp_2) => (
            choice(experience_base, id_exp_1, id_exp_2)?,
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
        InferenceInstruction::Comparison(id_exp_1, id_exp_2) => (
            comparison(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::Analogy(id_exp_1, id_exp_2) => (
            analogy(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::Resemblance(id_exp_1, id_exp_2) => (
            resemblance(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::UnionExtension(id_exp_1, id_exp_2) => (
            union_extension(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::IntersectionExtension(id_exp_1, id_exp_2) => (
            intersection_extension(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::DifferenceExtension(id_exp_1, id_exp_2) => (
            difference_extension(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::UnionIntension(id_exp_1, id_exp_2) => (
            union_intension(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::IntersectionIntension(id_exp_1, id_exp_2) => (
            intersection_intension(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
        InferenceInstruction::DifferenceIntension(id_exp_1, id_exp_2) => (
            difference_intension(experience_base, id_exp_1, id_exp_2)?,
            id_exp_1,
            id_exp_2,
        ),
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

    use crate::models::experience::experience_base::ExperienceBase;

    #[test]
    fn test_experience_base_new() {
        let experience_base = ExperienceBase::new();
        assert_eq!(experience_base.experiences.len(), 0);
    }
}
