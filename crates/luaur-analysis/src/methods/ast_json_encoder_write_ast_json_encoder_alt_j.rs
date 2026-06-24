//! Node: `cxx:Method:Luau.Analysis:Analysis/src/AstJsonEncoder.cpp:180:ast_json_encoder_write`
//! Source: `Analysis/src/AstJsonEncoder.cpp` (AstJsonEncoder.cpp:180-183, hand-ported)

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
    pub fn write_long_long(&mut self, i: u64) {
        self.write_raw_string_view(&i.to_string());
    }
}
