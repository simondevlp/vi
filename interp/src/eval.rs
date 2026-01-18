use parser::syntax::expr;

use crate::{
    Evaluator,
    diag::{Diag, DiagData, EvalError},
    obj::{NumericalObj, Object, Operation, OperationKind, TupleObj},
};

pub trait Evaluable {
    fn evaluate(&self, eval: &Evaluator) -> Result<Object, Diag>;
}

impl Evaluable for expr::Expr {
    fn evaluate(&self, eval: &Evaluator) -> Result<Object, Diag> {
        self.0.evaluate(eval)
    }
}

impl Evaluable for expr::AddAffixedExpr {
    fn evaluate(&self, eval: &Evaluator) -> Result<Object, Diag> {
        match &self.lhs {
            Some(op1) => Operation {
                kind: if self.rhs.0 {
                    OperationKind::Add
                } else {
                    OperationKind::Subtract
                },
                operands: (op1.evaluate(eval)?, self.rhs.1.evaluate(eval)?),
            }
            .evaluate(eval),
            None => self.rhs.1.evaluate(eval),
        }
    }
}

impl Evaluable for expr::MulAffixedExpr {
    fn evaluate(&self, eval: &Evaluator) -> Result<Object, Diag> {
        match &self.lhs {
            Some(op1) => Operation {
                kind: if self.rhs.0 {
                    OperationKind::Multiply
                } else {
                    OperationKind::Divide
                },
                operands: (op1.evaluate(eval)?, self.rhs.1.evaluate(eval)?),
            }
            .evaluate(eval),
            None => self.rhs.1.evaluate(eval),
        }
    }
}

impl Evaluable for expr::PrefixedExpr {
    fn evaluate(&self, eval: &Evaluator) -> Result<Object, Diag> {
        match &self.prefix {
            Some(expr::PrefixedExprKind::Minus) => Operation {
                kind: OperationKind::NegativePrefix,
                operands: (Object::Undefined, self.expr.evaluate(eval)?),
            }
            .evaluate(eval),
            None => self.expr.evaluate(eval),
        }
    }
}

impl Evaluable for expr::PathExpr {
    fn evaluate(&self, eval: &Evaluator) -> Result<Object, Diag> {
        match self {
            expr::PathExpr::Root(root) => root.evaluate(eval),
            expr::PathExpr::WithFields { .. } => unimplemented!(),
        }
    }
}

impl Evaluable for expr::TerminalExpr {
    fn evaluate(&self, eval: &Evaluator) -> Result<Object, Diag> {
        match self {
            expr::TerminalExpr::Field(field) => field.evaluate(eval),
            expr::TerminalExpr::Literal(lit) => lit.evaluate(eval),
            expr::TerminalExpr::Tuple(lit) => lit.evaluate(eval),
        }
    }
}

impl Evaluable for expr::Field {
    fn evaluate(&self, eval: &Evaluator) -> Result<Object, Diag> {
        if let Some(_) = &self.args {
            unimplemented!()
        } else {
            let name = eval.parser.get_snippet(&self.name.0);
            match eval.current_scope().get(&name) {
                Some(obj) => Ok(obj.clone()),
                None => Err(Diag {
                    line: eval.cur_line(),
                    data: DiagData::EvalError(EvalError::NotFoundInScope { name }),
                }),
            }
        }
    }
}

impl Evaluable for expr::TupleExpr {
    fn evaluate(&self, eval: &Evaluator) -> Result<Object, Diag> {
        match self.0.len() {
            1 => Ok(self.0[0].evaluate(eval)?),
            _ => {
                let mut values = Vec::new();
                for expr in &self.0 {
                    let val = expr.evaluate(eval)?;
                    values.push(val);
                }
                Ok(Object::Tuple(TupleObj(values)))
            }
        }
    }
}

impl Evaluable for expr::terminal::Literal {
    fn evaluate(&self, eval: &Evaluator) -> Result<Object, Diag> {
        match self {
            expr::terminal::Literal::Decimal(lit) => lit.evaluate(eval),
            expr::terminal::Literal::Float(lit) => lit.evaluate(eval),
            expr::terminal::Literal::DoubleQuotedString(_) => unimplemented!(),
        }
    }
}

impl Evaluable for expr::terminal::Decimal {
    fn evaluate(&self, eval: &Evaluator) -> Result<Object, Diag> {
        let lit = eval.parser.get_snippet(&self.0);
        let mut value = 0f64;
        for (i, c) in lit.chars().rev().enumerate() {
            if c == '_' {
                continue;
            }
            let Some(digit) = c.to_digit(10) else {
                return Err(Diag {
                    line: eval.cur_line(),
                    data: DiagData::EvalError(EvalError::MalformedLiteral {
                        lit: lit.to_string(),
                    }),
                });
            };
            value += (digit as f64) * 10f64.powi(i as i32);
        }
        Ok(Object::Numerical(NumericalObj(value)))
    }
}

impl Evaluable for expr::terminal::Float {
    fn evaluate(&self, interpreter: &Evaluator) -> Result<Object, Diag> {
        let lit = interpreter.parser.get_snippet(&self.0);
        let mut value = 0f64;
        let parts: Vec<&str> = lit.split('.').collect();
        if parts.len() != 2 {
            return Err(Diag {
                line: interpreter.cur_line(),
                data: DiagData::EvalError(EvalError::MalformedLiteral {
                    lit: lit.to_string(),
                }),
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
                    data: DiagData::EvalError(EvalError::MalformedLiteral {
                        lit: lit.to_string(),
                    }),
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
                    data: DiagData::EvalError(EvalError::MalformedLiteral {
                        lit: lit.to_string(),
                    }),
                });
            };
            value += (digit as f64) * 10f64.powi(-(i as i32 + 1));
        }
        Ok(Object::Numerical(NumericalObj(value)))
    }
}
