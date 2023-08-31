use crate::syntax::{SyntaxTree, Token};

#[derive(Debug)]
pub enum ParsingError {
    EmptyInput,
    ExpectedLexeme,
    ExpectedNumberToken { found: Option<Token> },
    ExpectedIdentifierToken { found: Option<Token> },
    ExpectedColonToken { found: Option<Token> },
    ExpectedDataTypeToken { found: Option<Token> },
    ExpectedAssignToken { found: Option<Token> },
    ExpectedKeywordToken { found: Option<Token> },
    ExpectedExpression { found: Option<SyntaxTree> },
    ExpectedIdentifier { found: Option<SyntaxTree> },
    ExpectedDataType { found: Option<SyntaxTree> },
    ExpectedDeclKeyword { found: Option<SyntaxTree> },
    ExpectedAssignment { found: Option<SyntaxTree> },
    ExpectedTerm { found: Option<SyntaxTree> },
    ExpectedFactor { found: Option<SyntaxTree> },
    ExpectedLiteral { found: Option<SyntaxTree> },
    ExpectedNumber { found: Option<SyntaxTree> },
    ExpectedDeclaration { found: Option<SyntaxTree> },
    ExpectedStatement { found: Option<SyntaxTree> },
    FailedToParseInteger,
}
