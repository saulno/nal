#[derive(Debug, PartialEq, Eq)]
pub enum InferenceInstruction {
    Transitivity(usize, usize),
}

impl InferenceInstruction {
    pub fn new(args: &[String]) -> Result<InferenceInstruction, &str> {
        match args[0].as_str() {
            "transitivity" | "trans" | "t" => match args[1].parse::<usize>() {
                Ok(id1) => match args[2].parse::<usize>() {
                    Ok(id2) => Ok(InferenceInstruction::Transitivity(id1, id2)),
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
    fn test_new_transitivity() {
        assert_eq!(
            InferenceInstruction::new(&vec![
                "transitivity".to_string(),
                "1".to_string(),
                "2".to_string()
            ])
            .unwrap(),
            InferenceInstruction::Transitivity(1, 2)
        );

        assert_eq!(
            InferenceInstruction::new(&vec!["trans".to_string(), "1".to_string(), "2".to_string()])
                .unwrap(),
            InferenceInstruction::Transitivity(1, 2)
        );

        assert_eq!(
            InferenceInstruction::new(&vec!["t".to_string(), "1".to_string(), "2".to_string()])
                .unwrap(),
            InferenceInstruction::Transitivity(1, 2)
        );

        assert_eq!(
            InferenceInstruction::new(&vec![
                "transitivity".to_string(),
                "1".to_string(),
                "a".to_string()
            ]),
            Err("Invalid inference instruction: Expected <id1> <id2>")
        );
    }
}
