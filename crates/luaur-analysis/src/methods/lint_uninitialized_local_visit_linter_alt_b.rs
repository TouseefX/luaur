use crate::methods::lint_uninitialized_local_visit_assign::lint_uninitialized_local_visit_assign;
use crate::records::lint_uninitialized_local::LintUninitializedLocal;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

impl LintUninitializedLocal {
    pub fn visit_ast_stat_assign(&mut self, node: *mut AstStatAssign) -> bool {
        unsafe {
            let node_ref = &*node;

            for i in 0..node_ref.vars.size {
                let var = *node_ref.vars.data.add(i);
                lint_uninitialized_local_visit_assign(self, var);
            }

            for i in 0..node_ref.values.size {
                let value = *node_ref.values.data.add(i);
                luaur_ast::visit::ast_expr_visit(value, self);
            }
        }

        false
    }
}
