pub mod tokens;

/// Wrapper component.
/// In this way, when parsing, everything can just return node of this type.
#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxTree {
    Program { statement_list: Vec<Statement> },
    Statement(Statement),
    Declaration(Declaration),
    Keyword(Keyword),
    Identifier(String),
    Expression(Expression),
    Term(Term),
    Factor(Factor),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Declaration(Declaration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    keyword: DeclarationKeyword,
    identifier: String,
    data_type: DataType,
    value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    DeclarationKeyword(DeclarationKeyword),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationKeyword {
    Let,
    Mut,
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
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Number,
}
