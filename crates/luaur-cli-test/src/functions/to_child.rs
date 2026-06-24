use crate::functions::convert_repl_requirer::{
    convert_navigation_status, luarequire_NavigateResult,
};
use crate::records::repl_requirer::ReplRequirer;
use core::ffi::{c_char, c_void, CStr};

#[allow(non_snake_case)]
pub unsafe extern "C" fn to_child(
    _l: *mut c_void,
    ctx: *mut c_void,
    name: *const c_char,
) -> luarequire_NavigateResult {
    let req = ctx as *mut ReplRequirer;
    let name_str = unsafe { CStr::from_ptr(name).to_string_lossy() };
    let status = unsafe { (*req).vfs.to_child(&name_str) };
    convert_navigation_status(status)
}
