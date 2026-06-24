use core::ffi::{c_char, CStr};

use luaur_ast::records::location::Location;

pub fn report(name: *const c_char, location: &Location, r#type: &str, message: &str) {
    let name = unsafe { CStr::from_ptr(name).to_string_lossy() };

    eprintln!(
        "{}({},{}): {}: {}",
        name,
        location.begin.line + 1,
        location.begin.column + 1,
        r#type,
        message
    );
}
