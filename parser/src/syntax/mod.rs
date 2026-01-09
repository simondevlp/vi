use lexer::lexeme;

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
        parser.skip_ws_if_any(true);
        loop {
            parser.skip_ws_if_any(true);
            if matches!(parser.cur_lexeme.kind, lexeme::Kind::Eof) {
                break;
            }
            statements.push(Statement::accept(parser)?);
        }
        Ok(Programme { statements })
    }
}

pub type Span = (u32, u32);
