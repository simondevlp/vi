use std::fmt::Display;

use crate::obj::OperationKind;

pub struct Diag {
    pub line: u32,
    pub data: DiagData,
}

impl Display for Diag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {}: {}", self.line, self.data)
    }
}

pub enum DiagData {
    ParseError(parser::diag::Diag),
    EvalError(EvalError),
}

impl Display for DiagData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(diag) => write!(f, "Parse error: {}", diag),
            Self::EvalError(err) => write!(f, "Evaluation error: {}", err),
        }
    }
}

pub enum EvalError {
    MalformedLiteral { lit: String },
    NotFoundInScope { name: String },
    AlreadyDeclaredInScope { name: String },
    UndefinedOperation { op: OperationKind, operand: String },
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedLiteral { lit } => {
                write!(f, "Malformed literal: {}", lit)
            }
            Self::NotFoundInScope { name } => {
                write!(f, "Identifier '{}' is not found in scope", name)
            }
            Self::AlreadyDeclaredInScope { name } => {
                write!(f, "Identifier '{}' is already declared in scope", name)
            }
            Self::UndefinedOperation { op, operand } => {
                write!(f, "Undefined operation: {} with {}", op, operand)
            }
        }
    }
}
