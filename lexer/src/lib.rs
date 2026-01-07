pub mod lexeme;
pub mod lexer;

#[macro_export]
macro_rules! assert_lexeme {
    ($lexeme:expr, $kind:pat, $len:expr) => {{
        assert!(
            matches!($lexeme.kind, $kind) && $lexeme.len == $len,
            "Expected {} with length {}, got {} with length {}",
            stringify!($kind),
            $len,
            $lexeme.kind,
            $lexeme.len
        );
    }};
}

#[macro_export]
macro_rules! assert_lexer_lexeme {
    ($lexer:expr, $kind:pat, $len:expr) => {{
        let ____lexeme = $lexer.next();
        $crate::assert_lexeme!(____lexeme, $kind, $len);
    }};
}

#[macro_export]
macro_rules! assert_lexer_eof {
    ($lexer:expr) => {{
        $crate::assert_lexer_lexeme!($lexer, Kind::Eof, 0);
    }};
}

#[cfg(test)]
pub mod tests {
    use crate::{lexeme::Kind, lexer::Lexer};

    #[test]
    fn always_pass() {
        assert!(true);
    }

    #[test]
    fn empty_repeated_eofs() {
        let mut lexer = Lexer::new("");
        assert_lexer_eof!(lexer);
        assert_lexer_eof!(lexer);
        assert_lexer_eof!(lexer);
    }

    #[test]
    fn whitespaces() {
        let mut lexer = Lexer::new("\t\r\u{2003}");
        assert_lexer_lexeme!(lexer, Kind::Whitespaces, 3);
        assert_lexer_eof!(lexer);
    }

    #[test]
    fn word_spaces() {
        let mut lexer = Lexer::new("  ");
        assert_lexer_lexeme!(lexer, Kind::WordSpaces, 2);
        assert_lexer_eof!(lexer);
    }

    #[test]
    fn word() {
        let mut lexer = Lexer::new("hello_world123");
        assert_lexer_lexeme!(lexer, Kind::Word, 14);
        assert_lexer_eof!(lexer);
    }

    #[test]
    fn word_with_unicode() {
        let mut lexer = Lexer::new("xin_chào123");
        assert_lexer_lexeme!(lexer, Kind::Word, 11);
        assert_lexer_eof!(lexer);
    }

    #[test]
    fn decimal() {
        let mut lexer = Lexer::new("123456");
        assert_lexer_lexeme!(lexer, Kind::Decimal, 6);
        assert_lexer_eof!(lexer);
    }

    #[test]
    fn string() {
        let mut lexer = Lexer::new("\"Hello, World!\" 'Another one'");
        assert_lexer_lexeme!(lexer, Kind::String, 15);
        assert_lexer_lexeme!(lexer, Kind::WordSpaces, 1);
        assert_lexer_lexeme!(lexer, Kind::String, 13);
        assert_lexer_eof!(lexer);
    }

    #[test]
    fn comment() {
        let mut lexer = Lexer::new("# This is a comment\nNext line");
        assert_lexer_lexeme!(lexer, Kind::Comment, 19);
        assert_lexer_lexeme!(lexer, Kind::Eol, 1);
        assert_lexer_lexeme!(lexer, Kind::Word, 4);
        assert_lexer_lexeme!(lexer, Kind::WordSpaces, 1);
        assert_lexer_lexeme!(lexer, Kind::Word, 4);
        assert_lexer_eof!(lexer);
    }

    #[test]
    fn invalid() {
        let mut lexer = Lexer::new("@$%");
        assert_lexer_lexeme!(lexer, Kind::Invalid, 1);
        assert_lexer_lexeme!(lexer, Kind::Invalid, 1);
        assert_lexer_lexeme!(lexer, Kind::Invalid, 1);
        assert_lexer_eof!(lexer);
    }

    #[test]
    fn cho_stmt() {
        let mut lexer = Lexer::new("cho variable_name");
        assert_lexer_lexeme!(lexer, Kind::Word, 3); // cho
        assert_lexer_lexeme!(lexer, Kind::WordSpaces, 1); // space
        assert_lexer_lexeme!(lexer, Kind::Word, 13); // variable_name
        assert_lexer_eof!(lexer);
    }
}
