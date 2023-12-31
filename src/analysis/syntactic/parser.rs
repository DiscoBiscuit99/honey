// SYNTACTIC ANALYSIS //

use colored::Colorize;

use crate::structures::{
    parse_tree::{
        Arguments, Block, ConditionalBlock, Expression, If, Param, Program, Statement, Type,
    },
    tokens::Token,
};

pub fn parse(tokens: &[Token]) -> Program {
    match Parser::new(tokens.to_owned()).parse_program() {
        Ok(program) => program,
        Err(e) => {
            println!("Failed to parse the program: {e}");
            std::process::exit(1);
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn peek_two(&self) -> Option<(&Token, &Token)> {
        let first = self.tokens.get(self.position);
        let second = self.tokens.get(self.position + 1);

        if first.is_some() && second.is_some() {
            Some((first.unwrap(), second.unwrap()))
        } else {
            None
        }
    }

    fn consume(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        if token.is_some() {
            self.position += 1;
        }
        token
    }

    fn expect_err_msg(&self, expected: &Token, actual: Option<&Token>) -> String {
        let next = if let Some(token) = actual {
            format!("{}", token)
        } else {
            format!("none")
        };
        let expected_part = format!("expected {}", expected.to_string().bold());
        let found_part = format!("found {}", next.to_string().bold());
        format!("{}, {}", expected_part, found_part)
            .red()
            .to_string()
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.peek() == Some(&expected) {
            self.consume();
            Ok(())
        } else {
            Err(self.expect_err_msg(&expected, self.peek()))
        }
    }

    fn rewind(&mut self, position: usize) {
        self.position = position;
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match self.consume() {
            Some(Token::NumberKeyword) => Ok(Type::Number),
            Some(Token::IntKeyword) => Ok(Type::Int),
            Some(Token::Nil) => Ok(Type::Nil),
            // closure
            // Some(Token::OpenParen) => {
            //     let mut param_types = Vec::new();
            //     while self.peek() != Some(&Token::CloseParen) {
            //         let param = self.parse_param()?;
            //         param_types.push(param);
            //         if self.peek() == Some(&Token::Comma) {
            //             self.consume();
            //         }
            //     }
            //     self.expect(Token::CloseParen)?;
            //     self.expect(Token::Arrow)?;
            //     let return_type = self.parse_type()?;
            //     Ok(Type::FuncType {
            //         parameters: param_types,
            //         return_type: Box::new(return_type),
            //     })
            // }
            _ => Err("expected a type".to_string()),
        }
    }

    fn parse_single_parameter(&mut self) -> Result<Param, String> {
        if let Some(Token::Identifier(name)) = self.consume() {
            self.expect(Token::Colon)?;
            let datatype = self.parse_type()?;
            Ok(Param::Parameter { name, datatype })
        } else {
            Err("expected an identifier".to_string())
        }
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Param>, String> {
        match self.consume() {
            Some(Token::OpenParen) => {
                let mut param_types = Vec::new();
                while self.peek() != Some(&Token::CloseParen) {
                    let param = self.parse_single_parameter()?;
                    param_types.push(param);
                    if self.peek() == Some(&Token::Comma) {
                        self.consume();
                    }
                }
                self.expect(Token::CloseParen)?;
                Ok(param_types)
            }
            _ => Err("expected an parameter list".to_string()),
        }
    }

    fn parse_arguments(&mut self) -> Result<Arguments, String> {
        self.expect(Token::OpenParen)?;

        let mut arguments = vec![];

        let argument = self.parse_expression()?;
        arguments.push(argument);

        while let Some(Token::Comma) = self.peek() {
            self.expect(Token::Comma)?;
            let argument = self.parse_expression()?;
            arguments.push(argument);
        }

        self.expect(Token::CloseParen)?;

        Ok(arguments)
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let next = self.consume();
        if let Some(Token::OpenParen) = next {
            let expr = self.parse_expression()?;
            self.expect(Token::CloseParen)?;
            Ok(expr)
        } else {
            match next {
                Some(Token::NumberLiteral(n)) => Ok(Expression::NumberLiteral(n)),
                Some(Token::Identifier(id)) => {
                    if let Some(Token::OpenParen) = self.peek() {
                        let arguments = self.parse_arguments()?;
                        Ok(Expression::FunctionCall {
                            name: id,
                            arguments,
                        })
                    } else {
                        Ok(Expression::Identifier(id))
                    }
                }
                _ => Err("expected a factor".to_string()),
            }
        }
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.peek().cloned() {
            match token {
                Token::Asterisk => {
                    self.consume();
                    let right = self.parse_factor()?;
                    left = Expression::Multiplication {
                        multiplicant: Box::new(left),
                        multiplier: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.consume();
                    let right = self.parse_factor()?;
                    left = Expression::Division {
                        dividend: Box::new(left),
                        divisor: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_comparison(&mut self, mut left: Expression) -> Result<Expression, String> {
        while let Some(token) = self.peek().cloned() {
            match token {
                Token::OpenAngle => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = Expression::LessThan {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::CloseAngle => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = Expression::GreaterThan {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::LessEqual => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = Expression::LessThanOrEqual {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::GreaterEqual => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = Expression::GreaterThanOrEqual {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::Equal => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = Expression::Equal {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::NotEqual => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = Expression::NotEqual {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let peeked = self.peek();
        let expr = if let Some(Token::OpenBrace) = peeked {
            let block = self.parse_block()?;
            Expression::Block {
                statements: block.statements,
                return_value: block.return_value,
            }
        } else if let Some(Token::If) = peeked {
            Expression::If(self.parse_if()?)
        } else {
            let mut left = self.parse_term()?;

            while let Some(token) = self.peek().cloned() {
                match token {
                    Token::Plus => {
                        self.consume(); // Consume the '+' token
                        let right = self.parse_term()?;
                        left = Expression::Addition {
                            augend: Box::new(left),
                            addend: Box::new(right),
                        };
                    }
                    Token::Minus => {
                        self.consume(); // Consume the '-' token
                        let right = self.parse_term()?;
                        left = Expression::Subtraction {
                            minuend: Box::new(left),
                            subtrahend: Box::new(right),
                        };
                    }
                    _ => break,
                }
            }

            if let Some(
                Token::NotEqual
                | Token::Equal
                | Token::GreaterEqual
                | Token::LessEqual
                | Token::OpenAngle
                | Token::CloseAngle,
            ) = self.peek()
            {
                left = self.parse_comparison(left)?;
            }

            left
        };

        Ok(expr)
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        self.expect(Token::OpenBrace)?; // Expect '{'

        let mut return_value = Expression::Nil;
        let mut statements = Vec::new();
        while let Some(token) = self.peek().cloned() {
            match token {
                Token::CloseBrace => break,
                _ => {
                    let save_point = self.position;
                    if let Some(stmt) = self.parse_statement().ok() {
                        statements.push(stmt);
                    } else {
                        self.rewind(save_point);
                        if let Some(expr) = self.parse_expression().ok() {
                            return_value = expr;
                            break;
                        }
                    };
                }
            }
        }

        self.expect(Token::CloseBrace)?; // Expect '}'

        Ok(Block {
            statements,
            return_value: Box::new(return_value),
        })
    }

    fn parse_if(&mut self) -> Result<If, String> {
        self.expect(Token::If)?;

        let if_block = ConditionalBlock {
            condition: Box::new(self.parse_expression()?),
            block: self.parse_block()?,
        };

        let mut else_if_blocks = vec![];
        while let Some((Token::Else, Token::If)) = self.peek_two() {
            self.consume(); // consume 'else'
            self.consume(); // consume 'if'

            else_if_blocks.push(ConditionalBlock {
                condition: Box::new(self.parse_expression()?),
                block: self.parse_block()?,
            });
        }

        let else_block = if let Some(Token::Else) = self.peek() {
            self.consume();
            self.parse_block().ok()
        } else {
            None
        };

        Ok(If {
            if_block,
            else_if_blocks: vec![],
            else_block,
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            // variable/constant declaration
            Some(Token::Let | Token::Const) => {
                let mutable = self.consume() == Some(Token::Const);

                if let Some(Token::Identifier(name)) = self.consume() {
                    self.expect(Token::Colon)?;
                    let datatype = self.parse_type()?;

                    self.expect(Token::Assignment)?;
                    let value = self.parse_expression()?;

                    self.expect(Token::SemiColon)?;

                    Ok(Statement::Declaration {
                        mutable,
                        name,
                        datatype,
                        value,
                    })
                } else {
                    Err("expected an identifier".to_string())
                }
            }
            // function declaration
            Some(Token::PubKeyword | Token::FnKeyword) => {
                let public = self.peek() == Some(&Token::PubKeyword);

                if public {
                    // consume the `pub` keyword
                    self.consume();
                }

                self.expect(Token::FnKeyword)?;

                if let Some(Token::Identifier(name)) = self.consume() {
                    let parameters = self.parse_parameter_list()?;
                    self.expect(Token::Colon)?;
                    let return_type = self.parse_type()?; // return type of the function
                    let body = self.parse_block()?;
                    Ok(Statement::FuncDeclaration {
                        public,
                        name,
                        parameters,
                        return_type,
                        body,
                    })
                } else {
                    Err("expected an identifier".to_string())
                }
            }
            // block
            Some(Token::OpenBrace) => {
                let block = self.parse_block()?;
                //self.expect(Token::SemiColon)?;
                Ok(Statement::ExpressionStatement(Expression::Block {
                    statements: block.statements,
                    return_value: block.return_value,
                }))
            }
            // re-assignment
            Some(Token::Identifier(_) | Token::NumberLiteral(_)) => {
                let expression = self.parse_expression()?;
                self.expect(Token::SemiColon)?;
                Ok(Statement::ExpressionStatement(expression))
            }
            _ => Err("expected a statement".to_string()),
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Vec::new();

        while self.peek().is_some() {
            program.push(self.parse_statement()?);
        }

        Ok(Program {
            statements: program,
        })
    }
}
