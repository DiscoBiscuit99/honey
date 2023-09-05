use honey::{
    lexer, parser,
    syntax::{Token, TokenKind},
};

const DECLARATION_SRC: &str = r#"
let x: number = 1;  # immutable value
mut y: number = 0;  # mutable value
"#;

const SIMPLE_FN_SRC: &str = r#"
let double_me = x: number -> number {
    x * 2
}
"#;

fn main() {
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
            kind: TokenKind::Minus,
            lexeme: Some("-".to_string()),
        },
        Token {
            kind: TokenKind::LeftParenthesis,
            lexeme: Some("(".to_string()),
        },
        Token {
            kind: TokenKind::Integer,
            lexeme: Some("1".to_string()),
        },
        Token {
            kind: TokenKind::Plus,
            lexeme: Some("+".to_string()),
        },
        Token {
            kind: TokenKind::Integer,
            lexeme: Some("1".to_string()),
        },
        Token {
            kind: TokenKind::RightParenthesis,
            lexeme: Some(")".to_string()),
        },
        Token {
            kind: TokenKind::Times,
            lexeme: Some("*".to_string()),
        },
        Token {
            kind: TokenKind::Integer,
            lexeme: Some("2".to_string()),
        },
        Token {
            kind: TokenKind::Divide,
            lexeme: Some("/".to_string()),
        },
        Token {
            kind: TokenKind::Integer,
            lexeme: Some("3".to_string()),
        },
        Token {
            kind: TokenKind::Semicolon,
            lexeme: Some(String::from(";")),
        },
        Token {
            kind: TokenKind::Keyword,
            lexeme: Some("mut".to_string()),
        },
        Token {
            kind: TokenKind::Identifier,
            lexeme: Some("b".to_string()),
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

    let block_tokens = vec![
        Token {
            kind: TokenKind::Keyword,
            lexeme: Some(String::from("let")),
        },
        Token {
            kind: TokenKind::Identifier,
            lexeme: Some(String::from("result")),
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
            kind: TokenKind::Assignment,
            lexeme: Some(String::from("=")),
        },
        Token {
            kind: TokenKind::LeftCurly,
            lexeme: Some(String::from("{")),
        },
        Token {
            kind: TokenKind::Number,
            lexeme: Some(String::from("3")),
        },
        Token {
            kind: TokenKind::Times,
            lexeme: Some(String::from("*")),
        },
        Token {
            kind: TokenKind::Number,
            lexeme: Some(String::from("3")),
        },
        Token {
            kind: TokenKind::RightCurly,
            lexeme: Some(String::from("}")),
        },
        Token {
            kind: TokenKind::Semicolon,
            lexeme: Some(String::from(";")),
        },
    ];

    // let ast = parser::parse(&tokens);
    let block_ast = parser::parse(&block_tokens);

    println!("{block_ast:#?}");
}
