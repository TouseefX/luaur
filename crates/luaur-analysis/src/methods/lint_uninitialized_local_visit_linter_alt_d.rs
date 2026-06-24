use crate::records::lint_uninitialized_local::LintUninitializedLocal;
use luaur_ast::records::ast_expr_local::AstExprLocal;

impl LintUninitializedLocal {
    pub fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
        let node_ref = unsafe { &*node };
        let local = node_ref.local;
        let local_ref = self.locals.get_or_insert(local);
        if local_ref.first_use.is_null() {
            local_ref.first_use = node;
        }
        false
    }
}
