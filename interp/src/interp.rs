use parser::syntax::stmt::{self};

use crate::{
    Evaluator,
    diag::{Diag, DiagData, EvalError},
    eval::Evaluable,
    obj::Object,
};

pub trait Interpretable {
    fn interpret(&self, interpreter: &mut Evaluator) -> Result<(), Diag>;
}

impl Interpretable for stmt::Statement {
    fn interpret(&self, interpreter: &mut Evaluator) -> Result<(), Diag> {
        match self {
            stmt::Statement::Invocation(invocation_stmt) => invocation_stmt.interpret(interpreter),
            stmt::Statement::Cho(cho_stmt) => cho_stmt.interpret(interpreter),
            stmt::Statement::In(in_stmt) => in_stmt.interpret(interpreter),
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
            None => Object::Undefined,
        };
        let name = interpreter.snippet(&self.lhs.0).to_string();
        match interpreter.current_scope_mut().declare(name, value) {
            true => Ok(()),
            false => Err(Diag {
                line: interpreter.cur_line(),
                data: DiagData::EvalError(EvalError::AlreadyDeclaredInScope {
                    name: interpreter.snippet(&self.lhs.0).to_string().to_string(),
                }),
            }),
        }
    }
}

impl Interpretable for stmt::InStatement {
    fn interpret(&self, interpreter: &mut Evaluator) -> Result<(), Diag> {
        let expr = self.expr.evaluate(interpreter)?;
        eprintln!("{}", expr);
        Ok(())
    }
}
