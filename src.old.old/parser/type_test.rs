use crate::lexer::tokens::{DataType, Identifier, Keyword};

#[derive(Debug, Clone, PartialEq)]
pub enum Program {
    StatementList { statements: Vec<Statement> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Declaration(Declaration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    keyword: Keyword,
    identifier: Identifier,
    data_type: DataType,
    expression: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Term(Box<Term>),
    Addition {
        left: Box<Expression>,
        right: Box<Term>,
    },
    Subtraction {
        left: Box<Expression>,
        right: Box<Term>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Factor(Factor),
    Multiplication { left: Box<Term>, right: Factor },
    Division { left: Box<Term>, right: Factor },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Factor {
    Literal(Literal),
    ParenthesizedExpression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(Number),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(i64),
    Float(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literal(Literal),
}
