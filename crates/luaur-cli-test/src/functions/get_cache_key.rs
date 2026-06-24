use crate::functions::write::write;
use crate::records::repl_requirer::ReplRequirer;
use alloc::string::String;
use core::ffi::{c_char, c_void};
use luaur_require::enums::luarequire_write_result::luarequire_WriteResult;

#[allow(non_snake_case)]
pub unsafe extern "C" fn get_cache_key(
    _l: *mut core::ffi::c_void,
    ctx: *mut c_void,
    buffer: *mut c_char,
    buffer_size: usize,
    size_out: *mut usize,
) -> luarequire_WriteResult {
    let req = ctx as *mut ReplRequirer;
    let path = unsafe { (*req).vfs.get_absolute_file_path() };
    write(Some(path), buffer, buffer_size, size_out)
}
