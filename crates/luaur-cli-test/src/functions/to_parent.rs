use crate::functions::convert_repl_requirer::convert_navigation_status;
use crate::records::repl_requirer::ReplRequirer;
use core::ffi::c_void;

#[allow(non_snake_case)]
pub unsafe extern "C" fn to_parent(
    _l: *mut c_void,
    ctx: *mut c_void,
) -> crate::functions::convert_repl_requirer::luarequire_NavigateResult {
    let req = ctx as *mut ReplRequirer;
    let status = unsafe { (*req).vfs.to_parent() };
    convert_navigation_status(status)
}
