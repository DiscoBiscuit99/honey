use crate::syntax::tokens::*;

#[derive(Debug)]
pub enum LexingError {
    ExpectedChar,
    ExpectedOperator,
    ExpectedSpecial,
    ExpectedNumber,
    ExpectedLiteral,
    ExpectedDecimal { found: Option<Token> },
}
