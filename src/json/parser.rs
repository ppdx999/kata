use super::data::{Token, TokenKind, Value, Type, Object, Property, Array};
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
        if self.peek(TokenKind::LeftBrace) {
            let object = self.object()?;
            return Ok(Type::Object(Box::new(object)));
        }

        if self.peek(TokenKind::Identifier("Array".to_string())) {
            let array = self.array()?;
            return Ok(Type::Array(Box::new(array)));
        }

        // Primitive types

        let token = self.token.take().unwrap();

        if let TokenKind::Identifier(identifier) = token.kind {
            self.token = token.next;

            match identifier.as_str() {
                "null" => Ok(Type::Null),
                "string" => Ok(Type::String),
                "number" => Ok(Type::Number),
                "boolean" => Ok(Type::Boolean),
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
        let value = self.value()?;
        self.expect(TokenKind::EOF)?;
        Ok(value)
    }

    fn value(&mut self) -> Result<Value, SchemaError> {
        if self.peek(TokenKind::LeftBrace) {
            Ok(Value::Object(self.object()?))
        }
        else if self.peek(TokenKind::Identifier("Array".to_string())) {
            Ok(Value::Array(self.array()?))
        }
        else {
            Ok(Value::Type(self.expect_type()?))
        }
    }

    fn array(&mut self) -> Result<Array, SchemaError> {
        self.expect(TokenKind::Identifier("Array".to_string()))?;
        self.expect(TokenKind::LessThan)?;
        let type_ = self.expect_type()?;
        self.expect(TokenKind::GreaterThan)?;
        Ok(Array::new(type_))
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

        let type_ = self.expect_type()?;

        Ok(Property::new(name, type_))
    }
}
