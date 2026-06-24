use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_common::functions::format::format;

impl AstJsonEncoder {
    pub fn write_string(&mut self, sv: &str) {
        self.write_raw_string_view("\"");

        for c in sv.chars() {
            let c_char = c as core::ffi::c_char;
            if c == '"' {
                self.write_raw_string_view("\\\"");
            } else if c == '\\' {
                self.write_raw_string_view("\\\\");
            } else if c < ' ' {
                let formatted = format(format_args!("\\u{:04x}", c as u32));
                self.write_raw_string_view(&formatted);
            } else if c == '\n' {
                self.write_raw_string_view("\\n");
            } else {
                self.write_raw_c_char(c_char);
            }
        }

        self.write_raw_string_view("\"");
    }
}
