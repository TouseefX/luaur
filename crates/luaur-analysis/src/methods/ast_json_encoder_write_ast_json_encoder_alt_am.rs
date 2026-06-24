//! Source: `Analysis/src/AstJsonEncoder.cpp:496-506` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_generic_type_pack::AstGenericTypePack;

impl AstJsonEncoder {
    pub fn write_ast_generic_type_pack(&mut self, generic_type_pack: *mut AstGenericTypePack) {
        let g = unsafe { &*generic_type_pack };
        self.write_raw_string_view("{");
        let c = self.push_comma();
        self.write_type_string_view("AstGenericTypePack");
        self.write("name", &g.name);
        if !g.default_value.is_null() {
            self.write("luauType", &g.default_value);
        }
        self.pop_comma(c);
        self.write_raw_string_view("}");
    }
}
