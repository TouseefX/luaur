//! Source: `Analysis/src/AstJsonEncoder.cpp:237-252` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_local::AstLocal;

impl AstJsonEncoder {
    pub fn write_ast_local(&mut self, local: *mut AstLocal) {
        let l = unsafe { &*local };
        self.write_raw_string_view("{");
        let c = self.push_comma();
        if !l.annotation.is_null() {
            self.write("luauType", &l.annotation);
        } else {
            // C++ write("luauType", nullptr)
            if self.comma {
                self.write_raw_string_view(",");
            }
            self.comma = true;
            self.write_raw_string_view("\"luauType\":");
            self.write_raw_string_view("null");
        }
        self.write("name", &l.name);
        if luaur_common::FFlag::LuauConst2.get() {
            self.write("isConst", &l.is_const);
        }
        self.write_type_string_view("AstLocal");
        self.write("location", &l.location);
        self.pop_comma(c);
        self.write_raw_string_view("}");
    }
}
