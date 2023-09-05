use crate::syntax::{
    BasicType, Block, DeclKeyword, Declaration, Expression, Factor, FuncType, Identifier, Number,
    Parameter, ParseTree, Term, Token, TokenKind, Type,
};

#[test]
fn parse_simple_function_declaration() {
    // Arrange
    let tokens = vec![
        Token {
            kind: TokenKind::Keyword,
            lexeme: Some(String::from("let")),
        },
        Token {
            kind: TokenKind::Identifier,
            lexeme: Some(String::from("f")),
        },
        Token {
            kind: TokenKind::Colon,
            lexeme: Some(String::from(":")),
        },
        Token {
            kind: TokenKind::LeftParenthesis,
            lexeme: Some(String::from("(")),
        },
        Token {
            kind: TokenKind::Identifier,
            lexeme: Some(String::from("x")),
        },
        Token {
            kind: TokenKind::Colon,
            lexeme: Some(String::from(":")),
        },
        Token {
            kind: TokenKind::DataType,
            lexeme: Some(String::from("number")),
        },
        Token {
            kind: TokenKind::RightParenthesis,
            lexeme: Some(String::from(")")),
        },
        Token {
            kind: TokenKind::RightArrow,
            lexeme: Some(String::from("->")),
        },
        Token {
            kind: TokenKind::DataType,
            lexeme: Some(String::from("number")),
        },
        Token {
            kind: TokenKind::Assignment,
            lexeme: Some("=".to_string()),
        },
        Token {
            kind: TokenKind::Number,
            lexeme: Some("3".to_string()),
        },
        Token {
            kind: TokenKind::Semicolon,
            lexeme: Some(String::from(";")),
        },
    ];

    let expected = ParseTree::Declaration(Declaration {
        keyword: DeclKeyword::Let,
        identifier: Identifier(String::from("f")),
        data_type: Type::FuncType(FuncType {
            param_list: vec![Parameter {
                identifier: Identifier(String::from("x")),
                data_type: Type::BasicType(BasicType::Number),
            }],
            return_type: Box::new(Type::BasicType(BasicType::Number)),
        }),
        expression: Expression::Term(Term::Factor(Factor::Number(Number::Integer(3)))),
    });

    // Act
    let (ast, _rest) =
        super::parse_declaration(&tokens).expect("failed to parse function declaration");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_parameter() {
    // Arrange
    let tokens = vec![
        Token {
            kind: TokenKind::Identifier,
            lexeme: Some("x".to_string()),
        },
        Token {
            kind: TokenKind::Colon,
            lexeme: Some(":".to_string()),
        },
        Token {
            kind: TokenKind::DataType,
            lexeme: Some("number".to_string()),
        },
    ];

    let expected = ParseTree::Parameter(Parameter {
        identifier: Identifier(String::from("x")),
        data_type: Type::BasicType(BasicType::Number),
    });

    // Act
    let (ast, _rest) = super::parse_parameter(&tokens).expect("failed to parse parameter");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_parameter_list() {
    // Arrange
    let tokens = vec![
        Token {
            kind: TokenKind::Identifier,
            lexeme: Some("x".to_string()),
        },
        Token {
            kind: TokenKind::Colon,
            lexeme: Some(":".to_string()),
        },
        Token {
            kind: TokenKind::DataType,
            lexeme: Some("number".to_string()),
        },
        Token {
            kind: TokenKind::Comma,
            lexeme: Some(",".to_string()),
        },
        Token {
            kind: TokenKind::Identifier,
            lexeme: Some("y".to_string()),
        },
        Token {
            kind: TokenKind::Colon,
            lexeme: Some(":".to_string()),
        },
        Token {
            kind: TokenKind::DataType,
            lexeme: Some("number".to_string()),
        },
    ];

    let expected = ParseTree::ParameterList {
        parameters: vec![
            Parameter {
                identifier: Identifier(String::from("x")),
                data_type: Type::BasicType(BasicType::Number),
            },
            Parameter {
                identifier: Identifier(String::from("y")),
                data_type: Type::BasicType(BasicType::Number),
            },
        ],
    };

    // Act
    let (ast, _rest) =
        super::parse_parameter_list(&tokens).expect("failed to parse parameter list");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_block() {
    // Arrange
    let tokens = vec![
        Token {
            kind: TokenKind::LeftCurly,
            lexeme: Some("{".to_string()),
        },
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
            lexeme: Some("3".to_string()),
        },
        Token {
            kind: TokenKind::RightCurly,
            lexeme: Some("}".to_string()),
        },
    ];

    let expected = ParseTree::Block(Block {
        statements: vec![],
        return_expression: Expression::Term(Term::Product {
            multiplicant: Box::new(Term::Factor(Factor::Number(Number::Integer(3)))),
            multiplier: Factor::Number(Number::Integer(3)),
        }),
    });

    // Act
    let (ast, _rest) = super::parse_block(&tokens).expect("failed to parse block");

    // Assert
    assert_eq!(ast, expected);
}

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
        Token {
            kind: TokenKind::Semicolon,
            lexeme: Some(";".to_string()),
        },
    ];

    let expected = ParseTree::Declaration(Declaration {
        keyword: DeclKeyword::Let,
        identifier: Identifier("a".to_string()),
        data_type: Type::BasicType(BasicType::Int),
        expression: Expression::Term(Term::Factor(Factor::Number(Number::Integer(1)))),
    });

    // Act
    let (ast, _rest) = super::parse_declaration(&tokens).expect("failed to parse declaration");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_block_expression() {
    // Arrange
    let tokens = vec![
        Token {
            kind: TokenKind::LeftCurly,
            lexeme: Some("{".to_string()),
        },
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
            lexeme: Some("3".to_string()),
        },
        Token {
            kind: TokenKind::RightCurly,
            lexeme: Some("}".to_string()),
        },
    ];

    let expected = ParseTree::Expression(Expression::Block(Box::new(Block {
        statements: vec![],
        return_expression: Expression::Term(Term::Product {
            multiplicant: Box::new(Term::Factor(Factor::Number(Number::Integer(3)))),
            multiplier: Factor::Number(Number::Integer(3)),
        }),
    })));

    // Act
    let (ast, _rest) = super::parse_expression(&tokens).expect("failed to parse block expression");

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

    let expected = ParseTree::Expression(Expression::Term(Term::Factor(Factor::Number(
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

    let expected = ParseTree::Expression(Expression::Sum {
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

    let expected = ParseTree::Expression(Expression::Difference {
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

    let expected = ParseTree::Term(Term::Factor(Factor::Number(Number::Integer(3))));

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

    let expected = ParseTree::Term(Term::Product {
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

    let expected = ParseTree::Term(Term::Quotient {
        dividend: Box::new(Term::Factor(Factor::Number(Number::Integer(3)))),
        divisor: Factor::Number(Number::Integer(4)),
    });

    // Act
    let (ast, _rest) = super::parse_term(&tokens).expect("failed to parse division term");

    // Assert
    assert_eq!(ast, expected);
}

#[test]
fn parse_factor_parenthesized_expression() {
    // Arrange
    let tokens = vec![
        Token {
            kind: TokenKind::LeftParenthesis,
            lexeme: Some("(".to_string()),
        },
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
        Token {
            kind: TokenKind::RightParenthesis,
            lexeme: Some(")".to_string()),
        },
    ];

    let expected = ParseTree::Factor(Factor::ParentheizedExpression(Box::new(Expression::Sum {
        augend: Box::new(Expression::Term(Term::Factor(Factor::Number(
            Number::Integer(3),
        )))),
        addend: Term::Factor(Factor::Number(Number::Integer(3))),
    })));

    // Act
    let (ast, _rest) =
        super::parse_factor(&tokens).expect("failed to parse parenthesized expression as factor");

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

    let expected = ParseTree::Factor(Factor::Number(Number::Integer(3)));

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
    assert_eq!(ast, ParseTree::Number(Number::Float(3.14)));
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
    assert_eq!(ast, ParseTree::Number(Number::Integer(3)));
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
    assert_eq!(ast, ParseTree::Number(Number::Float(3.14)));
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
    assert_eq!(ast, ParseTree::Number(Number::Integer(3)));
}
