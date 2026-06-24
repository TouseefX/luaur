use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
    pub fn write_c_char(&mut self, c: core::ffi::c_char) {
        let buf = [c as u8];
        self.write_string(unsafe { core::str::from_utf8_unchecked(&buf) });
    }
}
