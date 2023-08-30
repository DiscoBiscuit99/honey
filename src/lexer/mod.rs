use crate::syntax::{Token, TokenKind, Tokens};

mod errors;
use errors::LexingError;

pub fn lex(input: &str) -> Result<Tokens, LexingError> {
    let (tokens, _) = parse_statement(input)?;
    Ok(tokens)
}

fn parse_statement(input: &str) -> Result<(Tokens, String), LexingError> {
    parse_declaration(input)
}

fn parse_declaration(input: &str) -> Result<(Tokens, String), LexingError> {
    let mut tokens = vec![];

    let (decl_keyword, rest) = parse_keyword_lexeme(input)?;
    tokens.push(decl_keyword);

    let (identifier, rest) = parse_identifier_lexeme(&rest)?;
    tokens.push(identifier);

    let (colon, rest) = parse_special_lexeme(&rest)?;
    if colon.kind != TokenKind::Colon {
        Err(LexingError::UnexpectedToken {
            expected: Some(TokenKind::Colon),
            found: Some(colon.clone()),
        })?;
    }
    tokens.push(colon);

    let (data_type, rest) = parse_identifier_lexeme(&rest)?;
    tokens.push(data_type);

    let (assignment, rest) = parse_special_lexeme(&rest)?;
    if assignment.kind != TokenKind::Assignment {
        Err(LexingError::UnexpectedToken {
            expected: Some(TokenKind::Assignment),
            found: Some(assignment.clone()),
        })?;
    }
    tokens.push(assignment);

    let (mut expression, rest) = parse_expression(&rest)?;
    tokens.append(&mut expression);

    Ok((tokens, rest))
}

fn parse_expression(input: &str) -> Result<(Tokens, String), LexingError> {
    let (numeric, rest) = parse_numeric_lexeme(input)?;
    Ok((vec![numeric], rest))
}

fn parse_numeric_lexeme(input: &str) -> Result<(Token, String), LexingError> {
    let mut chars = input.chars().peekable();

    // Collect the first token literal from the given input.
    let mut lexeme = String::new();
    while let Some(c) = chars.next() {
        if !c.is_alphabetic() && !(c == '_') {
            break;
        }
        lexeme.push(c);
    }

    let rest = chars.collect();

    let token = Token {
        kind: TokenKind::Identifier,
        lexeme: Some(lexeme),
    };

    Ok((token, rest))
}

fn parse_special_lexeme(input: &str) -> Result<(Token, String), LexingError> {
    let mut chars = input.chars().peekable();

    let next = chars.next();
    let token = if let Some(c) = next {
        match c {
            ':' => Ok(Token {
                kind: TokenKind::Colon,
                lexeme: Some(c.to_string()),
            }),
            '=' => Ok(Token {
                kind: TokenKind::Assignment,
                lexeme: Some(c.to_string()),
            }),
            '+' => Ok(Token {
                kind: TokenKind::Plus,
                lexeme: Some(c.to_string()),
            }),
            '-' => Ok(Token {
                kind: TokenKind::Minus,
                lexeme: Some(c.to_string()),
            }),
            _ => Err(LexingError::ExpectedSpecial { found: Some(c) }),
        }
    } else {
        Err(LexingError::ExpectedSpecial { found: None })
    }?;

    let rest = chars.collect();

    Ok((token, rest))
}

fn parse_identifier_lexeme(input: &str) -> Result<(Token, String), LexingError> {
    let mut chars = input.chars().peekable();

    // Collect the first token literal from the given input.
    let mut lexeme = String::new();
    while let Some(c) = chars.next() {
        if !c.is_alphabetic() && !(c == '_') {
            break;
        }
        lexeme.push(c);
    }

    let rest = chars.collect();

    let token = Token {
        kind: TokenKind::Identifier,
        lexeme: Some(lexeme),
    };

    Ok((token, rest))
}

fn parse_keyword_lexeme(input: &str) -> Result<(Token, String), LexingError> {
    let (keyword, rest) = parse_identifier_lexeme(input)?;

    let token = if keyword
        .lexeme
        .clone()
        .is_some_and(|lexeme| &lexeme == "let" || &lexeme == "mut")
    {
        Ok(Token {
            kind: TokenKind::Keyword,
            lexeme: keyword.lexeme,
        })
    } else {
        Err(LexingError::UnexpectedToken {
            expected: Some(TokenKind::Keyword),
            found: Some(keyword),
        })
    }?;

    Ok((token, rest))
}
