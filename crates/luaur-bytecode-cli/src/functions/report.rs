use core::ffi::c_char;
use core::ffi::CStr;

pub fn report(
    name: *const c_char,
    location: &luaur_ast::records::location::Location,
    r#type: *const c_char,
    message: *const c_char,
) {
    unsafe {
        let name_str = CStr::from_ptr(name).to_string_lossy();
        let type_str = CStr::from_ptr(r#type).to_string_lossy();
        let message_str = CStr::from_ptr(message).to_string_lossy();

        eprintln!(
            "{}({},{}): {}: {}",
            name_str,
            location.begin.line + 1,
            location.begin.column + 1,
            type_str,
            message_str
        );
    }
}
