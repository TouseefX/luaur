use crate::records::lint_local_hygiene::LintLocalHygiene;
use luaur_ast::records::ast_name::AstName;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_table::DenseDefault;

impl DenseDefault for crate::records::global_linter_alt_c::Global {
    fn dense_default() -> Self {
        Self::default()
    }
}

impl DenseDefault for crate::records::local_linter::Local {
    fn dense_default() -> Self {
        Self::default()
    }
}

impl LintLocalHygiene {
    pub fn lint_local_hygiene() -> Self {
        LintLocalHygiene {
            context: core::ptr::null_mut(),
            locals: DenseHashMap::new(core::ptr::null_mut()),
            imports: DenseHashMap::new(AstName::new()),
            globals: DenseHashMap::new(AstName::new()),
        }
    }
}
