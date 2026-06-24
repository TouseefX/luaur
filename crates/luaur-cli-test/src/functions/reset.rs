use crate::functions::convert_repl_requirer::{
    convert_navigation_status, luarequire_NavigateResult,
};
use crate::records::repl_requirer::ReplRequirer;
use core::ffi::{c_char, c_void, CStr};
use luaur_cli_lib::methods::vfs_navigator_reset_to_std_in::vfs_navigator_reset_to_std_in;

#[allow(non_snake_case)]
pub unsafe extern "C" fn reset(
    _l: *mut c_void,
    ctx: *mut c_void,
    requirer_chunkname: *const c_char,
) -> luarequire_NavigateResult {
    let req = ctx as *mut ReplRequirer;
    let chunkname = CStr::from_ptr(requirer_chunkname).to_string_lossy();

    if chunkname == "=stdin" {
        convert_navigation_status(vfs_navigator_reset_to_std_in(&mut (*req).vfs))
    } else if !chunkname.is_empty() && chunkname.as_bytes()[0] == b'@' {
        convert_navigation_status((*req).vfs.reset_to_path(&chunkname[1..]))
    } else {
        luarequire_NavigateResult::NAVIGATE_NOT_FOUND
    }
}
