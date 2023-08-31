use crate::syntax::SyntaxTree;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Declaration(Box<SyntaxTree>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Term(Box<SyntaxTree>),
    Sum {
        augend: Box<SyntaxTree>,
        addend: Box<SyntaxTree>,
    },
    Difference {
        minuend: Box<SyntaxTree>,
        subtrahend: Box<SyntaxTree>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SparseTerm {
    Factor(Box<SyntaxTree>),
    Product {
        multiplicant: Box<SyntaxTree>,
        multiplier: Box<SyntaxTree>,
    },
    Quotient {
        dividend: Box<SyntaxTree>,
        divisor: Box<SyntaxTree>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Factor {
    Number(Box<SyntaxTree>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    NumberLiteral(Box<SyntaxTree>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberLiteral {
    Number(Box<SyntaxTree>),
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
