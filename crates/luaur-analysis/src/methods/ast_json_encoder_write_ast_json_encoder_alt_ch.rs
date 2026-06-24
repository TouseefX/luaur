//! Source: `Analysis/src/AstJsonEncoder.cpp:1162-1172` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_attr::AstAttr;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_attr(&mut self, node: *mut AstAttr) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstAttr", |e| {
            e.write("name", &n.name);
        });
    }
}
