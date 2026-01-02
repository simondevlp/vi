use lexer::lexeme::Kind;

#[derive(Debug)]
pub enum Keyword {
    Cho,
}

impl Keyword {
    pub fn as_str(&self) -> &str {
        match self {
            Keyword::Cho => "cho",
        }
    }
}

#[derive(Debug)]
pub struct Ident {
    pub start: u32,
    pub len: u32,
}

impl Ident {
    pub fn new(start: u32, len: u32) -> Self {
        Self { start, len }
    }

    pub fn accept(parser: &mut crate::parser::Parser) -> Option<Self> {
        match parser.cur_lexeme.kind {
            Kind::Word => {
                let start = parser.cur_pos;
                let mut len = parser.cur_lexeme.len;
                loop {
                    let mut added = 0;
                    let ws = parser.next_lexeme();
                    match ws.kind {
                        Kind::WordSpaces => {
                            added += ws.len;
                        }
                        _ => {
                            break;
                        }
                    };
                    let word = parser.next_lexeme();
                    match word.kind {
                        Kind::Word => {
                            len += word.len + added;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                Some(Self::new(start, len))
            }
            _ => None,
        }
    }
}
