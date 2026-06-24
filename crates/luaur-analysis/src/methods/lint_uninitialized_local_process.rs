use crate::methods::lint_uninitialized_local_report::lint_uninitialized_local_report;
use crate::records::lint_context::LintContext;
use crate::records::lint_uninitialized_local::LintUninitializedLocal;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl LintUninitializedLocal {
    pub fn process(context: &mut LintContext) {
        let mut pass = LintUninitializedLocal {
            context: context as *mut LintContext,
            locals: DenseHashMap::new(core::ptr::null_mut()),
        };

        unsafe {
            let root = (*pass.context).root;
            luaur_ast::visit::ast_stat_visit(root, &mut pass);
        }

        lint_uninitialized_local_report(&mut pass);
    }
}
