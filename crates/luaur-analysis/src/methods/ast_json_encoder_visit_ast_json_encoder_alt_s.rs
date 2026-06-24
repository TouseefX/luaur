use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_unary::AstExprUnary;

impl AstJsonEncoder {
    pub fn visit_ast_expr_unary(&mut self, node: *mut AstExprUnary) -> bool {
        unsafe {
            self.write_ast_expr_unary(node);
        }
        false
    }
}
