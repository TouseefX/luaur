use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_binary::AstExprBinary;

impl AstJsonEncoder {
    pub fn visit_ast_expr_binary(&mut self, node: *mut AstExprBinary) -> bool {
        self.write_ast_expr_binary(node);
        false
    }
}
