use crate::syntax::{
    DataType, DeclKeyword, Declaration, Expression, Factor, Identifier, Literal, Number,
    SyntaxTree, Term, Token, TokenKind,
};

#[test]
fn parse_declaration() {
    // Arrange
    let tokens = vec![
        Token {
            kind: TokenKind::Keyword,
            lexeme: Some("let".to_string()),
        },
        Token {
            kind: TokenKind::Identifier,
            lexeme: Some("a".to_string()),
        },
        Token {
            kind: TokenKind::Colon,
            lexeme: Some(":".to_string()),
        },
        Token {
            kind: TokenKind::DataType,
            lexeme: Some("int".to_string()),
        },
        Token {
            kind: TokenKind::Assignment,
            lexeme: Some("=".to_string()),
        },
        Token {
            kind: TokenKind::Integer,
            lexeme: Some("1".to_string()),
        },
    ];

    let expected = SyntaxTree::Declaration(Declaration {
        keyword: DeclKeyword::Let,
        identifier: Identifier("a".to_string()),
        data_type: DataType::Int,
        expression: Expression::Term(Term::Factor(Factor::Number(Number::Integer(1)))),
    });

    // Act
    let (ast, _rest) = super::parse_declaration(&tokens).expect("failed to parse declaration");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_basic_expression() {
    // Arrange
    let tokens = vec![Token {
        kind: TokenKind::Number,
        lexeme: Some("3".to_string()),
    }];

    let expected = SyntaxTree::Expression(Expression::Term(Term::Factor(Factor::Number(
        Number::Integer(3),
    ))));

    // Act
    let (ast, _rest) = super::parse_expression(&tokens).expect("failed to parse basic expression");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_sum_expression() {
    // Arrange
    let tokens = vec![
        Token {
            kind: TokenKind::Number,
            lexeme: Some("3".to_string()),
        },
        Token {
            kind: TokenKind::Plus,
            lexeme: Some("+".to_string()),
        },
        Token {
            kind: TokenKind::Number,
            lexeme: Some("3".to_string()),
        },
    ];

    let expected = SyntaxTree::Expression(Expression::Sum {
        augend: Box::new(Expression::Term(Term::Factor(Factor::Number(
            Number::Integer(3),
        )))),
        addend: Term::Factor(Factor::Number(Number::Integer(3))),
    });

    // Act
    let (ast, _rest) = super::parse_expression(&tokens).expect("failed to parse sum expression");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_difference_expression() {
    // Arrange
    let tokens = vec![
        Token {
            kind: TokenKind::Number,
            lexeme: Some("3".to_string()),
        },
        Token {
            kind: TokenKind::Minus,
            lexeme: Some("-".to_string()),
        },
        Token {
            kind: TokenKind::Number,
            lexeme: Some("3".to_string()),
        },
    ];

    let expected = SyntaxTree::Expression(Expression::Difference {
        minuend: Box::new(Expression::Term(Term::Factor(Factor::Number(
            Number::Integer(3),
        )))),
        subtrahend: Term::Factor(Factor::Number(Number::Integer(3))),
    });

    // Act
    let (ast, _rest) =
        super::parse_expression(&tokens).expect("failed to parse difference expression");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_factor_term() {
    // Arrange
    let tokens = vec![Token {
        kind: TokenKind::Number,
        lexeme: Some("3".to_string()),
    }];

    let expected = SyntaxTree::Term(Term::Factor(Factor::Number(Number::Integer(3))));

    // Act
    let (ast, _rest) = super::parse_term(&tokens).expect("failed to parse term");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_product_term() {
    // Arrange
    let tokens = vec![
        Token {
            kind: TokenKind::Number,
            lexeme: Some("3".to_string()),
        },
        Token {
            kind: TokenKind::Times,
            lexeme: Some("*".to_string()),
        },
        Token {
            kind: TokenKind::Number,
            lexeme: Some("4".to_string()),
        },
    ];

    let expected = SyntaxTree::Term(Term::Product {
        multiplicant: Box::new(Term::Factor(Factor::Number(Number::Integer(3)))),
        multiplier: Factor::Number(Number::Integer(4)),
    });

    // Act
    let (ast, _rest) = super::parse_term(&tokens).expect("failed to parse product term");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_division_term() {
    // Arrange
    let tokens = vec![
        Token {
            kind: TokenKind::Number,
            lexeme: Some("3".to_string()),
        },
        Token {
            kind: TokenKind::Divide,
            lexeme: Some("/".to_string()),
        },
        Token {
            kind: TokenKind::Number,
            lexeme: Some("4".to_string()),
        },
    ];

    let expected = SyntaxTree::Term(Term::Quotient {
        dividend: Box::new(Term::Factor(Factor::Number(Number::Integer(3)))),
        divisor: Factor::Number(Number::Integer(4)),
    });

    // Act
    let (ast, _rest) = super::parse_term(&tokens).expect("failed to parse division term");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_factor() {
    // Arrange
    let tokens = vec![Token {
        kind: TokenKind::Number,
        lexeme: Some("3".to_string()),
    }];

    let expected = SyntaxTree::Factor(Factor::Number(Number::Integer(3)));

    // Act
    let (ast, _rest) = super::parse_factor(&tokens).expect("failed to parse factor");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_unspecified_floating_point_number() {
    // Arrange
    let tokens = vec![Token {
        kind: TokenKind::Number,
        lexeme: Some("3.14".to_string()),
    }];

    // Act
    let (ast, _rest) = super::parse_number(&tokens).expect("failed to parse number");

    // Assert
    assert_eq!(ast, SyntaxTree::Number(Number::Float(3.14)));
}

#[test]
fn parse_unspecified_integer_number() {
    // Arrange
    let tokens = vec![Token {
        kind: TokenKind::Number,
        lexeme: Some("3".to_string()),
    }];

    // Act
    let (ast, _rest) = super::parse_number(&tokens).expect("failed to parse number");

    // Assert
    assert_eq!(ast, SyntaxTree::Number(Number::Integer(3)));
}

#[test]
fn parse_float() {
    // Arrange
    let tokens = vec![Token {
        kind: TokenKind::Float,
        lexeme: Some("3.14".to_string()),
    }];

    // Act
    let (ast, _rest) = super::parse_number(&tokens).expect("failed to parse number");

    // Assert
    assert_eq!(ast, SyntaxTree::Number(Number::Float(3.14)));
}

#[test]
fn parse_integer() {
    // Arrange
    let tokens = vec![Token {
        kind: TokenKind::Integer,
        lexeme: Some("3".to_string()),
    }];

    // Act
    let (ast, _rest) = super::parse_number(&tokens).expect("failed to parse number");

    // Assert
    assert_eq!(ast, SyntaxTree::Number(Number::Integer(3)));
}
