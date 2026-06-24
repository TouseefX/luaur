use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
    pub fn write_long_mut(&mut self, i: i64) {
        let s = i.to_string();
        self.write_raw_string_view(&s);
    }
}
