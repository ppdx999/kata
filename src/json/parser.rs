use super::data::{Token, TokenKind, Value, Type, Object, Property};
use super::error::{SchemaErrors, SchemaError};
use super::lexer::Lexer;

pub struct Parser {
    token: Option<Box<Token>>,
}

impl Parser {
    pub fn new(text: &str) -> Result<Parser, SchemaErrors> {
        let mut lexer = Lexer::new(text);
        Ok(Parser { token: lexer.token()? })
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), SchemaError> {
        let token = self.token.take().unwrap();
        if token.kind != kind {
            return Err(SchemaError::UnexpectedToken {
                expected_kind: kind,
                actual_kind: token.kind,
                location: token.location,
            });
        }
        self.token = token.next;
        Ok(())
    }

    fn expect_identifier(&mut self) -> Result<String, SchemaError> {
        let token = self.token.take().unwrap();
        if let TokenKind::Identifier(identifier) = token.kind {
            self.token = token.next;
            return Ok(identifier);
        }
        Result::Err(SchemaError::UnexpectedToken {
            expected_kind: TokenKind::Identifier("".to_string()),
            actual_kind: token.kind,
            location: token.location,
        })
    }

    fn expect_type(&mut self) -> Result<Type, SchemaError> {
        let token = self.token.take().unwrap();

        if let TokenKind::Identifier(identifier) = token.kind {
            self.token = token.next;

            match identifier.as_str() {
                "string" => Ok(Type::String),
                _ => Err(SchemaError::InvalidType {
                    type_: identifier,
                    location: token.location,
                }),
            }
        }
        else {
            Err(SchemaError::UnexpectedToken {
                expected_kind: TokenKind::Identifier("".to_string()),
                actual_kind: token.kind,
                location: token.location,
            })
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

    pub fn parse(&mut self) -> Result<Value, SchemaError> {
        let object = self.object()?;
        self.expect(TokenKind::EOF)?;
        Ok(Value::Object(object))
    }

    fn object(&mut self) -> Result<Object, SchemaError> {
        let mut object = Object::new();
        self.expect(TokenKind::LeftBrace)?;
        while !self.consume(TokenKind::RightBrace) {
            let property = self.property()?;
            object.properties.push(property);
            self.consume(TokenKind::Comma);
        };
        Ok(object)
    }

    fn property(&mut self) -> Result<Property, SchemaError> {
        let name = self.expect_identifier()?;
        self.expect(TokenKind::Colon)?;

        let type_ = if self.peek(TokenKind::LeftBrace) {
            let object = self.object()?;
            Type::Object(Box::new(object))
        } else {
            self.expect_type()?
        };

        Ok(Property::new(name, type_))
    }
}


#[test]
fn test_empty_object() {
    let mut parser = Parser::new("{}").unwrap();
    assert_eq!(parser.parse().unwrap(), Value::Object(Object::new()));
}

#[test]
fn test_simple_object() {
    let mut parser = Parser::new("{name: string}").unwrap();
    assert_eq!(parser.parse().unwrap(), Value::Object(Object {
        properties: vec![Property::new("name".to_string(), Type::String)]
    }));
}


#[test]
fn test_nested_object() {
    let mut parser = Parser::new("{name: string, address: {city: string, country: string}}").unwrap();
    assert_eq!(parser.parse().unwrap(), Value::Object(Object {
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


#[test]
fn test_invalid_type() {
    use super::data::Location;

    let mut parser = Parser::new("{name: invalid}").unwrap();
    assert_eq!(parser.parse().unwrap_err(), SchemaError::InvalidType {
        type_: "invalid".to_string(),
        location: Location { start: 7, end: 14 }
    });
}

#[test]
fn test_invalid_syntax() {
    use super::data::Location;
    let mut parser = Parser::new("{name}").unwrap();

    assert_eq!(parser.parse().unwrap_err(), SchemaError::UnexpectedToken {
        expected_kind: TokenKind::Colon,
        actual_kind: TokenKind::RightBrace,
        location: Location { start: 5, end: 6 }
    });
}
