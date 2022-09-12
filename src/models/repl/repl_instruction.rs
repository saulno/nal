#[derive(Debug, PartialEq)]
pub enum ReplInstruction {
    Help(),
    Exit(),
    Insert(Vec<String>),
    Remove(usize),
    List(),
    Query(Vec<String>),
    Clear(),
    Unknown(),
}

impl ReplInstruction {
    // Create new repl
    pub fn new(instructions: &[String]) -> Result<ReplInstruction, &str> {
        match instructions[0].as_str() {
            "/help" | "/h" => Ok(ReplInstruction::Help()),
            "/exit" | "/e" => Ok(ReplInstruction::Exit()),
            "/list" | "/l" => Ok(ReplInstruction::List()),
            "/clear" | "/c" => Ok(ReplInstruction::Clear()),
            "/insert" | "/i" => Ok(ReplInstruction::Insert(instructions[1..].to_vec())),
            "/remove" | "/r" => Ok(ReplInstruction::Remove(
                instructions[1].parse::<usize>().unwrap(),
            )),
            "/query" | "/q" => Ok(ReplInstruction::Query(instructions[1..].to_vec())),
            _ => Ok(ReplInstruction::Unknown()),
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            ReplInstruction::new(&vec!["/help".to_string()]).unwrap(),
            ReplInstruction::Help()
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/h".to_string()]).unwrap(),
            ReplInstruction::Help()
        );

        assert_eq!(
            ReplInstruction::new(&vec!["/exit".to_string()]).unwrap(),
            ReplInstruction::Exit()
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/e".to_string()]).unwrap(),
            ReplInstruction::Exit()
        );

        assert_eq!(
            ReplInstruction::new(&vec!["/list".to_string()]).unwrap(),
            ReplInstruction::List()
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/l".to_string()]).unwrap(),
            ReplInstruction::List()
        );

        assert_eq!(
            ReplInstruction::new(&vec!["/clear".to_string()]).unwrap(),
            ReplInstruction::Clear()
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/c".to_string()]).unwrap(),
            ReplInstruction::Clear()
        );

        assert_eq!(
            ReplInstruction::new(&vec!["/insert".to_string(), "something".to_string()]).unwrap(),
            ReplInstruction::Insert(vec!["something".to_string()])
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/i".to_string(), "something".to_string()]).unwrap(),
            ReplInstruction::Insert(vec!["something".to_string()])
        );

        assert_eq!(
            ReplInstruction::new(&vec!["/remove".to_string(), "1".to_string()]).unwrap(),
            ReplInstruction::Remove(1)
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/r".to_string(), "1".to_string()]).unwrap(),
            ReplInstruction::Remove(1)
        );

        assert_eq!(
            ReplInstruction::new(&vec!["/query".to_string(), "something".to_string()]).unwrap(),
            ReplInstruction::Query(vec!["something".to_string()])
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/q".to_string(), "something".to_string()]).unwrap(),
            ReplInstruction::Query(vec!["something".to_string()])
        );

        assert_eq!(
            ReplInstruction::new(&vec!["test".to_string()]).unwrap(),
            ReplInstruction::Unknown()
        );
    }
}
