use crate::json::data::{Token, TokenKind, Value, Type, Object, Property};
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

    fn peek(&self, kind: TokenKind) -> bool {
        if let Some(token) = &self.token {
            return token.kind == kind;
        }
        false
    }

    pub fn parse(&mut self) -> Value {
        let object = self.object();
        self.expect(TokenKind::EOF);
        Value::Object(object)
    }

    fn object(&mut self) -> Object {
        let mut object = Object::new();
        self.expect(TokenKind::LeftBrace);
        while !self.consume(TokenKind::RightBrace) {
            let property = self.property();
            object.properties.push(property);
            self.consume(TokenKind::Comma);
        };
        object
    }

    fn property(&mut self) -> Property {
        let name = self.expect_identifier();
        self.expect(TokenKind::Colon);

        let type_ = if self.peek(TokenKind::LeftBrace) {
            let object = self.object();
            Type::Object(Box::new(object))
        } else {
            self.expect_type()
        };

        Property::new(name, type_)
    }
}


#[test]
fn test_empty_object() {
    let mut parser = Parser::new("{}");
    assert_eq!(parser.parse(), Value::Object(Object::new()));
}

#[test]
fn test_simple_object() {
    let mut parser = Parser::new("{name: string}");
    assert_eq!(parser.parse(), Value::Object(Object {
        properties: vec![Property::new("name".to_string(), Type::String)]
    }));
}


#[test]
fn test_nested_object() {
    let mut parser = Parser::new("{name: string, address: {city: string, country: string}}");
    assert_eq!(parser.parse(), Value::Object(Object {
        properties: vec![
            Property::new("name".to_string(), Type::String),
            Property::new("address".to_string(), Type::Object(
                Box::new(
                    Object {
                        properties: vec![
                            Property::new("city".to_string(), Type::String),
                            Property::new("country".to_string(), Type::String),
                        ]
                    }
                )
            ))
        ]
    }));
}
