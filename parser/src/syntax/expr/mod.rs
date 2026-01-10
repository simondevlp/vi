use lexer::lexeme;

use crate::{
    diag::{BracketKind, Diag, DiagData, Error},
    parser::Parser,
    syntax::expr::terminal::Literal,
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
    pub lhs: MulAffixedExpr,
    pub rhs: Vec<(bool, MulAffixedExpr)>,
}

impl AddAffixedExpr {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let Some(lhs) = MulAffixedExpr::accept(parser)? else {
            return Ok(None);
        };
        let mut rhs = Vec::new();
        loop {
            parser.skip_ws_if_any(true);
            let op = match parser.cur_lexeme.kind {
                lexeme::Kind::Plus => true,
                lexeme::Kind::Minus => false,
                _ => break,
            };
            parser.next_non_ws_lexeme(true); // consume op
            let Some(expr) = MulAffixedExpr::accept(parser)? else {
                return Err(Diag {
                    line: parser.cur_line,
                    data: DiagData::Err(Error::MiscExpecting {
                        expected: "an expression after operator".to_string(),
                    }),
                    span: parser.cur_span(),
                });
            };
            rhs.push((op, expr));
        }
        Ok(Some(AddAffixedExpr { lhs, rhs }))
    }
}

#[derive(Debug)]
pub struct MulAffixedExpr {
    pub lhs: PrefixedExpr,
    pub rhs: Vec<(bool, PrefixedExpr)>,
}

impl MulAffixedExpr {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let Some(lhs) = PrefixedExpr::accept(parser)? else {
            return Ok(None);
        };
        let mut rhs = Vec::new();
        loop {
            parser.skip_ws_if_any(true);
            let op = match parser.cur_lexeme.kind {
                lexeme::Kind::Asterisk => true,
                lexeme::Kind::Slash => false,
                _ => break,
            };
            parser.next_non_ws_lexeme(true); // consume op
            let Some(expr) = PrefixedExpr::accept(parser)? else {
                return Err(Diag {
                    line: parser.cur_line,
                    data: DiagData::Err(Error::MiscExpecting {
                        expected: "an expression after operator".to_string(),
                    }),
                    span: parser.cur_span(),
                });
            };
            rhs.push((op, expr));
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
    pub terminal: TerminalExpr,
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
        match TerminalExpr::accept(parser)? {
            Some(terminal) => Ok(Some(PrefixedExpr { prefix, terminal })),
            None => Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::MiscExpecting {
                    expected: "an expression".to_string(),
                }),
                span: parser.cur_span(),
            }),
        }
    }
}

#[derive(Debug)]
pub enum TerminalExpr {
    Literal(Literal),
    Tuple(TupleExpr),
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
                parser.next_non_ws_lexeme(true);
                Some(TupleExpr(exprs))
            }
            _ => None,
        })
    }
}

impl TerminalExpr {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        Ok(if let Some(lit) = Literal::accept(parser)? {
            Some(TerminalExpr::Literal(lit))
        } else if let Some(tuple) = TupleExpr::accept(parser)? {
            Some(TerminalExpr::Tuple(tuple))
        } else {
            None
        })
    }
}
