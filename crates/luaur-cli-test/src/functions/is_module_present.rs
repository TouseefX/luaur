use crate::records::repl_requirer::ReplRequirer;
use core::ffi::c_void;
use luaur_cli_lib::functions::is_file::is_file;

#[allow(non_snake_case)]
pub unsafe extern "C" fn is_module_present(_l: *mut c_void, ctx: *mut c_void) -> bool {
    let req = ctx as *mut ReplRequirer;
    let path = unsafe { (*req).vfs.get_file_path() };
    is_file(&path)
}
