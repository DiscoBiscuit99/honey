use honey::{codegen::arch::x86_64::*, codegen::*, lexer, parser::*};

const TEST_SRC: &'static str = r#"
    1 + 2 + 3
"#;

fn main() {
    let tokens = lexer::lex(TEST_SRC).expect("failed to lex source");
    let ast = parse(&tokens).expect("failed to parse tokens");
    println!("{ast:#?}");

    let mut x86_assembler = Assembler::new(X86_64);
    x86_assembler.assemble_ast(&ast);
    x86_assembler.display();
}
