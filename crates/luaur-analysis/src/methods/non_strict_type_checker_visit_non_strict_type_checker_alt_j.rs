use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_stat_local::AstStatLocal;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_local(&mut self, local: *mut AstStatLocal) -> NonStrictContext {
        let local_ref = unsafe { &*local };
        let values = local_ref.values;
        for i in 0..values.size {
            let rhs = unsafe { *values.data.add(i) };
            self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
        }
        NonStrictContext::non_strict_context()
    }
}
