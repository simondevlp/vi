use std::fmt::Display;

use lexer::lexeme;

pub enum Error {
    UnexpectedLexeme {
        expected: Vec<lexeme::Kind>,
        found: lexeme::Kind,
    },
    UnexpectedToken {
        expected: Vec<String>,
        found: String,
    },
}

impl Diags {
    fn print_vec<T: Display + Sized>(vec: &Vec<T>) -> String {
        let mut result = String::new();
        for i in 0..vec.len() {
            result.push_str(&format!("{}", vec[i]).as_str());
            if i < vec.len() - 1 {
                result.push_str(", ");
            }
        }
        result
    }

    pub fn print(&self) {
        match self {
            Diags::Err(error) => match error {
                Error::UnexpectedLexeme { expected, found } => {
                    eprintln!(
                        "Unexpected lexeme: expected {}, found {}",
                        Self::print_vec(expected),
                        found
                    );
                }
                Error::UnexpectedToken { expected, found } => {
                    eprintln!(
                        "Unexpected token: expected {}, found {}",
                        Self::print_vec(expected),
                        found
                    );
                }
            },
        }
    }
}

pub enum Diags {
    Err(Error),
}
