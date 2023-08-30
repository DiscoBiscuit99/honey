mod type_test;
use type_test::*;

pub fn parse(tokens: Vec<Token>) -> Program {
    todo!()
}

fn parse_literal(tokens: Vec<Token>) -> (SyntaxTree, Vec<Token>) {
    let mut tokens = tokens.iter();
    let literal = tokens
        .next()
        .expect("failed to parse literal: expected literal, found nothing");

    let literal = match literal {
        Token::Literal(literal) => literal,
        _ => panic!("failed to parse literal: expected literal, found {literal:?}"),
    };

    let tokens = tokens.map(|t| t.clone()).collect();

    (literal, tokens)
}
