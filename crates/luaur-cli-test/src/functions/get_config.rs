use crate::functions::write::write;
use crate::records::repl_requirer::ReplRequirer;
use core::ffi::{c_char, c_void};
use luaur_require::enums::luarequire_write_result::luarequire_WriteResult;

#[allow(non_snake_case)]
pub unsafe extern "C" fn get_config(
    _l: *mut c_void,
    ctx: *mut c_void,
    buffer: *mut c_char,
    buffer_size: usize,
    size_out: *mut usize,
) -> luarequire_WriteResult {
    let req = ctx as *mut ReplRequirer;
    let config = unsafe { (*req).vfs.get_config() };
    write(config, buffer, buffer_size, size_out)
}
