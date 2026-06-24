use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_for_in(
        &mut self,
        for_in_statement: *mut AstStatForIn,
    ) -> NonStrictContext {
        let for_in_ref = unsafe { &*for_in_statement };

        // Visit variable annotations
        let vars = &for_in_ref.vars;
        for i in 0..vars.size {
            let var = unsafe { *vars.data.add(i) };
            let annotation = unsafe { (*var).annotation };
            if !annotation.is_null() {
                self.visit_ast_type(annotation);
            }
        }

        // Visit value expressions
        let values = &for_in_ref.values;
        for i in 0..values.size {
            let rhs = unsafe { *values.data.add(i) };
            self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
        }

        // Visit body
        self.visit_ast_stat(for_in_ref.body as *mut AstStat)
    }
}
