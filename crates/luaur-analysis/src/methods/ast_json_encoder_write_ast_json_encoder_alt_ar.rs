//! Source: `Analysis/src/AstJsonEncoder.cpp:570-580` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_table(&mut self, node: *mut AstExprTable) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprTable", |e| {
            e.write("items", &n.items);
        });
    }
}
