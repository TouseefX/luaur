use crate::records::ast_json_encoder::AstJsonEncoder;
use core::ffi::c_char;
use luaur_ast::records::ast_array::AstArray;

impl AstJsonEncoder {
    pub fn write_ast_array_c_char(&mut self, arr: AstArray<c_char>) {
        let slice = unsafe { core::slice::from_raw_parts(arr.data as *const u8, arr.size) };
        if let Ok(s) = core::str::from_utf8(slice) {
            self.write_string_view(s);
        }
    }
}
