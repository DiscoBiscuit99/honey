use crate::structures::parse_tree::{
    Block, ConditionalBlock, Expression, Floating, If, Integer, Number, Param, Program, Signed,
    Statement, Type,
};

pub fn validate(program: &Program) -> Result<Program, String> {
    Validator {}.validate(program)
}

pub struct Validator {}

impl Validator {
    fn validate(&mut self, program: &Program) -> Result<Program, String> {
        Ok(self.visit_program(program))
    }
}

pub trait Visitor {
    fn visit_program(&mut self, program: &Program) -> Program;
    fn visit_statement(&mut self, statement: &Statement) -> Statement;
    fn visit_expression(&mut self, expression: &Expression) -> Expression;
    fn visit_number(&mut self, number: &Number);
    fn visit_conditional_block(&mut self, conditional_block: &ConditionalBlock);
    fn visit_if(&mut self, if_statement: &If);
    fn visit_block(&mut self, block: &Block);
    fn visit_param(&mut self, param: &Param);
    fn visit_type(&mut self, datatype: &Type);
}

impl Visitor for Validator {
    fn visit_program(&mut self, program: &Program) -> Program {
        let mut new_program = Program { statements: vec![] };
        for stmt in &program.statements {
            new_program.statements.push(self.visit_statement(stmt));
        }
        new_program
    }

    fn visit_statement(&mut self, statement: &Statement) -> Statement {
        match statement {
            Statement::Declaration {
                mutable,
                name,
                datatype,
                value,
            } => Statement::Declaration {
                mutable: *mutable,
                name: name.clone(),
                datatype: datatype.clone(),
                value: self.visit_expression(&value),
            },
            _ => unimplemented!(),
        }
    }

    fn visit_expression(&mut self, expression: &Expression) -> Expression {
        match expression {
            Expression::Unit => expression.clone(),
            Expression::NumberLiteral(literal) => {
                if let Ok(integer) = literal.parse::<i64>() {
                    Expression::Number(Number::Int(Integer::Signed(Signed::Int64(integer))))
                } else if let Ok(double) = literal.parse::<f64>() {
                    Expression::Number(Number::Float(Floating::Double(double)))
                } else {
                    panic!("Failed to parse number literal");
                }
            }
            Expression::Addition { augend, addend } => {
                let reduced_augend = self.visit_expression(augend);
                let reduced_addend = self.visit_expression(addend);

                // TODO: improve on this infinite nesting somehow...
                match (reduced_augend, reduced_addend) {
                    (
                        Expression::Number(Number::Int(Integer::Signed(Signed::Int64(augend)))),
                        Expression::Number(Number::Int(Integer::Signed(Signed::Int64(addend)))),
                    ) => Expression::Number(Number::Int(Integer::Signed(Signed::Int64(
                        augend + addend,
                    )))),
                    (
                        Expression::Number(Number::Float(Floating::Double(augend))),
                        Expression::Number(Number::Float(Floating::Double(addend))),
                    ) => Expression::Number(Number::Float(Floating::Double(augend + addend))),
                    _ => panic!("Trying to sum two different types!"),
                }
            }
            _ => unimplemented!(),
        }
    }

    fn visit_number(&mut self, number: &Number) {
        todo!();
    }

    fn visit_conditional_block(&mut self, conditional_block: &ConditionalBlock) {
        todo!()
    }

    fn visit_if(&mut self, if_statement: &If) {
        todo!()
    }

    fn visit_block(&mut self, block: &Block) {
        todo!();
    }

    fn visit_param(&mut self, param: &Param) {
        todo!();
    }

    fn visit_type(&mut self, datatype: &Type) {
        todo!();
    }
}
