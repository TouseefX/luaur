use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_assign(&mut self, assign: *mut AstStatAssign) -> NonStrictContext {
        unsafe {
            let assign_ref = &*assign;
            let vars = assign_ref.vars;
            let values = assign_ref.values;

            for i in 0..vars.size {
                let lhs = unsafe { *vars.data.add(i) };
                self.visit_ast_expr_value_context(lhs, ValueContext::LValue);
            }

            for i in 0..values.size {
                let rhs = unsafe { *values.data.add(i) };
                self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
            }
        }

        NonStrictContext::non_strict_context()
    }
}
