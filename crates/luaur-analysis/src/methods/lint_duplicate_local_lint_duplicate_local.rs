use crate::records::lint_duplicate_local::LintDuplicateLocal;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl LintDuplicateLocal {
    pub fn lint_duplicate_local() -> Self {
        LintDuplicateLocal {
            context: core::ptr::null_mut(),
            locals: DenseHashMap::new(core::ptr::null_mut()),
        }
    }
}
