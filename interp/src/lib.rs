use parser::{parser::Parser, syntax::Programme};

use crate::interp::Interpretable;

pub mod diag;
pub mod eval;
pub mod interp;
pub mod obj;

pub struct Interpreter<'a> {
    parser: Parser<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(input: &'a str) -> Self {
        Interpreter {
            parser: Parser::new(input),
        }
    }

    pub fn parse(&mut self) -> Option<Programme> {
        self.parser.visit_programme()
    }

    pub fn cur_line(&self) -> u32 {
        self.parser.cur_line
    }

    pub fn interpret(&mut self, prog: &Option<Programme>) {
        match prog {
            Some(p) => {
                for stmt in &p.statements {
                    match stmt.interpret(self) {
                        Ok(_) => {}
                        Err(diag) => {
                            eprintln!("Error at line {}: {}", diag.line, diag.data);
                            break;
                        }
                    }
                }
            }
            None => {
                println!("No programme parsed.");
            }
        }
    }
}
