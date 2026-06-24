use crate::records::expr_printer::ExprPrinter;
use luaur_ast::functions::to_string_ast::to_string_ast_expr_unary_op;
use luaur_ast::records::ast_expr_unary::AstExprUnary;

impl ExprPrinter {
    pub fn visit_ast_expr_unary(&mut self, node: *mut AstExprUnary) -> bool {
        unsafe {
            let n = &*node;
            self.result.push_str(&to_string_ast_expr_unary_op(n.op));
            self.visit_ast_expr(n.expr);
        }
        false
    }
}
