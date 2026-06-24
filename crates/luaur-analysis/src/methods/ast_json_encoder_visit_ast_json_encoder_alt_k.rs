use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_local::AstExprLocal;

impl AstJsonEncoder {
    pub fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
        self.write_ast_expr_local(node);
        false
    }
}
