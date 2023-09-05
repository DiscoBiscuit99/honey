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
    LeftParenthesis,
    RightParenthesis,
    LeftCurly,
    RightCurly,
    Eof,
    Times,
    Divide,
    Comma,
    RightArrow,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Option<String>,
}
