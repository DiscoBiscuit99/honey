use crate::codegen::arch::Arch;

pub struct X86_64;

impl Arch for X86_64 {
    fn emit_header(&self) -> String {
        ["; Header", "section .text", "global _start", "_start:"].join("\n")
    }

    fn emit_footer(&mut self) -> String {
        ["; Footer", "mov rax, 60", "xor rdi, rdi", "syscall"].join("\n")
    }

    fn emit_add(&mut self, dest: &str, src1: &str, src2: &str) -> String {
        [
            format!("mov {}, {}", dest, src1),
            format!("add {}, {}", dest, src2),
        ]
        .join("\n")
    }
}
