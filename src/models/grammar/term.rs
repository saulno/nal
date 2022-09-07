pub struct  Term {
    pub word: String,
}

impl Term {
    // Create new term
    pub fn new(word: &str) -> Result<Term, &str> {
        if word.contains(" ") {
            return Err("Term can't contain whitespaces");
        } 

        if word.eq("") {
            return Err("Term can't be empty");
        }

        if word.eq("?") {
            return Err("Term can't be a question mark (?)");
        }

        Ok(Term {
            word: word.to_string(),
        })
    }
}