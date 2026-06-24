use crate::records::lint_uninitialized_local::LintUninitializedLocal;
use luaur_ast::records::ast_stat_local::AstStatLocal;

impl LintUninitializedLocal {
    pub fn visit_ast_stat_local(&mut self, node: *mut AstStatLocal) -> bool {
        let node_ref = unsafe { &*node };
        let last = if node_ref.values.size > 0 {
            unsafe { *node_ref.values.data.add(node_ref.values.size - 1) }
        } else {
            core::ptr::null_mut()
        };
        let vararg = !last.is_null()
            && (luaur_ast::rtti::ast_node_is::<luaur_ast::records::ast_expr_varargs::AstExprVarargs>(
                unsafe { &*(last as *mut luaur_ast::records::ast_node::AstNode) },
            ) || luaur_ast::rtti::ast_node_is::<luaur_ast::records::ast_expr_call::AstExprCall>(
                unsafe { &*(last as *mut luaur_ast::records::ast_node::AstNode) },
            ));

        for i in 0..node_ref.vars.size {
            let var = unsafe { *node_ref.vars.data.add(i) };
            let l = self.locals.get_or_insert(var);
            l.defined = true;
            l.initialized = vararg || i < node_ref.values.size;
        }
        true
    }
}
