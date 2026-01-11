use std::fmt::Display;

pub enum BracketKind {
    Parenthesis,
}

impl Display for BracketKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parenthesis => write!(f, "parenthesis"),
        }
    }
}

pub enum Error {
    MiscExpecting { expected: String },
    BracketNotClosed { kind: BracketKind },
}

impl Display for DiagData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Err(error) => match error {
                Error::MiscExpecting { expected } => {
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
    pub data: DiagData,
}

impl Display for Diag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "On line {}: {}", self.line, self.data)
    }
}
