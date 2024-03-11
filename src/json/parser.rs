use crate::json::data::{Token, TokenKind, Node, NodeKind, Type, Object, Property};
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

    fn expect_identifier(&mut self) -> String {
        let token = self.token.take().unwrap();
        if let TokenKind::Identifier(identifier) = token.kind {
            self.token = token.next;
            return identifier;
        }
        panic!("Unexpected token {:?}", token);
    }

    fn expect_type(&mut self) -> Type {
        let type_ = self.expect_identifier();
        match type_.as_str() {
            "string" => Type::String,
            _ => panic!("Invalid type: {}", type_),
        }
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
        let mut object = Object::new();
        self.expect(TokenKind::LeftBrace);
        while !self.consume(TokenKind::RightBrace) {
            let property = self.property();
            object.properties.push(property);
        };
        Node::new(NodeKind::Object(object))
    }

    fn property(&mut self) -> Property {
        let name = self.expect_identifier();
        self.expect(TokenKind::Colon);
        let type_ = self.expect_type();

        Property::new(name, type_)
    }
}


#[test]
fn test_empty_object() {
    let mut parser = Parser::new("{}");
    assert_eq!(parser.parse(), Node::new(NodeKind::Object(Object::new())));
}

#[test]
fn test_simple_object() {
    let mut parser = Parser::new("{name: string}");
    assert_eq!(parser.parse(), Node::new(NodeKind::Object(Object {
        properties: vec![Property::new("name".to_string(), Type::String)]
    })));
}
