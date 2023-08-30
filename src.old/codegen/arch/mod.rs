pub mod x86_64;

pub trait Arch {
    fn emit_header(&self) -> String;
    fn emit_footer(&self) -> String;
    fn emit_save_register(&self, reg: &str) -> String;
    fn emit_restore_register(&self, reg: &str) -> String;
    fn emit_add(&self, augend: &str, addend: &str) -> String;
    fn emit_sub(&self, minuend: &str, subtrahend: &str) -> String;
}
