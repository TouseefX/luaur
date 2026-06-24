use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_node_ast_node(&mut self, node: *mut AstNode) {
        unsafe {
            self.write("location", &(*node).location);
        }
    }
}
