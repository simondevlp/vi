use lexer::lexeme;

use crate::{diag::Diag, parser::Parser, syntax::stmt::Statement};

pub mod stmt;
pub mod terminal;

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
            match Statement::accept(parser) {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }
        Ok(Programme { statements })
    }
}

#[derive(Debug)]
pub struct Span {
    pub start: u32,
    pub len: u32,
}
