use lexer::lexeme;

use crate::{
    accept::Acceptor,
    diag::{Diag, DiagData, Error},
    parser::Parser,
    syntax::expr::{
        Expr, PathExpr,
        terminal::{Ident, Keyword},
    },
};

#[derive(Debug)]
pub enum Statement {
    Cho(ChoStatement),
    Invocation(InvocationStatement),
}

impl Acceptor for Statement {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
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
    pub lhs: Ident,
    pub rhs: Option<Expr>,
}

impl Acceptor for ChoStatement {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let Some(Keyword::Cho) = Keyword::accept(parser)? else {
            return Ok(None);
        };
        let Some(lhs) = Ident::accept(parser)? else {
            return Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::MiscExpecting {
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
                        data: DiagData::Err(Error::MiscExpecting {
                            expected: "the right-hand side expression for assignment".to_string(),
                        }),
                    });
                };
                Some(ChoStatement {
                    lhs,
                    rhs: Some(rhs),
                })
            }
            _ => Some(ChoStatement { lhs, rhs: None }),
        })
    }
}

#[derive(Debug)]
pub struct InvocationStatement(pub PathExpr);

impl Acceptor for InvocationStatement {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        if let Some(path_expr) = PathExpr::accept(parser)? {
            if path_expr.rhs.is_empty() {
                Err(Diag {
                    line: parser.cur_line,
                    data: DiagData::Err(Error::MiscExpecting {
                        expected: "a statement".to_string(),
                    }),
                })
            } else {
                Ok(Some(InvocationStatement(path_expr)))
            }
        } else {
            Ok(None)
        }
    }
}
