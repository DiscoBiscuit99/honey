use std::{iter::Peekable, str::Chars};

use crate::Token;

pub fn lex(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    lexer.lex()
}

pub struct Lexer<'s> {
    source: Peekable<Chars<'s>>,
    tokens: Vec<Token>,
    position: usize,
    line: usize,
    column: usize,
}

impl<'s> Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            source: input.chars().peekable(),
            tokens: vec![],
            position: 0,
            line: 0,
            column: 0,
        }
    }

    fn lex(&mut self) -> Vec<Token> {
        while let Some(&c) = self.source.peek() {
            if c.is_whitespace() {
                // ignore whitespace
                self.source.next();
            } else if c == '#' {
                // scan a comment
                self.consume_comment();
            } else if c.is_alphabetic() || c == '_' {
                // scan identifier
                self.consume_identifier();
            } else if c.is_numeric() {
                // scan number
                self.consume_number();
            } else {
                match c {
                    '=' => {
                        self.source.next();
                        self.tokens.push(Token::Equal);
                    }
                    ':' => {
                        self.source.next();
                        self.tokens.push(Token::Colon);
                    }
                    ';' => {
                        self.source.next();
                        self.tokens.push(Token::SemiColon);
                    }
                    ',' => {
                        self.source.next();
                        self.tokens.push(Token::Comma);
                    }
                    '+' => {
                        self.source.next();
                        self.tokens.push(Token::Plus);
                    }
                    '-' => {
                        self.source.next();
                        if self.source.peek() == Some(&'>') {
                            self.source.next();
                            self.tokens.push(Token::Arrow);
                        } else {
                            self.tokens.push(Token::Minus);
                        }
                    }
                    '*' => {
                        self.source.next();
                        self.tokens.push(Token::Asterisk);
                    }
                    '/' => {
                        self.source.next();
                        self.tokens.push(Token::Slash);
                    }
                    '{' => {
                        self.source.next();
                        self.tokens.push(Token::OpenBrace);
                    }
                    '}' => {
                        self.source.next();
                        self.tokens.push(Token::CloseBrace);
                    }
                    '(' => {
                        self.source.next();
                        self.tokens.push(Token::OpenParen);
                    }
                    ')' => {
                        self.source.next();
                        self.tokens.push(Token::CloseParen);
                    }
                    _ => {
                        println!("Unknown token: {}", c);
                        self.source.next();
                    }
                }
            }
        }
        self.tokens.clone()
    }

    fn consume_comment(&mut self) {
        while let Some(&c) = self.source.peek() {
            if c == '\n' {
                break;
            }
            self.source.next();
        }
    }

    fn consume_identifier(&mut self) {
        let mut ident = String::new();
        while let Some(&c) = self.source.peek() {
            if c.is_alphabetic() || c == '_' {
                ident.push(self.source.next().unwrap());
            } else {
                break;
            }
        }
        match ident.as_str() {
            "let" => self.tokens.push(Token::Let),
            "mut" => self.tokens.push(Token::Mut),
            "number" => self.tokens.push(Token::NumberKeyword),
            "unit" => self.tokens.push(Token::UnitKeyword),
            _ => self.tokens.push(Token::Identifier(ident)),
        }
    }

    fn consume_number(&mut self) {
        // scan number
        let mut number = String::new();
        while let Some(&c) = self.source.peek() {
            if c.is_numeric() {
                number.push(self.source.next().unwrap());
            } else {
                break;
            }
        }
        self.tokens.push(Token::NumberLiteral(number));
    }
}
