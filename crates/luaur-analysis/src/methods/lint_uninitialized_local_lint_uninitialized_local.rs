use crate::records::lint_uninitialized_local::LintUninitializedLocal;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl LintUninitializedLocal {
    pub fn lint_uninitialized_local_lint_uninitialized_local() -> Self {
        LintUninitializedLocal {
            context: core::ptr::null_mut(),
            locals: DenseHashMap::new(core::ptr::null_mut()),
        }
    }
}
