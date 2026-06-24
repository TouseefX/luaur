use crate::records::expr_or_local::ExprOrLocal;
use luaur_ast::records::ast_expr::AstExpr;

impl ExprOrLocal {
    pub fn set_expr(&mut self, new_expr: *mut AstExpr) {
        self.expr = new_expr;
        self.local = core::ptr::null_mut();
    }
}
