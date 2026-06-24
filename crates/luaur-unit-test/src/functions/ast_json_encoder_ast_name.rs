use core::ffi::CStr;
use luaur_ast::records::ast_name::AstName;

pub fn ast_name(value: &'static CStr) -> AstName {
    AstName {
        value: value.as_ptr(),
    }
}
