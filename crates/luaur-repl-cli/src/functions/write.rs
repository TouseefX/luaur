use core::ffi::c_char;

#[repr(C)]
pub enum luarequire_WriteResult {
    WRITE_SUCCESS = 0,
    WRITE_BUFFER_TOO_SMALL = 1,
    WRITE_FAILURE = 2,
}

pub unsafe fn write(
    contents: *const core::ffi::c_void,
    buffer: *mut c_char,
    buffer_size: usize,
    size_out: *mut usize,
) -> luarequire_WriteResult {
    if contents.is_null() {
        return luarequire_WriteResult::WRITE_FAILURE;
    }

    let s = contents as *const alloc::string::String;
    let contents = &*s;

    let null_terminated_size = contents.len() + 1;

    if buffer_size < null_terminated_size {
        if !size_out.is_null() {
            *size_out = null_terminated_size;
        }
        return luarequire_WriteResult::WRITE_BUFFER_TOO_SMALL;
    }

    if !buffer.is_null() {
        let src = contents.as_bytes();
        let dst = core::slice::from_raw_parts_mut(buffer as *mut u8, null_terminated_size);
        dst[..src.len()].copy_from_slice(src);
        dst[src.len()] = 0;
    }

    if !size_out.is_null() {
        *size_out = null_terminated_size;
    }

    luarequire_WriteResult::WRITE_SUCCESS
}
