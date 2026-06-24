use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_stat_while::AstStatWhile;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_while(&mut self, while_statement: *mut AstStatWhile) -> NonStrictContext {
        unsafe {
            let condition = (*while_statement).condition;
            let condition_context = self.visit_ast_expr_value_context(
                condition,
                crate::enums::value_context::ValueContext::RValue,
            );
            let body = (*while_statement).body;
            self.visit_ast_stat_block(body);
            let body_context = NonStrictContext::non_strict_context();
            NonStrictContext::disjunction(
                self.builtin_types,
                self.arena,
                &condition_context,
                &body_context,
            )
        }
    }
}
