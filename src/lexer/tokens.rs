#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(Identifier),
    DataType(DataType),
    Special(Special),
    Literal(Literal),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int,
    Float,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Special {
    Colon,
    StatementStop,
    Assignment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);
