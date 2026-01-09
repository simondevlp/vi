use std::fmt::Display;

use lexer::lexeme;

pub struct TokenString(pub String);

impl TokenString {
    pub fn from_str(s: &str) -> Self {
        TokenString(s.to_string())
    }
}

impl Display for TokenString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.0)
    }
}

pub enum Error {
    UnexpectedLexeme {
        expected: Vec<lexeme::Kind>,
        found: lexeme::Kind,
    },
    UnexpectedToken {
        expected: Vec<TokenString>,
        found: TokenString,
    },
}

impl DiagData {
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
}

impl Display for DiagData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Err(error) => match error {
                Error::UnexpectedLexeme { expected, found } => {
                    write!(
                        f,
                        "Unexpected lexeme: expected {}, found {}",
                        Self::print_vec(expected),
                        found
                    )
                }
                Error::UnexpectedToken { expected, found } => {
                    write!(
                        f,
                        "Unexpected token: expected {}, found {}",
                        Self::print_vec(expected),
                        found
                    )
                }
            },
        }
    }
}

pub enum DiagData {
    Err(Error),
}

pub struct Diag {
    pub line: u32,
    pub data: DiagData,
}

impl Display for Diag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "On line {}: {}", self.line, self.data)
    }
}
