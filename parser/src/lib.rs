mod diag;
mod parser;
mod syntax;

#[cfg(test)]
mod tests {
    use lexer::{assert_lexeme, lexeme};

    use crate::syntax::{stmt::Statement, terminal::Ident};

    #[test]
    fn always_pass() {
        assert!(true);
    }

    #[test]
    fn parser_skip() {
        let input = "   \t    cho myVar";
        let mut parser = crate::parser::Parser::new(input);
        parser.next_non_ws_lexeme();
        assert_lexeme!(parser.cur_lexeme, lexeme::Kind::Word, 3);
    }

    #[test]
    fn ident_accept() {
        let input = "myVar  cho";
        let mut parser = crate::parser::Parser::new(input);
        let ident = Ident::accept(&mut parser);
        assert!(
            matches!(ident, Some(Ident { start: 0, len: 10 })),
            "Got {:?}",
            ident
        );
    }

    #[test]
    fn cho_stmt() {
        let input = "cho  myVariable";
        let mut parser = crate::parser::Parser::new(input);
        match parser.visit_programme() {
            None => {
                parser.print_diags();
                panic!("Failed to parse programme")
            }
            Some(prog) => {
                assert_eq!(prog.statements.len(), 1);
                match &prog.statements[0] {
                    Statement::Cho(cho_stmt) => {
                        assert!(
                            matches!(cho_stmt.lhs, Ident { start: 5, len: 10 }),
                            "Got {:?}",
                            cho_stmt.lhs
                        );
                    }
                }
            }
        };
    }
}
