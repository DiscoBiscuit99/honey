use crate::syntax::Tokens;

mod errors;
use errors::LexingError;

pub fn lex(_src: &str) -> Result<Tokens, LexingError> {
    let tokens = vec![];
    Ok(tokens)
}

fn parse_decl(src: &str) -> Result<Tokens, LexingError> {
    todo!()
}
