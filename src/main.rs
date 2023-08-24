use functor::{lexer, parser};

const TEST_SRC: &'static str = r#"
    let code_name: int = 2;
    let test: float = 2.3;
"#;

fn main() {
    let tokens = lexer::lex(TEST_SRC);
    println!("\nGenerated tokens:\n{tokens:?}");

    let ast = parser::parse(tokens);
    println!("\nGenerated parse tree:\n{ast:#?}");
}
