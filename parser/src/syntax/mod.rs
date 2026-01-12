use lexer::lexeme;

use crate::{accept::Acceptor, diag::Diag, parser::Parser, syntax::stmt::Statement};

pub mod expr;
pub mod stmt;
mod type_;

#[derive(Debug)]
pub struct Programme {
    pub statements: Vec<Statement>,
}

impl Acceptor for Programme {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let mut statements = Vec::new();
        loop {
            parser.skip_ws_if_any(true);
            if let Some(stmt) = Statement::accept(parser)? {
                statements.push(stmt);
            } else {
                break;
            }
        }
        Ok(Some(Programme { statements }))
    }
}

#[derive(Debug)]
pub struct Indent;

impl Acceptor for Indent {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        Ok(match parser.cur_lexeme.kind {
            lexeme::Kind::WordSpaces if parser.cur_lexeme.len == 4 => {
                parser.next_non_ws_lexeme(true);
                Some(Indent)
            }
            _ => None,
        })
    }
}

#[derive(Debug)]
pub struct Block<T: Sized + Acceptor> {
    pub items: Vec<T>,
}

impl<T: Sized + Acceptor> Acceptor for Block<T> {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag>
    where
        Self: Sized,
    {
        let mut items = Vec::new();
        let Some(_) = Indent::accept(parser)? else {
            return Ok(None);
        };
        loop {
            parser.skip_ws_if_any(true);
            if let Some(item) = T::accept(parser)? {
                items.push(item);
            } else {
                break;
            }
        }
        Ok(Some(Block { items }))
    }
}

pub type Span = (u32, u32);
