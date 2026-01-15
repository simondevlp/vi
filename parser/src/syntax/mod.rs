use crate::{diag::Diag, parser::Parser, syntax::stmt::Statement};

pub mod expr;
pub mod stmt;

#[derive(Debug)]
pub struct Programme {
    pub statements: Vec<Statement>,
}

impl Programme {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        let mut statements = Vec::new();
        loop {
            parser.skip_ws_if_any(true);
            if let Some(stmt) = Statement::accept(parser)? {
                statements.push(stmt);
            } else {
                break;
            }
        }
        Ok(Programme { statements })
    }
}
