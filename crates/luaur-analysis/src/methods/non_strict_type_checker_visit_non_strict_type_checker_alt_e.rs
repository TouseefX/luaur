use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_repeat(
        &mut self,
        repeat_statement: *mut AstStatRepeat,
    ) -> NonStrictContext {
        unsafe {
            let body = (*repeat_statement).body;
            let body_context = self.visit_ast_stat_block(body);
            let condition = (*repeat_statement).condition;
            let condition_context = self.visit_ast_expr_value_context(
                condition,
                crate::enums::value_context::ValueContext::RValue,
            );
            NonStrictContext::disjunction(
                self.builtin_types,
                self.arena,
                &body_context,
                &condition_context,
            )
        }
    }
}
