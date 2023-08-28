use crate::syntax::*;

pub mod arch;
use arch::Arch;

pub trait Codegen<A: Arch> {
    fn to_asm(&self, asm: &mut Assembler<A>);
}

pub struct Assembler<A: Arch> {
    arch: A,
    lines: Vec<String>, // to store the emitted assembly lines
}

impl<A: Arch> Assembler<A> {
    // You can now delegate to self.arch for architecture-specific code generation
    pub fn new(arch: A) -> Assembler<A> {
        Assembler {
            arch,
            lines: vec![],
        }
    }

    pub fn emit(&mut self, asm: &str) {
        self.lines.push("  ".to_string() + asm)
    }

    pub fn assemble_ast(&mut self, ast: &impl Codegen<A>) {
        self.lines.push(self.arch.emit_header());

        ast.to_asm(self);

        self.lines.push(self.arch.emit_footer());
    }

    pub fn display(&self) {
        for line in &self.lines {
            println!("{}", line);
        }
    }
}

impl<A: Arch> Codegen<A> for Literal {
    fn to_asm(&self, asm: &mut Assembler<A>) {
        match self {
            Literal::Number(Number::Int(i)) => {
                asm.emit(&format!("mov rax, {}", i));
            }
            Literal::Number(Number::Float(f)) => unimplemented!(),
        }
    }
}

impl<A: Arch> Codegen<A> for Factor {
    fn to_asm(&self, asm: &mut Assembler<A>) {
        match self {
            Factor::Literal(literal) => {
                literal.to_asm(asm);
            }
            _ => unimplemented!(),
        }
    }
}

impl<A: Arch> Codegen<A> for Term {
    fn to_asm(&self, asm: &mut Assembler<A>) {
        match self {
            Term::Factor(factor) => {
                factor.to_asm(asm);
            }
            _ => unimplemented!(),
        }
    }
}

impl<A: Arch> Codegen<A> for Expression {
    fn to_asm(&self, asm: &mut Assembler<A>) {
        match self {
            Expression::Term(term) => {
                term.to_asm(asm);
            }
            Expression::Sum { augend, addend } => {
                asm.emit("; Start of sum expression");
                augend.to_asm(asm);
                asm.emit("push rax"); // Save the result of the left-hand expression
                addend.to_asm(asm);
                asm.emit("pop rbx"); // Pop the left-hand result into rbx
                asm.emit("add rax, rbx"); // Perform the addition
                asm.emit("; End of sum expression");
            }
            _ => unimplemented!(),
        }
    }
}

impl<A: Arch> Codegen<A> for SyntaxTree {
    fn to_asm(&self, asm: &mut Assembler<A>) {
        match self {
            SyntaxTree::Expression(expression) => expression.to_asm(asm),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {}
