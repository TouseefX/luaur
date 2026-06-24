use core::fmt::Write;
use luaur_ast::records::ast_name::AstName;

pub fn operator_lt_ostream_ast_name(stream: &mut dyn Write, name: &AstName) -> core::fmt::Result {
    if !name.value.is_null() {
        let s = unsafe { core::ffi::CStr::from_ptr(name.value) };
        write!(stream, "{}", s.to_string_lossy())
    } else {
        write!(stream, "<empty>")
    }
}
