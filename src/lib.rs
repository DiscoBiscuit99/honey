pub mod analysis;
pub mod structures;

pub mod prelude {
    pub use crate::analysis::{lexical::lex, syntactic::parse, semantic::validate};
}
