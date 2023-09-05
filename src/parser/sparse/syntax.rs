use crate::syntax::ParseTree;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Declaration(Box<ParseTree>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Term(Box<ParseTree>),
    Sum {
        augend: Box<ParseTree>,
        addend: Box<ParseTree>,
    },
    Difference {
        minuend: Box<ParseTree>,
        subtrahend: Box<ParseTree>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SparseTerm {
    Factor(Box<ParseTree>),
    Product {
        multiplicant: Box<ParseTree>,
        multiplier: Box<ParseTree>,
    },
    Quotient {
        dividend: Box<ParseTree>,
        divisor: Box<ParseTree>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Factor {
    Number(Box<ParseTree>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    NumberLiteral(Box<ParseTree>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberLiteral {
    Number(Box<ParseTree>),
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
