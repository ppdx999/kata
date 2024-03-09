#[derive(Debug, PartialEq)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    LeftBrace,
    RightBrace,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub location: Location,
    pub next: Option<Box<Token>>,
}

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Object,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        match kind {
            NodeKind::Object => Node {
                kind,
            },
        }
    }
}
