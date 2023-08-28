use honey::{lexer, parser::*};

const TEST_SRC: &'static str = r#"
    1 + 2 - 3 + 4
"#;

fn main() {
    let tokens = lexer::lex(TEST_SRC).expect("failed to lex source");
    let tree = parse(&tokens).expect("failed to parse tokens");
    println!("{tree:#?}");
}
