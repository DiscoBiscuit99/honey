use crate::syntax::SyntaxTree;

use super::errors::ParsingError;

mod syntax;

pub fn squash(ast: &SyntaxTree) -> Result<SyntaxTree, ParsingError> {
    todo!()
}

pub fn squash_term(term: &super::Term) -> syntax::SparseTerm {
    todo!()
}
