use crate::{diag::Diag, parser::Parser};

pub trait Acceptor {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag>
    where
        Self: Sized;
}
