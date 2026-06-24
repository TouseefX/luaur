//! Source: `Analysis/src/AstJsonEncoder.cpp:1555-1560` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;

pub fn to_json(node: *mut AstNode) -> alloc::string::String {
    let mut encoder = AstJsonEncoder::ast_json_encoder_ast_json_encoder();
    unsafe {
        luaur_ast::visit::ast_node_visit(node, &mut encoder);
    }
    encoder.str()
}
