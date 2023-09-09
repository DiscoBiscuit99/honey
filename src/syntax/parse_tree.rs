#[derive(Debug, Clone)]
pub enum Type {
    Number,
    Unit,
    FuncType {
        parameters: Vec<Param>,
        return_type: Box<Type>,
    },
}

#[derive(Debug, Clone)]
pub enum Param {
    Parameter { name: String, datatype: Type },
}

#[derive(Debug, Clone)]
pub enum Expression {
    NumberLiteral(String),
    Identifier(String),
    Addition {
        augend: Box<Expression>,
        addend: Box<Expression>,
    },
    Subtraction {
        minuend: Box<Expression>,
        subtrahend: Box<Expression>,
    },
    Multiplication {
        multiplicant: Box<Expression>,
        multiplier: Box<Expression>,
    },
    Division {
        dividend: Box<Expression>,
        divisor: Box<Expression>,
    },
    Block {
        statements: Vec<Statement>,
        return_value: Box<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
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
pub type Arguments = Vec<Expression>;
