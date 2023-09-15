// LEXICAL ANALYSIS //

use std::{iter::Peekable, str::Chars};

use crate::structures::tokens::Token;

mod error;
use error::{err_msg, LexingError};

pub fn lex(input: &str) -> Vec<Token> {
    match Lexer::new(input).lex() {
        Ok(program) => program,
        Err(e) => {
            println!("{}", err_msg(e));
            std::process::exit(1);
        }
    }
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
            line: 1,
            column: 1,
        }
    }

    fn lex(&mut self) -> Result<Vec<Token>, LexingError> {
        while let Some(&c) = self.source.peek() {
            if c.is_whitespace() {
                // ignore whitespace
                self.consume_char();
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
                        self.consume_char();
                        //if self.source.peek() == Some(&)
                        if let Some('=') = self.source.peek() {
                            self.consume_char();
                            self.tokens.push(Token::Equal);
                        } else {
                            self.tokens.push(Token::Assignment);
                        }
                    }
                    '!' => {
                        self.consume_char();
                        if let Some('=') = self.source.peek() {
                            self.consume_char();
                            self.tokens.push(Token::NotEqual);
                        } else {
                            self.tokens.push(Token::Negate);
                        }
                    }
                    '<' => {
                        self.consume_char();
                        if self.source.peek() == Some(&'=') {
                            self.consume_char();
                            self.tokens.push(Token::LessEqual);
                        } else {
                            self.tokens.push(Token::OpenAngle);
                        }
                    }
                    '>' => {
                        self.consume_char();
                        if self.source.peek() == Some(&'=') {
                            self.consume_char();
                            self.tokens.push(Token::GreaterEqual);
                        } else {
                            self.tokens.push(Token::CloseAngle);
                        }
                    }
                    ':' => {
                        self.consume_char();
                        self.tokens.push(Token::Colon);
                    }
                    ';' => {
                        self.consume_char();
                        self.tokens.push(Token::SemiColon);
                    }
                    ',' => {
                        self.consume_char();
                        self.tokens.push(Token::Comma);
                    }
                    '+' => {
                        self.consume_char();
                        self.tokens.push(Token::Plus);
                    }
                    '-' => {
                        self.consume_char();
                        if self.source.peek() == Some(&'>') {
                            self.consume_char();
                            self.tokens.push(Token::Arrow);
                        } else {
                            self.tokens.push(Token::Minus);
                        }
                    }
                    '*' => {
                        self.consume_char();
                        self.tokens.push(Token::Asterisk);
                    }
                    '/' => {
                        self.consume_char();
                        self.tokens.push(Token::Slash);
                    }
                    '{' => {
                        self.consume_char();
                        self.tokens.push(Token::OpenBrace);
                    }
                    '}' => {
                        self.consume_char();
                        self.tokens.push(Token::CloseBrace);
                    }
                    '(' => {
                        self.consume_char();
                        self.tokens.push(Token::OpenParen);
                    }
                    ')' => {
                        self.consume_char();
                        self.tokens.push(Token::CloseParen);
                    }
                    _ => {
                        return Err(LexingError::UnknownCharacter {
                            line: self.line,
                            column: self.column,
                        });
                    }
                }
            }
        }
        Ok(self.tokens.clone())
    }

    fn consume_char(&mut self) -> Option<char> {
        if let Some(&c) = self.source.peek() {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }

        self.position += 1;
        self.source.next()
    }

    fn consume_comment(&mut self) {
        while let Some(&c) = self.source.peek() {
            if c == '\n' {
                break;
            }
            self.consume_char();
        }
    }

    fn consume_identifier(&mut self) {
        let mut ident = String::new();
        while let Some(&c) = self.source.peek() {
            if c.is_alphabetic() || c == '_' {
                ident.push(self.consume_char().unwrap());
            } else {
                break;
            }
        }
        // check if it's a keyword
        match ident.as_str() {
            "let" => self.tokens.push(Token::Let),
            "mut" => self.tokens.push(Token::Mut),
            "if" => self.tokens.push(Token::If),
            "else" => self.tokens.push(Token::Else),
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
                number.push(self.consume_char().unwrap());
            } else {
                break;
            }
        }
        self.tokens.push(Token::NumberLiteral(number));
    }
}
