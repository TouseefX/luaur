use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_constant_nil::AstExprConstantNil;

impl AstJsonEncoder {
    pub fn write_ast_expr_constant_nil(&mut self, node: *mut AstExprConstantNil) {
        self.write_node_ast_node_string_view_f(
            node as *mut luaur_ast::records::ast_node::AstNode,
            "AstExprConstantNil",
            |_| {},
        );
    }
}
