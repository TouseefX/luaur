use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_stat_error::AstStatError;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_error(&mut self, error: *mut AstStatError) -> NonStrictContext {
        unsafe {
            let error = &*error;
            for i in 0..error.statements.size {
                let stat = *error.statements.data.add(i);
                self.visit_ast_stat(stat);
            }
            for i in 0..error.expressions.size {
                let expr = *error.expressions.data.add(i);
                self.visit_ast_expr_value_context(
                    expr,
                    crate::enums::value_context::ValueContext::RValue,
                );
            }
        }
        NonStrictContext::non_strict_context()
    }
}
