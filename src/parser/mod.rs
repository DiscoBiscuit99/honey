use colored::Colorize;

use crate::syntax::parse_tree::Arguments;
use crate::syntax::parse_tree::Block;
use crate::syntax::parse_tree::ConditionalBlock;
use crate::syntax::parse_tree::Expression;
use crate::syntax::parse_tree::If;
use crate::syntax::parse_tree::Param;
use crate::syntax::parse_tree::Program;
use crate::syntax::parse_tree::Statement;
use crate::syntax::parse_tree::Type;
use crate::syntax::tokens::Token;

pub fn parse(tokens: &[Token]) -> Program {
    match Parser::new(tokens.to_vec()).parse_program() {
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

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.peek() == Some(&expected) {
            self.consume();
            Ok(())
        } else {
            Err(self.expect_err_msg(&expected, self.peek()))
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match self.consume() {
            Some(Token::NumberKeyword) => Ok(Type::Number),
            Some(Token::UnitKeyword) => Ok(Type::Unit),
            Some(Token::OpenParen) => {
                let mut param_types = Vec::new();
                while self.peek() != Some(&Token::CloseParen) {
                    let param = self.parse_param()?;
                    param_types.push(param);
                    if self.peek() == Some(&Token::Comma) {
                        self.consume();
                    }
                }
                self.expect(Token::CloseParen)?;
                self.expect(Token::Arrow)?;
                let return_type = self.parse_type()?;
                Ok(Type::FuncType {
                    parameters: param_types,
                    return_type: Box::new(return_type),
                })
            }
            _ => Err("expected a type".to_string()),
        }
    }

    fn parse_param(&mut self) -> Result<Param, String> {
        if let Some(Token::Identifier(name)) = self.consume() {
            self.expect(Token::Colon)?;
            let datatype = self.parse_type()?;
            Ok(Param::Parameter { name, datatype })
        } else {
            Err("expected an identifier".to_string())
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
        let expr = if let Some(Token::OpenBrace) = self.peek() {
            Expression::Block(self.parse_block()?)
        } else if let Some(Token::If) = self.peek() {
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

        let mut return_value = Expression::Unit;
        let mut statements = Vec::new();
        while let Some(token) = self.peek().cloned() {
            match token {
                Token::CloseBrace => break,
                _ => {
                    let parse_result = self.parse_statement();
                    if let Some(stmt) = parse_result.ok() {
                        statements.push(stmt);
                    } else {
                        return_value = self.parse_expression()?;
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
            // consume both 'else' and 'if'
            self.consume();
            self.consume();

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
            Some(Token::Let) | Some(Token::Mut) => {
                let mutable = if self.consume() == Some(Token::Mut) {
                    true
                } else {
                    false
                };
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
            Some(Token::OpenBrace) => {
                let block = self.parse_block()?;
                Ok(Statement::ExpressionStatement(Expression::Block(block)))
            }
            _ => Err("expected a statement".to_string()),
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Vec::new();

        while self.peek().is_some() {
            program.push(self.parse_statement()?);
        }

        Ok(program)
    }
}
