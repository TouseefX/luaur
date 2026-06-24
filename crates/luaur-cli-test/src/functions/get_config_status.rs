use crate::functions::convert_repl_requirer_alt_b::{
    convert_vfs_navigator_config_status, luarequire_ConfigStatus,
};
use crate::records::repl_requirer::ReplRequirer;
use core::ffi::c_void;

#[allow(non_snake_case)]
pub unsafe extern "C" fn get_config_status(
    _l: *mut c_void,
    ctx: *mut c_void,
) -> luarequire_ConfigStatus {
    let req = ctx as *mut ReplRequirer;
    let status = unsafe { (*req).vfs.get_config_status() };
    convert_vfs_navigator_config_status(status)
}
