use crate::records::expr_printer::ExprPrinter;
use luaur_ast::records::ast_expr::AstExpr;

impl ExprPrinter {
    pub fn visit_ast_expr(&mut self, _node: *mut AstExpr) -> bool {
        self.result.push_str("<expr>");
        false
    }
}
