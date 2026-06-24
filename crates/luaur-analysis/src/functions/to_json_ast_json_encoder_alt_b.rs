//! Source: `Analysis/src/AstJsonEncoder.cpp:1562-1570` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::comment::Comment;

pub fn to_json(
    node: *mut AstNode,
    comment_locations: alloc::vec::Vec<Comment>,
) -> alloc::string::String {
    let mut encoder = AstJsonEncoder::ast_json_encoder_ast_json_encoder();
    encoder.write_raw_string_view("{\"root\":");
    unsafe {
        luaur_ast::visit::ast_node_visit(node, &mut encoder);
    }
    encoder.write_raw_string_view(",\"commentLocations\":[");
    encoder.write_comments(comment_locations);
    encoder.write_raw_string_view("]}");
    encoder.str()
}
