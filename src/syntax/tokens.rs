#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    IntegerLiteral,
    Identifier,
    Keyword,
    Colon,
    Assignment,
    Plus,
    Minus,
    LParen,
    RParen,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Option<String>,
}

pub type Tokens = Vec<Token>;
