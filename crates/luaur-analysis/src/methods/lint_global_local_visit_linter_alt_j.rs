use crate::records::lint_global_local::LintGlobalLocal;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;

impl LintGlobalLocal {
    pub fn visit_ast_stat_for_in(&mut self, node: *mut AstStatForIn) -> bool {
        let reset_to_false = self.set_conditional_execution();

        let values = unsafe { (*node).values };
        for i in 0..values.size {
            unsafe {
                luaur_ast::visit::ast_expr_visit(*values.data.add(i), self);
            }
        }

        unsafe {
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
