mod lexeme;
mod lexer;

#[cfg(test)]
mod tests {
    use crate::{lexeme::Kind, lexer::Lexer};

    fn empty() {
        let mut lexer = Lexer::new("");
        assert!(matches!(lexer.next().kind, Kind::Eof) && lexer.next().len == 0);
    }
}
