use crate::records::lint_local_hygiene::LintLocalHygiene;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

impl LintLocalHygiene {
    pub fn visit_ast_stat_assign(&mut self, node: *mut AstStatAssign) -> bool {
        let vars = unsafe { (*node).vars };
        for i in 0..vars.size {
            let var = unsafe { *vars.data.add(i) };

            if unsafe {
                !luaur_ast::rtti::ast_node_is::<luaur_ast::records::ast_expr_local::AstExprLocal>(
                    &(*var).base,
                )
            } {
                unsafe {
                    luaur_ast::visit::ast_expr_visit(var, self);
                }
            }
        }

        let values = unsafe { (*node).values };
        for i in 0..values.size {
            unsafe {
                luaur_ast::visit::ast_expr_visit(*values.data.add(i), self);
            }
        }

        false
    }
}
