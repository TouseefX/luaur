use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic;

impl AstJsonEncoder {
    pub fn visit_ast_type_pack_variadic(&mut self, node: *mut AstTypePackVariadic) -> bool {
        self.write_ast_type_pack_variadic(node);
        false
    }
}
