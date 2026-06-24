use core::ffi::c_void;

pub unsafe extern "C" fn userdata_alignment_dtor(_data: *mut c_void) {}
