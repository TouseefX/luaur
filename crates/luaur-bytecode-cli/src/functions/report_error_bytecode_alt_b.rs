use core::ffi::c_char;

use luaur_compiler::records::compile_error::CompileError;

use crate::functions::report::report;

pub fn report_error_c_char_luau_compile_error(name: *const c_char, error: &CompileError) {
    // The C++ source calls report(name, error.getLocation(), "CompileError", error.what()).
    // The `report` function in this crate is defined with 4 arguments:
    // (name: *const c_char, location: &Location, type: *const c_char, message: *const c_char).

    report(
        name,
        error.get_location(),
        c"CompileError".as_ptr(),
        error.what(),
    );
}
