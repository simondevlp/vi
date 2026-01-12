use lexer::lexeme::{self, Lexeme};

use crate::{
    accept::Acceptor,
    diag::{Diag, DiagData, Error},
    parser::Parser,
    syntax::{
        Block,
        expr::{
            Expr, PathExpr,
            terminal::{Field, Ident, Keyword, TerminalExpr},
        },
    },
};

#[derive(Debug)]
pub enum Statement {
    Cho(ChoStatement),
    Lop(LopStatement),
    Invocation(InvocationStatement),
}

impl Acceptor for Statement {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        if let Some(cho_stmt) = ChoStatement::accept(parser)? {
            Ok(Some(Statement::Cho(cho_stmt)))
        } else if let Some(lop_stmt) = LopStatement::accept(parser)? {
            Ok(Some(Statement::Lop(lop_stmt)))
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
        if !Keyword::check(parser, Keyword::Cho) {
            return Ok(None);
        };
        parser.next_non_ws_lexeme(true); // consume 'cho'
        let Some(lhs) = Ident::accept(parser)? else {
            return Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::Expecting {
                    expected: "the left-hand side for declaration",
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
                        data: DiagData::Err(Error::Expecting {
                            expected: "the right-hand side expression for assignment",
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
pub struct LopStatement {
    pub name: Ident,
    pub block: Block<ChoStatement>,
}

impl Acceptor for LopStatement {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        if !Keyword::check(parser, Keyword::Lop) {
            return Ok(None);
        };
        parser.next_non_ws_lexeme(true); // consume 'lớp'
        let Some(name) = Ident::accept(parser)? else {
            return Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::Expecting {
                    expected: "name of the class",
                }),
            });
        };
        parser.skip_ws_if_any(false);
        let Lexeme {
            kind: lexeme::Kind::Eol,
            ..
        } = parser.cur_lexeme
        else {
            return Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::Expecting {
                    expected: "end of line leading to class body",
                }),
            });
        };
        parser.next_lexeme(); // consume EOL
        let Some(block) = Block::<ChoStatement>::accept(parser)? else {
            return Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::Expecting {
                    expected: "class body",
                }),
            });
        };
        Ok(Some(LopStatement { name, block }))
    }
}

#[derive(Debug)]
pub struct InvocationStatement(pub PathExpr);

impl Acceptor for InvocationStatement {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        if let Some(path_expr) = PathExpr::accept(parser)? {
            if path_expr.rhs.is_empty()
                && !matches!(path_expr.lhs, TerminalExpr::Field(Field::Method(_)))
            {
                Err(Diag {
                    line: parser.cur_line,
                    data: DiagData::Err(Error::Expecting {
                        expected: "a statement",
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
