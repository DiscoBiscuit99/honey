/// Wrapper component.
/// In this way, when parsing, everything can just return node of this type.
#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxTree {
    Expression(Expression),
    Term(Term),
    Factor(Factor),
    Literal(Literal),
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
    Literal(Literal),
    ParentheticalExpression(Box<Expression>),
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
    Plus,
    Minus,
}

pub type Tokens = Vec<Token>;
