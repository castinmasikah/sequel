crate::dialects::{Dialect, ansi::Ansi};

pub struct Tokenizer<'a> {
    dialect: &'a dyn Dialect,
    pub query: String,
    pub line: u64,
    pub col: u64,
}

impl<'a> Tokenizer<'a> {
    /// Create a new tokenizer
    pub fn new(dialect:&'a dyn Dialect, query: &str) -> Self {
        Tokenizer {
            dialect,
            query:  query.to_string(),
            line: 0,
            col: 0,
        }
    }

    // Goes through the SQL classifying token, returns a list of tokens
    pub fn tokenize(&mut self) {

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_tokenizer() {
        let dialect = Ansi {};
        let tokenizer = Tokenizer::new(&dialect, "SELECT * FROM tablename;");
        assert_eq!(tokenizer.query, "SELECT * FROM tablename;" );
    }
}
