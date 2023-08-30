use honey::lexer;

const DECLARATION_SRC: &str = r#"
let x = 1;  # immutable value
mut y = 0;  # mutable value
y += x;     # result: y <- y + x
"#;

const SIMPLE_FN_SRC: &str = r#"
let double_me = x: number -> number {
    x * 2
}
"#;

fn main() {
    let tokens = lexer::lex(DECLARATION_SRC).expect("failed to lex declaration source");
    dbg!(tokens);
}
