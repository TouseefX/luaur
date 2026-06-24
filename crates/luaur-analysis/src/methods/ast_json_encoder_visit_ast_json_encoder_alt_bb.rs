use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric;

impl AstJsonEncoder {
    pub fn visit_ast_type_pack_generic(&mut self, node: *mut AstTypePackGeneric) -> bool {
        unsafe {
            self.write_ast_type_pack_generic(node);
        }
        false
    }
}
