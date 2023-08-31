use crate::syntax::{Token, TokenKind, Tokens};

mod errors;
use errors::LexingError;

pub fn lex(input: &str) -> Result<Tokens, LexingError> {
    todo!()
}
