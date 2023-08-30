use std::{fs::OpenOptions, io::prelude::*, process::Command};

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
    pub fn new(arch: A) -> Assembler<A> {
        Assembler {
            arch,
            lines: vec![],
        }
    }

    pub fn emit(&mut self, asm: &str) {
        self.lines.push(asm.to_string())
    }

    pub fn assemble_ast(&mut self, ast: &impl Codegen<A>) {
        self.lines.push(self.arch.emit_header());

        ast.to_asm(self);

        self.lines.push(self.arch.emit_footer());
    }

    pub fn compile(&self) {
        self.write_to_file("testing.asm");

        // nasm -g -F dwarf source.asm -o source.o

        Command::new("nasm")
            .arg("-f")
            .arg("elf64") // specify the format
            .arg("-g")
            .arg("-F")
            .arg("dwarf")
            .arg("testing.asm")
            .arg("-o")
            .arg("testing.o")
            .output()
            .expect("Failed to execute NASM");

        // Link the object file to create the executable
        Command::new("ld")
            .arg("testing.o")
            .arg("-o")
            .arg("testing")
            .output()
            .expect("Failed to execute LD");
    }

    pub fn display(&self) {
        for line in &self.lines {
            println!("{}", line);
        }
    }

    fn write_to_file(&self, filename: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();

        for line in &self.lines {
            if let Err(e) = writeln!(file, "{}", line) {
                eprintln!("Couldn't write to file: {}", e);
            }
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
                addend.to_asm(asm);
                asm.emit(&asm.arch.emit_save_register("rax"));
                augend.to_asm(asm);
                asm.emit(&asm.arch.emit_restore_register("rbx"));
                asm.emit(&asm.arch.emit_add("rax", "rbx"));
                asm.emit("; End of sum expression");
            }
            Expression::Difference {
                minuend,
                subtrahend,
            } => {
                asm.emit("; Start of subtraction expression");
                subtrahend.to_asm(asm);
                asm.emit(&asm.arch.emit_save_register("rax"));
                minuend.to_asm(asm);
                asm.emit(&asm.arch.emit_restore_register("rbx"));
                asm.emit(&asm.arch.emit_sub("rax", "rbx"));
                asm.emit("; End of subtraction expression");
            }
        }
    }
}

impl<A: Arch> Codegen<A> for SyntaxTree {
    fn to_asm(&self, asm: &mut Assembler<A>) {
        match self {
            SyntaxTree::Expression(expression) => expression.to_asm(asm),
            SyntaxTree::Term(term) => term.to_asm(asm),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {}
