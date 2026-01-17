use parser::syntax::stmt::{self};

use crate::{
    Evaluator,
    diag::{Diag, DiagData, EvalError},
    eval::Evaluable,
    obj::ValueObj,
};

pub trait Interpretable {
    fn interpret(&self, interpreter: &mut Evaluator) -> Result<(), Diag>;
}

impl Interpretable for stmt::Statement {
    fn interpret(&self, interpreter: &mut Evaluator) -> Result<(), Diag> {
        match self {
            stmt::Statement::Invocation(invocation_stmt) => invocation_stmt.interpret(interpreter),
            stmt::Statement::Cho(cho_stmt) => cho_stmt.interpret(interpreter),
        }
    }
}

impl Interpretable for stmt::InvocationStatement {
    fn interpret(&self, _interpreter: &mut Evaluator) -> Result<(), Diag> {
        // For now, just print out the invocation statement
        println!("Interpreting invocation: {:?}", self);
        Ok(())
    }
}

impl Interpretable for stmt::ChoStatement {
    fn interpret(&self, interpreter: &mut Evaluator) -> Result<(), Diag> {
        let value = match &self.rhs {
            Some(expr) => expr.evaluate(interpreter)?,
            None => ValueObj::Undefined,
        };
        match interpreter
            .global
            .declare(interpreter.snippet(&self.lhs.0), value)
        {
            true => Ok(()),
            false => Err(Diag {
                line: interpreter.cur_line(),
                data: DiagData::EvalError(EvalError::AlreadyDeclaredInScope {
                    name: interpreter.snippet(&self.lhs.0).to_string(),
                }),
            }),
        }
    }
}
