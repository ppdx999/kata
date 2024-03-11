use crate::json::data::{Token, TokenKind, Node, NodeKind};
use crate::json::lexer::Lexer;

pub struct Parser {
    token: Option<Box<Token>>,
}

impl Parser {
    pub fn new(text: &str) -> Parser {
        let mut lexer = Lexer::new(text);
        Parser { token: lexer.token() }
    }

    fn expect(&mut self, kind: TokenKind) {
        let token = self.token.take().unwrap();
        if token.kind != kind {
            panic!("Unexpected token {:?}", token);
        }
        self.token = token.next;
    }

    pub fn parse(&mut self) -> Node {
        let token = self.token.take().unwrap();
        let kind = token.kind;
        self.token = token.next;

        match kind {
            TokenKind::LeftBrace => {
                let node = self.object();
                self.expect(TokenKind::RightBrace);
                self.expect(TokenKind::EOF);
                node
            }
            _ => panic!("Unexpected token"),
        }
    }

    fn object(&mut self) -> Node {
        Node::new(NodeKind::Object)
    }
}


#[test]
fn test_parser() {
    let mut parser = Parser::new("{}");
    assert_eq!(parser.parse(), Node::new(NodeKind::Object));
}
