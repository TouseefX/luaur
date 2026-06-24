use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_error::AstExprError;

impl AstJsonEncoder {
    pub fn visit_ast_expr_error(&mut self, node: *mut AstExprError) -> bool {
        unsafe {
            self.write_ast_expr_error(node);
        }
        false
    }
}
