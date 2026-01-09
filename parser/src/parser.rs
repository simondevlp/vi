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
    pub cur_line: u32,
    diag: Vec<crate::diag::Diag>,
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
            cur_line: 1,
            diag: Vec::new(),
        };
        parser.cur_lexeme = parser.lexer.next();
        parser
    }

    pub fn next_lexeme(&mut self) -> &Lexeme {
        self.cur_pos += self.cur_lexeme.len;
        self.cur_lexeme = self.lexer.next();
        if matches!(self.cur_lexeme.kind, lexeme::Kind::Eol) {
            self.cur_line += 1;
        }
        &self.cur_lexeme
    }

    pub fn skip_ws_if_any(&mut self, including_eol: bool) -> &Lexeme {
        match self.cur_lexeme.kind {
            lexeme::Kind::Whitespaces | lexeme::Kind::WordSpaces => {
                self.next_non_ws_lexeme(including_eol)
            }
            lexeme::Kind::Eol if including_eol => self.next_non_ws_lexeme(including_eol),
            _ => &self.cur_lexeme,
        }
    }

    pub fn next_non_ws_lexeme(&mut self, including_eol: bool) -> &Lexeme {
        loop {
            self.next_lexeme();
            match self.cur_lexeme.kind {
                lexeme::Kind::Whitespaces | lexeme::Kind::WordSpaces => {}
                lexeme::Kind::Eol if including_eol => {}
                _ => return &self.cur_lexeme,
            }
        }
    }

    pub fn get_snippet(&self, start: u32, len: u32) -> &str {
        &self.input[start as usize..(start + len) as usize]
    }

    pub fn cur_lexeme_snippet(&self) -> &str {
        self.get_snippet(self.cur_pos, self.cur_lexeme.len)
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
            eprintln!("{}", diag);
        }
    }
}
