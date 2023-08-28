pub mod x86_64;

pub trait Arch {
    fn emit_header(&self) -> String;

    fn emit_footer(&mut self) -> String;

    fn emit_add(&mut self, dest: &str, src1: &str, src2: &str) -> String;
}
