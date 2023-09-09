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
pub struct Block {
    pub statements: Vec<Statement>,
    pub return_value: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expression>,
    pub then_block: Block,
    pub else_block: Option<Block>,
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
    LessThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    GreaterThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LessThanOrEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    GreaterThanOrEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Equal {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    NotEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    Block(Block),
    If(If),
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
    IfStatement(If),
}

pub type Program = Vec<Statement>;
pub type Arguments = Vec<Expression>;
