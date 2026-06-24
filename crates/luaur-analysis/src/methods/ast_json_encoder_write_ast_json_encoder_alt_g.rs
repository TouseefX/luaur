use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
    pub fn write_long_long_mut(&mut self, i: i64) {
        self.write_raw_string_view(&i.to_string());
    }
}
