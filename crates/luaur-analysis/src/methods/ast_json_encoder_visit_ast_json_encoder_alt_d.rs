use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_group::AstExprGroup;

impl AstJsonEncoder {
    pub fn visit_ast_expr_group(&mut self, node: *mut AstExprGroup) -> bool {
        self.write_ast_expr_group(node);
        false
    }
}
