mod tokens;
pub use tokens::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxTree {
    StatementList { statements: Vec<Statement> },
    Statement(Statement),
    Expression(Expression),
    Declaration(Declaration),
    Number(Number),
    Identifier(Identifier),
    DataType(DataType),
    Assignment(Assignment),
    Keyword(Keyword),
    Term(Term),
    Factor(Factor),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Declaration(Declaration),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Term(Term),
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
    pub data_type: DataType,
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
pub enum DataType {
    Number,
    Int,
    Float,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub identifier: Identifier,
    pub data_type: Option<DataType>,
    pub expression: Expression,
}
