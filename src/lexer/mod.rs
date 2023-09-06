use crate::Token;

pub fn lex(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    lexer.lex()
}

pub struct Lexer<'s> {
    source: &'s str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'s> Lexer<'_> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source,
            position: 0,
            line: 0,
            column: 0,
        }
    }

    fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = self.source.chars().peekable();

        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else if c == '#' {
                // comment starts
                while let Some(&c) = chars.peek() {
                    if c == '\n' {
                        break;
                    }
                    chars.next();
                }
            } else if c.is_alphabetic() || c == '_' {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphabetic() || c == '_' {
                        ident.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "let" => tokens.push(Token::Let),
                    "mut" => tokens.push(Token::Mut),
                    "number" => tokens.push(Token::NumberKeyword),
                    "unit" => tokens.push(Token::UnitKeyword),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            } else if c.is_numeric() {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::NumberLiteral(number));
            } else {
                match c {
                    '=' => {
                        chars.next();
                        tokens.push(Token::Equal);
                    }
                    ':' => {
                        chars.next();
                        tokens.push(Token::Colon);
                    }
                    ';' => {
                        chars.next();
                        tokens.push(Token::SemiColon);
                    }
                    ',' => {
                        chars.next();
                        tokens.push(Token::Comma);
                    }
                    '+' => {
                        chars.next();
                        tokens.push(Token::Plus);
                    }
                    '-' => {
                        chars.next();
                        if chars.peek() == Some(&'>') {
                            chars.next();
                            tokens.push(Token::Arrow);
                        } else {
                            tokens.push(Token::Minus);
                        }
                    }
                    '*' => {
                        chars.next();
                        tokens.push(Token::Asterisk);
                    }
                    '/' => {
                        chars.next();
                        tokens.push(Token::Slash);
                    }
                    '{' => {
                        chars.next();
                        tokens.push(Token::OpenBrace);
                    }
                    '}' => {
                        chars.next();
                        tokens.push(Token::CloseBrace);
                    }
                    '(' => {
                        chars.next();
                        tokens.push(Token::OpenParen);
                    }
                    ')' => {
                        chars.next();
                        tokens.push(Token::CloseParen);
                    }
                    _ => {
                        println!("Unknown token: {}", c);
                        chars.next();
                    }
                }
            }
        }
        tokens
    }
}
