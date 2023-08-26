use honey::parser::*;
use honey::syntax::*;

const TEST_SRC: &'static str = r#"
    1 + 2 - 3
"#;

fn main() {
    let tokens = vec![
        Token::Literal(Literal::Number(Number::Int(1))),
        Token::Plus,
        Token::Literal(Literal::Number(Number::Int(2))),
        Token::Minus,
        Token::Literal(Literal::Number(Number::Int(3))),
    ];

    let tree = parse(&tokens);
    println!("{tree:#?}");
}
