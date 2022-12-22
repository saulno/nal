#[derive(Debug, PartialEq, Eq)]
pub enum InferenceInstruction {
    Revision(usize, usize),
    Choice(usize, usize),
    // for inheritance
    Deduction(usize, usize),
    Induction(usize, usize),
    Exemplification(usize, usize),
    Abduction(usize, usize),
    Conversion(usize),
    // for similarity
    Comparison(usize, usize),
    Analogy(usize, usize),
    Resemblance(usize, usize),
}

impl InferenceInstruction {
    pub fn new(args: &[String]) -> Result<InferenceInstruction, &str> {
        match args[0].as_str() {
            "revision" | "rev" | "r" => match args[1].parse::<usize>() {
                Ok(id1) => match args[2].parse::<usize>() {
                    Ok(id2) => Ok(InferenceInstruction::Revision(id1, id2)),
                    Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
                },
                Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
            },
            "choice" | "cho" | "ch" => match args[1].parse::<usize>() {
                Ok(id1) => match args[2].parse::<usize>() {
                    Ok(id2) => Ok(InferenceInstruction::Choice(id1, id2)),
                    Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
                },
                Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
            },
            "deduction" | "ded" | "d" => match args[1].parse::<usize>() {
                Ok(id1) => match args[2].parse::<usize>() {
                    Ok(id2) => Ok(InferenceInstruction::Deduction(id1, id2)),
                    Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
                },
                Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
            },
            "induction" | "ind" | "i" => match args[1].parse::<usize>() {
                Ok(id1) => match args[2].parse::<usize>() {
                    Ok(id2) => Ok(InferenceInstruction::Induction(id1, id2)),
                    Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
                },
                Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
            },
            "exemplification" | "exe" | "e" => match args[1].parse::<usize>() {
                Ok(id1) => match args[2].parse::<usize>() {
                    Ok(id2) => Ok(InferenceInstruction::Exemplification(id1, id2)),
                    Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
                },
                Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
            },
            "abduction" | "abd" | "a" => match args[1].parse::<usize>() {
                Ok(id1) => match args[2].parse::<usize>() {
                    Ok(id2) => Ok(InferenceInstruction::Abduction(id1, id2)),
                    Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
                },
                Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
            },
            "conversion" | "cnv" | "c" => match args[1].parse::<usize>() {
                Ok(id) => Ok(InferenceInstruction::Conversion(id)),
                Err(_) => Err("Invalid inference instruction: Expected <id>"),
            },
            "comparison" | "com" => match args[1].parse::<usize>() {
                Ok(id1) => match args[2].parse::<usize>() {
                    Ok(id2) => Ok(InferenceInstruction::Comparison(id1, id2)),
                    Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
                },
                Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
            },
            "analogy" | "ana" => match args[1].parse::<usize>() {
                Ok(id1) => match args[2].parse::<usize>() {
                    Ok(id2) => Ok(InferenceInstruction::Analogy(id1, id2)),
                    Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
                },
                Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
            },
            "resemblance" | "res" => match args[1].parse::<usize>() {
                Ok(id1) => match args[2].parse::<usize>() {
                    Ok(id2) => Ok(InferenceInstruction::Resemblance(id1, id2)),
                    Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
                },
                Err(_) => Err("Invalid inference instruction: Expected <id1> <id2>"),
            },
            _ => Err("Invalid inference instruction"),
        }
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_inference() {
        assert_eq!(
            InferenceInstruction::new(&vec![
                "revision".to_string(),
                "1".to_string(),
                "a".to_string()
            ]),
            Err("Invalid inference instruction: Expected <id1> <id2>")
        );

        assert_eq!(
            InferenceInstruction::new(&vec![
                "revision".to_string(),
                "1".to_string(),
                "2".to_string()
            ])
            .unwrap(),
            InferenceInstruction::Revision(1, 2)
        );

        assert_eq!(
            InferenceInstruction::new(&vec![
                "choice".to_string(),
                "1".to_string(),
                "2".to_string()
            ])
            .unwrap(),
            InferenceInstruction::Choice(1, 2)
        );

        assert_eq!(
            InferenceInstruction::new(&vec![
                "deduction".to_string(),
                "1".to_string(),
                "2".to_string()
            ])
            .unwrap(),
            InferenceInstruction::Deduction(1, 2)
        );
    }
}
