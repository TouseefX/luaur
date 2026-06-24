use alloc::string::String;
use luaur_analysis::functions::to_json_ast_json_encoder::to_json as encode_to_json;
use luaur_ast::records::ast_node::AstNode;

pub fn json<T>(node: *mut T) -> String {
    encode_to_json(node as *mut AstNode)
}
