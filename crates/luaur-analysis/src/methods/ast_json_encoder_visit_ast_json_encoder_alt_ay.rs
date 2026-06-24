//! Source: `Analysis/src/AstJsonEncoder.cpp:1495-1499` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_pack::AstTypePack;

impl AstJsonEncoder {
    pub fn visit_ast_type_pack(&mut self, node: *mut AstTypePack) -> bool {
        self.write_ast_node(node as *mut AstNode);
        false
    }
}
