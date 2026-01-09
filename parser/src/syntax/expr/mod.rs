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
    Grouped(Box<Expr>),
}

impl TerminalExpr {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        if let Some(lit) = Literal::accept(parser) {
            return Ok(TerminalExpr::Literal(lit));
        }
        if matches!(parser.cur_lexeme.kind, lexeme::Kind::LeftParen) {
            parser.next_non_ws_lexeme(true); // consume '('
            let expr = Expr::accept(parser)?;
            parser.skip_ws_if_any(true);
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
            return Ok(TerminalExpr::Grouped(Box::new(expr)));
        }
        Err(Diag {
            line: parser.cur_line,
            data: DiagData::Err(Error::MiscExpecting {
                expected: "an expression".to_string(),
            }),
            span: parser.cur_span(),
        })
    }
}
