use lexer::lexeme::{self, Kind};

use crate::{
    accept::Acceptor,
    diag::{BracketKind, Diag, DiagData, Error},
    parser::Parser,
    syntax::{Span, expr::Expr},
};

#[derive(Debug)]
pub enum Keyword {
    Cho,
}

impl Acceptor for Keyword {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let ret = Ok(match parser.cur_lexeme_snippet() {
            "cho" => {
                parser.next_non_ws_lexeme(true);
                Some(Keyword::Cho)
            }
            _ => None,
        });
        ret
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

impl Acceptor for Literal {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
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

impl Acceptor for Ident {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
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

impl Acceptor for Float {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
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

impl Acceptor for Decimal {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
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

impl Acceptor for DoubleQuotedString {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
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

impl Acceptor for Field {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
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

impl Acceptor for TupleExpr {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
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
                    });
                }
                parser.next_non_ws_lexeme(true);
                Some(TupleExpr(exprs))
            }
            _ => None,
        })
    }
}

impl Acceptor for TerminalExpr {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
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
