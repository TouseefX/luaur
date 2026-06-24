use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;

impl AstJsonEncoder {
    pub fn visit_ast_expr_constant_string(&mut self, node: *mut AstExprConstantString) -> bool {
        self.write_ast_expr_constant_string(node);
        false
    }
}
