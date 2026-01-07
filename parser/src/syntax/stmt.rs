use lexer::lexeme::{self, Kind};

use crate::{
    diag::{Diags, Error},
    parser::Parser,
    syntax::terminal::{Ident, Keyword},
};

#[derive(Debug)]
pub enum Statement {
    BlankStatement,
    Cho(ChoStatement),
}

impl Statement {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diags> {
        if matches!(parser.cur_lexeme.kind, Kind::Eol | Kind::Eof) {
            return Ok(Statement::BlankStatement);
        } else if parser.cur_lexeme_snippet_is(Keyword::Cho.as_str()) {
            let cho_stmt = ChoStatement::accept(parser)?;
            Ok(Statement::Cho(cho_stmt))
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
    pub fn accept(parser: &mut Parser) -> Result<Self, Diags> {
        if parser.cur_lexeme_snippet() != Keyword::Cho.as_str() {
            return Err(Diags::Err(Error::UnexpectedToken {
                expected: vec![Keyword::Cho.as_str().to_string()],
                found: parser.cur_lexeme_snippet().to_string(),
            }));
        }
        parser.next_non_ws_lexeme();
        let Some(lhs) = Ident::accept(parser) else {
            return Err(Diags::Err(Error::UnexpectedLexeme {
                expected: vec![lexeme::Kind::Word],
                found: parser.cur_lexeme.kind,
            }));
        };
        Ok(ChoStatement { lhs })
    }
}
