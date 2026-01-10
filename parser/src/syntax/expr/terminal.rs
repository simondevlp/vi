use lexer::lexeme::{self, Kind};

use crate::{
    diag::{Diag, DiagData, Error},
    parser::Parser,
    syntax::Span,
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

    pub fn accept(parser: &mut Parser, kw: Self) -> Result<Self, Diag> {
        match parser.cur_lexeme.kind {
            Kind::Word if parser.cur_lexeme_snippet_is(kw.as_str()) => {
                parser.next_non_ws_lexeme(true);
                Ok(kw)
            }
            _ => Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::MiscExpecting {
                    expected: format!("'{}'", kw.as_str()),
                }),
                span: parser.cur_span(),
            }),
        }
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
    Ident(Ident),
    Float(Float),
    Decimal(Decimal),
    DoubleQuotedString(DoubleQuotedString),
}

impl Literal {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        match parser.cur_lexeme.kind {
            lexeme::Kind::Word => Ok(Literal::Ident(Ident::accept(parser)?)),
            lexeme::Kind::Float => Ok(Literal::Float(Float::accept(parser)?)),
            lexeme::Kind::Decimal => Ok(Literal::Decimal(Decimal::accept(parser)?)),
            lexeme::Kind::String => Ok(Literal::DoubleQuotedString(DoubleQuotedString::accept(
                parser,
            )?)),
            _ => Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::MiscExpecting {
                    expected: "a literal".to_string(),
                }),
                span: parser.cur_span(),
            }),
        }
    }
}

impl Ident {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
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
                Ok(Self((start, len)))
            }
            _ => Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::MiscExpecting {
                    expected: "an identifier".to_string(),
                }),
                span: parser.cur_span(),
            }),
        }
    }
}

impl Float {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        match parser.cur_lexeme.kind {
            Kind::Float => {
                let span = parser.cur_span();
                parser.next_lexeme();
                Ok(Self(span))
            }
            _ => Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::MiscExpecting {
                    expected: "a float literal".to_string(),
                }),
                span: parser.cur_span(),
            }),
        }
    }
}

impl Decimal {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        match parser.cur_lexeme.kind {
            Kind::Decimal => {
                let span = parser.cur_span();
                parser.next_lexeme();
                Ok(Self(span))
            }
            _ => Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::MiscExpecting {
                    expected: "a decimal literal".to_string(),
                }),
                span: parser.cur_span(),
            }),
        }
    }
}

impl DoubleQuotedString {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        match parser.cur_lexeme.kind {
            Kind::String => {
                let span = parser.cur_span();
                parser.next_lexeme();
                Ok(Self(span))
            }
            _ => Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::MiscExpecting {
                    expected: "a double quoted string literal".to_string(),
                }),
                span: parser.cur_span(),
            }),
        }
    }
}
