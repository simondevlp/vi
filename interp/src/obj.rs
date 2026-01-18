use std::fmt::Display;

use crate::{
    Evaluator,
    diag::{Diag, DiagData, EvalError},
    eval::Evaluable,
};

#[derive(Clone)]
pub struct NumericalObj(pub f64);

impl Display for NumericalObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
pub struct StringObj(pub &'static str);

impl Display for StringObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

#[derive(Clone)]
pub struct TupleObj(pub Vec<Object>);

impl Display for TupleObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements: Vec<String> = self.0.iter().map(|v| format!("{}", v)).collect();
        write!(f, "({})", elements.join(", "))
    }
}

#[derive(Clone)]
pub enum Object {
    Undefined,
    Infinity { positive: bool },
    Numerical(NumericalObj),
    String(StringObj),
    Tuple(TupleObj),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::Undefined => "undefined".to_string(),
                Object::Numerical(num) => format!("{}", num),
                Object::Infinity { positive } => {
                    if *positive {
                        "+inf".to_string()
                    } else {
                        "-inf".to_string()
                    }
                }
                Object::String(s) => format!("{}", s),
                Object::Tuple(t) => format!("{}", t),
            }
        )
    }
}

impl Object {
    fn to_string(&self) -> String {
        match self {
            Self::Undefined => "an undefined value".to_string(),
            Object::Numerical(_) | Object::Infinity { .. } => "a number".to_string(),
            Object::String(_) => "a string".to_string(),
            Object::Tuple(_) => "a tuple".to_string(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    NegativePrefix,
}

impl Display for OperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            OperationKind::Add => "add",
            OperationKind::Subtract => "subtract",
            OperationKind::Multiply => "multiply",
            OperationKind::Divide => "divide",
            OperationKind::NegativePrefix => "negative prefix",
        };
        write!(f, "{}", op_str)
    }
}

pub struct Operation {
    pub kind: OperationKind,
    pub operands: (Object, Object),
}

impl Evaluable for Operation {
    fn evaluate(&self, eval: &Evaluator) -> Result<Object, Diag> {
        match &self.kind {
            OperationKind::Add => Ok(Object::Numerical(NumericalObj(match &self.operands {
                (Object::Numerical(op1), Object::Numerical(op2)) => op1.0 + op2.0,
                _ => {
                    return Err(Diag {
                        line: eval.cur_line(),
                        data: DiagData::EvalError(EvalError::UndefinedOperation {
                            op: self.kind,
                            operand: format!(
                                "{} and {}",
                                self.operands.0.to_string(),
                                self.operands.1.to_string()
                            ),
                        }),
                    });
                }
            }))),
            OperationKind::Subtract => Ok(Object::Numerical(NumericalObj(match &self.operands {
                (Object::Numerical(op1), Object::Numerical(op2)) => op1.0 - op2.0,
                _ => {
                    return Err(Diag {
                        line: eval.cur_line(),
                        data: DiagData::EvalError(EvalError::UndefinedOperation {
                            op: self.kind,
                            operand: format!(
                                "{} and {}",
                                self.operands.0.to_string(),
                                self.operands.1.to_string()
                            ),
                        }),
                    });
                }
            }))),
            OperationKind::Multiply => Ok(Object::Numerical(NumericalObj(match &self.operands {
                (Object::Numerical(op1), Object::Numerical(op2)) => op1.0 * op2.0,
                _ => {
                    return Err(Diag {
                        line: eval.cur_line(),
                        data: DiagData::EvalError(EvalError::UndefinedOperation {
                            op: self.kind,
                            operand: format!(
                                "{} and {}",
                                self.operands.0.to_string(),
                                self.operands.1.to_string()
                            ),
                        }),
                    });
                }
            }))),
            OperationKind::Divide => Ok(Object::Numerical(NumericalObj(match &self.operands {
                (Object::Numerical(op1), Object::Numerical(op2)) => {
                    if op2.0 == 0. {
                        return Ok(Object::Infinity {
                            positive: op1.0.is_sign_positive(),
                        });
                    } else {
                        op1.0 / op2.0
                    }
                }
                _ => {
                    return Err(Diag {
                        line: eval.cur_line(),
                        data: DiagData::EvalError(EvalError::UndefinedOperation {
                            op: self.kind,
                            operand: format!(
                                "{} and {}",
                                self.operands.0.to_string(),
                                self.operands.1.to_string()
                            ),
                        }),
                    });
                }
            }))),
            OperationKind::NegativePrefix => {
                Ok(Object::Numerical(NumericalObj(match &self.operands {
                    (Object::Numerical(_), Object::Numerical(op2)) => -op2.0,
                    _ => {
                        return Err(Diag {
                            line: eval.cur_line(),
                            data: DiagData::EvalError(EvalError::UndefinedOperation {
                                op: self.kind,
                                operand: format!("{}", self.operands.1.to_string()),
                            }),
                        });
                    }
                })))
            }
        }
    }
}
