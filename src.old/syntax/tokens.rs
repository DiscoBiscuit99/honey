use crate::syntax::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    DataType(DataType),
    Identifier(String),
    Keyword(Keyword),
    Literal(Literal),
    Plus,
    Minus,
    Assignment,
    Colon,
}

pub type Tokens = Vec<Token>;
