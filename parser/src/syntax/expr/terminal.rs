use lexer::lexeme::{self, Kind};

use crate::{
    diag::{BracketKind, Diag, DiagData, Error},
    parser::Parser,
    syntax::{Span, expr::Expr},
};

#[derive(Debug)]
pub enum Keyword {
    Cho,
}

impl Keyword {
    pub fn as_str(&self) -> &str {
        match self {
            Keyword::Cho => "cho",
        }
    }

    pub fn accept(parser: &mut Parser, kw: Self) -> Result<Option<Self>, Diag> {
        Ok(match parser.cur_lexeme.kind {
            Kind::Word if parser.cur_lexeme_snippet_is(kw.as_str()) => {
                parser.next_non_ws_lexeme(true);
                Some(kw)
            }
            _ => None,
        })
    }
}

#[derive(Debug)]
pub struct Ident(pub Span);
#[derive(Debug)]
pub struct Float(pub Span);
#[derive(Debug)]
pub struct Decimal(pub Span);
#[derive(Debug)]
pub struct DoubleQuotedString(pub Span);

#[derive(Debug)]
pub enum Literal {
    Float(Float),
    Decimal(Decimal),
    DoubleQuotedString(DoubleQuotedString),
}

impl Literal {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        Ok(if let Some(float) = Float::accept(parser)? {
            Some(Literal::Float(float))
        } else if let Some(decimal) = Decimal::accept(parser)? {
            Some(Literal::Decimal(decimal))
        } else if let Some(dqs) = DoubleQuotedString::accept(parser)? {
            Some(Literal::DoubleQuotedString(dqs))
        } else {
            None
        })
    }
}

impl Ident {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        Ok(match parser.cur_lexeme.kind {
            Kind::Word => {
                let start = parser.cur_pos;
                let mut len = parser.cur_lexeme.len;
                loop {
                    let mut added = 0;
                    let ws = parser.next_lexeme();
                    match ws.kind {
                        Kind::WordSpaces => {
                            added += ws.len;
                        }
                        _ => {
                            break;
                        }
                    };
                    let word = parser.next_lexeme();
                    match word.kind {
                        Kind::Word => {
                            len += word.len + added;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                Some(Self((start, len)))
            }
            _ => None,
        })
    }
}

impl Float {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        Ok(match parser.cur_lexeme.kind {
            Kind::Float => {
                let span = parser.cur_span();
                parser.next_lexeme();
                Some(Self(span))
            }
            _ => None,
        })
    }
}

impl Decimal {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        Ok(match parser.cur_lexeme.kind {
            Kind::Decimal => {
                let span = parser.cur_span();
                parser.next_lexeme();
                Some(Self(span))
            }
            _ => None,
        })
    }
}

impl DoubleQuotedString {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        eprintln!("DoubleQuotedString::accept at pos {}", parser.cur_pos);
        Ok(match parser.cur_lexeme.kind {
            Kind::String => {
                let span = parser.cur_span();
                parser.next_lexeme();
                Some(Self(span))
            }
            _ => None,
        })
    }
}

#[derive(Debug)]
pub enum Field {
    Property(Ident),
    Method(Invocation),
}

impl Field {
    pub fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let Some(callee) = Ident::accept(parser)? else {
            return Ok(None);
        };
        parser.skip_ws_if_any(true);
        Ok(match TupleExpr::accept(parser)? {
            Some(tuple) => Some(Field::Method(Invocation {
                root: callee,
                tuple,
            })),
            None => Some(Field::Property(callee)),
        })
    }
}

#[derive(Debug)]
pub struct Invocation {
    pub root: Ident,
    pub tuple: TupleExpr,
}

#[derive(Debug)]
pub enum TerminalExpr {
    Literal(Literal),
    Tuple(TupleExpr),
    Field(Field),
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
        } else if let Some(field) = Field::accept(parser)? {
            Some(TerminalExpr::Field(field))
        } else {
            None
        })
    }
}
