use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

impl AstJsonEncoder {
    pub fn visit_ast_expr_index_name(&mut self, node: *mut AstExprIndexName) -> bool {
        self.write_ast_expr_index_name(node);
        false
    }
}
