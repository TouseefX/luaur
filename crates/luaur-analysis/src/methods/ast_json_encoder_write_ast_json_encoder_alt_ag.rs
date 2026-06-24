//! Source: `Analysis/src/AstJsonEncoder.cpp:409-422` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_index_name(&mut self, node: *mut AstExprIndexName) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprIndexName", |e| {
            e.write("expr", &n.expr);
            e.write("index", &n.index);
            e.write("indexLocation", &n.index_location);
            // write("op", node->op) -- C++ char overload: a one-char string
            if e.comma {
                e.write_raw_string_view(",");
            }
            e.comma = true;
            e.write_raw_string_view("\"op\":");
            e.write_c_char(n.op);
        });
    }
}
