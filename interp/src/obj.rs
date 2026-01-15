use std::fmt::Display;

use crate::{
    Interpreter,
    diag::{Diag, DiagData, EvalError},
    eval::Evaluable,
};

pub struct NumericalObj(pub f64);

impl Display for NumericalObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct StringObj(pub String);

impl Display for StringObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

pub struct TupleObj(pub Vec<ValueObjKind>);

impl Display for TupleObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements: Vec<String> = self.0.iter().map(|v| format!("{}", v)).collect();
        write!(f, "({})", elements.join(", "))
    }
}

pub enum ValueObjKind {
    Numerical(NumericalObj),
    String(StringObj),
    Tuple(TupleObj),
}

impl Display for ValueObjKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueObjKind::Numerical(num) => write!(f, "{}", num),
            ValueObjKind::String(s) => write!(f, "{}", s),
            ValueObjKind::Tuple(tup) => write!(f, "{}", tup),
        }
    }
}

impl ValueObjKind {
    fn to_string(&self) -> String {
        match self {
            ValueObjKind::Numerical(_) => "a number",
            ValueObjKind::String(_) => "a string",
            ValueObjKind::Tuple(_) => "a tuple",
        }
        .to_string()
    }
}

#[derive(Clone, Copy)]
pub enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct Operation {
    pub kind: OperationKind,
    pub operands: (ValueObjKind, ValueObjKind),
}

impl Evaluable for Operation {
    fn evaluate(&self, interpreter: &Interpreter) -> Result<ValueObjKind, Diag> {
        match &self.kind {
            OperationKind::Add => match &self.operands.0 {
                ValueObjKind::Numerical(op1) => match &self.operands.1 {
                    ValueObjKind::Numerical(op2) => {
                        Ok(ValueObjKind::Numerical(NumericalObj(op1.0 + op2.0)))
                    }
                    _ => Err(Diag {
                        line: interpreter.cur_line(),
                        data: DiagData::EvalError(EvalError::UndefinedOperation {
                            op: self.kind,
                            operand: self.operands.1.to_string(),
                        }),
                    }),
                },
                _ => Err(Diag {
                    line: interpreter.cur_line(),
                    data: DiagData::EvalError(EvalError::UndefinedOperation {
                        op: self.kind,
                        operand: self.operands.0.to_string(),
                    }),
                }),
            },
            OperationKind::Subtract => match &self.operands.0 {
                ValueObjKind::Numerical(op1) => match &self.operands.1 {
                    ValueObjKind::Numerical(op2) => {
                        Ok(ValueObjKind::Numerical(NumericalObj(op1.0 - op2.0)))
                    }
                    _ => Err(Diag {
                        line: interpreter.cur_line(),
                        data: DiagData::EvalError(EvalError::UndefinedOperation {
                            op: self.kind,
                            operand: self.operands.1.to_string(),
                        }),
                    }),
                },
                _ => Err(Diag {
                    line: interpreter.cur_line(),
                    data: DiagData::EvalError(EvalError::UndefinedOperation {
                        op: self.kind,
                        operand: self.operands.0.to_string(),
                    }),
                }),
            },
            OperationKind::Multiply => match &self.operands.0 {
                ValueObjKind::Numerical(op1) => match &self.operands.1 {
                    ValueObjKind::Numerical(op2) => {
                        Ok(ValueObjKind::Numerical(NumericalObj(op1.0 * op2.0)))
                    }
                    _ => Err(Diag {
                        line: interpreter.cur_line(),
                        data: DiagData::EvalError(EvalError::UndefinedOperation {
                            op: self.kind,
                            operand: self.operands.1.to_string(),
                        }),
                    }),
                },
                _ => Err(Diag {
                    line: interpreter.cur_line(),
                    data: DiagData::EvalError(EvalError::UndefinedOperation {
                        op: self.kind,
                        operand: self.operands.0.to_string(),
                    }),
                }),
            },
            OperationKind::Divide => match &self.operands.0 {
                ValueObjKind::Numerical(op1) => match &self.operands.1 {
                    ValueObjKind::Numerical(op2) => {
                        Ok(ValueObjKind::Numerical(NumericalObj(op1.0 / op2.0)))
                    }
                    _ => Err(Diag {
                        line: interpreter.cur_line(),
                        data: DiagData::EvalError(EvalError::UndefinedOperation {
                            op: self.kind,
                            operand: self.operands.1.to_string(),
                        }),
                    }),
                },
                _ => Err(Diag {
                    line: interpreter.cur_line(),
                    data: DiagData::EvalError(EvalError::UndefinedOperation {
                        op: self.kind,
                        operand: self.operands.0.to_string(),
                    }),
                }),
            },
        }
    }
}

impl Display for OperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            OperationKind::Add => "+",
            OperationKind::Subtract => "-",
            OperationKind::Multiply => "*",
            OperationKind::Divide => "/",
        };
        write!(f, "{}", op_str)
    }
}
