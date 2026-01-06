use crate::{diag::Diags, parser::Parser, syntax::stmt::Statement};

pub mod stmt;
pub mod terminal;

#[derive(Debug)]
pub struct Programme {
    pub statements: Vec<Statement>,
}

impl Programme {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diags> {
        let mut statements = Vec::new();
        loop {
            match Statement::accept(parser) {
                Ok(Some(stmt)) => statements.push(stmt),
                Ok(None) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Programme { statements })
    }
}
