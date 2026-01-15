use parser::syntax::stmt::{self};

use crate::{Interpreter, diag::Diag, eval::Evaluable};

pub trait Interpretable {
    fn interpret(&self, interpreter: &Interpreter) -> Result<(), Diag>;
}

impl Interpretable for stmt::Statement {
    fn interpret(&self, interpreter: &Interpreter) -> Result<(), Diag> {
        match self {
            stmt::Statement::Invocation(invocation_stmt) => invocation_stmt.interpret(interpreter),
            stmt::Statement::Cho(cho_stmt) => cho_stmt.interpret(interpreter),
        }
    }
}

impl Interpretable for stmt::InvocationStatement {
    fn interpret(&self, interpreter: &Interpreter) -> Result<(), Diag> {
        // For now, just print out the invocation statement
        println!("Interpreting invocation: {:?}", self);
        Ok(())
    }
}

impl Interpretable for stmt::ChoStatement {
    fn interpret(&self, interpreter: &Interpreter) -> Result<(), Diag> {
        println!(
            "{} equals {}",
            interpreter.parser.get_snippet(&self.lhs.0),
            match &self.rhs {
                Some(expr) => format!("{}", expr.0.evaluate(interpreter)?),
                None => "undefined".to_string(),
            }
        );
        Ok(())
    }
}
