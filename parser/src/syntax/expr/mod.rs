use lexer::lexeme;

use crate::{
    diag::{BracketKind, Diag, DiagData, Error},
    parser::Parser,
    syntax::expr::terminal::{Ident, Literal},
};

pub mod terminal;

#[derive(Debug)]
pub struct Expr(pub AddAffixedExpr);

impl Expr {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        Ok(match AddAffixedExpr::accept(parser)? {
            Some(add_expr) => Some(Expr(add_expr)),
            None => None,
        })
    }
}

#[derive(Debug)]
pub struct AddAffixedExpr {
    pub lhs: Option<Box<AddAffixedExpr>>,
    pub rhs: (bool, MulAffixedExpr),
}

impl AddAffixedExpr {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let mut lhs = None;
        let mut rhs = (
            true,
            match MulAffixedExpr::accept(parser)? {
                Some(expr) => expr,
                None => return Ok(None),
            },
        );
        loop {
            parser.skip_ws_if_any(true);
            let op = match parser.cur_lexeme.kind {
                lexeme::Kind::Plus => true,
                lexeme::Kind::Minus => false,
                _ => break,
            };
            parser.next_non_ws_lexeme(true); // consume op
            lhs = Some(Box::new(AddAffixedExpr { lhs, rhs }));
            rhs = (
                op,
                match MulAffixedExpr::accept(parser)? {
                    Some(expr) => expr,
                    None => {
                        return Err(Diag {
                            line: parser.cur_line,
                            data: DiagData::Err(Error::Expecting {
                                expected: "an expression after operator".to_string(),
                            }),
                            span: parser.cur_span(),
                        });
                    }
                },
            );
        }
        Ok(Some(AddAffixedExpr { lhs, rhs }))
    }
}

#[derive(Debug)]
pub struct MulAffixedExpr {
    pub lhs: Option<Box<MulAffixedExpr>>,
    pub rhs: (bool, PrefixedExpr),
}

impl MulAffixedExpr {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let mut lhs = None;
        let mut rhs = (
            true,
            match PrefixedExpr::accept(parser)? {
                Some(expr) => expr,
                None => return Ok(None),
            },
        );
        loop {
            parser.skip_ws_if_any(true);
            let op = match parser.cur_lexeme.kind {
                lexeme::Kind::Asterisk => true,
                lexeme::Kind::Slash => false,
                _ => break,
            };
            parser.next_non_ws_lexeme(true); // consume op
            lhs = Some(Box::new(MulAffixedExpr { lhs, rhs }));
            rhs = (
                op,
                match PrefixedExpr::accept(parser)? {
                    Some(expr) => expr,
                    None => {
                        return Err(Diag {
                            line: parser.cur_line,
                            data: DiagData::Err(Error::Expecting {
                                expected: "an expression after operator".to_string(),
                            }),
                            span: parser.cur_span(),
                        });
                    }
                },
            );
        }
        Ok(Some(MulAffixedExpr { lhs, rhs }))
    }
}

#[derive(Debug)]
pub enum PrefixedExprKind {
    Minus,
}
#[derive(Debug)]
pub struct PrefixedExpr {
    pub prefix: Option<PrefixedExprKind>,
    pub expr: PathExpr,
}

impl PrefixedExpr {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let prefix = match parser.cur_lexeme.kind {
            lexeme::Kind::Minus => {
                parser.next_non_ws_lexeme(true);
                Some(PrefixedExprKind::Minus)
            }
            _ => None,
        };
        match PathExpr::accept(parser)? {
            Some(expr) => Ok(Some(PrefixedExpr { prefix, expr })),
            None => Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::Expecting {
                    expected: "an expression".to_string(),
                }),
                span: parser.cur_span(),
            }),
        }
    }
}

#[derive(Debug)]
pub enum PathExpr {
    Root(TerminalExpr),
    WithFields { lhs: Box<PathExpr>, rhs: Field },
}

impl PathExpr {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let root = PathExpr::Root(match TerminalExpr::accept(parser)? {
            Some(expr) => expr,
            None => return Ok(None),
        });
        parser.skip_ws_if_any(false);
        match parser.cur_lexeme.kind {
            lexeme::Kind::Period => {
                parser.next_non_ws_lexeme(true); // consume dot
                let mut lhs_inner = root;
                let mut rhs = match Field::accept(parser)? {
                    Some(expr) => expr,
                    None => {
                        return Err(Diag {
                            line: parser.cur_line,
                            data: DiagData::Err(Error::Expecting {
                                expected: "a terminal expression after `.`".to_string(),
                            }),
                            span: parser.cur_span(),
                        });
                    }
                };
                loop {
                    parser.skip_ws_if_any(false);
                    if !matches!(parser.cur_lexeme.kind, lexeme::Kind::Period) {
                        break;
                    }
                    parser.next_non_ws_lexeme(true); // consume dot
                    lhs_inner = PathExpr::WithFields {
                        lhs: Box::new(lhs_inner),
                        rhs,
                    };
                    rhs = match Field::accept(parser)? {
                        Some(expr) => expr,
                        None => {
                            return Err(Diag {
                                line: parser.cur_line,
                                data: DiagData::Err(Error::Expecting {
                                    expected: "a terminal expression after `.`".to_string(),
                                }),
                                span: parser.cur_span(),
                            });
                        }
                    };
                }
                Ok(Some(PathExpr::WithFields {
                    lhs: Box::new(lhs_inner),
                    rhs,
                }))
            }
            _ => Ok(Some(root)),
        }
    }
}

#[derive(Debug)]
pub enum TerminalExpr {
    Field(Field),
    Literal(Literal),
    Tuple(TupleExpr),
}

#[derive(Debug)]
pub struct Field {
    pub name: Ident,
    pub args: Option<TupleExpr>,
}

impl Field {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let name = match Ident::accept(parser)? {
            Some(ident) => ident,
            None => return Ok(None),
        };
        parser.skip_ws_if_any(false);
        let args = match TupleExpr::accept(parser)? {
            Some(tuple) => Some(tuple),
            None => None,
        };
        Ok(Some(Field { name, args }))
    }
}

#[derive(Debug)]
pub struct TupleExpr(pub Vec<Expr>);

impl TupleExpr {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        Ok(match parser.cur_lexeme.kind {
            lexeme::Kind::LeftParen => {
                parser.next_non_ws_lexeme(true);
                let mut exprs = Vec::new();
                loop {
                    let Some(expr) = Expr::accept(parser)? else {
                        break;
                    };
                    exprs.push(expr);
                    parser.skip_ws_if_any(true);
                    if matches!(parser.cur_lexeme.kind, lexeme::Kind::Comma) {
                        parser.next_non_ws_lexeme(true);
                        continue;
                    } else {
                        break;
                    }
                }
                if !matches!(parser.cur_lexeme.kind, lexeme::Kind::RightParen) {
                    return Err(Diag {
                        line: parser.cur_line,
                        data: DiagData::Err(Error::BracketNotClosed {
                            kind: BracketKind::Parenthesis,
                        }),
                        span: (parser.cur_pos, 1),
                    });
                }
                parser.next_non_ws_lexeme(false);
                Some(TupleExpr(exprs))
            }
            _ => None,
        })
    }
}

impl TerminalExpr {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        Ok(if let Some(field) = Field::accept(parser)? {
            Some(TerminalExpr::Field(field))
        } else if let Some(lit) = Literal::accept(parser)? {
            Some(TerminalExpr::Literal(lit))
        } else if let Some(tuple) = TupleExpr::accept(parser)? {
            Some(TerminalExpr::Tuple(tuple))
        } else {
            None
        })
    }
}
