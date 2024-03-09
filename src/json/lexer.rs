use crate::json::data::{Token, TokenKind, Location};

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

    pub fn token(&mut self) -> Option<Box<Token>> {
        let mut head = self.read_next_token();
        let mut current = &mut head;
        while current.kind != TokenKind::EOF {
            let token = self.read_next_token();
            current.next = Some(Box::new(token));
            current = current.next.as_mut().unwrap();
        }
        Some(Box::new(head))
    }

    fn read_next_token(&mut self) -> Token {
        let start = self.offset();

        let kind = match self.chars.next() {
            Some('{') => TokenKind::LeftBrace,
            Some('}') => TokenKind::RightBrace,
            Some(':') => TokenKind::Colon,
            Some(',') => TokenKind::Comma,
            Some(char) if char.is_alphanumeric() => {
                let mut identifier = char.to_string();
                loop {
                    match self.chars.clone().next() {
                        Some(c) if c.is_alphanumeric() => {
                            identifier.push(c);
                            self.chars.next();
                        }
                        _ => break,
                    }
                }
                TokenKind::Identifier(identifier)
            }
            Some(' ') => return self.read_next_token(),
            Some(char) => panic!("Unexpected character: {}", char),
            None => TokenKind::EOF,
        };

        let end = self.offset();

        Token {
            kind,
            location: Location { start, end },
            next: None,
        }
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }
}

#[test]
fn test_empty_object() {
    let mut lexer = Lexer::new("{}");
    let token = lexer.token();

    assert_eq!(token, Some(Box::new(Token {
        kind: TokenKind::LeftBrace,
        location: Location { start: 0, end: 1 },
        next: Some(Box::new(Token {
            kind: TokenKind::RightBrace,
            location: Location { start: 1, end: 2 },
            next: Some(Box::new(Token {
                kind: TokenKind::EOF,
                location: Location { start: 2, end: 2 },
                next: None,
            })),
        })),
    })));
}


#[test]
fn test_simple_object() {
    let mut lexer = Lexer::new("{name: string}");
    let token = lexer.token();

    assert_eq!(token, Some(Box::new(Token {
        kind: TokenKind::LeftBrace,
        location: Location { start: 0, end: 1 },
        next: Some(Box::new(Token {
            kind: TokenKind::Identifier("name".to_string()),
            location: Location { start: 1, end: 5 },
            next: Some(Box::new(Token {
                kind: TokenKind::Colon,
                location: Location { start: 5, end: 6 },
                next: Some(Box::new(Token {
                    kind: TokenKind::Identifier("string".to_string()),
                    location: Location { start: 7, end: 13 },
                    next: Some(Box::new(Token {
                        kind: TokenKind::RightBrace,
                        location: Location { start: 13, end: 14 },
                        next: Some(Box::new(Token {
                            kind: TokenKind::EOF,
                            location: Location { start: 14, end: 14 },
                            next: None,
                        })),
                    })),
                })),
            })),
        })),
    })));
}

#[test]
fn test_nested_object() {
    let mut lexer = Lexer::new("{name: {first: string, last: string}}");
    let token = lexer.token();

    assert_eq!(token, Some(Box::new(Token {
        kind: TokenKind::LeftBrace,
        location: Location { start: 0, end: 1 },
        next: Some(Box::new(Token {
            kind: TokenKind::Identifier("name".to_string()),
            location: Location { start: 1, end: 5 },
            next: Some(Box::new(Token {
                kind: TokenKind::Colon,
                location: Location { start: 5, end: 6 },
                next: Some(Box::new(Token {
                    kind: TokenKind::LeftBrace,
                    location: Location { start: 7, end: 8 },
                    next: Some(Box::new(Token {
                        kind: TokenKind::Identifier("first".to_string()),
                        location: Location { start: 8, end: 13 },
                        next: Some(Box::new(Token {
                            kind: TokenKind::Colon,
                            location: Location { start: 13, end: 14 },
                            next: Some(Box::new(Token {
                                kind: TokenKind::Identifier("string".to_string()),
                                location: Location { start: 15, end: 21 },
                                next: Some(Box::new(Token {
                                    kind: TokenKind::Comma,
                                    location: Location { start: 21, end: 22 },
                                    next: Some(Box::new(Token {
                                        kind: TokenKind::Identifier("last".to_string()),
                                        location: Location { start: 23, end: 27 },
                                        next: Some(Box::new(Token {
                                            kind: TokenKind::Colon,
                                            location: Location { start: 27, end: 28 },
                                            next: Some(Box::new(Token {
                                                kind: TokenKind::Identifier("string".to_string()),
                                                location: Location { start: 29, end: 35 },
                                                next: Some(Box::new(Token {
                                                    kind: TokenKind::RightBrace,
                                                    location: Location { start: 35, end: 36 },
                                                    next: Some(Box::new(Token {
                                                        kind: TokenKind::RightBrace,
                                                        location: Location { start: 36, end: 37 },
                                                        next: Some(Box::new(Token {
                                                            kind: TokenKind::EOF,
                                                            location: Location { start: 37, end: 37 },
                                                            next: None,
                                                        })),
                                                    })),
                                                })),
                                            })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        })),
    })))
}
