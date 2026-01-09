use lexer::lexeme::Kind;

use crate::{diag::TokenString, parser::Parser, syntax::Span};

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

    pub fn to_token_string(&self) -> TokenString {
        TokenString::from_str(self.as_str())
    }
}

#[derive(Debug)]
pub struct Ident(pub Span);
#[derive(Debug)]
pub struct Float(pub Span);
#[derive(Debug)]
pub struct Decimal(pub Span);

#[derive(Debug)]
pub enum Literal {
    Ident(Ident),
    Float(Float),
    Decimal(Decimal),
}

impl Literal {
    pub fn accept(parser: &mut Parser) -> Option<Self> {
        if let Some(ident) = Ident::accept(parser) {
            return Some(Literal::Ident(ident));
        }
        if let Some(float) = Float::accept(parser) {
            return Some(Literal::Float(float));
        }
        if let Some(decimal) = Decimal::accept(parser) {
            return Some(Literal::Decimal(decimal));
        }
        None
    }
}

impl Ident {
    pub fn accept(parser: &mut Parser) -> Option<Self> {
        match parser.cur_lexeme.kind {
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
                Some(Self(Span { start, len }))
            }
            _ => None,
        }
    }
}

impl Float {
    pub fn accept(parser: &mut Parser) -> Option<Self> {
        match parser.cur_lexeme.kind {
            Kind::Float => {
                let span = Span {
                    start: parser.cur_pos,
                    len: parser.cur_lexeme.len,
                };
                parser.next_lexeme();
                Some(Self(span))
            }
            _ => None,
        }
    }
}

impl Decimal {
    pub fn accept(parser: &mut Parser) -> Option<Self> {
        match parser.cur_lexeme.kind {
            Kind::Decimal => {
                let span = Span {
                    start: parser.cur_pos,
                    len: parser.cur_lexeme.len,
                };
                parser.next_lexeme();
                Some(Self(span))
            }
            _ => None,
        }
    }
}
