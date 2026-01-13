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
    Float,
    Decimal,
    String,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    Colon,
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
            Self::Invalid => "an invalid character",
            Self::Eof => "end of file",
            Self::Eol => "end of line",
            Self::WordSpaces => "word spaces",
            Self::Whitespaces => "whitespaces",
            Self::Comment => "a comment",
            Self::Word => "a word",
            Self::Float => "a floating point number",
            Self::Decimal => "a decimal number",
            Self::String => "a string",
            Self::Plus => "a plus sign '+'",
            Self::Minus => "a minus sign '-'",
            Self::Asterisk => "an asterisk '*'",
            Self::Slash => "a slash '/'",
            Self::Equal => "an equal sign '='",
            Self::Colon => "a colon ':'",
            Self::Period => "a period '.'",
            Self::Comma => "a comma ','",
            Self::Greater => "a greater than sign '>'",
            Self::Less => "a less than sign '<'",
            Self::LeftBrace => "a left brace '{'",
            Self::RightBrace => "a right brace '}'",
            Self::LeftParen => "a left parenthesis '('",
            Self::RightParen => "a right parenthesis ')'",
            Self::LeftBracket => "a left bracket '['",
            Self::RightBracket => "a right bracket ']'",
        };
        write!(f, "{}", kind_str)
    }
}

#[derive(Debug)]
pub struct Lexeme {
    pub kind: Kind,
    pub len: u32,
}
