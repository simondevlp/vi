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
    pub fn print(&self) {
        match self {
            Diags::Err(error) => match error {
                Error::UnexpectedLexeme { expected, found } => {
                    eprintln!(
                        "Unexpected lexeme: expected {:?}, found {:?}",
                        expected, found
                    );
                }
                Error::UnexpectedToken { expected, found } => {
                    eprintln!(
                        "Unexpected token: expected {:?}, found {:?}",
                        expected, found
                    );
                }
            },
        }
    }
}

pub enum Diags {
    Err(Error),
}
