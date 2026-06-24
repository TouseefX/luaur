//! Source: `Analysis/src/AstJsonEncoder.cpp:1459-1463` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_type_reference::AstTypeReference;

impl AstJsonEncoder {
    pub fn visit_ast_type_reference(&mut self, node: *mut AstTypeReference) -> bool {
        self.write_ast_type_reference(node);
        false
    }
}
