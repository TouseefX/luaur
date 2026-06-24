use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
    pub fn ast_json_encoder_ast_json_encoder() -> Self {
        let mut encoder = AstJsonEncoder {
            chunks: alloc::vec::Vec::new(),
            comma: false,
        };
        encoder.new_chunk();
        encoder
    }
}
