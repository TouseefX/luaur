use core::ffi::c_char;

use luaur_ast::records::parse_error::ParseError;

use crate::functions::report::report;

pub fn report_error_c_char_luau_parse_error(name: *const c_char, error: &ParseError) {
    // The C++ source calls report(name, error.getLocation(), "SyntaxError", error.what()).
    // We use the exact Rust identifiers for the dependencies:
    // - error.get_location() returns &Location
    // - error.what() returns &str
    // - report takes (*const c_char, &Location, *const c_char, *const c_char)

    report(
        name,
        error.get_location(),
        c"SyntaxError".as_ptr(),
        error.what().as_ptr() as *const c_char,
    );
}
