use honey::{codegen::arch::x86_64::*, codegen::*, lexer, parser::*};

const TEST_SRC: &'static str = r#"
    1 - 2 + 3
"#;

fn main() {
    let tokens = lexer::lex(TEST_SRC).expect("failed to lex source");
    let test_ast = parse(&tokens).expect("failed to parse tokens");
    println!("{test_ast:#?}");

    let mut x86_assembler = Assembler::new(X86_64);

    // let test_ast = SyntaxTree::Term(Term::Quotient {
    //     dividend: Box::new(Term::Factor(Factor::Literal(Literal::Number(Number::Int(
    //         4,
    //     ))))),
    //     divisor: Factor::Literal(Literal::Number(Number::Int(2))),
    // });

    x86_assembler.assemble_ast(&test_ast);
    x86_assembler.display();
    x86_assembler.compile();
}
