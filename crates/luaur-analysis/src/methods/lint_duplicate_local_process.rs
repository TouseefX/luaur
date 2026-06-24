use crate::records::lint_context::LintContext;
use crate::records::lint_duplicate_local::LintDuplicateLocal;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl LintDuplicateLocal {
    pub fn process(context: &mut LintContext) {
        let mut pass = LintDuplicateLocal {
            context: context as *mut LintContext,
            locals: DenseHashMap::new(core::ptr::null_mut()),
        };

        unsafe {
            let root = (*pass.context).root;
            luaur_ast::visit::ast_stat_visit(root, &mut pass);
        }
    }
}
