use crate::functions::convert_repl_requirer::{
    convert_navigation_status, luarequire_NavigateResult,
};
use crate::records::repl_requirer::ReplRequirer;
use core::ffi::{c_char, c_void, CStr};
use luaur_cli_lib::functions::is_absolute_path::is_absolute_path;

#[allow(non_snake_case)]
pub unsafe extern "C" fn jump_to_alias(
    _l: *mut c_void,
    ctx: *mut c_void,
    path: *const c_char,
) -> luarequire_NavigateResult {
    let req = ctx as *mut ReplRequirer;
    let path_str = unsafe { CStr::from_ptr(path).to_string_lossy() };

    if !is_absolute_path(&path_str) {
        return luarequire_NavigateResult::NAVIGATE_NOT_FOUND;
    }

    let status = unsafe { (*req).vfs.reset_to_path(&path_str) };
    convert_navigation_status(status)
}
