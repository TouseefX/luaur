use crate::macros::prop::prop;
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;

impl AstJsonEncoder {
    pub fn visit_ast_type_pack_explicit(&mut self, node: *mut AstTypePackExplicit) -> bool {
        self.write_ast_type_pack_explicit(node);
        false
    }
}
