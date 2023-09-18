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
    pub if_block: ConditionalBlock,
    pub else_if_blocks: Vec<ConditionalBlock>,
    pub else_block: Option<Block>,
}

#[derive(Debug, Clone)]
pub struct ConditionalBlock {
    pub condition: Box<Expression>,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub enum Signed {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    ISize(isize),
}

#[derive(Debug, Clone)]
pub enum Unsigned {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    USize(usize),
}

#[derive(Debug, Clone)]
pub enum Integer {
    Signed(Signed),
    Unsigned(Unsigned),
}

#[derive(Debug, Clone)]
pub enum Floating {
    Float(f32),
    Double(f64),
}

#[derive(Debug, Clone)]
pub enum Numeric {
    Int(Integer),
    Float(Floating),
}

#[derive(Debug, Clone)]
pub enum Expression {
    NumberLiteral(String),
    Number(Numeric),
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
