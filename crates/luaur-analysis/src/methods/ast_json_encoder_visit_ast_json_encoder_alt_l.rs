use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

impl AstJsonEncoder {
    pub fn visit_ast_expr_global(&mut self, node: *mut AstExprGlobal) -> bool {
        self.write_ast_expr_global(node);
        false
    }
}
