use lexer::lexeme;

use crate::{Span, diag::Diag, parser::Parser};

#[derive(Debug)]
pub enum Keyword {
    Cho,
    In,
}

impl Keyword {
    pub fn as_str(&self) -> &str {
        match self {
            Keyword::Cho => "cho",
            Keyword::In => "in",
        }
    }

    pub fn accept(parser: &mut Parser, kw: Self) -> Result<Option<Self>, Diag> {
        Ok(match parser.cur_lexeme.kind {
            lexeme::Kind::Word if parser.cur_lexeme_snippet_is(kw.as_str()) => {
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
            lexeme::Kind::Word => {
                let start = parser.cur_pos;
                let mut len = parser.cur_lexeme.len;
                loop {
                    let mut added = 0;
                    let ws = parser.next_lexeme();
                    match ws.kind {
                        lexeme::Kind::WordSpaces => {
                            added += ws.len;
                        }
                        _ => {
                            break;
                        }
                    };
                    let word = parser.next_lexeme();
                    match word.kind {
                        lexeme::Kind::Word => {
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
            lexeme::Kind::Float => {
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
            lexeme::Kind::Decimal => {
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
        Ok(match parser.cur_lexeme.kind {
            lexeme::Kind::String => {
                let span = parser.cur_span();
                parser.next_lexeme();
                Some(Self(span))
            }
            _ => None,
        })
    }
}
