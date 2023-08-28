use crate::syntax::{tokens::*, *};

mod errors;
use errors::*;

pub fn lex(src: &str) -> Result<Tokens, LexingError> {
    let literals = lex_literals(src)?;
    literals_to_tokens(literals)
}

fn literals_to_tokens(literals: Vec<String>) -> Result<Tokens, LexingError> {
    literals
        .into_iter()
        .map(|lit| literal_to_token(&lit))
        .collect()
}

fn literal_to_token(literal: &str) -> Result<Token, LexingError> {
    let first_char = literal.chars().next().expect("Empty literal?");
    if first_char.is_numeric() {
        numeric_literal_to_token(literal)
    } else {
        special_literal_to_token(literal)
    }
}

fn numeric_literal_to_token(literal: &str) -> Result<Token, LexingError> {
    let first_char = literal.chars().next().expect("Empty numeric literal?");
    if first_char.is_numeric() {
        if literal.contains(".") {
            Ok(Token::Literal(Literal::Number(Number::Float(
                literal
                    .parse::<f64>()
                    .expect("Failed to parse floating point literal?"),
            ))))
        } else {
            Ok(Token::Literal(Literal::Number(Number::Int(
                literal
                    .parse::<i64>()
                    .expect("Failed to parse integer point literal?"),
            ))))
        }
    } else {
        Err(LexingError::ExpectedLiteral)
    }
}

fn special_literal_to_token(literal: &str) -> Result<Token, LexingError> {
    let first_char = literal.chars().next().expect("Empty numeric literal?");
    match first_char {
        '+' => Ok(Token::Plus),
        '-' => Ok(Token::Minus),
        _ => Err(LexingError::ExpectedSpecial),
    }
}

/// Takes a source and produces a collection of token literals.
fn lex_literals(src: &str) -> Result<Vec<String>, LexingError> {
    let mut token_literals = vec![];
    let mut src = src.to_string();

    while !src.is_empty() {
        let (token_literal, rest) = lex_literal(&src)?;
        token_literals.push(token_literal);
        src = rest;
    }

    Ok(token_literals)
}

/// Returns the first literal of the given source string.
fn lex_literal(src: &str) -> Result<(String, String), LexingError> {
    let src = src.trim();
    let mut chars = src.chars().peekable();

    if chars.peek().is_some_and(|c| c.is_numeric()) {
        lex_numeric_literal(src)
    } else if chars.peek().is_some_and(|c| !c.is_alphanumeric()) {
        // Special character?
        lex_special_literal(src)
    } else {
        Err(LexingError::ExpectedLiteral)
    }
}

fn lex_numeric_literal(src: &str) -> Result<(String, String), LexingError> {
    let mut chars = src.chars().peekable();

    if chars.peek().is_some_and(|c| !c.is_numeric()) {
        Err(LexingError::ExpectedNumber)?;
    }

    let mut token_literal = String::new();
    while let Some(c) = chars.peek() {
        if !c.is_numeric() && !(*c == '.') {
            break;
        }
        token_literal.push(*c);
        chars.next();
    }

    let rest = chars.collect();
    Ok((token_literal, rest))
}

fn lex_special_literal(src: &str) -> Result<(String, String), LexingError> {
    let mut chars = src.chars().peekable();

    let mut token_literal = String::new();
    match chars.peek() {
        Some(c) => {
            if *c == '+' || *c == '-' {
                token_literal.push(*c);
                chars.next();
            }
        }
        None => Err(LexingError::ExpectedSpecial)?,
    }

    let rest = chars.collect();
    Ok((token_literal, rest))
}

#[cfg(test)]
mod tests {
    use crate::syntax::{tokens::*, *};

    #[test]
    fn lex_numeric_literal() {
        // Arrange
        let src = "1";
        let expected = "1".to_string();

        // Act
        let (literal, _) = super::lex_numeric_literal(src).expect("failed to lex numeric literal");

        // Assert
        assert_eq!(literal, expected);
    }

    #[test]
    fn lex_literals() {
        // Arrange
        let src = "1 + 2";
        let expected = vec!["1".to_string(), "+".to_string(), "2".to_string()];

        // Act
        let literals = super::lex_literals(src).expect("failed to lex a collection of literals");

        // Assert
        assert_eq!(literals, expected);
    }

    #[test]
    fn literals_to_tokens() {
        // Arrange
        let src = "1 + 2";
        let expected = vec![
            Token::Literal(Literal::Number(Number::Int(1))),
            Token::Plus,
            Token::Literal(Literal::Number(Number::Int(2))),
        ];
        let literals = super::lex_literals(src).expect("failed to lex a collection of literals");

        // Act
        let tokens =
            super::literals_to_tokens(literals).expect("failed to conver literals to tokens");

        // Assert
        assert_eq!(tokens, expected);
    }
}
