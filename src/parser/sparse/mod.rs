use crate::syntax::ParseTree;

use super::errors::ParsingError;

mod syntax;

pub fn squash(ast: &ParseTree) -> Result<ParseTree, ParsingError> {
    todo!()
}

pub fn squash_term(term: &super::Term) -> syntax::SparseTerm {
    todo!()
}
