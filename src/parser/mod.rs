use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::syntax::{
    Assignment, DataType, DeclKeyword, Declaration, Expression, Factor, Identifier, Keyword,
    Literal, Number, NumberLiteral, Statement, SyntaxTree, Term, TokenKind, Tokens,
};

#[cfg(test)]
mod tests;

mod errors;
use errors::*;

mod sparse;

lazy_static! {
    static ref DATA_TYPE_MAP: HashMap<&'static str, DataType> = {
        HashMap::from([
            ("number", DataType::Number),
            ("int", DataType::Int),
            ("float", DataType::Float),
        ])
    };
}

fn get_data_type(lexeme: &str) -> Option<DataType> {
    DATA_TYPE_MAP.get(lexeme).cloned()
}

pub fn parse(tokens: &Tokens) -> SyntaxTree {
    parse_program(tokens).expect("Oh oh, something went wrong...")
}

fn parse_program(tokens: &Tokens) -> Result<SyntaxTree, ParsingError> {
    let (ast, _rest) = parse_statement_list(tokens)?;
    Ok(ast)
}

fn parse_statement_list(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let mut statements = vec![];

    let (statement_ast, mut rest_tokens) = parse_statement(tokens)?;

    let parse_result = if let SyntaxTree::Statement(ref statement) = statement_ast {
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

        let parse_result = if let SyntaxTree::Statement(ref statement) = next_statement_ast {
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

    let ast = SyntaxTree::StatementList { statements };

    Ok((ast, rest_tokens))
}

fn parse_statement(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let (declaration_ast, rest_tokens) = parse_declaration(tokens)?;
    let parse_result = if let SyntaxTree::Declaration(ref declaration) = declaration_ast {
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

    let ast = SyntaxTree::Statement(declaration);

    Ok((ast, rest_tokens))
}

fn parse_declaration(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let (keyword_ast, rest_tokens) = parse_keyword(tokens)?;
    let parse_result = if let SyntaxTree::Keyword(ref keyword) = keyword_ast {
        match keyword {
            Keyword::DeclKeyword(decl_keyword) => Some(decl_keyword),
        }
    } else {
        None
    };

    let keyword = parse_result.ok_or(ParsingError::ExpectedDeclKeyword {
        found: Some(keyword_ast.clone()),
    })?;

    let (assignment_ast, rest_tokens) = parse_typed_assignment(&rest_tokens)?;

    let (identifier, data_type, expression) =
        if let SyntaxTree::Assignment(ref assignment) = assignment_ast {
            Some((
                assignment.identifier.clone(),
                assignment.data_type.clone(),
                assignment.expression.clone(),
            ))
        } else {
            None
        }
        .ok_or(ParsingError::ExpectedAssignment {
            found: Some(assignment_ast.clone()),
        })?;

    let ast = SyntaxTree::Declaration(Declaration {
        keyword: keyword.clone(),
        identifier,
        data_type: data_type.ok_or(ParsingError::ExpectedDataType { found: None })?,
        expression,
    });

    Ok((ast, rest_tokens))
}

fn parse_keyword(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let (first_token, rest_tokens) = tokens
        .split_first()
        .ok_or(ParsingError::ExpectedKeywordToken { found: None })?;

    let parse_result = match first_token.kind {
        TokenKind::Keyword => match first_token.lexeme.as_deref() {
            Some("let") => Some(Keyword::DeclKeyword(DeclKeyword::Let)),
            Some("mut") => Some(Keyword::DeclKeyword(DeclKeyword::Mut)),
            _ => None,
        },
        _ => None,
    };

    let keyword = parse_result.ok_or(ParsingError::ExpectedKeywordToken {
        found: Some(first_token.clone()),
    })?;

    let ast = SyntaxTree::Keyword(keyword);

    Ok((ast, rest_tokens.to_vec()))
}

fn parse_typed_assignment(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let (identifier_ast, rest_tokens) = parse_identifier(tokens)?;
    let identifier = if let SyntaxTree::Identifier(identifier) = identifier_ast {
        Ok(identifier)
    } else {
        Err(ParsingError::ExpectedIdentifier {
            found: Some(identifier_ast.clone()),
        })
    }?;

    let (data_type_ast, rest_tokens) = parse_type_annotation(&rest_tokens)?;
    let data_type = if let SyntaxTree::DataType(data_type) = data_type_ast {
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
    let expression = if let SyntaxTree::Expression(expression) = expression_ast {
        Ok(expression)
    } else {
        Err(ParsingError::ExpectedExpression {
            found: Some(expression_ast.clone()),
        })
    }?;

    let ast = SyntaxTree::Assignment(Assignment {
        identifier,
        data_type: Some(data_type),
        expression,
    });

    Ok((ast, rest_tokens))
}

fn parse_assignment(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let (identifier_ast, rest_tokens) = parse_identifier(tokens)?;
    let identifier = if let SyntaxTree::Identifier(identifier) = identifier_ast {
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
    let expression = if let SyntaxTree::Expression(expression) = expression_ast {
        Ok(expression)
    } else {
        Err(ParsingError::ExpectedExpression {
            found: Some(expression_ast.clone()),
        })
    }?;

    let ast = SyntaxTree::Assignment(Assignment {
        identifier,
        data_type: None,
        expression,
    });

    Ok((ast, rest_tokens))
}

fn parse_identifier(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
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

    let ast = SyntaxTree::Identifier(identifier);

    Ok((ast, rest_tokens.to_vec()))
}

fn parse_type_annotation(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let (first_token, rest_tokens) = tokens.split_first().ok_or(ParsingError::EmptyInput)?;

    // Check if the first token is a colon
    if first_token.kind != TokenKind::Colon {
        return Err(ParsingError::ExpectedColonToken {
            found: Some(first_token.clone()),
        });
    }

    let data_type_token = rest_tokens
        .get(0)
        .ok_or(ParsingError::ExpectedDataTypeToken { found: None })?;

    let parse_result = match data_type_token.kind {
        TokenKind::DataType => data_type_token
            .lexeme
            .as_ref()
            .and_then(|lexeme| get_data_type(lexeme)),
        _ => None,
    };

    let data_type = parse_result.ok_or(ParsingError::ExpectedDataTypeToken {
        found: Some(data_type_token.clone()),
    })?;

    let ast = SyntaxTree::DataType(data_type);

    // Skip the first two tokens: the colon and the data type
    let remaining_tokens = rest_tokens[1..].to_vec();

    Ok((ast, remaining_tokens))
}

fn parse_expression(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let (term_ast, mut rest_tokens) = parse_term(tokens)?;
    let parse_result = if let SyntaxTree::Term(term) = term_ast {
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
                let right_term = if let SyntaxTree::Term(term) = right_term {
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
                let right_term = if let SyntaxTree::Term(term) = right_term {
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

    let ast = SyntaxTree::Expression(expression);

    Ok((ast, rest_tokens))
}

fn parse_term(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let (factor_ast, mut rest_tokens) = parse_factor(tokens)?;
    let parse_result = if let SyntaxTree::Factor(factor) = factor_ast {
        Some(factor)
    } else {
        None
    };

    let mut left_term =
        Term::Factor(parse_result.ok_or(ParsingError::ExpectedFactor { found: None })?);

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
                let right_factor = if let SyntaxTree::Factor(factor) = right_factor {
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
                let right_factor = if let SyntaxTree::Factor(factor) = right_factor {
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

    let ast = SyntaxTree::Term(left_term);

    Ok((ast, rest_tokens))
}

fn parse_factor(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let (literal_ast, rest_tokens) = parse_literal(tokens)?;
    let parse_result =
        if let SyntaxTree::Literal(Literal::NumberLiteral(NumberLiteral::Number(ref number))) =
            literal_ast
        {
            Some(number)
        } else {
            None
        };

    let number = parse_result.ok_or(ParsingError::ExpectedNumber {
        found: Some(literal_ast.clone()),
    })?;

    let ast = SyntaxTree::Factor(Factor::Number(number.clone()));

    Ok((ast, rest_tokens))
}

fn parse_literal(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let (number_ast, rest_tokens) = parse_number(tokens)?;
    let parse_result = if let SyntaxTree::Number(number) = number_ast {
        Some(Literal::NumberLiteral(NumberLiteral::Number(number)))
    } else {
        None
    };

    let literal = parse_result.ok_or(ParsingError::ExpectedLiteral { found: None })?;

    let ast = SyntaxTree::Literal(literal);

    Ok((ast, rest_tokens))
}

fn parse_number(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
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

    let ast = SyntaxTree::Number(number);

    Ok((ast, rest_tokens.to_vec())) // Assuming Tokens is a Vec or similar
}
