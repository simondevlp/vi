use lexer::lexeme;

use crate::{
    accept::Acceptor,
    diag::{Diag, DiagData, Error},
    parser::Parser,
    syntax::expr::terminal::{Field, TerminalExpr},
};

pub mod terminal;

#[derive(Debug)]
pub struct Expr(pub AddAffixedExpr);

impl Acceptor for Expr {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
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

impl Acceptor for AddAffixedExpr {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
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

impl Acceptor for MulAffixedExpr {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
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
    pub lhs: Option<PrefixedExprKind>,
    pub rhs: PathExpr,
}

impl Acceptor for PrefixedExpr {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let prefix = match parser.cur_lexeme.kind {
            lexeme::Kind::Minus => {
                parser.next_non_ws_lexeme(true);
                Some(PrefixedExprKind::Minus)
            }
            _ => None,
        };
        Ok(match PathExpr::accept(parser)? {
            Some(rhs) => Some(PrefixedExpr { lhs: prefix, rhs }),
            None => None,
        })
    }
}

#[derive(Debug)]
pub struct PathExpr {
    pub lhs: TerminalExpr,
    pub rhs: Vec<Field>,
}

impl Acceptor for PathExpr {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let Some(lhs) = TerminalExpr::accept(parser)? else {
            return Ok(None);
        };
        let mut rhs = Vec::new();
        loop {
            parser.skip_ws_if_any(true);
            if !matches!(parser.cur_lexeme.kind, lexeme::Kind::Period) {
                break;
            }
            parser.next_non_ws_lexeme(true); // consume '.'
            let Some(field) = Field::accept(parser)? else {
                return Err(Diag {
                    line: parser.cur_line,
                    data: DiagData::Err(Error::MiscExpecting {
                        expected: "a field after '.'".to_string(),
                    }),
                });
            };
            rhs.push(field);
        }
        Ok(Some(PathExpr { lhs, rhs }))
    }
}
