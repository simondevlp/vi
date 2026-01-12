use crate::lexeme::{Kind, Lexeme};

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn remaining(&self) -> &'a str {
        &self.input[self.pos..]
    }

    fn peek_char(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.peek_char();
        if let Some(c) = ch {
            self.pos += c.len_utf8();
        }
        ch
    }

    fn advance_char_while(&mut self, condition: impl Fn(char) -> bool) {
        while let Some(ch) = self.peek_char() {
            if condition(ch) {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn check_is_alpha(ch: char) -> bool {
        matches!(
            ch,
            'a'..='z' // lowercase
                | 'á' // 'a' variants
                | 'à'
                | 'ả'
                | 'ã'
                | 'ạ'
                | 'ă' // 'ă' and variants
                | 'ắ'
                | 'ằ'
                | 'ẳ'
                | 'ẵ'
                | 'ặ'
                | 'â' // 'â' and variants
                | 'ấ'
                | 'ầ'
                | 'ẩ'
                | 'ẫ'
                | 'ậ'
                | 'đ' // 'đ'
                | 'é' // 'e' variants
                | 'è'
                | 'ẻ'
                | 'ẽ'
                | 'ẹ'
                | 'ê' // 'ê' and variants
                | 'ế'
                | 'ề'
                | 'ể'
                | 'ễ'
                | 'ệ'
                | 'í' // 'i' variants
                | 'ì'
                | 'ỉ'
                | 'ĩ'
                | 'ị'
                | 'ó' // 'o' variants
                | 'ò'
                | 'ỏ'
                | 'õ'
                | 'ọ'
                | 'ô' // 'ô' and variants
                | 'ố'
                | 'ồ'
                | 'ổ'
                | 'ỗ'
                | 'ộ'
                | 'ơ' // 'ơ' and variants
                | 'ớ'
                | 'ờ'
                | 'ở'
                | 'ỡ'
                | 'ợ'
                | 'ú' // 'u' variants
                | 'ù'
                | 'ủ'
                | 'ũ'
                | 'ụ'
                | 'ư' // 'ư' and variants
                | 'ứ'
                | 'ừ'
                | 'ử'
                | 'ữ'
                | 'ự'
                | 'ý' // 'y' variants
                | 'ỳ'
                | 'ỷ'
                | 'ỹ'
                | 'ỵ'
                | 'A'..='Z' // UPPERCASE
                | 'Á' // 'A' VARIANTS
                | 'À'
                | 'Ả'
                | 'Ã'
                | 'Ạ'
                | 'Ă' // 'Ă' AND VARIANTS
                | 'Ắ'
                | 'Ằ'
                | 'Ẳ'
                | 'Ẵ'
                | 'Ặ'
                | 'Â' // 'Â' AND VARIANTS
                | 'Ấ'
                | 'Ầ'
                | 'Ẩ'
                | 'Ẫ'
                | 'Ậ'
                | 'Đ' // 'Đ'
                | 'É' // 'E' VARIANTS
                | 'È'
                | 'Ẻ'
                | 'Ẽ'
                | 'Ẹ'
                | 'Ê' // 'Ê' AND VARIANTS
                | 'Ế'
                | 'Ề'
                | 'Ể'
                | 'Ễ'
                | 'Ệ'
                | 'Í' // 'I' VARIANTS
                | 'Ì'
                | 'Ỉ'
                | 'Ĩ'
                | 'Ị'
                | 'Ó' // 'O' VARIANTS
                | 'Ò'
                | 'Ỏ'
                | 'Õ'
                | 'Ọ'
                | 'Ô' // 'Ô' AND VARIANTS
                | 'Ố'
                | 'Ồ'
                | 'Ổ'
                | 'Ỗ'
                | 'Ộ'
                | 'Ơ' // 'Ơ' AND VARIANTS
                | 'Ớ'
                | 'Ờ'
                | 'Ở'
                | 'Ỡ'
                | 'Ợ'
                | 'Ú' // 'U' VARIANTS
                | 'Ù'
                | 'Ủ'
                | 'Ũ'
                | 'Ụ'
                | 'Ư' // 'Ư' AND VARIANTS
                | 'Ứ'
                | 'Ừ'
                | 'Ử'
                | 'Ữ'
                | 'Ự'
                | 'Ý' // 'Y' VARIANTS
                | 'Ỳ'
                | 'Ỷ'
                | 'Ỹ'
                | 'Ỵ'
        )
    }

    pub fn next(&mut self) -> Lexeme {
        let start_pos = self.pos;
        let kind = match self.next_char() {
            None => Kind::Eof,
            Some('\n') => Kind::Eol,
            Some(' ') => {
                while let Some(' ') = self.peek_char() {
                    self.next_char();
                }
                Kind::WordSpaces
            }
            Some('#') => {
                self.advance_char_while(|c| c != '\n');
                Kind::Comment
            }
            Some(lead) if lead.is_whitespace() => {
                self.advance_char_while(|c| c.is_whitespace() && c != ' ' && c != '\n');
                Kind::Whitespaces
            }
            Some(lead) if Self::check_is_alpha(lead) || lead == '_' => {
                self.advance_char_while(|c| {
                    Self::check_is_alpha(c) || c == '_' || matches!(c, '0'..='9')
                });
                Kind::Word
            }
            Some(quote @ ('"' | '\'')) => {
                self.advance_char_while(|c| c != quote);
                self.next_char();
                Kind::String
            }
            Some('0'..='9') => {
                self.advance_char_while(|c| matches!(c, '0'..='9'));
                match self.peek_char() {
                    Some('.') => {
                        self.next_char();
                        self.advance_char_while(|c| matches!(c, '0'..='9'));
                        Kind::Float
                    }
                    _ => Kind::Decimal,
                }
            }
            Some('+') => Kind::Plus,
            Some('-') => Kind::Minus,
            Some('*') => Kind::Asterisk,
            Some('/') => Kind::Slash,
            Some('=') => Kind::Equal,
            Some('.') => Kind::Period,
            Some(',') => Kind::Comma,
            Some('>') => Kind::Greater,
            Some('<') => Kind::Less,
            Some('{') => Kind::LeftBrace,
            Some('}') => Kind::RightBrace,
            Some('(') => Kind::LeftParen,
            Some(')') => Kind::RightParen,
            Some('[') => Kind::LeftBracket,
            Some(']') => Kind::RightBracket,
            Some(_) => Kind::Invalid,
        };
        let byte_len = (self.pos - start_pos) as u32;
        Lexeme {
            kind,
            len: byte_len,
        }
    }
}
