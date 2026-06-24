use crate::records::expr_printer::ExprPrinter;
use luaur_ast::records::ast_expr_constant_nil::AstExprConstantNil;

impl ExprPrinter {
    pub fn visit_ast_expr_constant_nil(&mut self, _node: *mut AstExprConstantNil) -> bool {
        self.result.push_str("nil");
        false
    }
}
