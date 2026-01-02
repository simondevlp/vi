use lexer::lexeme::{self, Kind};

use crate::{
    diag::{Diags, Error},
    parser::Parser,
    syntax::terminal::{Ident, Keyword},
};

#[derive(Debug)]
pub enum Statement {
    Cho(ChoStatement),
}

impl Statement {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diags> {
        if matches!(parser.cur_lexeme.kind, Kind::Eof) {
            return Ok(None);
        }
        if parser.cur_lexeme_snippet_is(Keyword::Cho.as_str()) {
            let cho_stmt = ChoStatement::accept(parser)?;
            Ok(Some(Statement::Cho(cho_stmt.unwrap())))
        } else {
            Err(Diags::Err(Error::UnexpectedToken {
                expected: vec![Keyword::Cho.as_str().to_string()],
                found: parser.cur_lexeme_snippet().to_string(),
            }))
        }
    }
}

#[derive(Debug)]
pub struct ChoStatement {
    pub lhs: Ident,
}

impl ChoStatement {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diags> {
        if !parser.cur_lexeme_snippet_is(Keyword::Cho.as_str()) {
            return Ok(None);
        }
        parser.next_lexeme();
        parser.next_non_ws_lexeme();
        let Some(lhs) = Ident::accept(parser) else {
            return Err(Diags::Err(Error::UnexpectedLexeme {
                expected: vec![lexeme::Kind::Word],
                found: parser.cur_lexeme.kind,
            }));
        };
        Ok(Some(ChoStatement { lhs }))
    }
}
