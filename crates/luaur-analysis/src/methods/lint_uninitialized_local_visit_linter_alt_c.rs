use crate::methods::lint_uninitialized_local_visit_assign::lint_uninitialized_local_visit_assign;
use crate::records::lint_uninitialized_local::LintUninitializedLocal;
use luaur_ast::records::ast_stat_function::AstStatFunction;

impl LintUninitializedLocal {
    pub fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
        unsafe {
            let node_ref = &*node;
            lint_uninitialized_local_visit_assign(self, node_ref.name);
            luaur_ast::visit::ast_expr_visit(
                node_ref.func as *mut luaur_ast::records::ast_expr::AstExpr,
                self,
            );
        }

        false
    }
}
