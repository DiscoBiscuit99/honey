pub mod tokens;
use tokens::*;

/// Takes a source and produces a collection of tokens.
pub fn lex(src: &str) -> Vec<Token> {
    let literals = source_to_literals(src);
    literals_to_tokens(literals)
}

/// Takes a source and produces a collection of token literals.
pub fn source_to_literals(src: &str) -> Vec<String> {
    let mut token_literals = vec![];
    let mut src = src.to_string();

    while !src.is_empty() {
        let (token_literal, rest) = parse_literal(&src);
        token_literals.push(token_literal);
        src = rest;
    }

    token_literals
}

/// Returns the first literal of the given source string.
fn parse_literal(src: &str) -> (String, String) {
    let src = src.trim();
    let mut chars = src.chars().peekable();

    if chars.peek().is_some_and(|c| c.is_alphabetic()) {
        parse_identifier_literal(src)
    } else if chars.peek().is_some_and(|c| c.is_numeric()) {
        parse_numeric_literal(src)
    } else if chars.peek().is_some_and(|c| !c.is_alphanumeric()) {
        // Special character?
        parse_special_literal(src)
    } else {
        panic!("{src}");
    }
}

/// Returns the first identifier literal of the given input.
fn parse_identifier_literal(src: &str) -> (String, String) {
    let mut chars = src.chars().peekable();

    // Collect the first token literal from the given input.
    let mut token_literal = String::new();
    while let Some(c) = chars.peek() {
        if !c.is_alphabetic() && !(*c == '_') {
            break;
        }
        token_literal.push(*c);
        chars.next(); // consume the character.
    }

    let rest = chars.collect();
    (token_literal, rest)
}

fn parse_numeric_literal(src: &str) -> (String, String) {
    let mut chars = src.chars().peekable();

    let mut token_literal = String::new();
    while let Some(c) = chars.peek() {
        if !c.is_numeric() && !(*c == '.') {
            break;
        }
        token_literal.push(*c);
        chars.next();
    }

    let rest = chars.collect();
    (token_literal, rest)
}

fn parse_special_literal(src: &str) -> (String, String) {
    let mut chars = src.chars().peekable();

    let mut token_literal = String::new();
    match chars.peek() {
        Some(c) => {
            if *c == ':' {
                token_literal.push(*c);
                chars.next();
            } else if *c == '=' {
                token_literal.push(*c);
                chars.next();
            } else if *c == ';' {
                token_literal.push(*c);
                chars.next();
            }
        }
        None => panic!("No characters to parse."),
    }

    let rest = chars.collect();
    (token_literal, rest)
}

/// Converts the given collection of token literals to a collection of corresponding token types.
pub fn literals_to_tokens(token_literals: Vec<String>) -> Vec<Token> {
    token_literals
        .into_iter()
        .map(|literal| literal_to_token(&literal))
        .collect()
}

fn literal_to_token(literal: &str) -> Token {
    let first_char = literal.chars().next().expect("Empty literal?");
    if first_char.is_alphabetic() {
        identifier_literal_to_token(literal)
    } else if first_char.is_numeric() {
        numeric_literal_to_token(literal)
    } else {
        special_literal_to_token(literal)
    }
}

fn identifier_literal_to_token(literal: &str) -> Token {
    match literal {
        "let" => Token::Keyword(Keyword::Let),
        "int" => Token::DataType(DataType::Int),
        "float" => Token::DataType(DataType::Float),
        _ => Token::Identifier(Identifier(literal.to_string())),
    }
}

fn numeric_literal_to_token(literal: &str) -> Token {
    let first_char = literal.chars().next().expect("Empty numeric literal?");
    if first_char.is_numeric() {
        if literal.contains(".") {
            Token::Literal(Literal::Float(
                literal
                    .parse::<f64>()
                    .expect("Failed to parse floating point literal?"),
            ))
        } else {
            Token::Literal(Literal::Int(
                literal
                    .parse::<i64>()
                    .expect("Failed to parse integer point literal?"),
            ))
        }
    } else {
        Token::Unknown
    }
}

fn special_literal_to_token(literal: &str) -> Token {
    let first_char = literal.chars().next().expect("Empty numeric literal?");
    match first_char {
        ':' => Token::Special(Special::Colon),
        ';' => Token::Special(Special::StatementStop),
        '=' => Token::Special(Special::Assignment),
        _ => Token::Unknown,
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_numeric_literal() {
        let int_src = "3";
        let float_src = "3.14";

        let (int_literal, _) = super::parse_numeric_literal(int_src);
        let (float_literal, _) = super::parse_numeric_literal(float_src);

        assert_eq!(&int_literal, int_src);
        assert_eq!(&float_literal, float_src);
    }

    #[test]
    fn parse_identifier_literal() {
        let src = "code_name";
        let (identifier_literal, _) = super::parse_identifier_literal(src);
        assert_eq!(&identifier_literal, "code_name");
    }

    #[test]
    fn parse_literal() {
        let src = "let a: int = 2;";
        let (token_literal, rest) = super::parse_literal(src);

        assert_eq!(&token_literal, "let");
        assert_eq!(&rest, " a: int = 2;");

        let (token_literal, rest) = super::parse_literal(&rest);

        assert_eq!(&token_literal, "a");
        assert_eq!(&rest, ": int = 2;");

        let (token_literal, rest) = super::parse_literal(&rest);

        assert_eq!(&token_literal, ":");
        assert_eq!(&rest, " int = 2;");

        let (token_literal, rest) = super::parse_literal(&rest);

        assert_eq!(&token_literal, "int");
        assert_eq!(&rest, " = 2;");

        let (token_literal, rest) = super::parse_literal(&rest);

        assert_eq!(&token_literal, "=");
        assert_eq!(&rest, " 2;");

        let (token_literal, rest) = super::parse_literal(&rest);

        assert_eq!(&token_literal, "2");
        assert_eq!(&rest, ";");

        let (token_literal, rest) = super::parse_literal(&rest);

        assert_eq!(&token_literal, ";");
        assert_eq!(&rest, "");
    }

    #[test]
    fn source_to_literals() {
        let src = "let a: int = 2;";
        let token_literals = super::source_to_literals(src);
        assert_eq!(token_literals, vec!["let", "a", ":", "int", "=", "2", ";"]);
    }

    #[test]
    fn literals_to_tokens() {
        use crate::lexer::{DataType, Identifier, Keyword, Literal, Special, Token};

        let src = "let a: int = 2;";
        let token_literals = super::source_to_literals(src);
        let tokens = super::literals_to_tokens(token_literals);

        assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::Let),
                Token::Identifier(Identifier("a".to_string())),
                Token::Special(Special::Colon),
                Token::DataType(DataType::Int),
                Token::Special(Special::Assignment),
                Token::Literal(Literal::Int(2)),
                Token::Special(Special::StatementStop),
            ]
        );
    }

    #[test]
    fn lex() {
        use crate::lexer::{DataType, Identifier, Keyword, Literal, Special, Token};

        let src = "let a: int = 2;";
        let tokens = super::lex(src);

        assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::Let),
                Token::Identifier(Identifier("a".to_string())),
                Token::Special(Special::Colon),
                Token::DataType(DataType::Int),
                Token::Special(Special::Assignment),
                Token::Literal(Literal::Int(2)),
                Token::Special(Special::StatementStop),
            ]
        );
    }
}
