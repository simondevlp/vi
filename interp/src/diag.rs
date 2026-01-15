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
            DiagData::ParseError(diag) => write!(f, "Parse error: {}", diag),
            DiagData::EvalError(err) => write!(f, "Evaluation error: {}", err),
        }
    }
}

pub enum EvalError {
    MalformedLiteral(String),
    UndefinedOperation { op: OperationKind, operand: String },
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::MalformedLiteral(lit) => {
                write!(f, "Malformed literal: {}", lit)
            }
            EvalError::UndefinedOperation { op, operand } => {
                write!(f, "Undefined operation: {} with {}", op, operand)
            }
        }
    }
}
