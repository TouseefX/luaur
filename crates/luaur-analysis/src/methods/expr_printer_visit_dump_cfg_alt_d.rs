use crate::records::expr_printer::ExprPrinter;
use luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool;

impl ExprPrinter {
    pub fn visit_ast_expr_constant_bool(&mut self, node: *mut AstExprConstantBool) -> bool {
        unsafe {
            self.result
                .push_str(if (*node).value { "true" } else { "false" });
        }
        false
    }
}
