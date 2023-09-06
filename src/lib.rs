pub mod lexer;
pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Mut,
    NumberKeyword,
    UnitKeyword,
    WhereKeyword, // 🤔
    Equal,
    Colon,
    SemiColon,
    Comma,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Arrow,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Identifier(String),
    NumberLiteral(String),
}

#[derive(Debug, Clone)]
pub enum Type {
    Number,
    Unit,
    FuncType(Vec<Param>, Box<Type>),
}

#[derive(Debug, Clone)]
pub enum Param {
    Parameter(String, Type),
}

#[derive(Debug, Clone)]
pub enum Expression {
    NumberLiteral(String),
    Identifier(String),
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Block(Vec<Statement>, Box<Expression>),
    Unit,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Declaration {
        mutable: bool,
        name: String,
        datatype: Type,
        value: Expression,
    },
    ReAssignment {
        name: String,
        value: Expression,
    },
    ExpressionStatement(Expression),
}

pub type Program = Vec<Statement>;
