//! Source: `Analysis/src/AstJsonEncoder.cpp:271-274` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    // C++ write(AstNode* node) { node->visit(this); } -- virtual dispatch on
    // the node's dynamic type, landing in this encoder's visit overrides.
    pub fn write_ast_node(&mut self, node: *mut AstNode) {
        unsafe {
            luaur_ast::visit::ast_node_visit(node, self);
        }
    }
}
