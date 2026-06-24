use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
    pub fn write_long(&mut self, i: u64) {
        self.write_raw_string_view(&i.to_string());
    }
}
