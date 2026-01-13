use lexer::lexeme;

use crate::{
    accept::Acceptor,
    diag::{Diag, DiagData, Error},
    parser::Parser,
    syntax::expr::terminal::{Field, Ident},
};

#[derive(Debug)]
pub struct ClassPath {
    pub segments: Vec<Ident>,
}

impl Acceptor for ClassPath {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag>
    where
        Self: Sized,
    {
        let Some(Field::Property(first_segment)) = Field::accept(parser)? else {
            return Ok(None);
        };
        let mut segments = Vec::new();
        segments.push(first_segment);
        loop {
            parser.skip_ws_if_any(false);
            match parser.cur_lexeme.kind {
                lexeme::Kind::Period => {
                    parser.next_non_ws_lexeme(true); // consume '.'
                    parser.skip_ws_if_any(false);
                    let Some(Field::Property(segment)) = Field::accept(parser)? else {
                        return Err(Diag {
                            line: parser.cur_line,
                            data: DiagData::Err(Error::Expecting {
                                expected: "a segment identifier after '.' in class path",
                            }),
                        });
                    };
                    segments.push(segment);
                }
                _ => {
                    break;
                }
            }
        }
        Ok(Some(ClassPath { segments }))
    }
}

#[derive(Debug)]
pub struct Type {
    pub class_path: ClassPath,
}

impl Acceptor for Type {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let Some(class_path) = ClassPath::accept(parser)? else {
            return Ok(None);
        };
        Ok(Some(Type { class_path }))
    }
}

#[derive(Debug)]
pub struct NameTypePair {
    pub name: Ident,
    pub type_: Option<Type>,
}

impl Acceptor for NameTypePair {
    fn accept(parser: &mut Parser) -> Result<Option<Self>, Diag> {
        let Some(name) = Ident::accept(parser)? else {
            return Ok(None);
        };
        parser.skip_ws_if_any(false);
        if !matches!(parser.cur_lexeme.kind, lexeme::Kind::Colon) {
            return Ok(Some(NameTypePair { name, type_: None }));
        }
        parser.next_non_ws_lexeme(true); // consume ':'
        parser.skip_ws_if_any(false);
        let Some(type_) = Type::accept(parser)? else {
            return Err(Diag {
                line: parser.cur_line,
                data: DiagData::Err(Error::Expecting {
                    expected: "a type after ':' in name-type pair",
                }),
            });
        };
        Ok(Some(NameTypePair {
            name,
            type_: Some(type_),
        }))
    }
}
