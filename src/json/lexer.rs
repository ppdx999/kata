use super::data::{Token, TokenKind, Location};
use super::error::{SchemaErrors, SchemaError};

pub struct Lexer<'a> {
    source: &'a str,
    chars: std::str::Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.chars(),
        }
    }

    pub fn token(&mut self) -> Result<Option<Box<Token>>, SchemaErrors> {
        let mut head = self.read_next_token()?;
        let mut current = &mut head;

        while current.kind != TokenKind::EOF {
            let token = self.read_next_token()?;
            current.next = Some(Box::new(token));
            current = current.next.as_mut().unwrap();
        }

        Ok(Some(Box::new(head)))
    }

    fn is_identifier(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn read_next_token(&mut self) -> Result<Token, SchemaError> {
        let start = self.offset();

        let kind = match self.chars.next() {
            Some('{') => TokenKind::LeftBrace,
            Some('}') => TokenKind::RightBrace,
            Some(':') => TokenKind::Colon,
            Some(',') => TokenKind::Comma,
            Some('<') => TokenKind::LessThan,
            Some('>') => TokenKind::GreaterThan,
            Some('|') => TokenKind::VerticalBar,
            Some('\'') => {
                let mut string = String::new();
                loop {
                    match self.chars.next() {
                        Some('\'') => break,
                        Some('\\') => {
                            match self.chars.next() {
                                Some('\'') => string.push('\''),
                                Some(char) => {
                                    return Err(SchemaError::UnexpectedCharacter {
                                        text: char.to_string(),
                                        location: Location { start, end: self.offset() },
                                    });
                                }
                                None => {
                                    return Err(SchemaError::UnterminatedString {
                                        location: Location { start, end: self.offset() },
                                    });
                                }
                            }
                        },
                        Some(char) => string.push(char),
                        None => {
                            return Err(SchemaError::UnterminatedString {
                                location: Location { start, end: self.offset() },
                            });
                        }
                    }
                };
                TokenKind::Identifier(string)
            },
            Some('"') => {
                let mut string = String::new();
                loop {
                    match self.chars.next() {
                        Some('\"') => break,
                        Some('\\') => {
                            match self.chars.next() {
                                Some('\"') => string.push('\"'),
                                Some(char) => {
                                    return Err(SchemaError::UnexpectedCharacter {
                                        text: char.to_string(),
                                        location: Location { start, end: self.offset() },
                                    });
                                }
                                None => {
                                    return Err(SchemaError::UnterminatedString {
                                        location: Location { start, end: self.offset() },
                                    });
                                }
                            }
                        },
                        Some(char) => string.push(char),
                        None => {
                            return Err(SchemaError::UnterminatedString {
                                location: Location { start, end: self.offset() },
                            });
                        }
                    }
                };
                TokenKind::Identifier(string)
            },
            Some(char) if self.is_identifier(char) => {
                let mut identifier = char.to_string();
                loop {
                    match self.chars.clone().next() {
                        Some(c) if self.is_identifier(c) => {
                            identifier.push(c);
                            self.chars.next();
                        }
                        _ => break,
                    }
                }
                TokenKind::Identifier(identifier)
            },
            Some(' ') => return self.read_next_token(),
            Some('\n') => return self.read_next_token(),
            Some(char) => return Err(SchemaError::UnexpectedCharacter {
                text: char.to_string(),
                location: Location { start, end: self.offset() },
            }),
            None => TokenKind::EOF,
        };

        let end = self.offset();

        Ok(Token {
            kind,
            location: Location { start, end },
            next: None,
        })
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }
}
