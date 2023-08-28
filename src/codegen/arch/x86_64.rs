use crate::codegen::arch::Arch;

pub struct X86_64;

impl Arch for X86_64 {
    fn emit_header(&self) -> String {
        ["; Header", "section .text", "global _start", "_start:"].join("\n")
    }

    fn emit_footer(&self) -> String {
        ["; Footer", "mov rax, 60", "xor rdi, rdi", "syscall"].join("\n")
    }

    fn emit_save_register(&self, reg: &str) -> String {
        format!("push {}", reg)
    }

    fn emit_restore_register(&self, reg: &str) -> String {
        format!("pop {}", reg)
    }

    fn emit_add(&self, augend: &str, addend: &str) -> String {
        format!("add {}, {}", augend, addend)
    }

    fn emit_sub(&self, minuend: &str, subtrahend: &str) -> String {
        format!("sub {}, {}", minuend, subtrahend)
    }
}
