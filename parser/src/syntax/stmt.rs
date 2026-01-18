use lexer::lexeme;

use crate::{
    diag::{Diag, DiagData, Error},
    parser::Parser,
    syntax::expr::{
        Expr, Field,
        terminal::{Ident, Keyword},
    },
};

#[derive(Debug)]
pub enum Statement {
    Cho(ChoStatement),
    Invocation(InvocationStatement),
}

impl Statement {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        if let Some(cho_stmt) = ChoStatement::accept(parser)? {
            Ok(Some(Statement::Cho(cho_stmt)))
        } else if let Some(invoc_stmt) = InvocationStatement::accept(parser)? {
            Ok(Some(Statement::Invocation(invoc_stmt)))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub struct ChoStatement {
    pub kw: Keyword,
    pub lhs: Ident,
    pub rhs: Option<Expr>,
}

impl ChoStatement {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let Some(kw) = Keyword::accept(parser, Keyword::Cho)? else {
            return Ok(None);
        };
        let Some(lhs) = Ident::accept(parser)? else {
            return Err(Diag {
                line: parser.cur_line,
                span: parser.cur_span(),
                data: DiagData::Err(Error::Expecting {
                    expected: "the left-hand side for declaration".to_string(),
                }),
            });
        };
        parser.skip_ws_if_any(false);
        Ok(match parser.cur_lexeme.kind {
            lexeme::Kind::Equal => {
                parser.next_non_ws_lexeme(true); // consume '='
                let Some(rhs) = Expr::accept(parser)? else {
                    return Err(Diag {
                        line: parser.cur_line,
                        span: parser.cur_span(),
                        data: DiagData::Err(Error::Expecting {
                            expected: "the right-hand side expression for assignment".to_string(),
                        }),
                    });
                };
                Some(ChoStatement {
                    kw,
                    lhs,
                    rhs: Some(rhs),
                })
            }
            _ => Some(ChoStatement { kw, lhs, rhs: None }),
        })
    }
}

#[derive(Debug)]
pub struct InvocationStatement {
    pub path: Field,
}

impl InvocationStatement {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        match Field::accept(parser)? {
            Some(path) => Ok(Some(InvocationStatement { path })),
            None => Ok(None),
        }
    }
}
