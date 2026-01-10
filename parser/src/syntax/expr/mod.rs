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
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        let add_expr = AddAffixedExpr::accept(parser)?;
        Ok(Expr(add_expr))
    }
}

#[derive(Debug)]
pub struct AddAffixedExpr {
    pub lhs: MulAffixedExpr,
    pub rhs: Vec<(bool, MulAffixedExpr)>,
}

impl AddAffixedExpr {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        let lhs = MulAffixedExpr::accept(parser)?;
        let mut rhs = Vec::new();
        loop {
            parser.skip_ws_if_any(true);
            let op = match parser.cur_lexeme.kind {
                lexeme::Kind::Plus => true,
                lexeme::Kind::Minus => false,
                _ => break,
            };
            parser.next_non_ws_lexeme(true); // consume op
            let expr = MulAffixedExpr::accept(parser)?;
            rhs.push((op, expr));
        }
        Ok(AddAffixedExpr { lhs, rhs })
    }
}

#[derive(Debug)]
pub struct MulAffixedExpr {
    pub lhs: PrefixedExpr,
    pub rhs: Vec<(bool, PrefixedExpr)>,
}

impl MulAffixedExpr {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        let lhs = PrefixedExpr::accept(parser)?;
        let mut rhs = Vec::new();
        loop {
            parser.skip_ws_if_any(true);
            let op = match parser.cur_lexeme.kind {
                lexeme::Kind::Asterisk => true,
                lexeme::Kind::Slash => false,
                _ => break,
            };
            parser.next_non_ws_lexeme(true); // consume op
            let expr = PrefixedExpr::accept(parser)?;
            rhs.push((op, expr));
        }
        Ok(MulAffixedExpr { lhs, rhs })
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
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        let prefix = if matches!(parser.cur_lexeme.kind, lexeme::Kind::Minus) {
            parser.next_non_ws_lexeme(true);
            Some(PrefixedExprKind::Minus)
        } else {
            None
        };
        let terminal = TerminalExpr::accept(parser)?;
        Ok(PrefixedExpr { prefix, terminal })
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
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        parser.next_non_ws_lexeme(true); // consume '('
        let mut exprs = Vec::new();
        loop {
            let Ok(expr) = Expr::accept(parser) else {
                break;
            };
            exprs.push(expr);
            parser.skip_ws_if_any(true);
            match parser.cur_lexeme.kind {
                lexeme::Kind::Comma => {
                    parser.next_non_ws_lexeme(true); // consume ','
                }
                _ => break,
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
        parser.next_non_ws_lexeme(true); // consume ')'
        Ok(TupleExpr(exprs))
    }
}

impl TerminalExpr {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        if let Ok(lit) = Literal::accept(parser) {
            Ok(TerminalExpr::Literal(lit))
        } else if matches!(parser.cur_lexeme.kind, lexeme::Kind::LeftParen) {
            Ok(TerminalExpr::Tuple(TupleExpr::accept(parser)?))
        } else {
            Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::MiscExpecting {
                    expected: "an expression".to_string(),
                }),
                span: parser.cur_span(),
            })
        }
    }
}
