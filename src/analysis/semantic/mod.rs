use std::{collections::HashMap, ops::Deref};

use crate::structures::parse_tree::{
    Block, ConditionalBlock, Expression, Floating, If, Integer, Numeric, Program, Signed,
    Statement, Type,
};

/// Takes program and validates it.
/// Includes type checking and reduction of expressions.
pub fn validate(program: &Program) -> Validator {
    let mut validator = Validator::new();
    validator.visit_program(program);
    validator
}

pub type Name = String;

#[derive(Debug, Clone)]
pub struct Variable {
    mutable: bool,
    datatype: Type,
    value: Expression,
}

pub type StackFrame = HashMap<Name, Vec<Variable>>;

#[derive(Debug)]
pub struct Validator {
    scope: usize,
    symbol_table: Vec<StackFrame>,
}

impl Validator {
    fn new() -> Validator {
        Validator {
            scope: 0,
            symbol_table: vec![],
        }
    }

    fn accept_statement(&mut self, stmt: &Statement) {
        self.visit_statement(stmt);
    }

    fn accept_expression(&mut self, expr: &Expression) -> Expression {
        self.visit_expression(expr)
    }

    fn accept_block(&mut self, block: &Block) {
        self.visit_block(block);
    }

    fn accept_if(&mut self, if_stmt: &If) {
        self.visit_if(if_stmt);
    }

    pub fn accept_conditional_block(&mut self, conditional_block: &ConditionalBlock) {
        self.visit_conditional_block(conditional_block);
    }
}

pub trait Visitor {
    fn visit_program(&mut self, program: &Program) {
        for stmt in program {
            self.visit_statement(stmt);
        }
    }

    fn visit_statement(&mut self, stmt: &Statement);
    fn visit_expression(&mut self, expr: &Expression) -> Expression;
    fn visit_block(&mut self, block: &Block);
    fn visit_if(&mut self, if_stmt: &If);
    fn visit_conditional_block(&mut self, conditional_block: &ConditionalBlock);
}

impl Visitor for Validator {
    fn visit_program(&mut self, program: &Program) {
        for stmt in program {
            self.accept_statement(stmt);
        }
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        // Perform some initial logic specific to statements, if needed.
        // For example, update the symbol table, check for errors, etc.

        match stmt.to_owned() {
            Statement::Declaration {
                mutable,
                name,
                datatype,
                value,
            } => {
                // Handle variable declarations
                // Maybe update the symbol table with the new variable here

                let reduced_expr = self.accept_expression(&value);

                let variable = Variable {
                    mutable,
                    datatype,
                    value: reduced_expr,
                };

                if let Some(frame) = self.symbol_table.get_mut(self.scope) {
                    if let Some(associations) = frame.get_mut(&name) {
                        associations.push(variable.clone());
                    } else {
                        frame.insert(name, vec![variable]);
                    }
                } else {
                    let mut frame = HashMap::new();
                    frame.insert(name, vec![variable]);
                    self.symbol_table.push(frame);
                };
            }
            Statement::ReAssignment { name, value } => {
                // Handle variable reassignments
                // Maybe check that the variable exists and is mutable

                // Then visit the value expression
                self.accept_expression(&value);
            }
            Statement::ExpressionStatement(expr) => {
                // Just visit the expression
                self.accept_expression(&expr);
            }
            Statement::IfStatement(if_stmt) => {
                // Visit the IfStatement
                self.accept_if(&if_stmt);
            }
        }

        // Perform some final logic specific to statements, if needed.
    }

    fn visit_expression(&mut self, expr: &Expression) -> Expression {
        match expr {
            Expression::NumberLiteral(number) => {
                let parsed = number
                    .parse::<i64>()
                    .expect("failed to parse number literal in expression visitor");
                Expression::Number(Numeric::Int(Integer::Signed(Signed::Int64(parsed))))
            } // as reduced as can be
            Expression::Addition { augend, addend } => {
                let augend = self.visit_expression(&augend);
                let addend = self.visit_expression(&addend);

                let extracted_augend = if let Expression::Number(Numeric::Int(Integer::Signed(
                    Signed::Int64(number),
                ))) = augend
                {
                    number
                } else {
                    panic!("failed to extract augend in expressin visitor");
                };

                let extracted_addend = if let Expression::Number(Numeric::Int(Integer::Signed(
                    Signed::Int64(number),
                ))) = addend
                {
                    number
                } else {
                    panic!("failed to extract addend in expressin visitor");
                };

                Expression::Number(Numeric::Int(Integer::Signed(Signed::Int64(
                    extracted_augend + extracted_addend,
                ))))
            }
            _ => unimplemented!(),
        }
    }

    fn visit_block(&mut self, block: &Block) {
        // ... your visitation logic here
    }

    fn visit_if(&mut self, if_stmt: &If) {
        // ... your visitation logic here
    }

    fn visit_conditional_block(&mut self, conditional_block: &ConditionalBlock) {
        // ... your visitation logic here
    }
}
