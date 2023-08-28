use crate::syntax::{tokens::*, *};

#[derive(Debug)]
pub enum ParsingError {
    ExpectedNumber { found: Option<Token> },
    ExpectedLiteral { found: Option<SyntaxTree> },
    ExpectedFactor { found: Option<SyntaxTree> },
    ExpectedTerm { found: Option<SyntaxTree> },
    ExpectedToken(ExpectedToken),
    Unknown,
}

#[derive(Debug)]
pub enum ExpectedToken {
    Literal { found: Option<Token> },
    Number { found: Option<Token> },
}
