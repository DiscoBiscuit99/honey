use crate::syntax::{Token, TokenKind};

#[derive(Debug)]
pub enum LexingError {
    ExpectedSpecial {
        found: Option<char>,
    },
    UnexpectedToken {
        expected: Option<TokenKind>,
        found: Option<Token>,
    },
}
