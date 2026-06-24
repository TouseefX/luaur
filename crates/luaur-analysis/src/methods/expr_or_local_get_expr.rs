use crate::records::expr_or_local::ExprOrLocal;
use luaur_ast::records::ast_expr::AstExpr;

impl ExprOrLocal {
    pub fn get_expr(&self) -> *mut AstExpr {
        self.expr
    }
}
