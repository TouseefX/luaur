use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;

impl TypeChecker2 {
    pub fn visit_ast_stat_repeat(&mut self, repeat_statement: *mut AstStatRepeat) {
        unsafe {
            let body = (*repeat_statement).body;
            self.visit_ast_stat_block(body);
            let condition = (*repeat_statement).condition;
            self.visit_ast_expr_value_context(
                condition,
                crate::enums::value_context::ValueContext::RValue,
            );
        }
    }
}
