use crate::records::lint_global_local::LintGlobalLocal;
use luaur_ast::records::ast_stat_for::AstStatFor;

impl LintGlobalLocal {
    pub fn visit_ast_stat_for(&mut self, node: *mut AstStatFor) -> bool {
        let reset_to_false = self.set_conditional_execution();

        unsafe {
            luaur_ast::visit::ast_expr_visit((*node).from, self);
            luaur_ast::visit::ast_expr_visit((*node).to, self);

            if !(*node).step.is_null() {
                luaur_ast::visit::ast_expr_visit((*node).step, self);
            }

            luaur_ast::visit::ast_stat_block_visit(&*(*node).body, self);
        }

        if reset_to_false {
            self.function_stack
                .last_mut()
                .unwrap()
                .conditional_execution = false;
        }

        false
    }
}
