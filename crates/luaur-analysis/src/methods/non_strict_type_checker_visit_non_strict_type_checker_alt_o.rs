use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_stat_function::AstStatFunction;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_function(&mut self, stat_fn: *mut AstStatFunction) -> NonStrictContext {
        let func = unsafe { (*stat_fn).func };
        self.visit_ast_expr_function(func)
    }
}
