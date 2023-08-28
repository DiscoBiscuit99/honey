use crate::syntax::{tokens::*, *};

mod errors;
use errors::*;

pub fn parse(tokens: &Tokens) -> Result<SyntaxTree, ParsingError> {
    let (tree, _) = parse_expression(tokens)?;
    Ok(tree)
}

fn parse_expression(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let (left_term, mut remaining_tokens) = parse_term(tokens)?;

    // Unwrap the term.
    let mut left_term = if let SyntaxTree::Term(term) = left_term {
        Ok(Expression::Term(term))
    } else {
        Err(ParsingError::ExpectedTerm {
            found: Some(left_term),
        })
    }?;

    // Loop through and apply as many "+" or "-" as possible
    while let Some(token) = remaining_tokens.get(0) {
        match token {
            Token::Plus => {
                // Remove the operator token
                let _ = remaining_tokens.remove(0);

                // Parse the next term
                let (right_term, new_remaining_tokens) = parse_term(&remaining_tokens)?;

                // Update remaining_tokens
                remaining_tokens = new_remaining_tokens;

                // Unwrap the term.
                let right_term = if let SyntaxTree::Term(term) = right_term {
                    Ok(term)
                } else {
                    Err(ParsingError::ExpectedTerm {
                        found: Some(right_term),
                    })
                }?;

                // Update the left_term to include the new operation and right term
                left_term = Expression::Sum {
                    augend: Box::new(left_term),
                    addend: right_term,
                };
            }
            Token::Minus => {
                // Remove the operator token
                let _ = remaining_tokens.remove(0);

                // Parse the next term
                let (right_term, new_remaining_tokens) = parse_term(&remaining_tokens)?;

                // Update remaining_tokens
                remaining_tokens = new_remaining_tokens;

                // Unwrap the term.
                let right_term = if let SyntaxTree::Term(term) = right_term {
                    Ok(term)
                } else {
                    Err(ParsingError::ExpectedTerm {
                        found: Some(right_term),
                    })
                }?;

                // Update the left_term to include the new operation and right term
                left_term = Expression::Difference {
                    minuend: Box::new(left_term),
                    subtrahend: right_term,
                };
            }
            // Break if we find a token that isn't a '+' or '-'
            _ => break,
        }
    }

    let subtree = SyntaxTree::Expression(left_term);

    Ok((subtree, remaining_tokens))
}

fn parse_term(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let mut tokens_iter = tokens.iter().peekable();

    let _ = tokens_iter
        .peek()
        .ok_or(ParsingError::ExpectedTerm { found: None })?;

    let (subtree, remaining_tokens) = parse_factor(tokens)?;

    let term = if let SyntaxTree::Factor(fac) = subtree {
        Ok(SyntaxTree::Term(Term::Factor(fac)))
    } else {
        Err(ParsingError::ExpectedFactor {
            found: Some(subtree),
        })
    }?;

    Ok((term, remaining_tokens))
}

fn parse_factor(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let mut tokens_iter = tokens.iter().peekable();

    let next = tokens_iter
        .peek()
        .ok_or(ParsingError::ExpectedFactor { found: None })?;

    let (subtree, remaining_tokens) = if let Token::Literal(_) = next {
        let (literal, remaining_tokens) = parse_literal(tokens)?;

        let subtree = if let SyntaxTree::Literal(lit) = literal {
            Ok(SyntaxTree::Factor(Factor::Literal(lit)))
        } else {
            Err(ParsingError::ExpectedLiteral {
                found: Some(literal.clone()),
            })
        }?;

        Ok((subtree, remaining_tokens))
    } else {
        Err(ParsingError::Unknown)
    }?;

    Ok((subtree, remaining_tokens))
}

fn parse_literal(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let mut tokens_iter = tokens.iter().peekable();

    // Assert the next token exists.
    let next = tokens_iter
        .peek()
        .ok_or(ParsingError::ExpectedLiteral { found: None })?;

    // Assert that the next token is the expected token.
    let (subtree, remaining_tokens) = if let Token::Literal(Literal::Number(_)) = next {
        // parse number
        parse_number(tokens)
    } else {
        Err(ParsingError::ExpectedToken(ExpectedToken::Literal {
            found: Some((*next).clone()),
        }))
    }?;

    Ok((subtree, remaining_tokens))
}

/// Parses a number literal and returns it as well as the remaining tokens.
/// Returns a parsing error if failed.
fn parse_number(tokens: &Tokens) -> Result<(SyntaxTree, Tokens), ParsingError> {
    let mut tokens_iter = tokens.iter();

    // Assert the next token exists.
    let next = tokens_iter
        .next()
        .ok_or(ParsingError::ExpectedNumber { found: None })?;

    // Assert that the next token is the expected token.
    let num = if let Token::Literal(Literal::Number(num)) = next {
        Ok(num.clone())
    } else {
        Err(ParsingError::ExpectedToken(ExpectedToken::Number {
            found: Some(next.clone()),
        }))
    }?;

    // Wrap it up in a syntax tree.
    let subtree = SyntaxTree::Literal(Literal::Number(num));

    // Collect the remaining tokens
    let remaining_tokens = tokens_iter.cloned().collect();

    Ok((subtree, remaining_tokens))
}

#[cfg(test)]
mod tests {
    use crate::syntax::{tokens::*, *};

    #[test]
    fn parse_number() {
        // Arrange
        let tokens = vec![Token::Literal(Literal::Number(Number::Int(3)))];
        let expected = SyntaxTree::Literal(Literal::Number(Number::Int(3)));

        // Act
        let (tree, _tokens) = super::parse_number(&tokens).expect("failed to parse number");

        // Assert
        assert_eq!(tree, expected);
    }

    #[test]
    fn parse_literal() {
        // Arrange
        let tokens = vec![Token::Literal(Literal::Number(Number::Int(3)))];
        let expected = SyntaxTree::Literal(Literal::Number(Number::Int(3)));

        // Act
        let (tree, _tokens) = super::parse_literal(&tokens).expect("failed to parse literal");

        // Assert
        assert_eq!(tree, expected);
    }

    #[test]
    fn parse_factor() {
        // Arrange
        let tokens = vec![Token::Literal(Literal::Number(Number::Int(3)))];
        let expected = SyntaxTree::Factor(Factor::Literal(Literal::Number(Number::Int(3))));

        // Act
        let (tree, _tokens) = super::parse_factor(&tokens).expect("failed to parse factor");

        // Assert
        assert_eq!(tree, expected);
    }

    #[test]
    fn parse_term() {
        // Arrange
        let tokens = vec![Token::Literal(Literal::Number(Number::Int(3)))];
        let expected = SyntaxTree::Term(Term::Factor(Factor::Literal(Literal::Number(
            Number::Int(3),
        ))));

        // Act
        let (tree, _tokens) = super::parse_term(&tokens).expect("failed to parse term");

        // Assert
        assert_eq!(tree, expected);
    }

    #[test]
    fn parse_expression() {
        // Arrange
        let tokens = vec![
            Token::Literal(Literal::Number(Number::Int(3))),
            Token::Plus,
            Token::Literal(Literal::Number(Number::Int(3))),
            Token::Minus,
            Token::Literal(Literal::Number(Number::Int(2))),
        ];

        let expected = SyntaxTree::Expression(Expression::Difference {
            minuend: Box::new(Expression::Sum {
                augend: Box::new(Expression::Term(Term::Factor(Factor::Literal(
                    Literal::Number(Number::Int(3)),
                )))),
                addend: Term::Factor(Factor::Literal(Literal::Number(Number::Int(3)))),
            }),
            subtrahend: Term::Factor(Factor::Literal(Literal::Number(Number::Int(2)))),
        });

        // Act
        let (tree, _tokens) = super::parse_expression(&tokens).expect("failed to parse expression");

        // Assert
        assert_eq!(tree, expected);
    }
}
