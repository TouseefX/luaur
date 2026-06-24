use crate::records::find_expr_or_local::FindExprOrLocal;
use luaur_ast::records::ast_expr::AstExpr;

impl FindExprOrLocal {
    pub fn visit_ast_expr(&mut self, expr: *mut AstExpr) -> bool {
        let location = unsafe { (*expr).base.location };
        if self.is_closer_match(location) {
            self.result.set_expr(expr);
            return true;
        }
        false
    }
}
