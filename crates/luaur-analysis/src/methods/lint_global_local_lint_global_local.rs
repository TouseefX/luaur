use crate::records::lint_global_local::LintGlobalLocal;
use alloc::vec::Vec;
use luaur_ast::records::ast_name::AstName;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_table::DenseDefault;

impl DenseDefault for crate::records::global_linter_alt_b::Global {
    fn dense_default() -> Self {
        Self::default()
    }
}

impl LintGlobalLocal {
    pub fn lint_global_local() -> Self {
        Self {
            context: core::ptr::null_mut(),
            globals: DenseHashMap::new(AstName::new()),
            global_refs: Vec::new(),
            function_stack: Vec::new(),
        }
    }
}
