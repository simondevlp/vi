use lexer::lexeme;

use crate::{
    diag::{Diag, DiagData, Error},
    parser::Parser,
    syntax::expr::{
        Expr, TupleExpr,
        terminal::{Ident, Keyword},
    },
};

#[derive(Debug)]
pub enum Statement {
    Cho(ChoStatement),
    Invocation(InvocationStatement),
}

impl Statement {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        if matches!(parser.cur_lexeme.kind, lexeme::Kind::Word) {
            if parser.cur_lexeme_snippet_is(Keyword::Cho.as_str()) {
                let cho_stmt = ChoStatement::accept(parser)?;
                Ok(Statement::Cho(cho_stmt))
            } else {
                let invoc_stmt = InvocationStatement::accept(parser)?;
                Ok(Statement::Invocation(invoc_stmt))
            }
        } else {
            Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::MiscExpecting {
                    expected: "a statement".to_string(),
                }),
                span: parser.cur_span(),
            })
        }
    }
}

#[derive(Debug)]
pub struct ChoStatement {
    pub kw: Keyword,
    pub lhs: Ident,
    pub rhs: Option<Expr>,
}

impl ChoStatement {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        let kw = Keyword::accept(parser, Keyword::Cho)?;
        let lhs = Ident::accept(parser)?;
        parser.skip_ws_if_any(false);
        if !matches!(parser.cur_lexeme.kind, lexeme::Kind::Equal) {
            return Ok(ChoStatement { kw, lhs, rhs: None });
        }
        parser.next_non_ws_lexeme(true); // consumes '='
        let rhs = Expr::accept(parser)?;
        Ok(ChoStatement {
            kw,
            lhs,
            rhs: Some(rhs),
        })
    }
}

#[derive(Debug)]
pub struct InvocationStatement {
    pub callee: Ident,
    pub tuple: TupleExpr,
}

impl InvocationStatement {
    pub fn accept(parser: &mut Parser) -> Result<Self, Diag> {
        let callee = Ident::accept(parser)?;
        let tuple = TupleExpr::accept(parser)?;
        Ok(InvocationStatement { callee, tuple })
    }
}
