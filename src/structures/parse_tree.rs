use crate::analysis::semantic::parser::Visitor;

pub trait AstNode {
    fn accept(&self, visitor: &mut dyn Visitor);
}

#[derive(Debug, Clone)]
pub enum Type {
    Number,
    Unit,
    FuncType {
        parameters: Vec<Param>,
        return_type: Box<Type>,
    },
}

impl AstNode for Type {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_type(self);
    }
}

#[derive(Debug, Clone)]
pub enum Param {
    Parameter { name: String, datatype: Type },
}

impl AstNode for Param {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_param(self);
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub return_value: Box<Expression>,
}

impl AstNode for Block {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_block(self);
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub if_block: ConditionalBlock,
    pub else_if_blocks: Vec<ConditionalBlock>,
    pub else_block: Option<Block>,
}

impl AstNode for If {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_if(self);
    }
}

#[derive(Debug, Clone)]
pub struct ConditionalBlock {
    pub condition: Box<Expression>,
    pub block: Block,
}

impl AstNode for ConditionalBlock {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_conditional_block(self);
    }
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
pub enum Number {
    Int(Integer),
    Float(Floating),
}

impl AstNode for Number {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_number(self);
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    NumberLiteral(String),
    Number(Number),
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

impl AstNode for Expression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_expression(self);
    }
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

impl AstNode for Statement {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_statement(self);
    }
}

pub type Arguments = Vec<Expression>;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl AstNode for Program {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_program(self);
    }
}
