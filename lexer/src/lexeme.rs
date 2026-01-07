use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
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
            Kind::Invalid => "an invalid character",
            Kind::Eof => "end of file",
            Kind::Eol => "end of line",
            Kind::WordSpaces => "word spaces",
            Kind::Whitespaces => "whitespaces",
            Kind::Comment => "a comment",
            Kind::Word => "a word",
            Kind::Decimal => "a decimal number",
            Kind::String => "a string",
            Kind::Plus => "a plus sign '+'",
            Kind::Minus => "a minus sign '-'",
            Kind::Asterisk => "an asterisk '*'",
            Kind::Slash => "a slash '/'",
            Kind::Equal => "an equal sign '='",
            Kind::Period => "a period '.'",
            Kind::Comma => "a comma ','",
            Kind::Greater => "a greater than sign '>'",
            Kind::Less => "a less than sign '<'",
            Kind::LeftBrace => "a left brace '{'",
            Kind::RightBrace => "a right brace '}'",
            Kind::LeftParen => "a left parenthesis '('",
            Kind::RightParen => "a right parenthesis ')'",
            Kind::LeftBracket => "a left bracket '['",
            Kind::RightBracket => "a right bracket ']'",
        };
        write!(f, "{}", kind_str)
    }
}

#[derive(Debug)]
pub struct Lexeme {
    pub kind: Kind,
    pub len: u32,
}
