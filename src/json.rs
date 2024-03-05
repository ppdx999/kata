use std::io::BufRead;

#[derive(Debug, PartialEq)]
pub struct Schema {
    root: Node,
}

impl Schema {
    pub fn from_text(text: &str) -> Result<Self, String> {
        let mut parser = Parser::new(text);
        Ok(Schema { root: parser.parse() })
    }

    pub fn validate(&self, _reader: Box<dyn BufRead>) -> Result<(), String> {
        Ok(())
    }

}

#[test]
fn test_schema_from_text() {
    let schema = Schema::from_text("{}").unwrap();
    assert_eq!(schema, Schema {
        root: Node::new(NodeKind::Object),
    });
}

struct Parser {
    token: Option<Box<Token>>,
}

impl Parser {
    fn new(text: &str) -> Parser {
        let mut lexer = Lexer::new(text);

        let mut head = lexer.read_next_token();
        let mut current = &mut head;
        while current.kind != TokenKind::EOF {
            let token = lexer.read_next_token();
            current.next = Some(Box::new(token));
            current = current.next.as_mut().unwrap();
        }

        Parser { token: Some(Box::new(head)) }
    }

    fn expect(&mut self, kind: TokenKind) {
        let token = self.token.take().unwrap();
        if token.kind != kind {
            panic!("Unexpected token");
        }
        self.token = token.next;
    }

    fn consume(&mut self, kind: TokenKind) -> bool {
        if let Some(token) = &self.token {
            if token.kind == kind {
                self.token = self.token.take().unwrap().next;
                return true;
            }
        }
        false
    }

    fn parse(&mut self) -> Node {
        let node = self.object();
        self.expect(TokenKind::EOF);
        node
    }

    fn object(&mut self) -> Node {
        self.expect(TokenKind::LeftBrace);
        self.expect(TokenKind::RightBrace);
        Node::new(NodeKind::Object)
    }
}

#[test]
fn test_parser() {
    let mut parser = Parser::new("{}");
    assert_eq!(parser.parse(), Node::new(NodeKind::Object));
}

#[derive(Debug, PartialEq)]
struct Location {
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    LeftBrace,
    RightBrace,
    EOF,
}

#[derive(Debug, PartialEq)]
struct Token {
    kind: TokenKind,
    location: Location,
    next: Option<Box<Token>>,
}

struct Lexer<'a> {
    source: &'a str,
    chars: std::str::Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.chars(),
        }
    }

    fn read_next_token(&mut self) -> Token {
        let start = self.offset();
        let kind = self.read_next_token_kind();
        let end = self.offset();
        Token {
            kind,
            location: Location { start, end },
            next: None,
        }
    }

    fn read_next_token_kind(&mut self) -> TokenKind {
        match self.chars.next() {
            Some('{') => TokenKind::LeftBrace,
            Some('}') => TokenKind::RightBrace,
            None => TokenKind::EOF,
            _ => panic!("Unexpected character"),
        }
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("{}");

    assert_eq!(lexer.read_next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.read_next_token().kind, TokenKind::RightBrace);
    assert_eq!(lexer.read_next_token().kind, TokenKind::EOF);
}

#[derive(Debug, PartialEq)]
enum NodeKind {
    Object,
}

#[derive(Debug, PartialEq)]
struct Node {
    kind: NodeKind,
}

impl Node {
    fn new(kind: NodeKind) -> Self {
        match kind {
            NodeKind::Object => Node {
                kind,
            },
        }
    }
}
