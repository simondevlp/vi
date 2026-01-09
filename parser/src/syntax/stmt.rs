use lexer::lexeme;

use crate::{
    diag::{Diag, DiagData, Error},
    parser::Parser,
    syntax::expr::{
        Expr,
        terminal::{Ident, Keyword},
    },
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
                data: DiagData::Err(Error::MiscExpecting {
                    expected: "a statement".to_string(),
                }),
                span: parser.cur_span(),
            })
        }
    }
}

#[derive(Debug)]
pub struct ChoStatement {
    pub lhs: Ident,
    pub rhs: Option<Expr>,
}

impl ChoStatement {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        if parser.cur_lexeme_snippet() != Keyword::Cho.as_str() {
            return Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::MiscExpecting {
                    expected: "'cho'".to_string(),
                }),
                span: parser.cur_span(),
            });
        }
        parser.next_non_ws_lexeme(true); // consumes 'cho'
        let Some(lhs) = Ident::accept(parser) else {
            return Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::MiscExpecting {
                    expected: "an identifier".to_string(),
                }),
                span: parser.cur_span(),
            });
        };
        parser.skip_ws_if_any(false);
        if !matches!(parser.cur_lexeme.kind, lexeme::Kind::Equal) {
            return Ok(ChoStatement { lhs, rhs: None });
        }
        parser.next_non_ws_lexeme(true); // consumes '='
        let rhs = Expr::accept(parser)?;
        Ok(ChoStatement {
            lhs,
            rhs: Some(rhs),
        })
    }
}
