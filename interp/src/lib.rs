use parser::{Span, parser::Parser, syntax::Programme};

use crate::{interp::Interpretable, scope::Scope};

pub mod diag;
pub mod eval;
pub mod interp;
pub mod obj;
pub mod scope;

pub struct Evaluator<'a> {
    parser: Parser<'a>,
    scope_stack: Vec<Scope>,
}

impl<'a> Evaluator<'a> {
    pub fn new(input: &'a str) -> Self {
        Evaluator {
            parser: Parser::new(input),
            scope_stack: vec![Scope::global()],
        }
    }

    pub fn current_scope(&self) -> &Scope {
        self.scope_stack
            .last()
            .expect("There should always be a current scope")
    }

    pub fn current_scope_mut(&mut self) -> &mut Scope {
        self.scope_stack
            .last_mut()
            .expect("There should always be a current scope")
    }

    pub fn parse(&mut self) -> Option<Programme> {
        let prog = self.parser.visit_programme();
        if self.parser.diag.len() > 0 {
            self.parser.print_diags();
            None
        } else {
            prog
        }
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

    pub fn snippet(&self, span: &Span) -> String {
        self.parser.get_snippet(span)
    }
}
