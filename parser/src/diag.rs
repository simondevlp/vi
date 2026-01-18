use std::fmt::Display;

use lexer::lexeme;

pub enum BracketKind {
    Parenthesis,
    Brace,
    Bracket,
}

impl Display for BracketKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parenthesis => write!(f, "parenthesis"),
            Self::Brace => write!(f, "brace"),
            Self::Bracket => write!(f, "bracket"),
        }
    }
}

pub enum Error {
    UnexpectedLexeme {
        expected: Vec<lexeme::Kind>,
        found: lexeme::Kind,
    },
    Expecting {
        expected: String,
    },
    BracketNotClosed {
        kind: BracketKind,
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
                Error::Expecting { expected } => {
                    write!(f, "Expecting {}", expected)
                }
                Error::BracketNotClosed { kind } => {
                    write!(f, "This {} has not been closed.", kind)
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
    pub span: (u32, u32),
    pub data: DiagData,
}

impl Display for Diag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "On line {}: {}", self.line, self.data)
    }
}
