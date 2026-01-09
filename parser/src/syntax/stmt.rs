use lexer::lexeme;

use crate::{
    diag::{Diag, DiagData, Error, TokenString},
    parser::Parser,
    syntax::terminal::{Ident, Keyword, Literal},
};

#[derive(Debug)]
pub enum Statement {
    Cho(ChoStatement),
}

impl Statement {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        if parser.cur_lexeme_snippet_is(Keyword::Cho.as_str()) {
            let cho_stmt = ChoStatement::accept(parser)?;
            Ok(Statement::Cho(cho_stmt))
        } else {
            Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::UnexpectedToken {
                    expected: vec![Keyword::Cho.to_token_string()],
                    found: TokenString::from_str(parser.cur_lexeme_snippet()),
                }),
            })
        }
    }
}

#[derive(Debug)]
pub struct ChoStatement {
    pub lhs: Ident,
    pub rhs: Option<Literal>,
}

impl ChoStatement {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        if parser.cur_lexeme_snippet() != Keyword::Cho.as_str() {
            return Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::UnexpectedToken {
                    expected: vec![Keyword::Cho.to_token_string()],
                    found: TokenString::from_str(parser.cur_lexeme_snippet()),
                }),
            });
        }
        parser.skip_ws_if_any(true);
        let Some(lhs) = Ident::accept(parser) else {
            return Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::UnexpectedLexeme {
                    expected: vec![lexeme::Kind::Word],
                    found: parser.cur_lexeme.kind,
                }),
            });
        };
        parser.skip_ws_if_any(false);
        if !matches!(parser.cur_lexeme.kind, lexeme::Kind::Equal) {
            return Ok(ChoStatement { lhs, rhs: None });
        }
        parser.next_non_ws_lexeme(true); // consumes '='
        let Some(rhs) = Literal::accept(parser) else {
            return Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::UnexpectedLexeme {
                    expected: vec![
                        lexeme::Kind::Word,
                        lexeme::Kind::Float,
                        lexeme::Kind::Decimal,
                    ],
                    found: parser.cur_lexeme.kind,
                }),
            });
        };
        Ok(ChoStatement {
            lhs,
            rhs: Some(rhs),
        })
    }
}
