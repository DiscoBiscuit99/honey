use honey::{lexer, parser};

const DECLARATION_SRC: &str = r#"
# let x: number = 1;  # immutable value
# mut y: number = 0;  # mutable value
# y = y + x;          # reassignment with addition
# y = y - x;          # and with subtraction
# y = y * x;          # and with multiplication
# y = y / x;          # and with division
"#;

const SIMPLE_FN_SRC: &str = r#"
let boring: (x: number) -> number = x;

let double_me: (x: number) -> number = {
    x * 2
};

let double_us_and_add_us: (x: number, y: number) -> number = {
    x * 2 + y * 2
};
"#;

fn main() {
    let input = SIMPLE_FN_SRC;
    let tokens = lexer::lex(input);
    println!("Tokens: {:#?}", tokens);
    match parser::parse(tokens) {
        Ok(program) => println!("Parsed program: {:#?}", program),
        Err(e) => println!("Failed to parse: {}", e),
    }
}
