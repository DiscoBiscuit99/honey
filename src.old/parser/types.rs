use crate::lexer::tokens::{DataType, Identifier, Keyword, Literal, Special};

#[derive(Debug, PartialEq)]
pub enum SyntaxTree {
    Literal(Literal),
    Statement(Statement),
    StatementList { statements: Vec<Statement> },
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Declaration {
        keyword: Keyword,
        identifier: Identifier,
        type_annotation: TypeAnnotation,
        expression: Expression,
    },
}

#[derive(Debug, PartialEq)]
pub struct TypeAnnotation {
    pub prefix: Special,
    pub data_type: DataType,
}
