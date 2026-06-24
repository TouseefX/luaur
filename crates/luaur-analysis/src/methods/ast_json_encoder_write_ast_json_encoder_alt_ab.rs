//! Source: `Analysis/src/AstJsonEncoder.cpp:353-363` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_global(&mut self, node: *mut AstExprGlobal) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprGlobal", |e| {
            e.write("global", &n.name);
        });
    }
}
