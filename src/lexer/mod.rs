use crate::syntax::Token;

mod errors;
use errors::LexingError;

pub fn lex(input: &str) -> Result<Vec<Token>, LexingError> {
    todo!()
}
