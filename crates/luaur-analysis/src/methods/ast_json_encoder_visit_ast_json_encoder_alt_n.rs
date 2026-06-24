use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_call::AstExprCall;

impl AstJsonEncoder {
    pub fn visit_ast_expr_call(&mut self, node: *mut AstExprCall) -> bool {
        self.write_ast_expr_call(node);
        false
    }
}
