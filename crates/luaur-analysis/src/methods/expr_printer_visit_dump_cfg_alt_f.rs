use crate::records::expr_printer::ExprPrinter;
use luaur_ast::functions::to_string_ast_alt_b::to_string_ast_expr_binary_op;
use luaur_ast::records::ast_expr_binary::AstExprBinary;

impl ExprPrinter {
    pub fn visit_ast_expr_binary(&mut self, node: *mut AstExprBinary) -> bool {
        unsafe {
            let n = &*node;
            self.visit_ast_expr(n.left);
            self.result
                .push_str(&format!(" {} ", to_string_ast_expr_binary_op(n.op)));
            self.visit_ast_expr(n.right);
        }
        false
    }
}
