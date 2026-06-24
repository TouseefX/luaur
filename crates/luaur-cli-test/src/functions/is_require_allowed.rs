use core::ffi::c_char;

#[allow(non_snake_case)]
pub unsafe extern "C" fn is_require_allowed(
    _l: *mut core::ffi::c_void,
    _ctx: *mut core::ffi::c_void,
    requirer_chunkname: *const c_char,
) -> bool {
    let chunkname = unsafe {
        if requirer_chunkname.is_null() {
            return false;
        }
        core::ffi::CStr::from_ptr(requirer_chunkname).to_bytes()
    };

    if chunkname == b"=stdin" {
        true
    } else if !chunkname.is_empty() && chunkname[0] == b'@' {
        true
    } else {
        false
    }
}
