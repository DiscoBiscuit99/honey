#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Colon(String),
    Type(String),
    Assignment(String),
    Number(String),
    Plus(String),
    Minus(String),
}

pub type Tokens = Vec<Token>;
