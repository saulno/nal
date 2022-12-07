#[derive(Debug, PartialEq, Eq)]
pub enum ReplInstruction {
    Help(),
    Exit(),
    Assert(Vec<String>),
    Remove(usize),
    List(),
    Query(Vec<String>),
    Infer(Vec<String>),
    InferUpdate(Vec<String>),
    Clear(),
    File(String),
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
            "/assert" | "/a" => Ok(ReplInstruction::Assert(instructions[1..].to_vec())),
            "/remove" | "/r" => {
                // catch error if not a number
                match instructions[1].parse::<usize>() {
                    Ok(id) => Ok(ReplInstruction::Remove(id)),
                    Err(_) => Err("Invalid id: Expected a number"),
                }
            }
            "/query" | "/q" => Ok(ReplInstruction::Query(instructions[1..].to_vec())),
            "/infer" | "/i" => Ok(ReplInstruction::Infer(instructions[1..].to_vec())),
            "/infer+" | "/i+" => Ok(ReplInstruction::InferUpdate(instructions[1..].to_vec())),
            "/file" | "/f" => Ok(ReplInstruction::File(instructions[1].to_string())),
            _ => Ok(ReplInstruction::Unknown()),
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_help() {
        assert_eq!(
            ReplInstruction::new(&vec!["/help".to_string()]).unwrap(),
            ReplInstruction::Help()
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/h".to_string()]).unwrap(),
            ReplInstruction::Help()
        );
    }

    #[test]
    fn test_new_exit() {
        assert_eq!(
            ReplInstruction::new(&vec!["/exit".to_string()]).unwrap(),
            ReplInstruction::Exit()
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/e".to_string()]).unwrap(),
            ReplInstruction::Exit()
        );
    }

    #[test]
    fn test_new_list() {
        assert_eq!(
            ReplInstruction::new(&vec!["/list".to_string()]).unwrap(),
            ReplInstruction::List()
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/l".to_string()]).unwrap(),
            ReplInstruction::List()
        );
    }

    #[test]
    fn test_new_assert() {
        assert_eq!(
            ReplInstruction::new(&vec!["/assert".to_string(), "something".to_string()]).unwrap(),
            ReplInstruction::Assert(vec!["something".to_string()])
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/a".to_string(), "something".to_string()]).unwrap(),
            ReplInstruction::Assert(vec!["something".to_string()])
        );
    }

    #[test]
    fn test_new_clear() {
        assert_eq!(
            ReplInstruction::new(&vec!["/clear".to_string()]).unwrap(),
            ReplInstruction::Clear()
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/c".to_string()]).unwrap(),
            ReplInstruction::Clear()
        );
    }

    #[test]
    fn test_new_remove() {
        assert_eq!(
            ReplInstruction::new(&vec!["/remove".to_string(), "1".to_string()]).unwrap(),
            ReplInstruction::Remove(1)
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/r".to_string(), "1".to_string()]).unwrap(),
            ReplInstruction::Remove(1)
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/r".to_string(), "j".to_string()]),
            Err("Invalid id: Expected a number")
        );
    }

    #[test]
    fn test_new_query() {
        assert_eq!(
            ReplInstruction::new(&vec!["/query".to_string(), "something".to_string()]).unwrap(),
            ReplInstruction::Query(vec!["something".to_string()])
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/q".to_string(), "something".to_string()]).unwrap(),
            ReplInstruction::Query(vec!["something".to_string()])
        );
    }

    #[test]
    fn test_new_infer() {
        assert_eq!(
            ReplInstruction::new(&vec!["/infer".to_string(), "something".to_string()]).unwrap(),
            ReplInstruction::Infer(vec!["something".to_string()])
        );
        assert_eq!(
            ReplInstruction::new(&vec!["/i".to_string(), "something".to_string()]).unwrap(),
            ReplInstruction::Infer(vec!["something".to_string()])
        );
    }
}
