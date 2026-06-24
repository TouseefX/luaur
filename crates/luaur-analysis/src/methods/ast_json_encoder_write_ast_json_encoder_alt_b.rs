use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
    pub fn write_bool(&mut self, b: bool) {
        if b {
            self.write_raw_string_view("true");
        } else {
            self.write_raw_string_view("false");
        }
    }
}
