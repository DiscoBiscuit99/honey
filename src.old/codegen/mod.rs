use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;

use crate::syntax::*;

pub struct Assembler {
    arch: A,
    lines: Vec<String>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler { lines: Vec::new() }
    }

    pub fn emit_header(&mut self) {
        self.emit("; Header");
        self.emit("section .text");
        self.emit("global _start");
        self.emit("_start:");
        // ... and so on
    }

    pub fn emit_footer(&mut self) {
        self.emit("; Footer");
        self.emit("mov rax, 60"); // return value
        self.emit("xor rdi, rdi"); // sys_exit system call number
        self.emit("syscall"); // call kernel
    }

    pub fn emit_function_prologue(&mut self, name: &str) {
        self.emit(&format!("{}:", name));
        // ... set up stack frame etc.
    }

    pub fn emit_function_epilogue(&mut self) {
        // ... tear down stack frame etc.
    }

    pub fn emit(&mut self, asm: &str) {
        self.lines.push(asm.to_string());
    }

    pub fn write_to_file(&self, filename: &str) {
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

    pub fn compile(&self) {
        Command::new("nasm")
            .arg("-f")
            .arg("elf64") // specify the format
            .arg("output.asm")
            .output()
            .expect("Failed to execute NASM");

        // Link the object file to create the executable
        Command::new("ld")
            .arg("-o")
            .arg("output")
            .arg("output.o")
            .output()
            .expect("Failed to execute LD");
    }

    pub fn ast_to_asm(&mut self, ast: &SyntaxTree) {
        self.emit(&ast.to_asm());
    }
}

impl SyntaxTree {
    pub fn to_asm(&self) -> String {
        match self {
            SyntaxTree::Expression(expression) => expression.to_asm(),
            _ => unimplemented!(),
        }
    }
}

impl Literal {
    pub fn to_asm(&self) -> String {
        match self {
            Literal::Number(number) => match number {
                Number::Int(i) => format!("mov rax, {}", i),
                Number::Float(f) => unimplemented!(),
            },
        }
    }
}

impl Factor {
    pub fn to_asm(&self) -> String {
        match self {
            Factor::Literal(literal) => literal.to_asm(),
            _ => unimplemented!(),
        }
    }
}

impl Term {
    pub fn to_asm(&self) -> String {
        match self {
            Term::Factor(factor) => factor.to_asm(),
            _ => unimplemented!(),
        }
    }
}

impl Expression {
    pub fn to_asm(&self) -> String {
        match self {
            Expression::Term(term) => term.to_asm(),
            Expression::Sum { augend, addend } => {
                let augend_asm = augend.to_asm();
                let addend_asm = addend.to_asm();
                [
                    "; Start of sum expression",
                    &augend_asm,
                    "push rax", // Save the result of the left-hand expression
                    &addend_asm,
                    "pop rbx",      // Pop the left-hand result into rbx
                    "add rax, rbx", // Perform the addition
                    "; End of sum expression",
                ]
                .join("\n")
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::{Literal, Number};

    #[test]
    fn literal_to_asm() {
        // Arrange
        let literal = Literal::Number(Number::Int(3));
        let expected = "mov rax, 3".to_string();

        // Act
        let asm = literal.to_asm();

        // Assert
        assert_eq!(asm, expected);
    }
}
