mod lexeme;
mod lexer;

#[cfg(test)]
mod tests {
    use crate::{lexeme::Kind, lexer::Lexer};

    macro_rules! assert_lexeme {
        ($lexer:ident, $kind:pat, $len:expr) => {{
            let lexeme = $lexer.next();
            assert!(
                matches!(lexeme.kind, $kind) && lexeme.len == $len,
                "Expected {:?} with length {}, got {:?} with length {}",
                stringify!($kind),
                $len,
                lexeme.kind,
                lexeme.len
            );
        }};
    }

    macro_rules! assert_eof {
        ($lexer:ident) => {{
            assert_lexeme!($lexer, Kind::Eof, 0);
        }};
    }

    #[test]
    fn always_pass() {
        assert!(true);
    }

    #[test]
    fn empty_repeated_eofs() {
        let mut lexer = Lexer::new("");
        assert_eof!(lexer);
        assert_eof!(lexer);
        assert_eof!(lexer);
    }

    #[test]
    fn whitespaces() {
        let mut lexer = Lexer::new("\t\r\u{2003}");
        assert_lexeme!(lexer, Kind::Whitespaces, 3);
        assert_eof!(lexer);
    }

    #[test]
    fn word_spaces() {
        let mut lexer = Lexer::new("  ");
        assert_lexeme!(lexer, Kind::WordSpaces, 2);
        assert_eof!(lexer);
    }

    #[test]
    fn word() {
        let mut lexer = Lexer::new("hello_world123");
        assert_lexeme!(lexer, Kind::Word, 14);
        assert_eof!(lexer);
    }

    #[test]
    fn word_with_unicode() {
        let mut lexer = Lexer::new("xin_chào123");
        assert_lexeme!(lexer, Kind::Word, 11);
        assert_eof!(lexer);
    }

    #[test]
    fn decimal() {
        let mut lexer = Lexer::new("123456");
        assert_lexeme!(lexer, Kind::Decimal, 6);
        assert_eof!(lexer);
    }

    #[test]
    fn string() {
        let mut lexer = Lexer::new("\"Hello, World!\" 'Another one'");
        assert_lexeme!(lexer, Kind::String, 15);
        assert_lexeme!(lexer, Kind::WordSpaces, 1);
        assert_lexeme!(lexer, Kind::String, 13);
        assert_eof!(lexer);
    }

    #[test]
    fn comment() {
        let mut lexer = Lexer::new("# This is a comment\nNext line");
        assert_lexeme!(lexer, Kind::Comment, 19);
        assert_lexeme!(lexer, Kind::Eol, 1);
        assert_lexeme!(lexer, Kind::Word, 4);
        assert_lexeme!(lexer, Kind::WordSpaces, 1);
        assert_lexeme!(lexer, Kind::Word, 4);
        assert_eof!(lexer);
    }
}
