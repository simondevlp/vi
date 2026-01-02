use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Invalid,
    Eof,
    Eol,
    WordSpaces,
    Whitespaces,
    Comment,
    Word,
    Decimal,
    String,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    Period,
    Comma,
    Greater,
    Less,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind_str = match self {
            Kind::Invalid => "Invalid",
            Kind::Eof => "Eof",
            Kind::Eol => "Eol",
            Kind::WordSpaces => "WordSpaces",
            Kind::Whitespaces => "Whitespaces",
            Kind::Comment => "Comment",
            Kind::Word => "Word",
            Kind::Decimal => "Decimal",
            Kind::String => "String",
            Kind::Plus => "Plus",
            Kind::Minus => "Minus",
            Kind::Asterisk => "Asterisk",
            Kind::Slash => "Slash",
            Kind::Equal => "Equal",
            Kind::Period => "Period",
            Kind::Comma => "Comma",
            Kind::Greater => "Greater",
            Kind::Less => "Less",
            Kind::LeftBrace => "LeftBrace",
            Kind::RightBrace => "RightBrace",
            Kind::LeftParen => "LeftParen",
            Kind::RightParen => "RightParen",
            Kind::LeftBracket => "LeftBracket",
            Kind::RightBracket => "RightBracket",
        };
        write!(f, "{}", kind_str)
    }
}

#[derive(Debug)]
pub struct Lexeme {
    pub kind: Kind,
    pub len: u32,
}
