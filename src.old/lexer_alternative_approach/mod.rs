use crate::syntax::{Token, TokenKind, Tokens};

mod errors;
use errors::LexingError;

pub fn lex(input: &str) -> Tokens {
    let mut lexer = Lexer::new(input);
    let mut tokens = vec![];

    let mut next_token = lexer.next_token();
    while next_token.kind != TokenKind::Eof {
        tokens.push(next_token);
        next_token = lexer.next_token();
    }

    tokens
}

struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            // Skip whitespace
            if c.is_whitespace() {
                self.position += 1;
                continue;
            }

            // Match single character tokens
            match c {
                '+' => {
                    self.position += 1;
                    return Token {
                        kind: TokenKind::Plus,
                        lexeme: Some("+".to_string()),
                    };
                }
                '-' => {
                    self.position += 1;
                    return Token {
                        kind: TokenKind::Minus,
                        lexeme: Some("-".to_string()),
                    };
                }
                '(' => {
                    self.position += 1;
                    return Token {
                        kind: TokenKind::LParen,
                        lexeme: Some("(".to_string()),
                    };
                }
                ')' => {
                    self.position += 1;
                    return Token {
                        kind: TokenKind::RParen,
                        lexeme: Some(")".to_string()),
                    };
                }
                _ => {}
            }

            // Match identifiers and integer literals
            let start = self.position;
            if c.is_digit(10) {
                while self.position < self.input.len()
                    && self.input.chars().nth(self.position).unwrap().is_digit(10)
                {
                    self.position += 1;
                }
            } else if c.is_alphabetic() || c == '_' {
                while self.position < self.input.len()
                    && (self
                        .input
                        .chars()
                        .nth(self.position)
                        .unwrap()
                        .is_alphabetic()
                        || self.input.chars().nth(self.position).unwrap() == '_')
                {
                    self.position += 1;
                }
            }
            let lexeme: String = self.input[start..self.position].to_string();

            if !lexeme.is_empty() {
                return Token {
                    kind: if lexeme.chars().all(|c| c.is_digit(10)) {
                        TokenKind::IntegerLiteral
                    } else {
                        TokenKind::Identifier
                    },
                    lexeme: Some(lexeme),
                };
            }
        }

        // Return EOF when we reach the end of the input
        Token {
            kind: TokenKind::Eof,
            lexeme: None,
        }
    }
}
