use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::syntax::{
    Assignment, BasicType, Block, DeclKeyword, Declaration, Expression, Factor, FuncType,
    Identifier, Keyword, Literal, Number, NumberLiteral, Parameter, ParseTree, Statement, Term,
    Token, TokenKind, Type,
};

#[cfg(test)]
mod tests;

mod errors;
use errors::*;

mod sparse;

lazy_static! {
    static ref DATA_TYPE_MAP: HashMap<&'static str, BasicType> = {
        HashMap::from([
            ("number", BasicType::Number),
            ("int", BasicType::Int),
            ("float", BasicType::Float),
        ])
    };
}

fn get_data_type(lexeme: &str) -> Option<BasicType> {
    DATA_TYPE_MAP.get(lexeme).cloned()
}

pub fn parse(tokens: &[Token]) -> ParseTree {
    parse_program(tokens).expect("Oh oh, something went wrong...")
}

fn parse_program(tokens: &[Token]) -> Result<ParseTree, ParsingError> {
    let (ast, _rest) = parse_statement_list(tokens)?;
    Ok(ast)
}

fn parse_block(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let (first_token, rest_tokens) = tokens
        .split_first()
        .ok_or(ParsingError::ExpectedLeftCurl { found: None })?;

    if first_token.kind != TokenKind::LeftCurly {
        Err(ParsingError::ExpectedLeftCurl {
            found: Some(first_token.clone()),
        })?;
    }

    let (statement_list_ast, rest_tokens) = parse_statement_list(&rest_tokens)?;

    let parse_result = if let ParseTree::StatementList { ref statements } = statement_list_ast {
        Some(statements)
    } else {
        None
    };

    let statements = parse_result.ok_or(ParsingError::ExpectedStatementList {
        found: Some(statement_list_ast.clone()),
    })?;

    let (return_expression_ast, rest_tokens) = parse_expression(&rest_tokens)?;

    let parse_result = if let ParseTree::Expression(ref expression) = return_expression_ast {
        Some(expression)
    } else {
        None
    };

    let return_expression = parse_result.ok_or(ParsingError::ExpectedExpression {
        found: Some(return_expression_ast.clone()),
    })?;

    let (last_token, rest_tokens) = rest_tokens
        .split_first()
        .ok_or(ParsingError::ExpectedRightCurl { found: None })?;

    if last_token.kind != TokenKind::RightCurly {
        Err(ParsingError::ExpectedRightCurl {
            found: Some(last_token.clone()),
        })?;
    }

    let ast = ParseTree::Block(Block {
        statements: statements.to_vec(),
        return_expression: return_expression.clone(),
    });

    Ok((ast, rest_tokens.to_vec()))
}

fn parse_statement_list(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let mut statements = vec![];

    let parse_result = parse_statement(tokens);

    let (statement_ast, mut rest_tokens) = if let Some((ast, rest_tokens)) = parse_result.ok() {
        (ast, rest_tokens)
    } else {
        let ast = ParseTree::StatementList { statements };
        return Ok((ast, tokens.to_vec()));
    };

    let parse_result = if let ParseTree::Statement(ref statement) = statement_ast {
        Some(statement)
    } else {
        None
    };

    let first_statement = parse_result.ok_or(ParsingError::ExpectedStatement {
        found: Some(statement_ast.clone()),
    })?;

    statements.push(first_statement.clone());

    while !rest_tokens.is_empty() {
        let (next_statement_ast, next_rest_tokens) = parse_statement(&rest_tokens)?;

        let parse_result = if let ParseTree::Statement(ref statement) = next_statement_ast {
            Some(statement)
        } else {
            None
        };

        let next_statement = parse_result.ok_or(ParsingError::ExpectedStatement {
            found: Some(next_statement_ast.clone()),
        })?;

        statements.push(next_statement.clone());
        rest_tokens = next_rest_tokens;
    }

    let ast = ParseTree::StatementList { statements };

    Ok((ast, rest_tokens))
}

fn parse_statement(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let (declaration_ast, rest_tokens) = parse_declaration(tokens)?;
    let parse_result = if let ParseTree::Declaration(ref declaration) = declaration_ast {
        Some(declaration)
    } else {
        None
    };

    let declaration = Statement::Declaration(
        parse_result
            .ok_or(ParsingError::ExpectedDeclaration {
                found: Some(declaration_ast.clone()),
            })?
            .clone(),
    );

    let ast = ParseTree::Statement(declaration);

    Ok((ast, rest_tokens))
}

fn parse_declaration(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let (keyword_ast, rest_tokens) = parse_keyword(tokens)?;
    let parse_result = if let ParseTree::Keyword(ref keyword) = keyword_ast {
        match keyword {
            Keyword::DeclKeyword(decl_keyword) => Some(decl_keyword),
        }
    } else {
        None
    };

    let keyword = parse_result.ok_or(ParsingError::ExpectedDeclKeyword {
        found: Some(keyword_ast.clone()),
    })?;

    let (identifier_ast, rest_tokens) = parse_identifier(&rest_tokens)?;
    let identifier = if let ParseTree::Identifier(identifier) = identifier_ast {
        Ok(identifier)
    } else {
        Err(ParsingError::ExpectedIdentifier {
            found: Some(identifier_ast.clone()),
        })
    }?;

    let (colon_token, rest_tokens) = rest_tokens.split_first().ok_or(ParsingError::EmptyInput)?;

    // Check if the first token is a colon
    if colon_token.kind != TokenKind::Colon {
        return Err(ParsingError::ExpectedColonToken {
            found: Some(colon_token.clone()),
        });
    }

    let (type_ast, rest_tokens) = parse_type(rest_tokens)?;

    let parse_result = if let ParseTree::Type(ref data_type) = type_ast {
        Some(data_type)
    } else {
        None
    };

    let data_type = parse_result.ok_or(ParsingError::ExpectedDataType {
        found: Some(type_ast.clone()),
    })?;

    let (assignment_token, rest_tokens) = rest_tokens
        .split_first()
        .ok_or(ParsingError::ExpectedAssignment { found: None })?;

    if assignment_token.kind != TokenKind::Assignment {
        Err(ParsingError::ExpectedAssignToken {
            found: Some(assignment_token.clone()),
        })?;
    }

    let (expression_ast, rest_tokens) = parse_expression(&rest_tokens)?;

    let parse_result = if let ParseTree::Expression(ref expression) = expression_ast {
        Some(expression)
    } else {
        None
    };

    let expression = parse_result.ok_or(ParsingError::ExpectedExpression {
        found: Some(expression_ast.clone()),
    })?;

    let (_last_token, rest_tokens) = rest_tokens
        .split_first()
        .ok_or(ParsingError::ExpectedSemicolon { found: None })?;

    let ast = ParseTree::Declaration(Declaration {
        keyword: keyword.clone(),
        identifier,
        data_type: data_type.clone(),
        expression: expression.clone(),
    });

    Ok((ast, rest_tokens.to_vec()))
}

fn parse_keyword(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let (first_token, rest_tokens) = tokens
        .split_first()
        .ok_or(ParsingError::ExpectedKeywordToken { found: None })?;

    let parse_result = match first_token.kind {
        TokenKind::Keyword => match first_token.lexeme.as_deref() {
            Some("let") => Some(Keyword::DeclKeyword(DeclKeyword::Let)),
            // Some("mut") => Some(Keyword::DeclKeyword(DeclKeyword::Mut)),
            _ => None,
        },
        _ => None,
    };

    let keyword = parse_result.ok_or(ParsingError::ExpectedKeywordToken {
        found: Some(first_token.clone()),
    })?;

    let ast = ParseTree::Keyword(keyword);

    Ok((ast, rest_tokens.to_vec()))
}

fn parse_typed_assignment(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let (identifier_ast, rest_tokens) = parse_identifier(tokens)?;
    let identifier = if let ParseTree::Identifier(identifier) = identifier_ast {
        Ok(identifier)
    } else {
        Err(ParsingError::ExpectedIdentifier {
            found: Some(identifier_ast.clone()),
        })
    }?;

    let (data_type_ast, rest_tokens) = parse_type_annotation(&rest_tokens)?;
    let data_type = if let ParseTree::Type(data_type) = data_type_ast {
        Ok(data_type)
    } else {
        Err(ParsingError::ExpectedDataType {
            found: Some(data_type_ast.clone()),
        })
    }?;

    let (assignment_token, rest_tokens) = rest_tokens
        .split_first()
        .ok_or(ParsingError::ExpectedAssignToken { found: None })?;

    if assignment_token.kind != TokenKind::Assignment {
        Err(ParsingError::ExpectedAssignToken {
            found: Some(assignment_token.clone()),
        })?
    }

    let (expression_ast, rest_tokens) = parse_expression(&rest_tokens.to_vec())?;
    let expression = if let ParseTree::Expression(expression) = expression_ast {
        Ok(expression)
    } else {
        Err(ParsingError::ExpectedExpression {
            found: Some(expression_ast.clone()),
        })
    }?;

    let ast = ParseTree::Assignment(Assignment {
        identifier,
        data_type: Some(data_type),
        expression,
    });

    Ok((ast, rest_tokens))
}

fn parse_assignment(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let (identifier_ast, rest_tokens) = parse_identifier(tokens)?;
    let identifier = if let ParseTree::Identifier(identifier) = identifier_ast {
        Ok(identifier)
    } else {
        Err(ParsingError::ExpectedIdentifier {
            found: Some(identifier_ast.clone()),
        })
    }?;

    let (assignment_token, _rest_token) = rest_tokens
        .split_first()
        .ok_or(ParsingError::ExpectedAssignToken { found: None })?;

    if assignment_token.kind != TokenKind::Assignment {
        Err(ParsingError::ExpectedAssignToken {
            found: Some(assignment_token.clone()),
        })?
    }

    let (expression_ast, rest_tokens) = parse_expression(&rest_tokens)?;
    let expression = if let ParseTree::Expression(expression) = expression_ast {
        Ok(expression)
    } else {
        Err(ParsingError::ExpectedExpression {
            found: Some(expression_ast.clone()),
        })
    }?;

    let ast = ParseTree::Assignment(Assignment {
        identifier,
        data_type: None,
        expression,
    });

    Ok((ast, rest_tokens))
}

fn parse_identifier(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let (first_token, rest_tokens) = tokens.split_first().ok_or(ParsingError::EmptyInput)?;

    let parse_result = match first_token.kind {
        TokenKind::Identifier => first_token
            .lexeme
            .as_ref()
            .map(|lexeme| Identifier(lexeme.clone())),
        _ => None,
    };

    let identifier = parse_result.ok_or(ParsingError::ExpectedIdentifierToken {
        found: Some(first_token.clone()),
    })?;

    let ast = ParseTree::Identifier(identifier);

    Ok((ast, rest_tokens.to_vec()))
}

fn parse_parameter(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let (identifier_ast, rest_tokens) = parse_identifier(tokens)?;

    let parse_result = if let ParseTree::Identifier(ref identifier) = identifier_ast {
        Some(identifier)
    } else {
        None
    };

    let identifier = parse_result.ok_or(ParsingError::ExpectedIdentifier {
        found: Some(identifier_ast.clone()),
    })?;

    let (type_ast, rest_tokens) = parse_type_annotation(&rest_tokens)?;

    let parse_result = if let ParseTree::Type(ref data_type) = type_ast {
        Some(data_type)
    } else {
        None
    };

    let data_type = parse_result.ok_or(ParsingError::ExpectedDataType {
        found: Some(type_ast.clone()),
    })?;

    let ast = ParseTree::Parameter(Parameter {
        identifier: identifier.clone(),
        data_type: data_type.clone(),
    });

    Ok((ast, rest_tokens))
}

fn parse_parameter_list(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let mut parameter_list = vec![];

    let (parameter_ast, mut rest_tokens) = parse_parameter(tokens)?;

    let parse_result = if let ParseTree::Parameter(ref parameter) = parameter_ast {
        Some(parameter)
    } else {
        None
    };

    let parameter = parse_result.ok_or(ParsingError::ExpectedParameter {
        found: Some(parameter_ast.clone()),
    })?;

    parameter_list.push(parameter.clone());

    let mut next_token = rest_tokens.get(0);
    while next_token.is_some_and(|t| t.kind == TokenKind::Comma) {
        rest_tokens.remove(0); // consume the comma
        let (next_param_ast, remaining) = parse_parameter(&rest_tokens)?;

        let parse_result = if let ParseTree::Parameter(ref parameter) = next_param_ast {
            Some(parameter)
        } else {
            None
        };

        let next_parameter = parse_result.ok_or(ParsingError::ExpectedParameter {
            found: Some(next_param_ast.clone()),
        })?;

        parameter_list.push(next_parameter.clone());

        rest_tokens = remaining;
        next_token = rest_tokens.get(0);
    }

    let ast = ParseTree::ParameterList {
        parameters: parameter_list,
    };

    Ok((ast, rest_tokens.to_vec()))
}

fn parse_type(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let (first_token, rest_tokens) = tokens.split_first().ok_or(ParsingError::EmptyInput)?;

    let (type_ast, rest_tokens) = if first_token.kind == TokenKind::LeftParenthesis {
        // Parsing function type...

        dbg!("what");
        let (parameter_list_ast, remaining) = parse_parameter_list(&rest_tokens)?;

        let parse_result = if let ParseTree::ParameterList { ref parameters } = parameter_list_ast {
            Some(parameters)
        } else {
            None
        };

        let parameter_list = parse_result.ok_or(ParsingError::ExpectedParameter {
            found: Some(parameter_list_ast.clone()),
        })?;

        let (r_paren, remaining) = remaining
            .split_first()
            .ok_or(ParsingError::ExpectedRightParen { found: None })?;

        if r_paren.kind != TokenKind::RightParenthesis {
            Err(ParsingError::ExpectedRightParen {
                found: Some(r_paren.clone()),
            })?;
        }

        let (arrow_token, remaining) = remaining
            .split_first()
            .ok_or(ParsingError::ExpectedRightArrowToken { found: None })?;

        if arrow_token.kind != TokenKind::RightArrow {
            Err(ParsingError::ExpectedRightArrowToken {
                found: Some(arrow_token.clone()),
            })?;
        }

        let (return_type_ast, remaining) = parse_type(remaining)?;

        let parse_result = if let ParseTree::Type(ref data_type) = return_type_ast {
            Some(data_type)
        } else {
            None
        };

        let return_type = parse_result.ok_or(ParsingError::ExpectedReturnType {
            found: Some(return_type_ast.clone()),
        })?;

        let type_ast = ParseTree::Type(Type::FuncType(FuncType {
            param_list: parameter_list.to_vec(),
            return_type: Box::new(return_type.clone()),
        }));

        (type_ast, remaining)
    } else {
        // parse basic type
        let parse_result = match first_token.kind {
            TokenKind::DataType => first_token
                .lexeme
                .as_ref()
                .and_then(|lexeme| get_data_type(lexeme)),
            _ => None,
        };

        let data_type = parse_result.ok_or(ParsingError::ExpectedDataTypeToken {
            found: Some(first_token.clone()),
        })?;

        let type_ast = ParseTree::Type(Type::BasicType(data_type));

        (type_ast, rest_tokens.to_vec())
    };

    Ok((type_ast, rest_tokens.to_vec()))
}

fn parse_type_annotation(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let (first_token, rest_tokens) = tokens.split_first().ok_or(ParsingError::EmptyInput)?;

    // Check if the first token is a colon
    if first_token.kind != TokenKind::Colon {
        return Err(ParsingError::ExpectedColonToken {
            found: Some(first_token.clone()),
        });
    }

    parse_type(&rest_tokens)
}

fn parse_expression(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let mut tokens_iter = tokens.iter().peekable();
    let first_token = tokens_iter.peek();

    let (ast, rest_tokens) = if first_token.is_some_and(|t| t.kind == TokenKind::LeftCurly) {
        let (block_ast, rest_tokens) = parse_block(tokens)?;
        let parse_result = if let ParseTree::Block(ref block) = block_ast {
            Some(block)
        } else {
            None
        };

        let block = parse_result.ok_or(ParsingError::ExpectedBlock {
            found: Some(block_ast.clone()),
        })?;

        let expression_ast = ParseTree::Expression(Expression::Block(Box::new(block.clone())));

        (expression_ast, rest_tokens)
    } else {
        let (term_ast, mut rest_tokens) = parse_term(tokens)?;
        let parse_result = if let ParseTree::Term(term) = term_ast {
            Some(term)
        } else {
            None
        };

        let mut expression =
            Expression::Term(parse_result.ok_or(ParsingError::ExpectedTerm { found: None })?);

        // Loop through and apply as many "+" or "-" as possible
        while let Some(token) = rest_tokens.get(0) {
            match token.kind {
                TokenKind::Plus => {
                    // Remove the operator token
                    let _ = rest_tokens.remove(0);

                    // Parse the next term
                    let (right_term, new_rest_tokens) = parse_term(&rest_tokens)?;

                    // Update remaining_tokens
                    rest_tokens = new_rest_tokens;

                    // Unwrap the factor.
                    let right_term = if let ParseTree::Term(term) = right_term {
                        Ok(term)
                    } else {
                        Err(ParsingError::ExpectedTerm {
                            found: Some(right_term),
                        })
                    }?;

                    // Update the left_term to include the new operation and right term
                    expression = Expression::Sum {
                        augend: Box::new(expression),
                        addend: right_term,
                    };
                }
                TokenKind::Minus => {
                    // Remove the operator token
                    let _ = rest_tokens.remove(0);

                    // Parse the next term
                    let (right_term, new_rest_tokens) = parse_term(&rest_tokens)?;

                    // Update remaining_tokens
                    rest_tokens = new_rest_tokens;

                    // Unwrap the factor.
                    let right_term = if let ParseTree::Term(term) = right_term {
                        Ok(term)
                    } else {
                        Err(ParsingError::ExpectedTerm {
                            found: Some(right_term),
                        })
                    }?;

                    // Update the left_term to include the new operation and right term
                    expression = Expression::Difference {
                        minuend: Box::new(expression),
                        subtrahend: right_term,
                    };
                }
                // Break if we find a token that isn't a '+' or '-'
                _ => break,
            }
        }

        let expression_ast = ParseTree::Expression(expression);

        (expression_ast, rest_tokens)
    };

    Ok((ast, rest_tokens))
}

fn parse_term(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let mut tokens_iter = tokens.iter().peekable();
    let minus_token = if tokens_iter
        .peek()
        .is_some_and(|t| t.kind == TokenKind::Minus)
    {
        tokens_iter.next()
    } else {
        None
    };

    let rest_tokens: Vec<Token> = tokens_iter.cloned().collect();

    let (factor_ast, mut rest_tokens) = parse_factor(&rest_tokens)?;
    let parse_result = if let ParseTree::Factor(factor) = factor_ast {
        Some(factor)
    } else {
        None
    };

    let mut left_term = if minus_token.is_some() {
        Term::NegatedFactor(parse_result.ok_or(ParsingError::ExpectedFactor { found: None })?)
    } else {
        Term::Factor(parse_result.ok_or(ParsingError::ExpectedFactor { found: None })?)
    };

    // Loop through and apply as many "*" or "/" as possible
    while let Some(token) = rest_tokens.get(0) {
        match token.kind {
            TokenKind::Times => {
                // Remove the operator token
                let _ = rest_tokens.remove(0);

                // Parse the next term
                let (right_factor, new_rest_tokens) = parse_factor(&rest_tokens)?;

                // Update remaining_tokens
                rest_tokens = new_rest_tokens;

                // Unwrap the factor.
                let right_factor = if let ParseTree::Factor(factor) = right_factor {
                    Ok(factor)
                } else {
                    Err(ParsingError::ExpectedFactor {
                        found: Some(right_factor),
                    })
                }?;

                // Update the left_term to include the new operation and right term
                left_term = Term::Product {
                    multiplicant: Box::new(left_term),
                    multiplier: right_factor,
                };
            }
            TokenKind::Divide => {
                // Remove the operator token
                let _ = rest_tokens.remove(0);

                // Parse the next term
                let (right_factor, new_rest_tokens) = parse_factor(&rest_tokens)?;

                // Update remaining_tokens
                rest_tokens = new_rest_tokens;

                // Unwrap the factor.
                let right_factor = if let ParseTree::Factor(factor) = right_factor {
                    Ok(factor)
                } else {
                    Err(ParsingError::ExpectedFactor {
                        found: Some(right_factor),
                    })
                }?;

                // Update the left_term to include the new operation and right term
                left_term = Term::Quotient {
                    dividend: Box::new(left_term),
                    divisor: right_factor,
                };
            }
            // Break if we find a token that isn't a '*' or '/'
            _ => break,
        }
    }

    let ast = ParseTree::Term(left_term);

    Ok((ast, rest_tokens))
}

fn parse_factor(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let mut tokens_iter = tokens.iter().peekable();

    let (ast, rest_tokens) = if tokens_iter
        .peek()
        .is_some_and(|t| t.kind == TokenKind::LeftParenthesis)
    {
        let (_l_paren, rest_tokens) = tokens
            .split_first()
            .ok_or(ParsingError::ExpectedLeftParen { found: None })?;

        let (expression_ast, rest_tokens) = parse_expression(&rest_tokens)?;

        let parse_result = if let ParseTree::Expression(ref expression) = expression_ast {
            Some(expression)
        } else {
            None
        };

        let expression = parse_result.ok_or(ParsingError::ExpectedExpression {
            found: Some(expression_ast.clone()),
        })?;

        let ast = ParseTree::Factor(Factor::ParentheizedExpression(Box::new(expression.clone())));

        let (_r_paren, rest_tokens) = rest_tokens
            .split_first()
            .ok_or(ParsingError::ExpectedRightParen { found: None })?;

        (ast, rest_tokens.to_vec())
    } else {
        let (literal_ast, rest_tokens) = parse_literal(tokens)?;
        let parse_result =
            if let ParseTree::Literal(Literal::NumberLiteral(NumberLiteral::Number(ref number))) =
                literal_ast
            {
                Some(number)
            } else {
                None
            };

        let number = parse_result.ok_or(ParsingError::ExpectedNumber {
            found: Some(literal_ast.clone()),
        })?;

        let ast = ParseTree::Factor(Factor::Number(number.clone()));

        (ast, rest_tokens)
    };

    Ok((ast, rest_tokens))
}

fn parse_literal(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let (number_ast, rest_tokens) = parse_number(tokens)?;
    let parse_result = if let ParseTree::Number(number) = number_ast {
        Some(Literal::NumberLiteral(NumberLiteral::Number(number)))
    } else {
        None
    };

    let literal = parse_result.ok_or(ParsingError::ExpectedLiteral { found: None })?;

    let ast = ParseTree::Literal(literal);

    Ok((ast, rest_tokens))
}

fn parse_number(tokens: &[Token]) -> Result<(ParseTree, Vec<Token>), ParsingError> {
    let (first_token, rest_tokens) = tokens.split_first().ok_or(ParsingError::EmptyInput)?;

    let parse_result = match first_token.kind {
        TokenKind::Number => {
            let number = first_token
                .lexeme
                .as_ref()
                .ok_or(ParsingError::ExpectedLexeme)?
                .parse::<i64>()
                .ok()
                .map(Number::Integer);

            if number.is_none() {
                first_token
                    .lexeme
                    .as_ref()
                    .ok_or(ParsingError::ExpectedLexeme)?
                    .parse::<f64>()
                    .ok()
                    .map(Number::Float)
            } else {
                number
            }
        }

        TokenKind::Integer => first_token
            .lexeme
            .as_ref()
            .ok_or(ParsingError::ExpectedLexeme)?
            .parse::<i64>()
            .ok()
            .map(Number::Integer),

        TokenKind::Float => first_token
            .lexeme
            .as_ref()
            .ok_or(ParsingError::ExpectedLexeme)?
            .parse::<f64>()
            .ok()
            .map(Number::Float),

        _ => None,
    };

    let number = parse_result.ok_or(ParsingError::ExpectedNumberToken {
        found: Some(first_token.clone()),
    })?;

    let ast = ParseTree::Number(number);

    Ok((ast, rest_tokens.to_vec())) // Assuming Tokens is a Vec or similar
}
