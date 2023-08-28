use crate::syntax::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literal(Literal),
    Plus,
    Minus,
}

pub type Tokens = Vec<Token>;
