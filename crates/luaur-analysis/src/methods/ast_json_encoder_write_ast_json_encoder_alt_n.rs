use crate::records::ast_json_encoder::AstJsonEncoder;
use core::ffi::CStr;
use luaur_ast::records::ast_name::AstName;

impl AstJsonEncoder {
    pub fn write_ast_name(&mut self, name: AstName) {
        let s = if !name.value.is_null() {
            unsafe { CStr::from_ptr(name.value).to_string_lossy() }
        } else {
            "".into()
        };
        self.write_string(&s);
    }
}
