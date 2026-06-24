//! Source: `Analysis/src/AstJsonEncoder.cpp:484-494` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_generic_type::AstGenericType;

impl AstJsonEncoder {
    pub fn write_ast_generic_type(&mut self, generic_type: *mut AstGenericType) {
        let g = unsafe { &*generic_type };
        self.write_raw_string_view("{");
        let c = self.push_comma();
        self.write_type_string_view("AstGenericType");
        self.write("name", &g.name);
        if !g.default_value.is_null() {
            self.write("luauType", &g.default_value);
        }
        self.pop_comma(c);
        self.write_raw_string_view("}");
    }
}
