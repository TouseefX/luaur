use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
    pub fn write_nullptr_t(&mut self, _n: core::ffi::c_void) {
        self.write_string_view("null");
    }
}
