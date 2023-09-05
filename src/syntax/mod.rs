mod tokens;
pub use tokens::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseTree {
    Parameter(Parameter),
    ParameterList { parameters: Vec<Parameter> },
    Block(Block),
    StatementList { statements: Vec<Statement> },
    Statement(Statement),
    Expression(Expression),
    Declaration(Declaration),
    Number(Number),
    Identifier(Identifier),
    Type(Type),
    Assignment(Assignment),
    Keyword(Keyword),
    Term(Term),
    Factor(Factor),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub return_expression: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Declaration(Declaration),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Term(Term),
    Block(Box<Block>),
    Sum {
        augend: Box<Expression>,
        addend: Term,
    },
    Difference {
        minuend: Box<Expression>,
        subtrahend: Term,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Factor(Factor),
    NegatedFactor(Factor),
    Product {
        multiplicant: Box<Term>,
        multiplier: Factor,
    },
    Quotient {
        dividend: Box<Term>,
        divisor: Factor,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Factor {
    Number(Number),
    ParentheizedExpression(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    NumberLiteral(NumberLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberLiteral {
    Number(Number),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub keyword: DeclKeyword,
    pub identifier: Identifier,
    pub data_type: Type,
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclKeyword {
    Let,
    Mut,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    DeclKeyword(DeclKeyword),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    BasicType(BasicType),
    FuncType(FuncType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncType {
    pub param_list: Vec<Parameter>,
    pub return_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub identifier: Identifier,
    pub data_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BasicType {
    Number,
    Int,
    Float,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub identifier: Identifier,
    pub data_type: Option<Type>,
    pub expression: Expression,
}
