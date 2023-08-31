#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Let,
    DataType,
    Integer,
    Float,
    Number,
    Comment,
    Semicolon,
    Identifier,
    Keyword,
    Colon,
    Assignment,
    Plus,
    Minus,
    LParen,
    RParen,
    Eof,
    Times,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Option<String>,
}

pub type Tokens = Vec<Token>;
