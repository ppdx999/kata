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
