use crate::json::data::{Token, TokenKind, Node, NodeKind};
use crate::json::lexer::Lexer;

pub struct Parser {
    token: Option<Box<Token>>,
}

impl Parser {
    pub fn new(text: &str) -> Parser {
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

    pub fn parse(&mut self) -> Node {
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
