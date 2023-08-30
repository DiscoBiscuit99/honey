use crate::syntax::{tokens::*, *};

#[derive(Debug)]
pub enum ParsingError {
    ExpectedNumber { found: Option<Token> },
    ExpectedLiteral { found: Option<SyntaxTree> },
    ExpectedFactor { found: Option<SyntaxTree> },
    ExpectedTerm { found: Option<SyntaxTree> },
    ExpectedKeyword { found: Option<SyntaxTree> },
    ExpectedDeclKeyword { found: Option<Keyword> },
    ExpectedIdentifier { found: Option<SyntaxTree> },
    ExpectedToken(ExpectedToken),
    Unknown,
}

#[derive(Debug)]
pub enum ExpectedToken {
    Colon { found: Option<Token> },
    Assignment { found: Option<Token> },
    Identifier { found: Option<Token> },
    Keyword { found: Option<Token> },
    Literal { found: Option<Token> },
    Number { found: Option<Token> },
}
