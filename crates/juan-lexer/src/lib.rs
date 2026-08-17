pub mod tokens;

use std::iter::Peekable;
use std::str::Chars;

use crate::tokens::Token;
use crate::tokens::TokenKind;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.chars.next() {
            Some('.') => Token(TokenKind::Dot),
            Some('(') => Token(TokenKind::LParen),
            Some(')') => Token(TokenKind::RParen),
            Some('"') => self.read_string(),
            Some(c) if c.is_alphabetic() || c == '_' => self.read_ident(c),
            None => Token(TokenKind::Eof),
            Some(ch) => panic!("Unexpected character: {ch}")
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.chars.peek() {
            if ch.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    pub fn read_string(&mut self) -> Token {
        let mut string = String::new();
        while let Some(&ch) = self.chars.peek() {
            if ch != '"' {
                string.push(self.chars.next().unwrap());
            } else {
                self.chars.next();
                break;
            }
        }
        
        Token(TokenKind::Str(string))
    }

    pub fn read_ident(&mut self, first: char) -> Token {
        let mut ident = String::from(first);
        while let Some(&ch) = self.chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        match ident.as_str() {
            "module" => Token(TokenKind::Module),
            "fn" => Token(TokenKind::Fn),
            "import" => Token(TokenKind::Import),
            "end" => Token(TokenKind::End),

            _ => Token(TokenKind::Identifier(ident)),
        }
    }
}
