use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_stat_return::AstStatReturn;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_return(
        &mut self,
        return_statement: *mut AstStatReturn,
    ) -> NonStrictContext {
        let return_statement_ref = unsafe { &*return_statement };
        let list = return_statement_ref.list;
        for i in 0..list.size {
            let expr = unsafe { *list.data.add(i) };
            let _ = self.visit_ast_expr_value_context(expr, ValueContext::RValue);
        }
        NonStrictContext::non_strict_context()
    }
}
