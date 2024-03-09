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

    pub fn read_next_token(&mut self) -> Token {
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
            Some(' ') => self.read_next_token_kind(),
            Some(char) => panic!("Unexpected character: {}", char),
            None => TokenKind::EOF,
        }
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }
}

#[test]
fn test_empty_object() {
    let mut lexer = Lexer::new("{}");

    assert_eq!(lexer.read_next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.read_next_token().kind, TokenKind::RightBrace);
    assert_eq!(lexer.read_next_token().kind, TokenKind::EOF);
}


#[test]
fn test_simple_object() {
    let mut lexer = Lexer::new("{name: string}");

    assert_eq!(lexer.read_next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.read_next_token().kind, TokenKind::Identifier("name".to_string()));
    assert_eq!(lexer.read_next_token().kind, TokenKind::Colon);
    assert_eq!(lexer.read_next_token().kind, TokenKind::Identifier("string".to_string()));
    assert_eq!(lexer.read_next_token().kind, TokenKind::RightBrace);
    assert_eq!(lexer.read_next_token().kind, TokenKind::EOF);
}

#[test]
fn test_nested_object() {
    let mut lexer = Lexer::new("{name: {first: string, last: string}}");

    assert_eq!(lexer.read_next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.read_next_token().kind, TokenKind::Identifier("name".to_string()));
    assert_eq!(lexer.read_next_token().kind, TokenKind::Colon);
    assert_eq!(lexer.read_next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.read_next_token().kind, TokenKind::Identifier("first".to_string()));
    assert_eq!(lexer.read_next_token().kind, TokenKind::Colon);
    assert_eq!(lexer.read_next_token().kind, TokenKind::Identifier("string".to_string()));
    assert_eq!(lexer.read_next_token().kind, TokenKind::Comma);
    assert_eq!(lexer.read_next_token().kind, TokenKind::Identifier("last".to_string()));
    assert_eq!(lexer.read_next_token().kind, TokenKind::Colon);
    assert_eq!(lexer.read_next_token().kind, TokenKind::Identifier("string".to_string()));
    assert_eq!(lexer.read_next_token().kind, TokenKind::RightBrace);
    assert_eq!(lexer.read_next_token().kind, TokenKind::RightBrace);
    assert_eq!(lexer.read_next_token().kind, TokenKind::EOF);
}
