use lexer::lexeme;

use crate::{
    diag::{Diags, Error},
    parser::Parser,
    syntax::stmt::Statement,
};

pub mod stmt;
pub mod terminal;

#[derive(Debug)]
pub struct Programme {
    pub statements: Vec<Statement>,
}

impl Programme {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diags> {
        let mut statements = Vec::new();
        parser.skip_ws_if_any();
        loop {
            parser.skip_ws_if_any();
            match Statement::accept(parser) {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
            parser.skip_ws_if_any();
            match parser.cur_lexeme.kind {
                lexeme::Kind::Eol => {
                    parser.next_lexeme();
                }
                lexeme::Kind::Eof => {
                    break;
                }
                _ => {
                    return Err(Diags::Err(Error::UnexpectedLexeme {
                        expected: vec![lexeme::Kind::Eol, lexeme::Kind::Eof],
                        found: parser.cur_lexeme.kind,
                    }));
                }
            }
        }
        Ok(Programme { statements })
    }
}
