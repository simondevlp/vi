use lexer::{
    lexeme::{self, Lexeme},
    lexer::Lexer,
};

use crate::syntax::Programme;

pub struct Parser<'a> {
    input: &'a str,
    pub lexer: Lexer<'a>,
    pub cur_pos: u32,
    pub cur_lexeme: Lexeme,
    diag: Vec<crate::diag::Diags>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut parser = Parser {
            input,
            lexer: Lexer::new(input),
            cur_pos: 0,
            cur_lexeme: Lexeme {
                kind: lexeme::Kind::Invalid,
                len: 0,
            },
            diag: Vec::new(),
        };
        parser.cur_lexeme = parser.lexer.next();
        parser
    }

    pub fn next_lexeme(&mut self) -> &Lexeme {
        self.cur_pos += self.cur_lexeme.len;
        self.cur_lexeme = self.lexer.next();
        &self.cur_lexeme
    }

    pub fn next_non_ws_lexeme(&mut self) -> &Lexeme {
        loop {
            self.next_lexeme();
            match self.cur_lexeme.kind {
                lexer::lexeme::Kind::Whitespaces | lexer::lexeme::Kind::WordSpaces => continue,
                _ => return &self.cur_lexeme,
            }
        }
    }

    pub fn cur_lexeme_snippet(&self) -> &str {
        &self.input[self.cur_pos as usize..(self.cur_pos + self.cur_lexeme.len) as usize]
    }

    pub fn cur_lexeme_snippet_is(&self, expected: &str) -> bool {
        self.cur_lexeme_snippet() == expected
    }

    pub fn visit_programme(&mut self) -> Option<Programme> {
        let status = Programme::accept(self);
        match status {
            Ok(prog) => Some(prog),
            Err(diags) => {
                self.diag.push(diags);
                None
            }
        }
    }

    pub fn print_diags(&self) {
        for diag in &self.diag {
            diag.print();
        }
    }
}
