use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
    pub fn str(&mut self) -> alloc::string::String {
        self.chunks.join("")
    }
}
