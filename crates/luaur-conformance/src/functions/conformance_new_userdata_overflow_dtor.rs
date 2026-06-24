use std::ffi::c_void;

pub unsafe extern "C" fn conformance_new_userdata_overflow_dtor(_data: *mut c_void) {}
