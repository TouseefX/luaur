use crate::records::global_linter::Global;
use alloc::vec::Vec;
use luaur_ast::records::ast_name::AstName;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub fn get_deprecated_globals(names: &DenseHashMap<AstName, Global>) -> Vec<AstName> {
    let _ = names;
    Vec::new()
}
