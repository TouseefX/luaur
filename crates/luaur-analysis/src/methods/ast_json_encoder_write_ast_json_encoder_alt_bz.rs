//! Source: `Analysis/src/AstJsonEncoder.cpp:1072-1082` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_typeof::AstTypeTypeof;

impl AstJsonEncoder {
    pub fn write_ast_type_typeof(&mut self, node: *mut AstTypeTypeof) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeTypeof", |e| {
            e.write("expr", &n.expr);
        });
    }
}
