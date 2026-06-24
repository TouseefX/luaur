use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_type_union::AstTypeUnion;

impl AstJsonEncoder {
    pub fn visit_ast_type_union(&mut self, node: *mut AstTypeUnion) -> bool {
        unsafe {
            self.write_ast_type_union(node);
        }
        false
    }
}
