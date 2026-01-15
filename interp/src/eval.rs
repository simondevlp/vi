use parser::syntax::expr;

use crate::{
    Interpreter,
    diag::{Diag, DiagData, EvalError},
    obj::{NumericalObj, Operation, OperationKind, TupleObj, ValueObjKind},
};

pub trait Evaluable {
    fn evaluate(&self, interpreter: &Interpreter) -> Result<ValueObjKind, Diag>;
}

impl Evaluable for expr::Expr {
    fn evaluate(&self, interpreter: &Interpreter) -> Result<ValueObjKind, Diag> {
        self.0.evaluate(interpreter)
    }
}

impl Evaluable for expr::AddAffixedExpr {
    fn evaluate(&self, interpreter: &Interpreter) -> Result<ValueObjKind, Diag> {
        match &self.lhs {
            Some(op1) => Operation {
                kind: if self.rhs.0 {
                    OperationKind::Add
                } else {
                    OperationKind::Subtract
                },
                operands: (
                    op1.evaluate(interpreter)?,
                    self.rhs.1.evaluate(interpreter)?,
                ),
            }
            .evaluate(interpreter),
            None => self.rhs.1.evaluate(interpreter),
        }
    }
}

impl Evaluable for expr::MulAffixedExpr {
    fn evaluate(&self, interpreter: &Interpreter) -> Result<ValueObjKind, Diag> {
        match &self.lhs {
            Some(op1) => Operation {
                kind: if self.rhs.0 {
                    OperationKind::Multiply
                } else {
                    OperationKind::Divide
                },
                operands: (
                    op1.evaluate(interpreter)?,
                    self.rhs.1.evaluate(interpreter)?,
                ),
            }
            .evaluate(interpreter),
            None => self.rhs.1.evaluate(interpreter),
        }
    }
}

impl Evaluable for expr::PrefixedExpr {
    fn evaluate(&self, interpreter: &Interpreter) -> Result<ValueObjKind, Diag> {
        match &self.prefix {
            Some(expr::PrefixedExprKind::Minus) => Operation {
                kind: OperationKind::Subtract,
                operands: (
                    ValueObjKind::Numerical(NumericalObj(0.)),
                    self.terminal.evaluate(interpreter)?,
                ),
            }
            .evaluate(interpreter),
            None => self.terminal.evaluate(interpreter),
        }
    }
}

impl Evaluable for expr::TerminalExpr {
    fn evaluate(&self, interpreter: &Interpreter) -> Result<ValueObjKind, Diag> {
        match self {
            expr::TerminalExpr::Literal(lit) => lit.evaluate(interpreter),
            expr::TerminalExpr::Tuple(lit) => lit.evaluate(interpreter),
        }
    }
}

impl Evaluable for expr::TupleExpr {
    fn evaluate(&self, interpreter: &Interpreter) -> Result<ValueObjKind, Diag> {
        match self.0.len() {
            1 => Ok(self.0[0].evaluate(interpreter)?),
            _ => {
                let mut values = Vec::new();
                for expr in &self.0 {
                    let val = expr.evaluate(interpreter)?;
                    values.push(val);
                }
                Ok(ValueObjKind::Tuple(TupleObj(values)))
            }
        }
    }
}

impl Evaluable for expr::terminal::Literal {
    fn evaluate(&self, interpreter: &Interpreter) -> Result<ValueObjKind, Diag> {
        match self {
            expr::terminal::Literal::Decimal(lit) => lit.evaluate(interpreter),
            expr::terminal::Literal::Float(lit) => lit.evaluate(interpreter),
            expr::terminal::Literal::Ident(_) => unimplemented!(),
            expr::terminal::Literal::DoubleQuotedString(_) => unimplemented!(),
        }
    }
}

impl Evaluable for expr::terminal::Decimal {
    fn evaluate(&self, interpreter: &Interpreter) -> Result<ValueObjKind, Diag> {
        let lit = interpreter.parser.get_snippet(&self.0);
        let mut value = 0f64;
        for (i, c) in lit.chars().rev().enumerate() {
            if c == '_' {
                continue;
            }
            let Some(digit) = c.to_digit(10) else {
                return Err(Diag {
                    line: interpreter.cur_line(),
                    data: DiagData::EvalError(EvalError::MalformedLiteral(lit.to_string())),
                });
            };
            value += (digit as f64) * 10f64.powi(i as i32);
        }
        Ok(ValueObjKind::Numerical(NumericalObj(value)))
    }
}

impl Evaluable for expr::terminal::Float {
    fn evaluate(&self, interpreter: &Interpreter) -> Result<ValueObjKind, Diag> {
        let lit = interpreter.parser.get_snippet(&self.0);
        let mut value = 0f64;
        let parts: Vec<&str> = lit.split('.').collect();
        if parts.len() != 2 {
            return Err(Diag {
                line: interpreter.cur_line(),
                data: DiagData::EvalError(EvalError::MalformedLiteral(lit.to_string())),
            });
        }
        let int_part = parts[0];
        let frac_part = parts[1];
        for (i, c) in int_part.chars().rev().enumerate() {
            if c == '_' {
                continue;
            }
            let Some(digit) = c.to_digit(10) else {
                return Err(Diag {
                    line: interpreter.cur_line(),
                    data: DiagData::EvalError(EvalError::MalformedLiteral(lit.to_string())),
                });
            };
            value += (digit as f64) * 10f64.powi(i as i32);
        }
        for (i, c) in frac_part.chars().enumerate() {
            if c == '_' {
                continue;
            }
            let Some(digit) = c.to_digit(10) else {
                return Err(Diag {
                    line: interpreter.cur_line(),
                    data: DiagData::EvalError(EvalError::MalformedLiteral(lit.to_string())),
                });
            };
            value += (digit as f64) * 10f64.powi(-(i as i32 + 1));
        }
        Ok(ValueObjKind::Numerical(NumericalObj(value)))
    }
}
