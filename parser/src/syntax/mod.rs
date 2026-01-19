use lexer::lexeme;

use crate::{
    diag::{Diag, DiagData, Error},
    parser::Parser,
    syntax::stmt::Statement,
};

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
        parser.skip_ws_if_any(true);
        match parser.cur_lexeme.kind {
            lexeme::Kind::Eof => Ok(Programme { statements }),
            _ => Err(Diag {
                line: parser.cur_line,
                span: parser.cur_span(),
                data: DiagData::Err(Error::Expecting {
                    expected: "end of file".to_string(),
                }),
            }),
        }
    }
}
